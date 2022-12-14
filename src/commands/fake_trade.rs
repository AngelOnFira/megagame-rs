use async_trait::async_trait;

use serenity::{
    builder::CreateApplicationCommand, model::prelude::application_command::CommandDataOption,
};

use crate::{
    db_wrapper::DBWrapper,
    task_runner::tasks::{
        category::{CategoryHandler, CategoryTasks, CreateCategoryTasks},
        DiscordId, TaskType,
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
        db.add_task(TaskType::CategoryHandler(CategoryHandler {
            guild_id: DiscordId(345993194322001923),
            task: CategoryTasks::Create(CreateCategoryTasks::PublicCategory {
                name: "test".to_string(),
            }),
        }))
        .await;
        "Hey, I'm alive!".to_string()
    }
}
