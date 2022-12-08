use std::sync::Arc;

use async_trait::async_trait;
use entity::entities::team;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use serenity::{
    builder::CreateChannel,
    client::Context,
    model::{channel::ChannelType, prelude::PermissionOverwrite},
};

use super::Task;
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
        let channel_builder = match self.kind {
            CreateCategoryKind::Team { team_id } => {
                // Get the team from the database
                let team: Option<team::Model> = team::Entity::find_by_id(team_id as i32)
                    .one(&*db)
                    .await
                    .unwrap();

                |c: &mut CreateChannel| {
                    c.name(&self.category_name);
                    c.kind(ChannelType::Category);
                    c.permissions(vec![PermissionOverwrite {
                        allow: Permissions::VIEW_CHANNEL,
                        deny: Permissions::SEND_TTS_MESSAGES,
                        kind: PermissionOverwriteType::Member(UserId(1234)),
                    }]);
                    c
                }
            }
            CreateCategoryKind::Public => |c| {
                c.name(&self.category_name);
                c.kind(ChannelType::Category);
                c.permissions(vec![PermissionOverwrite {
                    allow: Permissions::VIEW_CHANNEL,
                    deny: Permissions::SEND_TTS_MESSAGES,
                    kind: PermissionOverwriteType::Member(UserId(1234)),
                }]);
                c
            },
        };

        // Create the category
        let _category = ctx
            .cache
            .guild(self.guild_id)
            .unwrap()
            .create_channel(&ctx.http, channel_builder)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};

    use crate::{db_wrapper::DBWrapper, task_runner::tasks::TaskType};

    use super::*;

    #[tokio::test]
    async fn test_create_channel() {
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
    }
}
