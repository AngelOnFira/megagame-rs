use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Currency {
    Table,
    Id,
    Name,
    Description,
    CurrencyType,
    Emoji,
}

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
                        ColumnDef::new(Currency::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Currency::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Currency::Description)
                            .string()
                            .default("".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Currency::CurrencyType)
                            .string()
                            .null()
                            .default("".to_owned()),
                    )
                    .col(ColumnDef::new(Currency::Emoji).string().null())
                    .to_owned(),
            )
            .await

        // class Transaction(models.Model):
        //     amount = models.IntegerField(default=0, null=True, blank=True)
        //     currency = models.ForeignKey(
        //         Currency, on_delete=models.PROTECT, null=True, blank=True
        //     )

        //     created_date = models.DateTimeField(default=timezone.now)
        //     modified_date = models.DateTimeField(default=timezone.now)

        //     from_wallet = models.ForeignKey(
        //         "Wallet",
        //         on_delete=models.PROTECT,
        //         default=None,
        //         related_name="credits",
        //         null=True,
        //         blank=True,
        //     )
        //     to_wallet = models.ForeignKey(
        //         "Wallet",
        //         on_delete=models.PROTECT,
        //         default=None,
        //         related_name="debits",
        //         null=True,
        //         blank=True,
        //     )

        //     initiating_player = models.ForeignKey(
        //         Player,
        //         on_delete=models.PROTECT,
        //         default=None,
        //         related_name="transaction",
        //         null=True,
        //         blank=True,
        //     )
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Currency::Table).to_owned())
            .await
    }
}
