use async_trait::async_trait;

use serenity::{
    all::ResolvedOption, builder::CreateCommand, model::prelude::GuildId, prelude::Context,
};

use crate::db_wrapper::DBWrapper;

pub mod fake_trade;
pub mod initialize_game;
pub mod nuke;

/// The `GameCommand` trait defines methods for registering and running game
/// commands within the Serenity Discord bot crate. The register method allows a
/// `CreateApplicationCommand` instance to be registered as a game command, and
/// the run method takes a list of `CommandDataOptions` as input and returns a
/// string result. These methods enable developers to easily create and execute
/// custom game commands within the Discord bot.
#[async_trait]
pub trait GameCommand {
    fn register() -> CreateCommand;
    async fn run(
        options: &[ResolvedOption],
        guild_id: GuildId,
        db: DBWrapper,
        ctx: Context,
    ) -> String;
}
