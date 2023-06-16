use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        application::interaction::application_command::ApplicationCommandInteraction,
        command::CommandOptionType, interaction::application_command::CommandDataOptionValue,
    },
    prelude::Context,
};

use songbird::input;

pub async fn run(cmd: &ApplicationCommandInteraction, ctx: &Context) -> Option<String> {
    let (channel_id, guild_id) = super::helpers::get_channel_to_join(cmd, ctx);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return Some("Must be in a channel.".to_string());
        }
    };

    let option = cmd
        .data
        .options
        .get(0)
        .expect("Expected URL option")
        .resolved
        .clone()
        .expect("Expected String");

    let url: String;

    if let CommandDataOptionValue::String(_url) = option {
        println!("{}", _url);
        url = _url;
    } else {
        return Some("Please provide an argument".to_string());
    }

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handler, result) = manager.join(guild_id, connect_to).await;

    if let Err(why) = result {
        println!("Failed to join voice channel: {:#?}", why);
        return Some("Failed to join voice channel".to_string());
    }

    let mut handler_lock = handler.lock().await;

    let is_url = url.starts_with("http");
    let result = match is_url {
        true => input::ytdl(&url).await,
        false => input::ytdl_search(&url).await,
    };

    let source = match result {
        Ok(source) => source,
        Err(why) => {
            println!("Error playing source: {:#?}", why);

            return Some("Error playing source".to_string());
        }
    };
    let res_url = source.metadata.source_url.clone().unwrap();
    let title = source.metadata.title.clone().unwrap();
    let track = handler_lock.play_only_source(source);

    return Some(format!("Playing **``{}``**\n{}", title, res_url));
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Play a Youtube video")
        .create_option(|option| {
            option
                .name("input")
                .description("Youtube URL or search query")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
