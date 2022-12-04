use sea_orm_migration::prelude::*;

use crate::m20221203_195037_team::Team;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Player {
    Table,
    Id,
    Name,
    TeamId,
    Guild,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Player::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Player::Name).string().not_null())
                    .col(ColumnDef::new(Player::TeamId).integer().not_null())
                    .col(ColumnDef::new(Player::Guild).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_team_id")
                            .from(Player::Table, Player::TeamId)
                            .to(Team::Table, Team::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await
    }
}
