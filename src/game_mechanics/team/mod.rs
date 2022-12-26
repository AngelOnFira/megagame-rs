use async_trait::async_trait;

use crate::{
    db_wrapper::{DBWrapper, TaskResult, TaskReturnData},
    task_runner::tasks::{
        category::{CategoryHandler, CategoryTasks},
        channel::{ChannelHandler, ChannelTasks, CreateChannelTasks},
        role::{CreateRoleTasks, RoleHandler, RoleTasks},
        DiscordId, TaskType,
    },
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
        let _role_create_status = db
            .add_await_task(TaskType::RoleHandler(RoleHandler {
                guild_id: DiscordId(self.guild_id),
                task: RoleTasks::Create(CreateRoleTasks::Role {
                    name: name.clone(),
                    color: 0x00ff00,
                }),
            }))
            .await;

        // Create the team category
        let category_create_status = db
            .add_await_task(TaskType::CategoryHandler(CategoryHandler {
                guild_id: DiscordId(self.guild_id),
                task: CategoryTasks::Create { name: name.clone() },
            }))
            .await;

        match category_create_status {
            TaskResult::Completed(TaskReturnData::CategoryModel(category_model)) => {
                // Create the team channel
                let _channel_create_status = db
                    .add_await_task(TaskType::ChannelHandler(ChannelHandler {
                        guild_id: DiscordId(self.guild_id),
                        task: ChannelTasks::Create(CreateChannelTasks::PublicChannel {
                            name: name.clone(),
                            category_id: DiscordId(category_model.discord_id.parse().unwrap()),
                        }),
                        category_id: DiscordId(category_model.discord_id.parse().unwrap()),
                    }))
                    .await;
            }
            _ => {}
        };
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
