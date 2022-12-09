use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Trade {
    Table,
    Id,
    InitiatingParty,
    ReceivingParty,
    Transactions,
}

// TODO: Fix this migration

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .create_table(
        //         Table::create()
        //             .table(Post::Table)
        //             .if_not_exists()
        //             .col(
        //                 ColumnDef::new(Post::Id)
        //                     .integer()
        //                     .not_null()
        //                     .auto_increment()
        //                     .primary_key(),
        //             )
        //             .col(ColumnDef::new(Post::Title).string().not_null())
        //             .col(ColumnDef::new(Post::Text).string().not_null())
        //             .to_owned(),
        //     )
        //     .await
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .drop_table(Table::drop().table(Post::Table).to_owned())
        //     .await
        Ok(())
    }
}
