use async_trait::async_trait;
use entity::entities::team;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use serenity::{
    all::{ButtonStyle, ReactionType},
    builder::{CreateButton, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption},
    model::prelude::{ChannelType, RoleId},
    utils::MessageBuilder,
};

use crate::{
    db_wrapper::{helpers::get_guild, TaskResult, TaskReturnData},
    game_mechanics::{
        menu::{MenuJobs, MenuMechanicsHandler},
        MechanicFunction,
    },
    task_runner::tasks::{
        category::{CategoryHandler, CategoryTasks},
        channel::{ChannelCreateData, ChannelHandler, ChannelTasks},
        message::{
            message_component::{MessageComponent, MessageData},
            MessageHandler, MessageTasks, SendChannelMessage,
        },
        role::{CreateRole, RoleHandler, RoleTasks},
        DiscordId, TaskType,
    },
};

use super::{MechanicHandler, MechanicHandlerWrapper};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamMechanicsHandler {
    pub guild_id: DiscordId,
    pub task: TeamJobs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TeamJobs {
    CreateTeam { name: String },
    AddPlayerToTeam,
    RemovePlayerFromTeam,
    DeleteTeam,
}

#[async_trait]
impl MechanicHandler for TeamMechanicsHandler {
    async fn handle(&self, handler: MechanicHandlerWrapper) {
        match &self.task {
            TeamJobs::CreateTeam { name } => self.create_team(handler, name).await,
            TeamJobs::AddPlayerToTeam => self.add_player_to_team(handler).await,
            TeamJobs::RemovePlayerFromTeam => self.remove_player_from_team(handler).await,
            TeamJobs::DeleteTeam => self.delete_team(handler).await,
        }
    }
}

impl TeamMechanicsHandler {
    async fn create_team(&self, handler: MechanicHandlerWrapper, name: &String) {
        // Get the guild
        let (_discord_guild, database_guild) =
            get_guild(handler.ctx, handler.db.clone(), self.guild_id).await;

        // Create the role
        let role_create_status = handler
            .db
            .add_await_task(TaskType::RoleHandler(RoleHandler {
                guild_id: DiscordId(*self.guild_id),
                task: RoleTasks::CreateRole(CreateRole {
                    name: name.clone(),
                    color: 0x00ff00,
                }),
            }))
            .await;

        let role_model = match role_create_status {
            TaskResult::Completed(TaskReturnData::RoleModel(role_model)) => role_model,
            _ => panic!("Role not created"),
        };

        // Create the team category
        let category_create_status = handler
            .db
            .add_await_task(TaskType::CategoryHandler(CategoryHandler {
                guild_id: DiscordId(*self.guild_id),
                task: CategoryTasks::Create { name: name.clone() },
            }))
            .await;

        let category_model = match category_create_status {
            TaskResult::Completed(TaskReturnData::CategoryModel(category_model)) => category_model,
            _ => panic!("Category not created"),
        };

        // Create the team channel
        let channel_create_status = handler
            .db
            .add_await_task(TaskType::ChannelHandler(ChannelHandler {
                guild_id: DiscordId(*self.guild_id),
                task: ChannelTasks::Create(ChannelCreateData {
                    name: name.clone(),
                    category_id: Some(DiscordId::from(category_model.discord_id)),
                    kind: ChannelType::Text,
                }),
            }))
            .await;

        let channel_model = match channel_create_status {
            TaskResult::Completed(TaskReturnData::ChannelModel(channel_model)) => channel_model,
            _ => panic!("Channel not created"),
        };

        // Write a message in the team channel that pings the role of the
        // players
        let _message_create_status = handler
            .db
            .add_await_task(TaskType::MessageHandler(MessageHandler {
                guild_id: DiscordId(*self.guild_id),
                task: MessageTasks::SendChannelMessage(SendChannelMessage {
                    channel_id: DiscordId::from(channel_model.discord_id),
                    message: MessageBuilder::new()
                        .push("Welcome to the team ")
                        .mention(&RoleId(DiscordId::from(role_model.discord_id).into()))
                        .push("!")
                        .build(),
                    ..Default::default()
                }),
            }))
            .await;

        fn sound_button(name: &str, _emoji: ReactionType) -> CreateButton {
            // To add an emoji to buttons, use .emoji(). The method accepts anything ReactionType or
            // anything that can be converted to it. For a list of that, search Trait Implementations in the
            // docs for From<...>.
            CreateButton::new(name)
        }

        // Add a team menu to the team channel
        let _message_create_status = handler
            .db
            .add_await_task(TaskType::MessageHandler(MessageHandler {
                guild_id: DiscordId::from(self.guild_id),
                task: MessageTasks::SendChannelMessage(SendChannelMessage {
                    channel_id: DiscordId::from(channel_model.discord_id),
                    message: MessageBuilder::new()
                        .push("Welcome to the team ")
                        .mention(&RoleId(DiscordId::from(role_model.discord_id).into()))
                        .push("!")
                        .build(),
                    select_menu: Some(MessageComponent::new(
                        CreateSelectMenu::new(
                            "",
                            CreateSelectMenuKind::String {
                                options: vec![
                                    CreateSelectMenuOption::new("🐈 meow", "Cat"),
                                    CreateSelectMenuOption::new("🐕 woof", "Dog"),
                                    CreateSelectMenuOption::new("🐎 neigh", "Horse"),
                                    CreateSelectMenuOption::new("🦙 hoooooooonk", "Alpaca"),
                                    CreateSelectMenuOption::new("🦀 crab rave", "Ferris"),
                                ],
                            },
                        )
                        .placeholder("No animal selected"),
                        None,
                    )),
                    buttons: vec![
                        MessageComponent::new(
                            CreateButton::new("")
                                .style(ButtonStyle::Primary)
                                .disabled(false)
                                .label("Start Trade")
                                .emoji("💱".parse::<ReactionType>().unwrap()),
                            Some(MessageData::Function(MechanicFunction::Menu(
                                MenuMechanicsHandler {
                                    guild_id: self.guild_id,
                                    task: MenuJobs::StartTradeMenu {
                                        channel_id: DiscordId::from(channel_model.discord_id),
                                    },
                                },
                            ))),
                        ),
                        MessageComponent::new(
                            CreateButton::new("")
                                .style(ButtonStyle::Primary)
                                .disabled(false)
                                .label("Open Comms")
                                .emoji("💬".parse::<ReactionType>().unwrap()),
                            Some(MessageData::Function(MechanicFunction::Menu(
                                MenuMechanicsHandler {
                                    guild_id: self.guild_id,
                                    task: MenuJobs::OpenComms {
                                        channel_id: DiscordId::from(channel_model.discord_id),
                                    },
                                },
                            ))),
                        ),
                        MessageComponent::new(
                            CreateButton::new("")
                                .style(ButtonStyle::Primary)
                                .disabled(false)
                                .label("Update Bank")
                                .emoji("💰".parse::<ReactionType>().unwrap()),
                            None,
                        ),
                        MessageComponent::new(
                            CreateButton::new("")
                                .style(ButtonStyle::Primary)
                                .disabled(false)
                                .label("Join Team")
                                .emoji("👋".parse::<ReactionType>().unwrap()),
                            Some(MessageData::Function(MechanicFunction::Menu(
                                MenuMechanicsHandler {
                                    guild_id: self.guild_id,
                                    task: MenuJobs::JoinTeam {
                                        channel_id: DiscordId::from(channel_model.discord_id),
                                    },
                                },
                            ))),
                        ),
                    ],
                }),
            }))
            .await;

        // Add the team to the database
        let _team_model = team::ActiveModel {
            // id: todo!(),
            // name: todo!(),
            // abreviation: todo!(),
            // guild: todo!(),
            // created_at: todo!(),
            // updated_at: todo!(),
            // emoji: todo!(),
            // wallet: todo!(),
            // role: todo!(),
            // category_id: todo!(),
            // general_channel_id: todo!(),
            // trade_channel_id: todo!(),
            // bank_embed_id: todo!(),
            name: Set(name.clone()),
            abreviation: Set(None),
            created_at: Set(None),
            fk_guild_id: Set(database_guild.discord_id),
            fk_team_role_id: Set(Some(role_model.discord_id)),
            fk_menu_channel_id: Set(Some(channel_model.discord_id)),
            ..Default::default()
        }
        .insert(&*handler.db)
        .await
        .unwrap();
    }

    async fn add_player_to_team(&self, _handler: MechanicHandlerWrapper) {
        // Add the player to the team
    }

    async fn remove_player_from_team(&self, _handler: MechanicHandlerWrapper) {
        // Remove the player from the team
    }

    async fn delete_team(&self, _handler: MechanicHandlerWrapper) {
        // Delete the team from the database
    }
}
