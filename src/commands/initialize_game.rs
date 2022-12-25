use async_trait::async_trait;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{application_command::CommandDataOption, GuildId},
    prelude::Context,
};

use crate::{
    db_wrapper::DBWrapper,
    game_mechanics::{
        team::{TeamJobs, TeamMechanicsHandler},
        MechanicHandler,
    },
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

    async fn run(_options: &[CommandDataOption], guild_id: GuildId, db: DBWrapper) -> String {
        // Make 3 teams, the Airship, the Galleon, and the Submarine
        for name in ["Airship", "Galleon", "Submarine"] {
            let role_result = TeamMechanicsHandler {
                task: TeamJobs::CreateTeam {
                    name: name.to_string(),
                },
                guild_id: guild_id.into(),
            }
            .handle(db.clone())
            .await;
        }

        "Made a team!".to_string()
    }
}
