use std::sync::Arc;

use async_trait::async_trait;
use entity::entities::{category, team};
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

use super::{Task, TaskTest};
use crate::db_wrapper::DBWrapper;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateCategoryKind {
    Team { team_id: u64 },
    Public,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategory {
    pub guild_id: u64,
    pub category_name: String,
    pub kind: CreateCategoryKind,
}

#[async_trait]
impl Task for CreateCategory {
    async fn handle(&self, ctx: Arc<Context>, db: DBWrapper) {
        let guild = ctx.cache.guild(self.guild_id).unwrap();

        let everyone_role = guild.role_by_name("everyone").unwrap();

        let channel_builder: Box<
            dyn FnOnce(&mut CreateChannel) -> &mut CreateChannel + Send + Sync,
        > = match self.kind {
            CreateCategoryKind::Team { team_id } => {
                // Get the team from the database
                let team: team::Model = team::Entity::find_by_id(team_id as i32)
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
            CreateCategoryKind::Public => Box::new(|c: &mut CreateChannel| {
                c.name(&self.category_name);
                c.kind(ChannelType::Category);
                c.permissions(vec![PermissionOverwrite {
                    allow: Permissions::VIEW_CHANNEL,
                    deny: Permissions::SEND_TTS_MESSAGES,
                    kind: PermissionOverwriteType::Role(everyone_role.id),
                }]);
                c
            }),
        };

        // Create the category
        let discord_category = guild
            .create_channel(&ctx.http, channel_builder)
            .await
            .unwrap();

        // If it's a team channel, safe it to the database
        if let CreateCategoryKind::Team { team_id } = self.kind {
            let mut team: team::ActiveModel = team::Entity::find_by_id(team_id as i32)
                .one(&*db)
                .await
                .unwrap()
                .unwrap()
                .into();

            team.name = Set(self.category_name.to_owned());

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
                    name: Set(self.category_name.to_owned()),
                    discord_id: Set(discord_category.id.0 as i32),
                    guild_id: Set(self.guild_id as i32),
                    ..Default::default()
                }
                .insert(&*db)
                .await
                .unwrap(),
            };

            team.category_id = Set(category.id);

            let team = team.update(&*db).await.unwrap();
        }
    }
}

impl TaskTest for CreateCategory {
    fn run_tests(ctx: Arc<Context>, db: DBWrapper) {
        assert!(tests::test_create_channel())
    }
}

mod tests {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};

    use crate::{db_wrapper::DBWrapper, task_runner::tasks::TaskType};

    use super::*;

    pub async fn test_create_channel() -> bool {
        let db_wrapper = DBWrapper::new_default_db().await;

        let channel_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();

        db_wrapper
            .add_task(TaskType::CreateCategory(CreateCategory {
                guild_id: 345993194322001923,
                category_name: channel_name,
                kind: CreateCategoryKind::Public,
            }))
            .await;

        // Sleep for 2 seconds, then check if the category was created
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // TODO: Check if the category was created

        // TODO: Make sure that the category is in the database

        true
    }
}
