use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::client::Context;

use super::Task;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct CreateMessage {}

#[async_trait]
impl Task for CreateMessage {
    async fn handle(&self, _ctx: Arc<Context>) {}
}
