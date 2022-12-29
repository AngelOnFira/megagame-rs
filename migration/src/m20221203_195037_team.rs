use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum Team {
    Table,
    Id,
    Name,
    Abreviation,
    CreatedAt,
    UpdatedAt,
    Emoji,
    Wallet,
    FKGuildId,
    FKTeamRoleId,
    FKTeamCategoryId,
    FKGeneralChannelId,
    FKTradeChannelId,
    FKMenuChannelId,
    FKBankEmbedId,
}

#[derive(Iden)]
enum Guild {
    Table,
    DiscordId,
}

#[derive(Iden)]
enum Channel {
    Table,
    DiscordId,
}

#[derive(Iden)]
enum Category {
    Table,
    DiscordId,
}

#[derive(Iden)]
enum Role {
    Table,
    DiscordId,
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
                    .col(ColumnDef::new(Team::CreatedAt).date())
                    .col(ColumnDef::new(Team::UpdatedAt).date())
                    .col(ColumnDef::new(Team::Emoji).string())
                    .col(ColumnDef::new(Team::Wallet).integer())
                    .col(ColumnDef::new(Team::FKBankEmbedId).integer())
                    // Team guild
                    .col(ColumnDef::new(Team::FKGuildId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_guild_fk")
                            .from(Team::Table, Team::FKGuildId)
                            .to(Guild::Table, Guild::DiscordId),
                    )
                    // Team role
                    .col(ColumnDef::new(Team::FKTeamRoleId).integer())
                    .foreign_key(
                        ForeignKey::create()
                        .name("team_role_fk")
                        .from(Team::Table, Team::FKTeamRoleId)
                        .to(Role::Table, Role::DiscordId),
                    )
                    // Team category
                    .col(ColumnDef::new(Team::FKTeamCategoryId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                        .name("team_category_fk")
                        .from(Team::Table, Team::FKTeamCategoryId)
                        .to(Category::Table, Category::DiscordId),
                    )
                    // Team general channel
                    .col(ColumnDef::new(Team::FKGeneralChannelId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                        .name("team_general_channel_fk")
                        .from(Team::Table, Team::FKGeneralChannelId)
                        .to(Channel::Table, Channel::DiscordId),
                    )
                    // Team trade channel
                    .col(ColumnDef::new(Team::FKTradeChannelId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                        .name("team_trade_channel_fk")
                        .from(Team::Table, Team::FKTradeChannelId)
                        .to(Channel::Table, Channel::DiscordId),
                    )
                    // Team menu channel
                    .col(ColumnDef::new(Team::FKMenuChannelId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_menu_channel_fk")
                            .from(Team::Table, Team::FKMenuChannelId)
                            .to(Channel::Table, Channel::DiscordId),
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
