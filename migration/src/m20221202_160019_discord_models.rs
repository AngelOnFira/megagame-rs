use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Guild {
    Table,
    DiscordId,
}

#[derive(Iden)]
enum Channel {
    Table,
    DiscordId,
    FKGuildId,
    Name,
}

#[derive(Iden)]
enum Role {
    Table,
    DiscordId,
    FKGuildId,
    Name,
}

#[derive(Iden)]
enum Category {
    Table,
    DiscordId,
    FKGuildId,
    Name,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Guild table
        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Guild::DiscordId).big_integer().primary_key())
                    .to_owned(),
            )
            .await?;

        // Create Channel table
        manager
            .create_table(
                Table::create()
                    .table(Channel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Channel::DiscordId)
                            .big_integer()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Channel::FKGuildId).big_integer().null())
                    .col(ColumnDef::new(Channel::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("channel_guild_fk")
                            .from(Channel::Table, Channel::FKGuildId)
                            .to(Guild::Table, Guild::DiscordId),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Role table
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Role::DiscordId).big_integer().primary_key())
                    .col(ColumnDef::new(Role::FKGuildId).big_integer().null())
                    .col(ColumnDef::new(Role::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("role_guild_fk")
                            .from(Role::Table, Role::FKGuildId)
                            .to(Guild::Table, Guild::DiscordId),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Category table
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::DiscordId)
                            .big_integer()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::FKGuildId).big_integer().null())
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("category_guild_fk")
                            .from(Category::Table, Category::FKGuildId)
                            .to(Guild::Table, Guild::DiscordId)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
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
