# Discord Youtube Player

## A Discord bot written in Rust that plays Youtube audio in voice channels

## Dependencies
This bot uses the Serenity and its voice extension Songbird. In order for
the bot to work one must have Opus, FFmpeg, and youtube-dl installed.

## Environment
The bot also requires a valid Discord bot token to be specified in a `.env`
as `BOT_TOKEN`.

## Running
Once the above requirements are met run the code normally, e.g. `cargo run`

## Usage
The bot has the following commands:
* `/deafen`: Deafens the bot
* `/join`: Manually tell the bot to join a voice call
* `/leave`: Manually tell the bot to leave a voice call
* `/mute`: Mute the bot
* `/ping`: A ping command
* `/play <url>`: Play a Youtube video
* `/undeafen`: Undeafen the bot
* `/unmute`: Unmute the bot