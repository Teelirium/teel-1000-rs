use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub async fn run(cmd: &ApplicationCommandInteraction, ctx: &Context) -> Option<String> {
    let (channel_id, guild_id) = super::helpers::get_channel_to_join(cmd, ctx);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return Some("Must be in a channel.".to_string());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Some("Joined channel.".to_string())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("join")
        .description("Manually tell the bot to join a voice call")
}
