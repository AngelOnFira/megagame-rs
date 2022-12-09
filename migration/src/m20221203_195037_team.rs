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

#[derive(Iden)]
enum Category {
    Table,
    Id,
    DiscordId,
    GuildId,
    Name,
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
                    .col(ColumnDef::new(Team::Abreviation).string())
                    .col(ColumnDef::new(Team::Guild).integer())
                    .col(ColumnDef::new(Team::CreatedAt).date())
                    .col(ColumnDef::new(Team::UpdatedAt).date())
                    .col(ColumnDef::new(Team::Emoji).string())
                    .col(ColumnDef::new(Team::Wallet).integer())
                    .col(ColumnDef::new(Team::Role).integer())
                    .col(ColumnDef::new(Team::CategoryId).integer().null())
                    .col(ColumnDef::new(Team::GeneralChannelId).integer().null())
                    .col(ColumnDef::new(Team::TradeChannelId).integer().null())
                    .col(ColumnDef::new(Team::MenuChannelId).integer().null())
                    .col(ColumnDef::new(Team::BankEmbedId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_category_channel_ffk")
                            .from(Team::Table, Team::CategoryId)
                            .to(Category::Table, Category::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_general_channel_fk")
                            .from(Team::Table, Team::GeneralChannelId)
                            .to(Channel::Table, Channel::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_trade_channel_fk")
                            .from(Team::Table, Team::TradeChannelId)
                            .to(Channel::Table, Channel::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_menu_channel_fk")
                            .from(Team::Table, Team::MenuChannelId)
                            .to(Channel::Table, Channel::Id)
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
