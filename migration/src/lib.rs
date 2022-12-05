pub use sea_orm_migration::prelude::*;

pub mod m20221203_145623_currency;
pub mod m20221203_194651_wallet;
pub mod m20221203_194657_transaction;
pub mod m20221203_194700_trade;
pub mod m20221203_195047_player;
pub mod m20221203_195037_team;
mod m20221204_194750_task;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221203_145623_currency::Migration),
            Box::new(m20221203_194651_wallet::Migration),
            Box::new(m20221203_194657_transaction::Migration),
            Box::new(m20221203_194700_trade::Migration),
            Box::new(m20221203_195047_player::Migration),
            Box::new(m20221203_195037_team::Migration),
            Box::new(m20221204_194750_task::Migration),
        ]
    }
}
