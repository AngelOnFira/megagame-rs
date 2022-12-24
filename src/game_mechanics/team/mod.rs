use async_trait::async_trait;

use crate::db_wrapper::DBWrapper;

use super::MechanicHandler;

pub struct TeamMechanicsHandler {
    task: TeamJobs,
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
    async fn create_team(&self, _name: &String, _db: DBWrapper) {
        // Add the team to the database

        // Create the role

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
