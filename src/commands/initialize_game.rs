use serenity::{
    builder::CreateApplicationCommand, model::prelude::application_command::CommandDataOption,
};

use super::GameCommand;

pub struct InitializeGame;

impl GameCommand for InitializeGame {
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("initialize")
            .description("Initialize the game")
    }

    fn run(_options: &[CommandDataOption]) -> String {
        "Hey".to_string()
    }
}
