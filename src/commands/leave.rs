use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub async fn run(cmd: &ApplicationCommandInteraction, ctx: &Context) -> Option<String> {
    let guild = ctx.cache.guild(cmd.guild_id.unwrap()).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            return Some(format!("Failed: {:?}", e));
        }

        return Some("Left the server.".to_string());
    } else {
        return Some("Not in a voice channel".to_string());
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leave")
        .description("Manually tell the bot to leave a voice call")
}
