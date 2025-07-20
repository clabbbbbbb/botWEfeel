# howWEfeel Discord Bot

A Discord bot that uses [howWEfeel](https://mehrezat.com/howWEfeel/home.html).

## Local development

1. `git clone https://github.com/clabbbbbbb/hWf-bot.git`
2. Set the `DISCORD_TOKEN` environment variable to your bot's token in https://discord.com/developers/applications.
3. `cargo run`
4. Profit

Or try it with Docker:
2a. `docker build -t botwefeel:latest .`
3a. `docker run --name='botwefeel' -e DISCORD_TOKEN='your_discord_token_here' botwefeel:latest`

## Roadmap

- [ ] Obviously add a chat bot
- [ ] probably refactor the code
