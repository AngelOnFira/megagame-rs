use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::{
    builder::{CreateActionRow, CreateButton},
    client::Context,
    http::CacheHttp,
    model::{
        id::ChannelId,
        interactions::message_component::{ActionRow, SelectMenuOption},
    },
};

use super::Task;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessage {}

#[async_trait]
impl Task for CreateMessage {
    async fn handle(&self, ctx: Arc<Context>) {}
}
