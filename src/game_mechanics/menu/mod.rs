use async_trait::async_trait;
use entity::entities::{player, role, team};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serenity::utils::MessageBuilder;

use crate::{
    db_wrapper::helpers::get_guild,
    task_runner::tasks::{
        message::{MessageHandler, MessageTasks, SendChannelMessage},
        role::{AddRoleToUser, RemoveRoleFromUser, RoleHandler, RoleTasks},
        DiscordId, TaskType,
    },
};

use super::{MechanicHandler, MechanicHandlerWrapper};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuMechanicsHandler {
    pub guild_id: DiscordId,
    pub task: MenuJobs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MenuJobs {
    StartTradeMenu { channel_id: DiscordId },
    OpenComms { channel_id: DiscordId },
    JoinTeam { channel_id: DiscordId },
}

#[async_trait]
impl MechanicHandler for MenuMechanicsHandler {
    async fn handle(&self, handler: MechanicHandlerWrapper) {
        match &self.task {
            MenuJobs::StartTradeMenu { channel_id } => {
                self.start_trade_menu(handler, *channel_id).await
            }
            MenuJobs::OpenComms { channel_id } => self.open_comms(handler, *channel_id).await,
            MenuJobs::JoinTeam { channel_id } => self.join_team(handler, *channel_id).await,
        }
    }
}

impl MenuMechanicsHandler {
    async fn start_trade_menu(&self, handler: MechanicHandlerWrapper, channel_id: DiscordId) {
        // Send a message to the channel
        let _message_create_status = handler
            .db
            .add_await_task(TaskType::MessageHandler(MessageHandler {
                guild_id: self.guild_id,
                task: MessageTasks::SendChannelMessage(SendChannelMessage {
                    channel_id,
                    message: MessageBuilder::new().push("Trade started!").build(),
                    select_menu: None,
                    buttons: Vec::new(),
                }),
            }))
            .await;
    }

    async fn open_comms(&self, handler: MechanicHandlerWrapper, channel_id: DiscordId) {
        // Get the team of the interacting player
        let discord_user_id: DiscordId =
            handler.interaction.unwrap().member.unwrap().user.id.into();

        // Get the player from the database
        let _player = player::Entity::find()
            .filter(player::Column::DiscordId.eq(*discord_user_id as i64))
            .one(&*handler.db)
            .await
            .unwrap()
            .unwrap();

        // Send a message to the channel
        let _message_create_status = handler
            .db
            .add_await_task(TaskType::MessageHandler(MessageHandler {
                guild_id: self.guild_id,
                task: MessageTasks::SendChannelMessage(SendChannelMessage {
                    channel_id,
                    message: MessageBuilder::new().push("Comms opened!").build(),
                    select_menu: None,
                    buttons: Vec::new(),
                }),
            }))
            .await;
    }

    async fn join_team(&self, handler: MechanicHandlerWrapper, channel_id: DiscordId) {
        // Get the guild from the database
        get_guild(handler.ctx.clone(), handler.db.clone(), self.guild_id).await;

        // Get the interacting user
        let user = handler.interaction.unwrap().member.unwrap().user;
        let user_id = DiscordId::from(user.id);

        // Get the player from the database or create it if it doesn't exist
        let player_option = player::Entity::find()
            .filter(player::Column::DiscordId.eq(*user_id as i64))
            .one(&*handler.db)
            .await
            .unwrap();

        let database_player = match player_option {
            Some(player) => player,
            None => player::ActiveModel {
                discord_id: Set(*user_id as i64),
                fk_guild_id: Set(*self.guild_id as i64),
                name: Set(user.name),
                ..Default::default()
            }
            .insert(&*handler.db)
            .await
            .unwrap(),
        };

        // If the player had a team, remove the role from them
        if let Some(team_id) = database_player.fk_team_id {
            // Get the team from the database
            let team = team::Entity::find_by_id(team_id)
                .one(&*handler.db)
                .await
                .unwrap()
                .unwrap();

            // Get the team's role from the database
            let team_role = role::Entity::find_by_id(team.fk_team_role_id.unwrap())
                .one(&*handler.db)
                .await
                .unwrap()
                .unwrap();

            // Remove the role from the player
            let _role_remove_status = handler
                .db
                .add_await_task(TaskType::RoleHandler(RoleHandler {
                    guild_id: self.guild_id,
                    task: RoleTasks::RemoveRoleFromUser(RemoveRoleFromUser {
                        user_id,
                        role_id: DiscordId::from(team_role.discord_id),
                    }),
                }))
                .await;
        }

        // Get the team from the database
        let team_database = team::Entity::find()
            .filter(team::Column::FkMenuChannelId.eq(Some(*channel_id as i64)))
            .one(&*handler.db)
            .await
            .unwrap()
            .unwrap();

        // Get the team's role from the database
        let team_role_database = role::Entity::find_by_id(team_database.fk_team_role_id.unwrap())
            .one(&*handler.db)
            .await
            .unwrap()
            .unwrap();

        // Add the role to the player
        let _role_add_status = handler
            .db
            .add_await_task(TaskType::RoleHandler(RoleHandler {
                guild_id: self.guild_id,
                task: RoleTasks::AddRoleToUser(AddRoleToUser {
                    user_id,
                    role_id: DiscordId::from(team_role_database.discord_id),
                }),
            }))
            .await;

        // Send a message to the channel
        // @<role> you have a new member, <player>!
        let _message_create_status = handler
            .db
            .add_await_task(TaskType::MessageHandler(MessageHandler {
                guild_id: self.guild_id,
                task: MessageTasks::SendChannelMessage(SendChannelMessage {
                    channel_id,
                    message: MessageBuilder::new().push("Comms opened!").build(),
                    select_menu: None,
                    buttons: Vec::new(),
                }),
            }))
            .await;
    }
}
