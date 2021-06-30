#!/usr/bin/env bash
set -x
set -eo pipefail
DB_USER="${POSTGRES_USER:=charlesonyewuenyi}" 
DB_PASSWORD="${POSTGRES_PASSWORD:=password}" 
DB_NAME="${POSTGRES_DB:=newsletter}" 
DB_PORT="${POSTGRES_PORT:=5432}"

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]; then
        docker run \
                -e POSTGRES_USER=${DB_USER} \
                -e POSTGRES_PASSWORD=${DB_PASSWORD} \
                -e POSTGRES_DB=${DB_NAME} \
                -p "${DB_PORT}":5432 \
                -d postgres \
                postgres -N 1000
fi

export PGPASSWORD="${DB_PASSWORD}"
sleep 10
# until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
#     >&2 echo "Postgres is still unavailable - sleeping"
#     sleep 1 
# done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"


# A new top-level directory should have now appeared in your project - migrations. This is where all migrations for our project will be stored by sqlx’s CLI.
# Under migrations you should already have one file called {timestamp}_create_subscriptions_table.sql
# - this is where we have to write the SQL code for our first migration
# Assuming you used the default parameters to launch Postgres in Docker!
# export DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter 
# sqlx migrate add create_subscriptions_table

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run
>&2 echo "Postgres has been migrated, ready to go!"