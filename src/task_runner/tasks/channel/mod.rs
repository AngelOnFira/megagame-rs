use std::sync::Arc;

use async_trait::async_trait;
use entity::entities::{channel, team};
use sea_orm::{ActiveModelTrait, Database, EntityTrait, ModelTrait, Set};
use serde::{Deserialize, Serialize};
use serenity::{builder::CreateChannel, client::Context, model::channel::ChannelType};
use tracing::log;

use super::{DatabaseId, DiscordId, Task, TaskTest};
use crate::db_wrapper::DBWrapper;

pub mod tests;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelHandler {
    pub guild_id: DiscordId,
    pub category_id: DiscordId,
    pub task: ChannelTasks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelTasks {
    Create(CreateChannelTasks),
    Delete(DeleteChannelTasks),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateChannelTasks {
    TeamChannel {
        team_id: DatabaseId,
        channel_id: DatabaseId,
    },
    PublicChannel {
        category_id: DiscordId,
        name: String,
    },
    TeamVoiceChannel {
        team_id: DatabaseId,
        name: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteChannelTasks {
    TeamChannel { team_id: DatabaseId },
    PublicChannel { id: DiscordId },
}

#[async_trait]
impl Task for ChannelHandler {
    async fn handle(&self, ctx: Arc<Context>, db: DBWrapper) {
        match &self.task {
            ChannelTasks::Create(task) => self.handle_channel_create(task, ctx, db).await,
            ChannelTasks::Delete(task) => self.handle_channel_delete(task, ctx, db).await,
        }
    }
}

impl ChannelHandler {
    async fn handle_channel_create(
        &self,
        task: &CreateChannelTasks,
        ctx: Arc<Context>,
        db: DBWrapper,
    ) {
        let guild = ctx.cache.guild(*self.guild_id).unwrap();

        let channel_builder: Box<
            dyn FnOnce(&mut CreateChannel) -> &mut CreateChannel + Send + Sync,
        > = match task {
            CreateChannelTasks::TeamChannel {
                team_id,
                channel_id,
            } => {
                // Get the team from the database
                let _team: team::Model = team::Entity::find_by_id(**team_id)
                    .one(&*db)
                    .await
                    .unwrap()
                    .unwrap();

                // There will already be a channel in the database for this
                // team, since it needs to be created before the team is.

                // Get the channel from the database
                let category: channel::Model = channel::Entity::find_by_id(**channel_id)
                    .one(&*db)
                    .await
                    .unwrap()
                    .unwrap();

                Box::new(move |c: &mut CreateChannel| {
                    c.name(category.name);
                    c.category(category.discord_id as u64);
                    c
                })
            }
            CreateChannelTasks::PublicChannel {
                ref name,
                category_id,
            } => Box::new(move |c: &mut CreateChannel| {
                c.name(name);
                c.category(**category_id);
                c
            }),
            CreateChannelTasks::TeamVoiceChannel { team_id, name } => {
                // Get the team from the database
                let team: team::Model = team::Entity::find_by_id(**team_id)
                    .one(&*db)
                    .await
                    .unwrap()
                    .unwrap();

                Box::new(move |c: &mut CreateChannel| {
                    c.name(name);
                    c.category(team.category_id.unwrap() as u64);
                    c.kind(ChannelType::Voice);
                    c
                })
            }
        };

        // Create the channel
        let discord_channel = guild
            .create_channel(&ctx.http, channel_builder)
            .await
            .unwrap();

        // If it's a team channel, safe it to the database
        if let CreateChannelTasks::TeamChannel {
            team_id: _,
            channel_id,
        } = task
        {
            // Get the channel from the database
            let mut category: channel::ActiveModel = channel::Entity::find_by_id(**channel_id)
                .one(&*db)
                .await
                .unwrap()
                .unwrap()
                .into();

            category.discord_id = Set(discord_channel.id.0 as i32);

            let _category = category.update(&*db).await.unwrap();
        }
    }

    async fn handle_channel_delete(
        &self,
        task: &DeleteChannelTasks,
        ctx: Arc<Context>,
        db: DBWrapper,
    ) {
        let channel_id: DiscordId = match task {
            DeleteChannelTasks::TeamChannel { team_id } => {
                // Get the team from the database
                let team: team::Model = team::Entity::find_by_id(**team_id)
                    .one(&*db)
                    .await
                    .unwrap()
                    .unwrap();

                // Get the channel from the database
                let channel: channel::Model =
                    channel::Entity::find_by_id(team.category_id.unwrap())
                        .one(&*db)
                        .await
                        .unwrap()
                        .unwrap();

                let channel_id = DiscordId(channel.discord_id as u64);

                // Delete it from the database
                let res = channel.delete(&*db).await;

                channel_id
            }
            DeleteChannelTasks::PublicChannel { id } => *id,
        };

        // Delete it from discord
        ctx.cache
            .channel(*channel_id)
            .unwrap()
            .delete(&ctx.http)
            .await
            .unwrap();
    }
}

#[async_trait]
impl TaskTest for ChannelHandler {
    async fn run_tests(_ctx: Arc<Context>, _db: DBWrapper) {
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
