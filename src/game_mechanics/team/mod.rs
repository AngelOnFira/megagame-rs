use async_trait::async_trait;
use serenity::{
    all::ReactionType,
    builder::{
        CreateButton, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
        CreateSelectMenuOption,
    },
    model::prelude::{ChannelType, RoleId},
    utils::MessageBuilder,
};

use crate::{
    db_wrapper::{DBWrapper, TaskResult, TaskReturnData},
    task_runner::tasks::{
        category::{CategoryHandler, CategoryTasks},
        channel::{ChannelCreateData, ChannelHandler, ChannelTasks},
        message::{
            message_component::MessageComponent, MessageHandler, MessageTasks, SendChannelMessage,
        },
        role::{CreateRoleTasks, RoleHandler, RoleTasks},
        DiscordId, TaskType,
    },
};

use super::MechanicHandler;

pub struct TeamMechanicsHandler {
    pub guild_id: u64,
    pub task: TeamJobs,
}

pub enum TeamJobs {
    CreateTeam { name: String },
    AddPlayerToTeam,
    RemovePlayerFromTeam,
    DeleteTeam,
}

#[async_trait]
impl MechanicHandler for TeamMechanicsHandler {
    async fn handle(&self, db: DBWrapper) {
        match &self.task {
            TeamJobs::CreateTeam { name } => self.create_team(name, db).await,
            TeamJobs::AddPlayerToTeam => self.add_player_to_team(db).await,
            TeamJobs::RemovePlayerFromTeam => self.remove_player_from_team(db).await,
            TeamJobs::DeleteTeam => self.delete_team(db).await,
        }
    }
}

impl TeamMechanicsHandler {
    async fn create_team(&self, name: &String, db: DBWrapper) {
        // Add the team to the database

        // Create the role
        let role_create_status = db
            .add_await_task(TaskType::RoleHandler(RoleHandler {
                guild_id: DiscordId(self.guild_id),
                task: RoleTasks::Create(CreateRoleTasks::Role {
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
        let category_create_status = db
            .add_await_task(TaskType::CategoryHandler(CategoryHandler {
                guild_id: DiscordId(self.guild_id),
                task: CategoryTasks::Create { name: name.clone() },
            }))
            .await;

        let category_model = match category_create_status {
            TaskResult::Completed(TaskReturnData::CategoryModel(category_model)) => category_model,
            _ => panic!("Category not created"),
        };

        // Create the team channel
        let channel_create_status = db
            .add_await_task(TaskType::ChannelHandler(ChannelHandler {
                guild_id: DiscordId(self.guild_id),
                task: ChannelTasks::Create(ChannelCreateData {
                    name: name.clone(),
                    category_id: Some(DiscordId(category_model.discord_id.parse().unwrap())),
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
        let _message_create_status = db
            .add_await_task(TaskType::MessageHandler(MessageHandler {
                guild_id: DiscordId(self.guild_id),
                task: MessageTasks::SendChannelMessage(SendChannelMessage {
                    channel_id: DiscordId(channel_model.discord_id.parse().unwrap()),
                    message: MessageBuilder::new()
                        .push("Welcome to the team ")
                        .mention(&RoleId(DiscordId::from(&role_model.discord_id).into()))
                        .push("!")
                        .build(),
                    select_menu: None,
                    buttons: Vec::new(),
                }),
            }))
            .await;

        fn sound_button(name: &str, emoji: ReactionType) -> CreateButton {
            // To add an emoji to buttons, use .emoji(). The method accepts anything ReactionType or
            // anything that can be converted to it. For a list of that, search Trait Implementations in the
            // docs for From<...>.
            CreateButton::new(name)
        }

        // Add a team menu to the team channel
        let _message_create_status = db
            .add_await_task(TaskType::MessageHandler(MessageHandler {
                guild_id: DiscordId(self.guild_id),
                task: MessageTasks::SendChannelMessage(SendChannelMessage {
                    channel_id: DiscordId(channel_model.discord_id.parse().unwrap()),
                    message: MessageBuilder::new()
                        .push("Welcome to the team ")
                        .mention(&RoleId(DiscordId::from(&role_model.discord_id).into()))
                        .push("!")
                        .build(),
                    select_menu: Some(
                        CreateSelectMenu::new(
                            "animal_select",
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
                        .custom_id("animal_select")
                        .placeholder("No animal selected"),
                    ),
                    buttons: vec![MessageComponent::<CreateButton>::new(None, db.clone())
                        .await
                        .component
                        .emoji("🐕".parse::<ReactionType>().unwrap())],
                }),
            }))
            .await;
    }

    async fn add_player_to_team(&self, _db: DBWrapper) {
        // Add the player to the team
    }

    async fn remove_player_from_team(&self, _db: DBWrapper) {
        // Remove the player from the team
    }

    async fn delete_team(&self, _db: DBWrapper) {
        // Delete the team from the database
    }
}
