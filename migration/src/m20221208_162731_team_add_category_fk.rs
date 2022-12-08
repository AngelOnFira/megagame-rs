use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum Team {
    Table,
    Category,
    CategoryId,
    GeneralChannel,
    GeneralChannelId,
    TradeChannel,
    TradeChannelId,
    MenuChannel,
    MenuChannelId,
    BankEmbed,
    BankEmbedId,
}

#[derive(Iden)]
enum Category {
    Table,
    Id,
}

#[derive(Iden)]
enum Channel {
    Table,
    Id,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Team::Table)
                    // Remove the old category column
                    .drop_column(Team::Category)
                    // Add the new category column
                    .add_column(ColumnDef::new(Team::CategoryId).integer().not_null())
                    // Add the foreign key
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("team_category_fk")
                            .from_tbl(Team::Table)
                            .from_col(Team::CategoryId)
                            .to_tbl(Category::Table)
                            .to_col(Category::Id)
                            .to_owned(),
                    )
                    // Remove the old general channel column
                    .drop_column(Team::GeneralChannel)
                    // Add the new general channel column
                    .add_column(ColumnDef::new(Team::GeneralChannelId).integer().not_null())
                    // Add the foreign key
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("team_general_channel_fk")
                            .from_tbl(Team::Table)
                            .from_col(Team::GeneralChannelId)
                            .to_tbl(Channel::Table)
                            .to_col(Channel::Id)
                            .to_owned(),
                    )
                    // Remove the old trade channel column
                    .drop_column(Team::TradeChannel)
                    // Add the new trade channel column
                    .add_column(ColumnDef::new(Team::TradeChannelId).integer().not_null())
                    // Add the foreign key
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("team_trade_channel_fk")
                            .from_tbl(Team::Table)
                            .from_col(Team::TradeChannelId)
                            .to_tbl(Channel::Table)
                            .to_col(Channel::Id)
                            .to_owned(),
                    )
                    // Remove the old menu channel column
                    .drop_column(Team::MenuChannel)
                    // Add the new menu channel column
                    .add_column(ColumnDef::new(Team::MenuChannelId).integer().not_null())
                    // Add the foreign key
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("team_menu_channel_fk")
                            .from_tbl(Team::Table)
                            .from_col(Team::MenuChannelId)
                            .to_tbl(Channel::Table)
                            .to_col(Channel::Id)
                            .to_owned(),
                    )
                    // Remove the old bank embed column
                    .drop_column(Team::BankEmbed)
                    // Add the new bank embed column
                    .add_column(ColumnDef::new(Team::BankEmbedId).integer().not_null())
                    // Add the foreign key
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("team_bank_embed_fk")
                            .from_tbl(Team::Table)
                            .from_col(Team::BankEmbedId)
                            .to_tbl(Channel::Table)
                            .to_col(Channel::Id)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Team::Table)
                    // Remove the foreign key
                    .drop_foreign_key(Alias::new("team_category_fk"))
                    // Remove the new category column
                    .drop_column(Team::CategoryId)
                    // Add the old category column
                    .add_column(ColumnDef::new(Team::Category).text().not_null())
                    // Remove the foreign key
                    .drop_foreign_key(Alias::new("team_general_channel_fk"))
                    // Remove the new general channel column
                    .drop_column(Team::GeneralChannelId)
                    // Add the old general channel column
                    .add_column(ColumnDef::new(Team::GeneralChannel).text().not_null())
                    // Remove the foreign key
                    .drop_foreign_key(Alias::new("team_trade_channel_fk"))
                    // Remove the new trade channel column
                    .drop_column(Team::TradeChannelId)
                    // Add the old trade channel column
                    .add_column(ColumnDef::new(Team::TradeChannel).text().not_null())
                    // Remove the foreign key
                    .drop_foreign_key(Alias::new("team_menu_channel_fk"))
                    // Remove the new menu channel column
                    .drop_column(Team::MenuChannelId)
                    // Add the old menu channel column
                    .add_column(ColumnDef::new(Team::MenuChannel).text().not_null())
                    // Remove the foreign key
                    .drop_foreign_key(Alias::new("team_bank_embed_fk"))
                    // Remove the new bank embed column
                    .drop_column(Team::BankEmbedId)
                    // Add the old bank embed column
                    .add_column(ColumnDef::new(Team::BankEmbed).text().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
