use std::sync::Arc;

use async_trait::async_trait;
use entity::entities::team;
use sea_orm::{EntityTrait, Set, ActiveModelTrait};
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
        let category = guild
            .create_channel(&ctx.http, channel_builder)
            .await
            .unwrap();

        // If it's a team channel, safe it to the database
        if let CreateCategoryKind::Team { team_id } = self.kind {
            let mut team: team::ActiveModel = team::Entity::find_by_id(team_id as i32)
                .one(&*db)
                .await
                .unwrap()
                .unwrap().into();

            team.name = Set(self.category_name.to_owned());
            team.category_id = Set(Some(category.id.0 as i32));

            let team = team.update(&*db).await.unwrap();
        }
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
