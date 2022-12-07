use async_trait::async_trait;
use serenity::{
    builder::CreateApplicationCommand, model::prelude::application_command::CommandDataOption,
};

use crate::{
    db_wrapper::DBWrapper,
    task_runner::tasks::{message_user::MessageUser, TaskType},
};

use super::GameCommand;

pub struct InitializeGame;

#[async_trait]
impl GameCommand for InitializeGame {
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("initialize")
            .description("Initialize the game")
    }

    async fn run(_options: &[CommandDataOption], _db: DBWrapper) -> String {
        let _task = TaskType::MessageUser(MessageUser {
            player_id: 133358326439346176,
            message: String::from("Good day"),
        });
        "Hey".to_string()
    }
}
