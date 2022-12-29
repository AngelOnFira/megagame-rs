use async_trait::async_trait;

use entity::entities::role;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use serenity::{builder::EditRole, client::Context};
use tracing::log;

use super::{DiscordId, Task, TaskTest};
use crate::db_wrapper::{helpers::get_guild, DBWrapper, TaskResult, TaskReturnData};

// pub mod tests;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleHandler {
    pub guild_id: DiscordId,
    pub task: RoleTasks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RoleTasks {
    Create(CreateRoleTasks),
    Delete(DeleteRoleTasks),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateRoleTasks {
    TeamRole { team_id: u64, channel_db_id: u64 },
    Role { name: String, color: u32 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteRoleTasks {
    TeamChannel { team_id: u64 },
    PublicChannel { id: u64 },
}

#[async_trait]
impl Task for RoleHandler {
    async fn handle(&self, ctx: Context, db: DBWrapper) -> TaskResult {
        match &self.task {
            RoleTasks::Create(task) => self.handle_role_create(task, ctx, db).await,
            RoleTasks::Delete(task) => self.handle_role_delete(task, ctx, db).await,
        }
    }
}

impl RoleHandler {
    async fn handle_role_create(
        &self,
        task: &CreateRoleTasks,
        ctx: Context,
        db: DBWrapper,
    ) -> TaskResult {
        let (discord_guild, _database_guild) =
            get_guild(ctx.clone(), db.clone(), self.guild_id).await;

        match task {
            CreateRoleTasks::TeamRole {
                team_id: _,
                channel_db_id: _,
            } => todo!(),
            CreateRoleTasks::Role { name, color: _ } => {
                // Create the role
                let role_discord = discord_guild
                    .create_role(&ctx.http, EditRole::new().name(name))
                    .await
                    .unwrap();

                // TODO: Set the guild

                // Add the role to the database
                let role_database = role::ActiveModel {
                    discord_id: Set(*DiscordId::from(role_discord.id.0) as i64),
                    // guild_id: Set(Some(self.guild_id)),
                    name: Set(role_discord.name),
                    ..Default::default()
                };

                let role_database = role_database.insert(&*db).await.unwrap();

                TaskResult::Completed(TaskReturnData::RoleModel(role_database))
            }
        }
    }

    async fn handle_role_delete(
        &self,
        _task: &DeleteRoleTasks,
        _ctx: Context,
        _db: DBWrapper,
    ) -> TaskResult {
        todo!()
    }
}

#[async_trait]
impl TaskTest for RoleHandler {
    async fn run_tests(_ctx: Context, _db: DBWrapper) {
        log::info!("Testing categories");
        // assert_not_error(test_create_channel(ctx, db).await);
    }
}

// #[derive(Debug)]
// pub enum CategoryCreateError {
//     CategoryAlreadyExists,
//     CategoryNotCreated,
//     CategoryNotInDatabase,
// }
