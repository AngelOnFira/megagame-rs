use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::{
    all::{ButtonStyle, ReactionType},
    builder::{CreateButton, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption},
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuMechanicsHandler {
    pub guild_id: u64,
    pub task: MenuJobs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MenuJobs {
    StartTradeMenu { channel_id: DiscordId },
}

#[async_trait]
impl MechanicHandler for MenuMechanicsHandler {
    async fn handle(&self, db: DBWrapper) {
        match &self.task {
            MenuJobs::StartTradeMenu { channel_id } => self.start_trade_menu(db, *channel_id).await,
        }
    }
}

impl MenuMechanicsHandler {
    async fn start_trade_menu(&self, db: DBWrapper, channel_id: DiscordId) {
        // Send a message to the channel
        let _message_create_status = db
            .add_await_task(TaskType::MessageHandler(MessageHandler {
                guild_id: DiscordId(self.guild_id),
                task: MessageTasks::SendChannelMessage(SendChannelMessage {
                    channel_id,
                    message: MessageBuilder::new().push("Trade started!").build(),
                    select_menu: None,
                    buttons: Vec::new(),
                }),
            }))
            .await;
    }
}
