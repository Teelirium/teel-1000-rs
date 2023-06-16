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

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            return Some("Not in a voice channel".to_string());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        return Some("Already deafened".to_string());

    } else {
        if let Err(e) = handler.deafen(true).await {
            return Some(format!("Failed: {:?}", e));
        }
        
        return Some("Deafened".to_string());
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("deafen").description("Deafen the bot")
}
