use std::sync::Arc;

use async_trait::async_trait;
use entity::entities::{channel, team};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serenity::{builder::CreateChannel, client::Context, model::channel::ChannelType};
use tracing::log;

use super::{get_guild, DatabaseId, DiscordId, Task, TaskTest};
use crate::{
    db_wrapper::{
        DBWrapper, TaskResult,
        TaskReturnData::{self, ChannelModel},
    },
    task_runner::tasks::{assert_not_error, channel::tests::tests::test_create_channel},
};

pub mod tests;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelHandler {
    pub guild_id: DiscordId,
    pub task: ChannelTasks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelTasks {
    Create(ChannelCreateData),
    Delete { id: DiscordId },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelCreateData {
    pub name: String,
    pub category_id: Option<DiscordId>,
    pub kind: ChannelType,
}

#[async_trait]
impl Task for ChannelHandler {
    async fn handle(&self, ctx: Arc<Context>, db: DBWrapper) -> TaskResult {
        match &self.task {
            ChannelTasks::Create(channel_create_task) => {
                self.handle_channel_create(channel_create_task, ctx, db)
                    .await
            }
            ChannelTasks::Delete { id } => self.handle_channel_delete(*id, ctx, db).await,
        }
    }
}

impl ChannelHandler {
    async fn handle_channel_create(
        &self,
        data: &ChannelCreateData,
        ctx: Arc<Context>,
        db: DBWrapper,
    ) -> TaskResult {
        let (discord_guild, database_guild) =
            get_guild(ctx.clone(), db.clone(), self.guild_id).await;

        let channel_builder: Box<
            dyn FnOnce(&mut CreateChannel) -> &mut CreateChannel + Send + Sync,
        > = Box::new(move |c: &mut CreateChannel| {
            c.name(data.name.clone());

            if let Some(category) = &data.category_id {
                c.category(*category);
            }

            c.kind(data.kind);
            c
        });

        // Create the channel
        let discord_channel = discord_guild
            .create_channel(&ctx.http, channel_builder)
            .await
            .unwrap();

        // Add it to the database
        let database_category = channel::ActiveModel {
            discord_id: Set(DiscordId(discord_channel.id.into()).into()),
            guild_fk_id: Set(Some(database_guild.id)),
            name: Set(data.name.clone()),
            ..Default::default()
        }
        .insert(&*db)
        .await
        .unwrap();

        // Return the database model
        TaskResult::Completed(ChannelModel(database_category))
    }

    async fn handle_channel_delete(
        &self,
        id: DiscordId,
        ctx: Arc<Context>,
        db: DBWrapper,
    ) -> TaskResult {
        // Delete the channel from Discord
        ctx.cache
            .channel(*id)
            .unwrap()
            .delete(&ctx.http)
            .await
            .unwrap();

        // Delete the channel from the database
        let channel = channel::Entity::find()
            .filter(channel::Column::DiscordId.eq(*id))
            .one(&*db)
            .await
            .unwrap()
            .unwrap();

        channel.delete(&*db).await.unwrap();

        TaskResult::Completed(TaskReturnData::None)
    }
}

#[async_trait]
impl TaskTest for ChannelHandler {
    async fn run_tests(ctx: Arc<Context>, db: DBWrapper) {
        log::info!("Testing categories");
        assert_not_error(test_create_channel(ctx, db).await);
    }
}

// #[derive(Debug)]
// pub enum CategoryCreateError {
//     CategoryAlreadyExists,
//     CategoryNotCreated,
//     CategoryNotInDatabase,
// }
