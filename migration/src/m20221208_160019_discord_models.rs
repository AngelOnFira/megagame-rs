use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Guild {
    Table,
    Id,
    DiscordId,
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
enum Role {
    Table,
    Id,
    DiscordId,
    GuildId,
    Name,
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
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Guild::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Guild::DiscordId).big_integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Channel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Channel::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Channel::DiscordId).big_integer().not_null())
                    .col(ColumnDef::new(Channel::GuildId).integer().not_null())
                    .col(ColumnDef::new(Channel::Name).string().not_null())
                    .col(ColumnDef::new(Channel::AllowNSFW).boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_team_id")
                            .from(Channel::Table, Channel::GuildId)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Role::DiscordId).big_integer().not_null())
                    .col(ColumnDef::new(Role::GuildId).integer().not_null())
                    .col(ColumnDef::new(Role::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_team_id")
                            .from(Role::Table, Role::GuildId)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::DiscordId).big_integer().not_null())
                    .col(ColumnDef::new(Category::GuildId).integer().not_null())
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_team_id")
                            .from(Category::Table, Category::GuildId)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Channel::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await?;

        Ok(())
    }
}
