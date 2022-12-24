use async_trait::async_trait;
use serenity::{
    builder::CreateApplicationCommand, model::prelude::application_command::CommandDataOption,
};

use crate::db_wrapper::DBWrapper;

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
        // Make 3 teams, the Airship, the Galleon, and the Submarine
        "Hey".to_string()
    }
}
