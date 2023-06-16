use serenity::{
    model::prelude::{
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOption},
            Interaction,
        },
        ChannelId, GuildId,
    },
    prelude::Context,
};

pub fn get_channel_to_join(
    cmd: &ApplicationCommandInteraction,
    ctx: &Context,
) -> (Option<ChannelId>, GuildId) {
    let user_id = cmd.member.clone().unwrap().user.id;
    let guild = ctx.cache.guild(cmd.guild_id.unwrap()).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&user_id)
        .and_then(|voice_state| voice_state.channel_id);

    (channel_id, guild_id)
}
