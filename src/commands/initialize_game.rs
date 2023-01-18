use async_trait::async_trait;
use serenity::{
    all::{GuildId, ResolvedOption},
    builder::CreateCommand,
    prelude::Context,
};

use crate::{
    db_wrapper::DBWrapper,
    game_mechanics::{
        menu::{MenuJobs, MenuMechanicsHandler},
        team::{TeamJobs, TeamMechanicsHandler},
        MechanicHandler, MechanicHandlerWrapper,
    },
    task_runner::tasks::DiscordId,
};

use super::GameCommand;

pub struct InitializeGame;

#[async_trait]
impl GameCommand for InitializeGame {
    fn register() -> CreateCommand {
        CreateCommand::new("initialize").description("Initialize the game")
    }

    async fn run(
        _options: &[ResolvedOption],
        guild_id: GuildId,
        db: DBWrapper,
        ctx: Context,
    ) -> String {
        let mut names = vec![];

        // Make 3 teams, the Airship, the Galleon, and the Submarine
        for name in ["Airship", "Galleon", "Submarine"] {
            // Add a random string to the name
            let name = format!("{}-{}", name, rand::random::<u16>());

            names.push(name.clone());

            TeamMechanicsHandler {
                task: TeamJobs::CreateTeam {
                    name: name.to_string(),
                },
                guild_id: DiscordId::from(guild_id),
            }
            .handle(MechanicHandlerWrapper {
                db: db.clone(),
                interaction: None,
                ctx: ctx.clone(),
            })
            .await;
        }

        // Make a general role channel to be able to change your team
        MenuMechanicsHandler {
            task: MenuJobs::RoleChangeMenu { team_names: names },
            guild_id: DiscordId::from(guild_id),
        }.handle(MechanicHandlerWrapper {
            db,
            interaction: None,
            ctx,
        }).await;

        "Made a team!".to_string()
    }
}
