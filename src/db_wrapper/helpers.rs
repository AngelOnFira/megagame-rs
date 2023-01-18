use entity::entities::{guild, player, team};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use serenity::{client::Context, model::prelude::Guild};

use crate::{db_wrapper::DBWrapper, task_runner::tasks::DiscordId};

#[derive(Debug)]
pub enum GameDatabaseError {
    PlayerNotFound,
    TeamNotFound,
    GuildNotFound,
}

/// Get the guild from the cache and the database. If the guild is not in the
/// database, it will be created.
pub async fn get_guild(ctx: Context, db: DBWrapper, guild_id: DiscordId) -> (Guild, guild::Model) {
    let discord_guild = ctx.cache.guild(guild_id).map(|g| g.clone()).unwrap();

    // Get or create the guild
    let guild_option = guild::Entity::find()
        .filter(guild::Column::DiscordId.eq(*guild_id as i64))
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

pub async fn get_player_team(
    ctx: Context,
    db: DBWrapper,
    guild_id: DiscordId,
    player_id: DiscordId,
) -> Result<team::Model, GameDatabaseError> {
    let guild = get_guild(ctx, db.clone(), guild_id).await.1;

    let player = player::Entity::find()
        .filter(player::Column::DiscordId.eq(*player_id as i64))
        .one(&*db)
        .await
        .unwrap();

    let player = match player {
        Some(player) => player,
        None => return Err(GameDatabaseError::PlayerNotFound),
    };

    let team = team::Entity::find()
        .filter(team::Column::Id.eq(player.fk_team_id))
        .one(&*db)
        .await
        .unwrap();

    let team = match team {
        Some(team) => team,
        None => return Err(GameDatabaseError::TeamNotFound),
    };

    if team.fk_guild_id != guild.discord_id {
        return Err(GameDatabaseError::GuildNotFound);
    }

    Ok(team)
}

pub async fn get_or_create_player(
    _ctx: Context,
    db: DBWrapper,
    guild_id: DiscordId,
    player_id: DiscordId,
    name: String,
) -> Result<player::Model, ()> {
    // Get the player from the database or create it if it doesn't exist
    let player_option = player::Entity::find()
        .filter(player::Column::DiscordId.eq(*player_id as i64))
        .one(&*db)
        .await
        .unwrap();

    let database_player = match player_option {
        Some(player) => player,
        None => player::ActiveModel {
            discord_id: Set(*player_id as i64),
            fk_guild_id: Set(*guild_id as i64),
            name: Set(name),
            ..Default::default()
        }
        .insert(&*db)
        .await
        .unwrap(),
    };

    Ok(database_player)
}
