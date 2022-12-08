use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::{client::Context, model::channel::ChannelType};

use crate::db_wrapper::DBWrapper;
use super::Task;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateCategoryKind {
    Team,
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
        // Get the team from the database
        // Create the category
        let _category = ctx
            .cache
            .guild(self.guild_id)
            .unwrap()
            .create_channel(&ctx.http, |c| {
                c.name(&self.category_name);
                c.kind(ChannelType::Category);
                c
            })
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
