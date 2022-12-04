pub use sea_orm_migration::prelude::*;

mod m20221203_145623_currency;
mod m20221203_194651_wallet;
mod m20221203_194657_transaction;
mod m20221203_194700_trade;
mod m20221203_195047_player;
mod m20221203_195057_team;

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
            Box::new(m20221203_195057_team::Migration),
        ]
    }
}
