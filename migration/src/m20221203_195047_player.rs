use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Player {
    Table,
    Id,
    DiscordId,
    Name,
    FKTeamId,
    FKGuildId,
}

#[derive(Iden)]
pub enum Team {
    Table,
    Id,
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
                    .col(ColumnDef::new(Player::DiscordId).big_integer().not_null())
                    .col(ColumnDef::new(Player::Name).string().not_null())
                    .col(ColumnDef::new(Player::FKTeamId).integer().null())
                    .col(ColumnDef::new(Player::FKGuildId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_team_id")
                            .from(Player::Table, Player::FKTeamId)
                            .to(Team::Table, Team::Id),
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
