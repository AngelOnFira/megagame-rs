use serenity::{builder::CreateApplicationCommand, model::prelude::application_command::CommandDataOption};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("trade").description("Start a test trade")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "Hey, I'm alive!".to_string()
}