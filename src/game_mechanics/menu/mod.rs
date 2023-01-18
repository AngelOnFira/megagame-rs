use async_trait::async_trait;
use entity::entities::{player, role, team};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serenity::{all::*, utils::MessageBuilder};

use crate::{
    db_wrapper::{
        helpers::{get_guild, get_or_create_player, get_player_team},
        TaskResult, TaskReturnData,
    },
    task_runner::tasks::{
        channel::{ChannelCreateData, ChannelHandler, ChannelTasks},
        message::{
            message_component::{MessageComponent, MessageData},
            MessageHandler, MessageTasks, SendChannelMessage,
        },
        role::{AddRoleToUser, RemoveRoleFromUser, RoleHandler, RoleTasks},
        DatabaseId, DiscordId, TaskType,
    },
};

use super::{MechanicFunction, MechanicHandler, MechanicHandlerWrapper};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuMechanicsHandler {
    pub guild_id: DiscordId,
    pub task: MenuJobs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MenuJobs {
    StartTradeMenu {
        channel_id: DiscordId,
    },
    RoleChangeMenu {
        team_names: Vec<String>,
    },
    OpenComms {
        channel_id: DiscordId,
    },
    /// Move a player to a team. `channel_id` is the channel that the request
    /// originated from, so that a response can be sent to the initiator.
    JoinTeam {
        /// Test
        channel_id: DiscordId,
        /// test2
        joining_team_id: DatabaseId,
    },
}

#[async_trait]
impl MechanicHandler for MenuMechanicsHandler {
    async fn handle(&self, handler: MechanicHandlerWrapper) {
        match &self.task {
            MenuJobs::StartTradeMenu { channel_id } => {
                self.start_trade_menu(handler, *channel_id).await
            }
            MenuJobs::OpenComms { channel_id } => self.open_comms(handler, *channel_id).await,
            MenuJobs::JoinTeam { channel_id, joining_team_id} => self.join_team(handler, *channel_id, *joining_team_id).await,
            MenuJobs::RoleChangeMenu { team_names } => {
                self.team_change_menu(handler, team_names.clone()).await
            }
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

        // Get the team of the interacting player
        let _database_team = get_player_team(
            handler.ctx.clone(),
            handler.db.clone(),
            self.guild_id,
            handler.interaction.unwrap().member.unwrap().user.id.into(),
        )
        .await
        .unwrap();
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

    async fn join_team(&self, handler: MechanicHandlerWrapper, channel_id: DiscordId, joining_team_id: DatabaseId) {
        // Get the guild from the database
        get_guild(handler.ctx.clone(), handler.db.clone(), self.guild_id).await;

        // Get the interacting user
        let user = handler.interaction.unwrap().member.unwrap().user;
        let user_id = DiscordId::from(user.id);

        // Get the player from the database
        let database_player = get_or_create_player(
            handler.ctx.clone(),
            handler.db.clone(),
            self.guild_id,
            user_id,
            user.name,
        )
        .await
        .unwrap();

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
        .filter(team::Column::Id.eq(*joining_team_id as i64))
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

    async fn team_change_menu(&self, handler: MechanicHandlerWrapper, team_names: Vec<String>) {
        // Get each team from the database from the name
        let mut teams = Vec::new();

        for team_name in team_names {
            let team = team::Entity::find()
                .filter(team::Column::Name.eq(team_name))
                .one(&*handler.db)
                .await
                .unwrap()
                .unwrap();

            teams.push(team);
        }

        // Make a team change channel
        let team_change_channel_status = handler
            .db
            .add_await_task(TaskType::ChannelHandler(ChannelHandler {
                task: ChannelTasks::Create(ChannelCreateData {
                    name: "team-change".to_string(),
                    category_id: None,
                    kind: ChannelType::Text,
                }),
                guild_id: self.guild_id,
            }))
            .await;

        let team_change_channel_model = match team_change_channel_status {
            TaskResult::Completed(TaskReturnData::ChannelModel(channel_model)) => channel_model,
            _ => panic!("Failed to create team change channel"),
        };

        // Create a message in the channel with buttons for each team
        // Add a team menu to the team channel
        let _message_create_status = handler
            .db
            .add_await_task(TaskType::MessageHandler(MessageHandler {
                guild_id: DiscordId::from(self.guild_id),
                task: MessageTasks::SendChannelMessage(SendChannelMessage {
                    channel_id: DiscordId::from(team_change_channel_model.discord_id),
                    message: MessageBuilder::new()
                        .push("Choose the team you'd like to join")
                        .build(),
                    select_menu: None,
                    buttons: teams
                        .iter()
                        .map(|team| {
                            MessageComponent::new(
                                CreateButton::new("")
                                    .style(ButtonStyle::Primary)
                                    .disabled(false)
                                    .label(format!("Join {}", team.name))
                                    .emoji("👋".parse::<ReactionType>().unwrap()),
                                Some(MessageData::Function(MechanicFunction::Menu(
                                    MenuMechanicsHandler {
                                        guild_id: self.guild_id,
                                        task: MenuJobs::JoinTeam {
                                            channel_id: DiscordId::from(
                                                team_change_channel_model.discord_id,
                                            ),
                                            joining_team_id: DatabaseId(team.id),
                                        },
                                    },
                                ))),
                            )
                        })
                        .collect(),
                }),
            }))
            .await;
    }
}
