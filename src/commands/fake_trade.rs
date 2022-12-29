use async_trait::async_trait;
use serenity::{
    all::{GuildId, ResolvedOption},
    builder::CreateCommand,
    prelude::Context,
};

use crate::{
    db_wrapper::DBWrapper,
    task_runner::tasks::{
        category::{CategoryHandler, CategoryTasks},
        DiscordId, TaskType,
    },
};

use super::GameCommand;

pub struct FakeTrade;

#[async_trait]
impl GameCommand for FakeTrade {
    fn register() -> CreateCommand {
        CreateCommand::new("trade").description("Start a test trade")
    }

    async fn run(
        _options: &[ResolvedOption],
        _guild_id: GuildId,
        db: DBWrapper,
        _ctx: Context,
    ) -> String {
        // Add a channel create task
        db.add_task(TaskType::CategoryHandler(CategoryHandler {
            guild_id: DiscordId(345993194322001923),
            task: CategoryTasks::Create {
                name: "test".to_string(),
            },
        }))
        .await;
        "Hey, I'm alive!".to_string()
    }
}
