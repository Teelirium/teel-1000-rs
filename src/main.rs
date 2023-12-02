use dotenv::dotenv;
use serenity::model::prelude::Activity;
use std::env;

mod commands;

// This trait adds the `register_songbird` and `register_songbird_with` methods
// to the client builder below, making it easy to install this voice client.
// The voice client can be retrieved in any command using `songbird::get(ctx).await`.
use songbird::SerenityInit;

// Import the `Context` to handle commands.
use serenity::client::Context;

use serenity::{
    async_trait,
    client::{Client, EventHandler},
    model::{application::command::Command, application::interaction::Interaction, gateway::Ready},
    prelude::GatewayIntents,
};

pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::competing("rust btw")).await;

        Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        })
        .await
        .expect("Couldn't create command");

        Command::create_global_application_command(&ctx.http, |command| {
            commands::join::register(command)
        })
        .await
        .expect("Couldn't create command");

        Command::create_global_application_command(&ctx.http, |command| {
            commands::leave::register(command)
        })
        .await
        .expect("Couldn't create command");

        Command::create_global_application_command(&ctx.http, |command| {
            commands::play::register(command)
        })
        .await
        .expect("Couldn't create command");

        Command::create_global_application_command(&ctx.http, |command| {
            commands::mute::register(command)
        })
        .await
        .expect("Couldn't create command");

        Command::create_global_application_command(&ctx.http, |command| {
            commands::unmute::register(command)
        })
        .await
        .expect("Couldn't create command");

        Command::create_global_application_command(&ctx.http, |command| {
            commands::deafen::register(command)
        })
        .await
        .expect("Couldn't create command");

        Command::create_global_application_command(&ctx.http, |command| {
            commands::undeafen::register(command)
        })
        .await
        .expect("Couldn't create command");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(why) = command.defer(&ctx.http).await {
                println!("Failed to defer: {:?}", why)
            };
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(),
                "join" => commands::join::run(&command, &ctx).await.unwrap(),
                "leave" => commands::leave::run(&command, &ctx).await.unwrap(),
                "play" => commands::play::run(&command, &ctx).await.unwrap(),
                // "mute" => commands::mute::run(&command.data.options, &ctx, &interaction)
                //     .await
                //     .unwrap(),
                // "unmute" => commands::unmute::run(&command.data.options, &ctx, &interaction)
                //     .await
                //     .unwrap(),
                // "deafen" => commands::deafen::run(&command.data.options, &ctx, &interaction)
                //     .await
                //     .unwrap(),
                // "undeafen" => commands::undeafen::run(&command.data.options, &ctx, &interaction)
                //     .await
                //     .unwrap(),
                _ => "Not implemented :(".to_string(),
            };

            if let Err(why) = command
                .edit_original_interaction_response(&ctx.http, |msg| msg.content(content.clone()))
                .await
            {
                println!("Cannot respond to slash command: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let handler = Handler {};

    // specify intents
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .register_songbird()
        .await
        .expect("Error creating client");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl-C");

    println!("Received Ctrl-C, shutting down.");
}
