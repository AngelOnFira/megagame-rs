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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategory {}

#[async_trait]
impl Task for CreateCategory {
    async fn handle(&self, ctx: Arc<Context>) {}
}
