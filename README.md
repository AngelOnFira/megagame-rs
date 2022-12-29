## Blank database

`sqlite3 db.sqlite3 "VACUUM;"`

## Running migrations

`sea-orm-cli migrate`

## Generating entities

```bash
sea-orm-cli generate entity \
    -o entity/src/entities \
    --expanded-format \
    --with-serde both
```

## Rebuiding the database

```bash
sea-orm-cli migrate
sea-orm-cli generate entity \
    -o entity/src/entities \
    --expanded-format \
    --with-serde both
```

## Making a new migration

```bash
sea-orm-cli migrate generate <name>
```
