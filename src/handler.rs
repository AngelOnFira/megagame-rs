use std::env;

use serenity::{
    async_trait,
    model::{
        application::{
            command::Command,
            interaction::{Interaction, InteractionResponseType},
        },
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};

use crate::commands::{self, fake_trade};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "trade" => commands::fake_trade::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for guild in ctx.cache.guilds().iter() {
            let commands = GuildId(guild.0)
                .set_application_commands(&ctx.http, |commands| {
                    commands.create_application_command(|command| {
                        commands::fake_trade::register(command)
                    })
                })
                .await;
        }
    }
}
