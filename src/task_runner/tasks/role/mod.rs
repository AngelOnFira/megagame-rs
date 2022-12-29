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
    CreateRole(CreateRole),
    DeleteRole(DeleteRole),
    AddRoleToUser(AddRoleToUser),
    RemoveRoleFromUser(RemoveRoleFromUser),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRole {
    pub name: String,
    pub color: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteRole {
    pub role_id: DiscordId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddRoleToUser {
    pub user_id: DiscordId,
    pub role_id: DiscordId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveRoleFromUser {
    pub user_id: DiscordId,
    pub role_id: DiscordId,
}

#[async_trait]
impl Task for RoleHandler {
    async fn handle(&self, ctx: Context, db: DBWrapper) -> TaskResult {
        match &self.task {
            RoleTasks::CreateRole(create_role_task) => {
                self.handle_role_create(create_role_task, ctx, db).await
            }
            RoleTasks::DeleteRole(delete_role_task) => {
                self.handle_role_delete(delete_role_task, ctx, db).await
            }
            RoleTasks::AddRoleToUser(add_role_to_user_task) => {
                self.handle_add_role_to_user(add_role_to_user_task, ctx, db)
                    .await
            }
            RoleTasks::RemoveRoleFromUser(remove_role_from_user_task) => {
                self.handle_remove_role_from_user(remove_role_from_user_task, ctx, db)
                    .await
            }
        }
    }
}

impl RoleHandler {
    async fn handle_role_create(
        &self,
        task: &CreateRole,
        ctx: Context,
        db: DBWrapper,
    ) -> TaskResult {
        let (discord_guild, _database_guild) =
            get_guild(ctx.clone(), db.clone(), self.guild_id).await;

        // Create the role
        let role_discord = discord_guild
            .create_role(&ctx.http, EditRole::new().name(&task.name))
            .await
            .unwrap();

        // TODO: Set the guild

        // Add the role to the database
        let role_database = role::ActiveModel {
            discord_id: Set(*DiscordId::from(role_discord.id.0) as i64),
            fk_guild_id: Set(Some(*self.guild_id as i64)),
            name: Set(role_discord.name),
            ..Default::default()
        };

        let role_database = role_database.insert(&*db).await.unwrap();

        TaskResult::Completed(TaskReturnData::RoleModel(role_database))
    }

    async fn handle_role_delete(
        &self,
        task: &DeleteRole,
        ctx: Context,
        db: DBWrapper,
    ) -> TaskResult {
        let (discord_guild, _database_guild) =
            get_guild(ctx.clone(), db.clone(), self.guild_id).await;

        let role_discord = discord_guild
            .delete_role(&ctx.http, task.role_id)
            .await
            .unwrap();

        // role_discord.delete(&ctx.http).await.unwrap();

        TaskResult::Completed(TaskReturnData::None)
    }

    async fn handle_add_role_to_user(
        &self,
        task: &AddRoleToUser,
        ctx: Context,
        db: DBWrapper,
    ) -> TaskResult {
        let (discord_guild, _database_guild) =
            get_guild(ctx.clone(), db.clone(), self.guild_id).await;

        // Get the member
        let mut member = ctx.cache.member(self.guild_id, task.user_id).unwrap();

        // Add the role
        member.add_role(&ctx.http, task.role_id).await.unwrap();

        TaskResult::Completed(TaskReturnData::None)
    }

    async fn handle_remove_role_from_user(
        &self,
        task: &RemoveRoleFromUser,
        ctx: Context,
        db: DBWrapper,
    ) -> TaskResult {
        let (discord_guild, _database_guild) =
            get_guild(ctx.clone(), db.clone(), self.guild_id).await;

        // Get the member
        let mut member = ctx.cache.member(self.guild_id, task.user_id).unwrap();

        // Remove the role
        member.remove_role(&ctx.http, task.role_id).await.unwrap();

        TaskResult::Completed(TaskReturnData::None)
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
