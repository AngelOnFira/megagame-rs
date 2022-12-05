use serenity::{builder::CreateApplicationCommand, model::prelude::application_command::CommandDataOption};


pub mod fake_trade;
pub mod initialize_game;

pub trait GameCommand {
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
    fn run(options: &[CommandDataOption]) -> String;
}