#!/bin/bash

sea-orm-cli migrate \
    --database-url=postgres://postgres:postgres@localhost:5432/postgres

rm -rf entity/src/entities
sea-orm-cli generate entity \
    -o entity/src/entities \
    --database-url=postgres://postgres:postgres@localhost:5432/postgres \
    --with-serde both \
    --expanded-format
