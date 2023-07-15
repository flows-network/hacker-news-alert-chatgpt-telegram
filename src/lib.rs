use anyhow;
use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use http_req::{request, request::Method::POST, request::Request, uri::Uri};
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};
use schedule_flows::schedule_cron_job;
use serde::Deserialize;
use serde_json;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tg_flows::{ChatId, Method, Telegram};
use web_scraper_flows::get_page_text;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    schedule_cron_job(
        String::from("18 * * * *"),
        String::from("cronjob scheduled"),
        callback,
    )
    .await;
}

async fn callback(_: Vec<u8>) {
    dotenv().ok();
    logger::init();
    let keyword = env::var("KEYWORD").unwrap_or("ChatGPT".to_string());
    let now = SystemTime::now();
    let dura = now.duration_since(UNIX_EPOCH).unwrap().as_secs() - 10000;
    let url = format!("https://hn.algolia.com/api/v1/search_by_date?tags=story&query={keyword}&numericFilters=created_at_i>{dura}");

    let mut writer = Vec::new();
    if let Ok(_) = request::get(url, &mut writer) {
        if let Ok(search) = serde_json::from_slice::<Search>(&writer) {
            for hit in search.hits {
                let _ = send_message_wrapper(hit).await;
            }
        }
    }
}

#[derive(Deserialize)]
pub struct Search {
    pub hits: Vec<Hit>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Hit {
    pub title: String,
    pub url: Option<String>,
    #[serde(rename = "objectID")]
    pub object_id: String,
    pub author: String,
    pub created_at_i: i64,
}

async fn get_summary_truncated(inp: &str) -> anyhow::Result<String> {
    let mut openai = OpenAIFlows::new();
    openai.set_retry_times(3);

    let news_body = inp
        .split_whitespace()
        .take(10000)
        .collect::<Vec<&str>>()
        .join(" ");

    let chat_id = format!("summary#99");
    let system = &format!("You're an AI assistant.");

    let co = ChatOptions {
        model: ChatModel::GPT35Turbo16K,
        restart: true,
        system_prompt: Some(system),
        max_tokens: Some(128),
        temperature: Some(0.8),
        ..Default::default()
    };

    let question = format!("summarize this within 100 words: {news_body}");

    match openai.chat_completion(&chat_id, &question, &co).await {
        Ok(r) => Ok(r.choice),
        Err(_e) => Err(anyhow::Error::msg(_e.to_string())),
    }
}

pub async fn send_message_wrapper(hit: Hit) -> anyhow::Result<()> {
    logger::init();
    let telegram_token = env::var("telegram_token").expect("Missing telegram_token");
    let tele = Telegram::new(telegram_token.clone());
    let username = env::var("username").unwrap_or("jaykchen".to_string());
    let body = serde_json::json!({ "chat_id": format!("@{}", username) });

    // let result: Value = tele.request(Method::GetChat, body.to_string().as_bytes())?;
    let chat_id = 2142063265;

    // match result.get("id") {
    //     Some(id) => {
    //         log::info!("result: {}", id.to_string());
    //         let _ = tele.send_message(ChatId(chat_id), id.to_string());
    //     }
    //     None => {
    //         log::info!("id not found");
    //         let _ = tele.send_message(ChatId(chat_id), "id not found");
    //     }
    // };

    let _ = tele.send_message(ChatId(chat_id), "hi");

    // let chat_id = result
    //     .get("id")
    //     .ok_or(anyhow::anyhow!("No 'id' field in the response"))?
    //     .as_i64()
    //     .ok_or(anyhow::anyhow!("Failed to convert 'id' to i64"))?;

    let title = &hit.title;
    let author = &hit.author;
    let post = format!("https://news.ycombinator.com/item?id={}", &hit.object_id);
    let mut inner_url = "".to_string();

    let _text = match &hit.url {
        Some(u) => {
            inner_url = u.clone();
            get_page_text(u)
                .await
                .unwrap_or("failed to scrape text with hit url".to_string())
        }
        None => get_page_text(&post)
            .await
            .unwrap_or("failed to scrape text with post url".to_string()),
    };

    let summary = if _text.split_whitespace().count() > 100 {
        get_summary_truncated(&_text).await?
    } else {
        format!("Bot found minimal info on webpage to warrant a summary, please see the text on the page the Bot grabbed below if there are any, or use the link above to see the news at its source:\n{_text}")
    };

    let source = if !inner_url.is_empty() {
        format!("<{inner_url}|source>")
    } else {
        "".to_string()
    };

    let msg = format!("- <{post}|*{title}*>\n{source} by {author}\n{summary}");
    // let _ = tele.send_message(ChatId(chat_id), msg);

    let uri = format!("https://api.telegram.org/bot{telegram_token}/sendMessage");

    let uri = Uri::try_from(uri.as_str()).unwrap();
    let mut writer = Vec::new();
    let params = serde_json::json!({
      "chat_id": chat_id,
      "text": msg,
      "parse_mode": "Markdown"
    });
    let params = serde_json::json!({
      "chat_id": chat_id,
      "text": "placeholder message",
      "parse_mode": "Markdown"
    });

    let body = serde_json::to_vec(&params)?;

    let _ = Request::new(&uri)
        .method(POST)
        .header("Content-Type", "application/json")
        .header("Content-Length", &body.len())
        .body(&body)
        .send(&mut writer)?;

    Ok(())
}
