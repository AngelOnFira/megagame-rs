use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::client::Context;

use super::Task;
use crate::db_wrapper::DBWrapper;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct CreateThread {}

#[async_trait]
impl Task for CreateThread {
    async fn handle(&self, _ctx: Arc<Context>, _db: DBWrapper) {}
}
