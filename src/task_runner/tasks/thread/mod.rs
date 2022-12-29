use std::sync::Arc;

use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use serenity::client::Context;
use tracing::log;

use super::{Task, TaskTest};
use crate::db_wrapper::{DBWrapper, TaskResult};

// pub mod tests;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadHandler {
    pub guild_id: u64,
    pub category_id: u64,
    pub task: ThreadTasks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ThreadTasks {
    Create(CreateThreadTasks),
    Delete(DeleteThreadTasks),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateThreadTasks {
    TeamThread { team_id: u64, channel_db_id: u64 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteThreadTasks {
    TeamChannel { team_id: u64 },
    PublicChannel { id: u64 },
}

#[async_trait]
impl Task for ThreadHandler {
    async fn handle(&self, ctx: Context, db: DBWrapper) -> TaskResult {
        match &self.task {
            ThreadTasks::Create(task) => self.handle_role_create(task, ctx, db).await,
            ThreadTasks::Delete(task) => self.handle_role_delete(task, ctx, db).await,
        }
    }
}

impl ThreadHandler {
    async fn handle_role_create(
        &self,
        _task: &CreateThreadTasks,
        ctx: Context,
        _db: DBWrapper,
    ) -> TaskResult {
        let _guild = ctx.cache.guild(self.guild_id).unwrap();

        todo!()
    }

    async fn handle_role_delete(
        &self,
        _task: &DeleteThreadTasks,
        _ctx: Context,
        _db: DBWrapper,
    ) -> TaskResult {
        todo!()
    }
}

#[async_trait]
impl TaskTest for ThreadHandler {
    async fn run_tests(_ctx: Context, _db: DBWrapper) {
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
