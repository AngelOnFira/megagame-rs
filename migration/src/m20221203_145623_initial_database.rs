use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // class Currency(models.Model):
        //     name = models.CharField(max_length=20, unique=True)
        //     description = models.TextField(default="")

        //     currency_type = models.CharField(
        //         max_length=3, choices=CurrencyType.choices, default=CurrencyType.HIDDEN
        //     )

        //     emoji = models.CharField(max_length=30, blank=True, null=True)
        manager
            .create_table(
                Table::create()
                    .table(Currency::Table)
                    .if_not_exists()
                    .col(
                        Column::create()
                            .name(Currency::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        Column::create()
                            .name(Currency::Name)
                            .text()
                            .not_null()
                            .unique(),
                    )
                    .col(
                        Column::create()
                            .name(Currency::Description)
                            .text()
                            .default(""),
                    )
                    .col(
                        Column::create()
                            .name(Currency::CurrencyType)
                            .text()
                            .not_null()
                            .default(CurrencyType::Hidden),
                    )
                    .col(Column::create().name(Currency::Emoji).text().default("")),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Currency {
    Table,
    Id,
    Name,
    Description,
    CurrencyType,
    Emoji,
}
