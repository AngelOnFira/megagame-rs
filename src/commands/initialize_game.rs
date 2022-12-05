use serenity::{
    builder::CreateApplicationCommand, model::prelude::application_command::CommandDataOption,
};


use crate::task_runner::tasks::{TaskType, message_user::MessageUser};

use super::GameCommand;

pub struct InitializeGame;

impl GameCommand for InitializeGame {
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("initialize")
            .description("Initialize the game")
    }

    fn run(_options: &[CommandDataOption]) -> String {
        let task = TaskType::MessageUser(MessageUser {
            player_id: 133358326439346176,
            message: String::from("Good day"),
        });
        "Hey".to_string()
    }
}
