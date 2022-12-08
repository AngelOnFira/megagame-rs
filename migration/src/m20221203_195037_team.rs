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
    CategoryId,
    GeneralChannelId,
    TradeChannelId,
    MenuChannelId,
    BankEmbedId,
}

#[derive(Iden)]
enum Channel {
    Table,
    Id,
    DiscordId,
    GuildId,
    Name,
    AllowNSFW,
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
                    .col(ColumnDef::new(Team::CategoryId).integer().not_null())
                    .col(ColumnDef::new(Team::GeneralChannelId).integer().not_null())
                    .col(ColumnDef::new(Team::TradeChannelId).integer().not_null())
                    .col(ColumnDef::new(Team::MenuChannelId).integer().not_null())
                    .col(ColumnDef::new(Team::BankEmbedId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_category_channel_fk")
                            .from(Team::Table, Team::CategoryId)
                            .to(Channel::Table, Channel::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_general_channel_fk")
                            .from(Team::Table, Team::GeneralChannelId)
                            .to(Channel::Table, Channel::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_trade_channel_fk")
                            .from(Team::Table, Team::TradeChannelId)
                            .to(Channel::Table, Channel::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_menu_channel_fk")
                            .from(Team::Table, Team::MenuChannelId)
                            .to(Channel::Table, Channel::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
