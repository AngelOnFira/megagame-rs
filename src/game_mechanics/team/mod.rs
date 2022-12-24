use async_trait::async_trait;

use crate::{
    db_wrapper::DBWrapper,
    task_runner::tasks::role::{CreateRoleTasks, RoleHandler, RoleTasks},
};

use super::MechanicHandler;

pub struct TeamMechanicsHandler {
    pub guild_id: u64,
    pub task: TeamJobs,
}

pub enum TeamJobs {
    CreateTeam { name: String },
    AddPlayerToTeam,
    RemovePlayerFromTeam,
    DeleteTeam,
}

#[async_trait]
impl MechanicHandler for TeamMechanicsHandler {
    async fn handle(&self, db: DBWrapper) {
        match &self.task {
            TeamJobs::CreateTeam { name } => self.create_team(name, db).await,
            TeamJobs::AddPlayerToTeam => self.add_player_to_team(db).await,
            TeamJobs::RemovePlayerFromTeam => self.remove_player_from_team(db).await,
            TeamJobs::DeleteTeam => self.delete_team(db).await,
        }
    }
}

impl TeamMechanicsHandler {
    async fn create_team(&self, name: &String, db: DBWrapper) {
        // Add the team to the database

        // Create the role
        let role_create_status = db
            .add_await_task(crate::task_runner::tasks::TaskType::RoleHandler(
                RoleHandler {
                    guild_id: self.guild_id,
                    task: RoleTasks::Create(CreateRoleTasks::Role {
                        name: name.clone(),
                        color: 0x00ff00,
                    }),
                },
            ))
            .await;

        dbg!(role_create_status);

        // Create the team category
    }

    async fn add_player_to_team(&self, _db: DBWrapper) {
        // Add the player to the team
    }

    async fn remove_player_from_team(&self, _db: DBWrapper) {
        // Remove the player from the team
    }

    async fn delete_team(&self, _db: DBWrapper) {
        // Delete the team from the database
    }
}
