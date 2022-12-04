use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum Team {
    Table,
    Id,
    Name,
    Abreviation,
    Guild,
    CreatedAt,
    UpdatedAt,
    Emoji,
    Wallet,
    Role,
    Category,
    GeneralChannel,
    TradeChannel,
    MenuChannel,
    BankEmbedId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Team::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Team::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Team::Name).string().not_null())
                    .col(ColumnDef::new(Team::Abreviation).string().not_null())
                    .col(ColumnDef::new(Team::Guild).integer().not_null())
                    .col(ColumnDef::new(Team::CreatedAt).date().not_null())
                    .col(ColumnDef::new(Team::UpdatedAt).date().not_null())
                    .col(ColumnDef::new(Team::Emoji).string().not_null())
                    .col(ColumnDef::new(Team::Wallet).integer().not_null())
                    .col(ColumnDef::new(Team::Role).integer().not_null())
                    .col(ColumnDef::new(Team::Category).integer().not_null())
                    .col(ColumnDef::new(Team::GeneralChannel).integer().not_null())
                    .col(ColumnDef::new(Team::TradeChannel).integer().not_null())
                    .col(ColumnDef::new(Team::MenuChannel).integer().not_null())
                    .col(ColumnDef::new(Team::BankEmbedId).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Team::Table).to_owned())
            .await
    }
}
