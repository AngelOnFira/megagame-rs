use std::sync::Arc;

use async_trait::async_trait;
use entity::entities::{category, guild, team};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
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

use super::{DiscordId, Task, TaskTest, DatabaseId};
use crate::{
    db_wrapper::DBWrapper,
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
    Create(CreateCategoryTasks),
    Delete(DeleteCategoryTasks),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateCategoryTasks {
    TeamCategory { team_id: DatabaseId },
    PublicCategory { name: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteCategoryTasks {
    TeamCategory { team_id: DatabaseId },
    PublicCategory { id: DiscordId },
}

#[async_trait]
impl Task for CategoryHandler {
    async fn handle(&self, ctx: Arc<Context>, db: DBWrapper) {
        match &self.task {
            CategoryTasks::Create(task) => self.handle_category_create(task, ctx, db).await,
            CategoryTasks::Delete(task) => self.handle_category_delete(task, ctx, db).await,
        }
    }
}

impl CategoryHandler {
    async fn handle_category_create(
        &self,
        task: &CreateCategoryTasks,
        ctx: Arc<Context>,
        db: DBWrapper,
    ) {
        let guild = ctx.cache.guild(*self.guild_id).unwrap();

        let everyone_role = guild.role_by_name("@everyone").unwrap();

        let channel_builder: Box<
            dyn FnOnce(&mut CreateChannel) -> &mut CreateChannel + Send + Sync,
        > = match task {
            CreateCategoryTasks::TeamCategory { team_id } => {
                // Get the team from the database
                let team: team::Model = team::Entity::find_by_id(**team_id)
                    .one(&*db)
                    .await
                    .unwrap()
                    .unwrap();

                Box::new(|c: &mut CreateChannel| {
                    c.name(team.name);
                    c.kind(ChannelType::Category);
                    c.permissions(vec![PermissionOverwrite {
                        allow: Permissions::VIEW_CHANNEL,
                        deny: Permissions::SEND_TTS_MESSAGES,
                        kind: PermissionOverwriteType::Role(everyone_role.id),
                    }]);
                    c
                })
            }
            CreateCategoryTasks::PublicCategory { ref name } => {
                Box::new(|c: &mut CreateChannel| {
                    c.name(name.clone());
                    c.kind(ChannelType::Category);
                    c.permissions(vec![PermissionOverwrite {
                        allow: Permissions::VIEW_CHANNEL,
                        deny: Permissions::SEND_TTS_MESSAGES,
                        kind: PermissionOverwriteType::Role(everyone_role.id),
                    }]);
                    c
                })
            }
        };

        // Create the category
        let discord_category = guild
            .create_channel(&ctx.http, channel_builder)
            .await
            .unwrap();

        // If it's a team category, safe it to the database
        if let CreateCategoryTasks::TeamCategory { team_id } = task {
            let mut team: team::ActiveModel = team::Entity::find_by_id(**team_id)
                .one(&*db)
                .await
                .unwrap()
                .unwrap()
                .into();

            team.name = Set(discord_category.name.clone());

            // Get or create the guild
            let guild_option = guild::Entity::find()
                .filter(guild::Column::DiscordId.eq(*self.guild_id))
                .one(&*db)
                .await
                .unwrap();

            let guild = match guild_option {
                Some(guild) => guild,
                None => guild::ActiveModel {
                    discord_id: Set(self.guild_id.into()),
                    ..Default::default()
                }
                .insert(&*db)
                .await
                .unwrap(),
            };

            // Create the category, or get it if it exists
            // TODO: Change this to upsert in the future
            let category_option = category::Entity::find()
                .filter(category::Column::DiscordId.eq(discord_category.id.0 as i32))
                .one(&*db)
                .await
                .unwrap();

            let category = match category_option {
                Some(category) => category,
                None => category::ActiveModel {
                    name: Set(discord_category.name),
                    discord_id: Set(discord_category.id.0 as i32),
                    guild_id: Set(Some(guild.id as i32)),
                    ..Default::default()
                }
                .insert(&*db)
                .await
                .unwrap(),
            };

            team.category_id = Set(Some(category.id));

            let _team = team.update(&*db).await.unwrap();
        }
    }

    async fn handle_category_delete(
        &self,
        task: &DeleteCategoryTasks,
        _ctx: Arc<Context>,
        _db: DBWrapper,
    ) {
        let category_id: DiscordId = match task {
            DeleteCategoryTasks::TeamCategory { team_id } => todo!(),
            DeleteCategoryTasks::PublicCategory { id } => todo!(),
        };
    }
}

#[async_trait]
impl TaskTest for CategoryHandler {
    async fn run_tests(ctx: Arc<Context>, db: DBWrapper) {
        log::info!("Testing categories");
        assert_not_error(test_create_category(ctx, db).await);
    }
}

#[derive(Debug)]
pub enum CategoryCreateError {
    CategoryAlreadyExists,
    CategoryNotCreated,
    CategoryNotInDatabase,
}
