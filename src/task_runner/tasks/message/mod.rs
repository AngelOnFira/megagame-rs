use std::sync::Arc;

use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use serenity::client::Context;
use tracing::log;

use super::{Task, TaskTest};
use crate::db_wrapper::{DBWrapper, TaskResult, TaskReturnData};

// pub mod tests;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageHandler {
    pub guild_id: u64,
    pub category_id: u64,
    pub task: MessageTasks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageTasks {
    Create(CreateMessageTasks),
    Delete(DeleteMessageTasks),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateMessageTasks {
    TeamMessage { team_id: u64, channel_db_id: u64 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteMessageTasks {
    TeamChannel { team_id: u64 },
    PublicChannel { id: u64 },
}

#[async_trait]
impl Task for MessageHandler {
    async fn handle(&self, ctx: Arc<Context>, db: DBWrapper) -> TaskResult {
        match &self.task {
            MessageTasks::Create(task) => self.handle_role_create(task, ctx, db).await,
            MessageTasks::Delete(task) => self.handle_role_delete(task, ctx, db).await,
        }
    }
}

impl MessageHandler {
    async fn handle_role_create(
        &self,
        _task: &CreateMessageTasks,
        ctx: Arc<Context>,
        _db: DBWrapper,
    ) -> TaskResult {
        let _guild = ctx.cache.guild(self.guild_id).unwrap();

        todo!()
    }

    async fn handle_role_delete(
        &self,
        _task: &DeleteMessageTasks,
        _ctx: Arc<Context>,
        _db: DBWrapper,
    ) -> TaskResult {
        todo!()
    }
}

#[async_trait]
impl TaskTest for MessageHandler {
    async fn run_tests(_ctx: Arc<Context>, _db: DBWrapper) {
        log::info!("Testing categories");
        // assert_not_error(test_create_channel(ctx, db).await);
    }
}

// #[derive(Debug)]
// pub enum CategoryCreateError {
//     CategoryAlreadyExists,
//     CategoryNotCreated,
//     CategoryNotInDatabase,
// }
