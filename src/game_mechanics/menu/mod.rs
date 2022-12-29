use async_trait::async_trait;
use entity::entities::player;
use serde::{Deserialize, Serialize};
use serenity::{all::ComponentInteraction, utils::MessageBuilder};

use crate::{
    db_wrapper::DBWrapper,
    task_runner::tasks::{
        message::{MessageHandler, MessageTasks, SendChannelMessage},
        DiscordId, TaskType,
    },
};

use super::{MechanicHandler, MechanicHandlerWrapper};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuMechanicsHandler {
    pub guild_id: u64,
    pub task: MenuJobs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MenuJobs {
    StartTradeMenu { channel_id: DiscordId },
    OpenComms { channel_id: DiscordId },
}

#[async_trait]
impl MechanicHandler for MenuMechanicsHandler {
    async fn handle(&self, handler: MechanicHandlerWrapper) {
        match &self.task {
            MenuJobs::StartTradeMenu { channel_id } => {
                self.start_trade_menu(handler, *channel_id).await
            }
            MenuJobs::OpenComms { channel_id } => self.open_comms(handler, *channel_id).await,
        }
    }
}

impl MenuMechanicsHandler {
    async fn start_trade_menu(&self, handler: MechanicHandlerWrapper, channel_id: DiscordId) {
        // Send a message to the channel
        let _message_create_status = handler
            .db
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

    async fn open_comms(&self, handler: MechanicHandlerWrapper, channel_id: DiscordId) {
        // Get the team of the interacting player
        let discord_user_id: DiscordId =
            handler.interaction.unwrap().member.unwrap().user.id.into();

        // Get the player from the database
        let player = player::Entity::find()
            .filter(player::Column::DiscordId.eq(discord_user_id.0))
            .one(&handler.db.pool)
            .await
            .unwrap();

        // Send a message to the channel
        let _message_create_status = handler
            .db
            .add_await_task(TaskType::MessageHandler(MessageHandler {
                guild_id: DiscordId(self.guild_id),
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
