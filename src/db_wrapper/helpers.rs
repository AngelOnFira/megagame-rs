use std::{fmt::Debug, num::NonZeroU64, ops::Deref, sync::Arc};

use async_trait::async_trait;
use entity::entities::guild;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serenity::{
    all::UserId,
    client::Context,
    model::prelude::{ChannelId, Guild, GuildId},
};

use crate::{
    db_wrapper::{DBWrapper, TaskResult},
    task_runner::tasks::DiscordId,
};

/// Get the guild from the cache and the database. If the guild is not in the
/// database, it will be created.
pub async fn get_guild(ctx: Context, db: DBWrapper, guild_id: DiscordId) -> (Guild, guild::Model) {
    let discord_guild = ctx.cache.guild(guild_id).map(|g| g.clone()).unwrap();

    // Get or create the guild
    let guild_option = guild::Entity::find()
        .filter(guild::Column::DiscordId.eq(guild_id.to_string()))
        .one(&*db)
        .await
        .unwrap();

    let database_guild = match guild_option {
        Some(guild) => guild,
        None => guild::ActiveModel {
            discord_id: Set(*guild_id as i64),
            ..Default::default()
        }
        .insert(&*db)
        .await
        .unwrap(),
    };

    (discord_guild, database_guild)
}
