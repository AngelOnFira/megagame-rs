use async_trait::async_trait;
use serenity::{
    all::{CommandOptionType, GuildId, ResolvedOption},
    builder::{CreateCommand, CreateCommandOption},
    prelude::Context,
};

use crate::{
    db_wrapper::DBWrapper,
    task_runner::tasks::{
        channel::{ChannelHandler, ChannelTasks},
        DatabaseId, DiscordId, TaskType,
    },
};

use super::GameCommand;

pub struct Nuke;

#[async_trait]
impl GameCommand for Nuke {
    fn register() -> CreateCommand {
        CreateCommand::new("reset")
            .description("Nuke the server")
            .add_option(CreateCommandOption::new(
                CommandOptionType::String,
                "password",
                "Confirm that you want to nuke the server",
            ))
    }

    async fn run(
        _options: &[ResolvedOption],
        guild_id: GuildId,
        db: DBWrapper,
        ctx: Context,
    ) -> String {
        // // Check the password
        // if let ResolvedValue::String(s) = options
        //     .iter()
        //     .find(|option| option.name == "password")
        //     .unwrap()
        //     .value
        // {
        //     if s != "nuke" {
        //         return "Invalid password".to_string();
        //     }
        // } else {
        //     return "Invalid password".to_string();
        // };

        let mut tasks: Vec<DatabaseId> = vec![];

        // Delete every role possible

        // Get all the roles
        let _roles = ctx
            .http
            .get_guild_roles(guild_id)
            .await
            .expect("Failed to get roles");

        // // Queue up all the deletions
        // for role in roles {
        //     tasks.push(
        //         db.add_task(TaskType::RoleHandler(RoleHandler {
        //             task: RoleTasks::DeleteRole(DeleteRole {
        //                 role_id: DiscordId::from(role.id),
        //             }),
        //             guild_id: DiscordId::from(guild_id),
        //         }))
        //         .await,
        //     );
        // }

        // Delete every channel possible

        // Get all the text channels/voice channels/categories
        let channels = ctx
            .http
            .get_channels(guild_id)
            .await
            .expect("Failed to get channels");

        // Queue up all the deletions
        for channel in channels {
            tasks.push(
                db.add_task(TaskType::ChannelHandler(ChannelHandler {
                    task: ChannelTasks::Delete {
                        id: DiscordId::from(channel.id),
                    },
                    guild_id: DiscordId::from(guild_id),
                }))
                .await,
            );
        }

        // Wait for all the tasks to finish
        for task in tasks {
            db.await_task(task).await;
        }

        "Nuked the server!".to_string()
    }
}
