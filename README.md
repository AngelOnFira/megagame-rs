# Megagame.rs

## Rebuiding the database

```bash
dropdb \
    -U postgres \
    -h localhost \
    -p 5432 \
    -w postgres; \
createdb \
    -U postgres \
    -h localhost \
    -p 5432 \
    -w postgres; \
sea-orm-cli migrate && \
sea-orm-cli generate entity \
    -o entity/src/entities \
    --expanded-format \
    --with-serde both
```

## Making a new migration

```bash
sea-orm-cli migrate generate <name>
```
