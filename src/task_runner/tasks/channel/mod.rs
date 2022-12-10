use std::sync::Arc;

use async_trait::async_trait;
use entity::entities::{category, channel, guild, team};
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

use super::{Task, TaskTest};
use crate::{
    db_wrapper::DBWrapper,
    task_runner::tasks::{assert_not_error, category::tests::tests::test_create_category},
};

pub mod tests;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelHandler {
    pub guild_id: u64,
    pub category_id: u64,
    pub task: ChannelTasks,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelTasks {
    Create(CreateChannelTasks),
    Delete(DeleteChannelTasks),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CreateChannelTasks {
    TeamChannel { team_id: u64, channel_db_id: u64 },
    PublicChannel { category_id: u64, name: String },
    TeamVoiceChannel { team_id: u64, name: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeleteChannelTasks {
    TeamChannel { team_id: u64 },
    PublicChannel { id: u64 },
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
        let guild = ctx.cache.guild(self.guild_id).unwrap();

        let everyone_role = guild.role_by_name("@everyone").unwrap();

        let channel_builder: Box<
            dyn FnOnce(&mut CreateChannel) -> &mut CreateChannel + Send + Sync,
        > = match task {
            CreateChannelTasks::TeamChannel {
                team_id,
                channel_db_id,
            } => {
                // Get the team from the database
                let team: team::Model = team::Entity::find_by_id(*team_id as i32)
                    .one(&*db)
                    .await
                    .unwrap()
                    .unwrap();

                // There will already be a channel in the database for this
                // team, since it needs to be created before the team is.

                // Get the channel from the database
                let category: channel::Model = channel::Entity::find_by_id(*channel_db_id as i32)
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
                c.category(*category_id);
                c
            }),
            CreateChannelTasks::TeamVoiceChannel { team_id, name } => {
                // Get the team from the database
                let team: team::Model = team::Entity::find_by_id(*team_id as i32)
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
            team_id,
            channel_db_id,
        } = task
        {
            // Get the channel from the database
            let mut category: channel::ActiveModel =
                channel::Entity::find_by_id(*channel_db_id as i32)
                    .one(&*db)
                    .await
                    .unwrap()
                    .unwrap()
                    .into();

            category.discord_id = Set(discord_channel.id.0 as i32);

            let category = category.update(&*db).await.unwrap();
        }
    }

    async fn handle_channel_delete(
        &self,
        task: &DeleteChannelTasks,
        ctx: Arc<Context>,
        db: DBWrapper,
    ) {
        todo!()
    }
}

#[async_trait]
impl TaskTest for ChannelHandler {
    async fn run_tests(ctx: Arc<Context>, db: DBWrapper) {
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
