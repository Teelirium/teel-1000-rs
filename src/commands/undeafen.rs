use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::{
        application_command::CommandDataOption,
        Interaction
    },
    prelude::Context
};

pub async fn run(_options: &[CommandDataOption], ctx: &Context, interaction: &Interaction) -> Option<String> {
    let guild_id = interaction.clone().application_command().unwrap().guild_id.unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.deafen(false).await {
            return Some(format!("Failed: {:?}", e));
        }

        return Some("Undeafened".to_string());
    } else {
        return Some("Not in a voice channel".to_string());

    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("undeafen").description("Undeafen the bot")
}
