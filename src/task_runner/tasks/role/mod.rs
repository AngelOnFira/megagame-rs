use std::sync::Arc;

use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use serenity::client::Context;
use tracing::log;

use super::{Task, TaskTest};
use crate::db_wrapper::DBWrapper;

// pub mod tests;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleHandler {
    pub guild_id: u64,
    pub category_id: u64,
    pub task: RoleTasks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RoleTasks {
    Create(CreateRoleTasks),
    Delete(DeleteRoleTasks),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateRoleTasks {
    TeamRole { team_id: u64, channel_db_id: u64 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteRoleTasks {
    TeamChannel { team_id: u64 },
    PublicChannel { id: u64 },
}

#[async_trait]
impl Task for RoleHandler {
    async fn handle(&self, ctx: Arc<Context>, db: DBWrapper) {
        match &self.task {
            RoleTasks::Create(task) => self.handle_role_create(task, ctx, db).await,
            RoleTasks::Delete(task) => self.handle_role_delete(task, ctx, db).await,
        }
    }
}

impl RoleHandler {
    async fn handle_role_create(&self, _task: &CreateRoleTasks, ctx: Arc<Context>, _db: DBWrapper) {
        let guild = ctx.cache.guild(self.guild_id).unwrap();

        guild.create_role(&ctx.http, |r| {
            r.name("test");
            r.color(0x00ff00);
            r
        }).await;
    }

    async fn handle_role_delete(
        &self,
        _task: &DeleteRoleTasks,
        _ctx: Arc<Context>,
        _db: DBWrapper,
    ) {
        todo!()
    }
}

#[async_trait]
impl TaskTest for RoleHandler {
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
