#!/bin/bash

rm -rf src/schema/
~/.cargo/bin/sea-orm-cli generate entity -o src/schema \
    --with-serde both \
    --tables \
        tasks_task
