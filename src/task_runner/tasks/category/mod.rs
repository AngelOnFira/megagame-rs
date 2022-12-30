use async_trait::async_trait;
use entity::entities::category;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serenity::{
    builder::CreateChannel,
    client::Context,
    model::{
        channel::{ChannelType, PermissionOverwriteType},
        permissions::Permissions,
        prelude::PermissionOverwrite,
    },
};
use tracing::log;

use super::{DiscordId, Task, TaskTest};
use crate::{
    db_wrapper::{helpers::get_guild, DBWrapper, TaskResult, TaskReturnData},
    task_runner::tasks::{assert_not_error, category::tests::tests::test_create_category},
};

pub mod tests;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryHandler {
    pub guild_id: DiscordId,
    pub task: CategoryTasks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CategoryTasks {
    Create { name: String },
    Delete { discord_id: DiscordId },
}

#[async_trait]
impl Task for CategoryHandler {
    async fn handle(&self, ctx: Context, db: DBWrapper) -> TaskResult {
        match &self.task {
            CategoryTasks::Create { name } => self.handle_category_create(name, ctx, db).await,
            CategoryTasks::Delete { discord_id } => {
                self.handle_category_delete(discord_id, ctx, db).await
            }
        }
    }
}

impl CategoryHandler {
    async fn handle_category_create(&self, name: &str, ctx: Context, db: DBWrapper) -> TaskResult {
        let (discord_guild, database_guild) =
            get_guild(ctx.clone(), db.clone(), self.guild_id).await;

        let everyone_role = discord_guild.role_by_name("@everyone").unwrap();

        // Create the category
        let discord_category = discord_guild
            .create_channel(
                &ctx.http,
                CreateChannel::new(name)
                    .kind(ChannelType::Category)
                    .permissions(vec![PermissionOverwrite {
                        allow: Permissions::VIEW_CHANNEL,
                        deny: Permissions::SEND_TTS_MESSAGES,
                        kind: PermissionOverwriteType::Role(everyone_role.id),
                    }]),
            )
            .await
            .unwrap();

        // Save the category to the database
        let database_category = category::ActiveModel {
            name: Set(discord_category.name),
            discord_id: Set(*DiscordId(discord_category.id.0.get()) as i64),
            fk_guild_id: Set(Some(database_guild.discord_id)),
            ..Default::default()
        }
        .insert(&*db)
        .await
        .unwrap();

        TaskResult::Completed(TaskReturnData::CategoryModel(database_category))
    }

    async fn handle_category_delete(
        &self,
        category_discord_id: &DiscordId,
        ctx: Context,
        db: DBWrapper,
    ) -> TaskResult {
        // Delete the category from Discord
        ctx.cache
            .channel(*category_discord_id)
            .unwrap()
            .delete(&ctx.http)
            .await
            .unwrap();

        // Delete the category from the database
        let category = category::Entity::find()
            .filter(category::Column::DiscordId.eq(**category_discord_id as i64))
            .one(&*db)
            .await
            .unwrap()
            .unwrap();

        category.delete(&*db).await.unwrap();

        TaskResult::Completed(TaskReturnData::None)
    }
}

#[async_trait]
impl TaskTest for CategoryHandler {
    async fn run_tests(ctx: Context, db: DBWrapper) {
        log::info!("Testing categories");
        assert_not_error(test_create_category(ctx, db).await);
    }
}

#[derive(Debug)]
pub enum CategoryCreateError {
    CategoryAlreadyExists,
    CategoryNotCreated,
    CategoryNotInDatabase,
    CategoryNotDeleted,
    CategoryNotSaved,
}
