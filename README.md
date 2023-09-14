# <p align="center">HackerNews Alert</p>
<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/createByTemplate/hacker-news-chatgpt-telegram">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

Feel hard to find the post that you're interested in tons of Hacker News posts? This telegram bot can help you monitor the Hacker News Post you're interested in and give you a summary generated ChatGPT as a telegram message. 

<img width="658" alt="image" src="https://github.com/flows-network/hacker-news-alert-chatgpt-telegram/assets/45785633/82b4b004-e67d-4689-a0de-802c2db8adce">

## How it works

This scheduled bot uses ChatGPT to summarize Hacker News posts. At the specified time, the bot searches for posts from the past hour, filters them based on your chosen keyword, and sends you a Telegram message with a summary.


## Deploy

1. Create a bot from the template
2. Add your OpenAI API key
3. Configure the Telegram bot to send direct messages (personal chat ID) or group message

### 0 Prerequisites

* You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

* Sign up on [flows.network](https://flows.network/) using your GitHub account. It is free.

### 1 Create a bot from a template

Go to [the Hacker News Alert ChatGPT Telegram template](https://flows.network/flow/createByTemplate/hacker-news-chatgpt-telegram).


<img width="658" alt="image" src="https://github.com/flows-network/hacker-news-alert-chatgpt-telegram/assets/45785633/8d5e9862-5bce-40ce-b1a2-07f53df2d525">


Review the `KEYWORD` variable to specify your keyword of interest (supporting only one keyword for each bot).

Click on the **Create and Build** button.

### 2 Add your OpenAI API key

Set up the OpenAI integration. Click on **Connect**, and enter your key. The default key name is `Default`.

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">](https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png)

Close the tab and go back to the flow.network page once you are done. Finally, click **Deploy**.

###  Configure the Telegram bot to send direct messages (personal chat ID) or group message

Set up the Telegram integration. You will need to

1. Add your Telegram API token, which you can get from @botfather
2. Add your Telegram Chat ID. The chat ID you use depends on whether you want the bot to send direct messages or group messages. Click [here](https://flows.network/blog/how-to-find-telegram-chat-id) to learn more about finding your Telegram chat ID.

<img width="658" alt="image" src="https://github.com/flows-network/hacker-news-alert-chatgpt-telegram/assets/45785633/1b6fcb91-4485-4270-bdc3-e4549d929ac5">

Finally, click **Deploy**.

## Wait for the magic!

You are now on the flow details page and the flow function takes a few seconds to build. Once the flow's status changes to `running`, your bot is ready to summarize Hacker News posts.

## FAQ

### How to customize the bot's scheduled messaging time?

To customize the time when the bot sends Discord messages, you can modify the value in the cron expression ("30 * * * *"). This expression means the bot sends messages at the 30th minute of every hour.

```
    schedule_cron_job(String::from("30 * * * *"), keyword, callback).await;
```

To adjust the timing, you can change the number 30 to your desired minute. For example, if you want the messages to be sent at the 15th minute of every hour, you can modify the expression to be ("15 * * * *").

By customizing the cron expression, you can set the desired timing for the bot to send Discord messages.







