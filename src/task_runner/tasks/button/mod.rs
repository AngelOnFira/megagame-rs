use std::sync::Arc;

use async_trait::async_trait;
use entity::entities::{category, channel, guild, team};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serenity::{
    builder::CreateChannel,
    client::Context,
    model::{
        channel::{ChannelType, PermissionOverwriteType},
        permissions::Permissions,
        prelude::PermissionOverwrite,
    },
};
use tracing::log;

use super::{Task, TaskTest};
use crate::{
    db_wrapper::DBWrapper,
    task_runner::tasks::{assert_not_error, category::tests::tests::test_create_category},
};

// pub mod tests;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonHandler {
    pub guild_id: u64,
    pub category_id: u64,
    pub task: ButtonTasks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ButtonTasks {
    Create(CreateButtonTasks),
    Delete(DeleteButtonTasks),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateButtonTasks {
    TeamButton { team_id: u64, channel_db_id: u64 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteButtonTasks {
    TeamChannel { team_id: u64 },
    PublicChannel { id: u64 },
}

#[async_trait]
impl Task for ButtonHandler {
    async fn handle(&self, ctx: Arc<Context>, db: DBWrapper) {
        match &self.task {
            ButtonTasks::Create(task) => self.handle_role_create(task, ctx, db).await,
            ButtonTasks::Delete(task) => self.handle_role_delete(task, ctx, db).await,
        }
    }
}

impl ButtonHandler {
    async fn handle_role_create(
        &self,
        task: &CreateButtonTasks,
        ctx: Arc<Context>,
        db: DBWrapper,
    ) {
        let guild = ctx.cache.guild(self.guild_id).unwrap();

    }

    async fn handle_role_delete(
        &self,
        task: &DeleteButtonTasks,
        ctx: Arc<Context>,
        db: DBWrapper,
    ) {
        todo!()
    }
}

#[async_trait]
impl TaskTest for ButtonHandler {
    async fn run_tests(ctx: Arc<Context>, db: DBWrapper) {
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
