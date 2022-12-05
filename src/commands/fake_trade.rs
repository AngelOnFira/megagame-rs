use serenity::{
    builder::CreateApplicationCommand, model::prelude::application_command::CommandDataOption,
};

use super::GameCommand;

pub struct FakeTrade;

impl GameCommand for FakeTrade {
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("trade").description("Start a test trade")
    }

    fn run(_options: &[CommandDataOption]) -> String {
        "Hey, I'm alive!".to_string()
    }
}
