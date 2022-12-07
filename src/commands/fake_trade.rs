use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use serenity::{
    builder::CreateApplicationCommand, model::prelude::application_command::CommandDataOption,
};

use crate::{
    db_wrapper::DBWrapper,
    task_runner::tasks::{
        create_category::{CreateCategory, CreateCategoryKind},
        create_channel::CreateChannel,
        TaskType,
    },
};

use super::GameCommand;

pub struct FakeTrade;

#[async_trait]
impl GameCommand for FakeTrade {
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("trade").description("Start a test trade")
    }

    async fn run(_options: &[CommandDataOption], db: DBWrapper) -> String {
        // Add a channel create task
        db.add_task(TaskType::CreateCategory(CreateCategory {
            guild_id: 345993194322001923,
            category_name: "Test Category".to_string(),
            kind: CreateCategoryKind::Public,
        })).await;
        "Hey, I'm alive!".to_string()
    }
}
