use std::sync::Arc;

use entity::entities::{category, team};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serenity::{all::ChannelType, prelude::Context};

use crate::{db_wrapper::DBWrapper, TEST_GUILD_ID};

use super::DatabaseId;

pub(crate) struct TestHelpers {
    ctx: Arc<Context>,
    db: DBWrapper,
}

pub enum DiscordConstruct {
    Channel { name: String },
    Category { name: String },
}

pub enum DatabaseConstruct {
    Team { id: DatabaseId },
    Category { name: String },
}

pub enum DiscordStatus {
    Exists,
    DoesNotExist,
}

pub enum DatabaseStatus {
    Exists,
    DoesNotExist,
}

impl TestHelpers {
    pub async fn new(ctx: Arc<Context>, db: DBWrapper) -> Self {
        Self { db, ctx }
    }

    pub fn generate_name() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect()
    }

    pub async fn generate_team(&self) -> team::Model {
        team::ActiveModel {
            name: Set(Self::generate_name()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await
        .unwrap()
    }

    pub async fn check_discord_status(&self, discord_construct: DiscordConstruct) -> DiscordStatus {
        match discord_construct {
            DiscordConstruct::Channel { name: _ } => todo!(),
            DiscordConstruct::Category { name } => {
                if self
                    .ctx
                    .cache
                    .guild(TEST_GUILD_ID)
                    .unwrap()
                    .channels
                    .iter()
                    .filter(|(_, channel)| {
                        if let ChannelType::Category = channel.kind {
                            channel.name == name
                        } else {
                            false
                        }
                    })
                    .count()
                    != 1
                {
                    DiscordStatus::DoesNotExist
                } else {
                    DiscordStatus::Exists
                }
            }
        }
    }

    pub async fn check_database_status(
        &self,
        database_construct: DatabaseConstruct,
    ) -> DatabaseStatus {
        match database_construct {
            DatabaseConstruct::Team { id: _ } => todo!(),
            DatabaseConstruct::Category { name } => {
                if category::Entity::find()
                    // TODO: This should match on more than just name
                    .filter(category::Column::Name.eq(name.clone()))
                    .one(&*self.db)
                    .await
                    .unwrap()
                    .is_none()
                {
                    DatabaseStatus::DoesNotExist
                } else {
                    DatabaseStatus::Exists
                }
            }
        }
    }
}
