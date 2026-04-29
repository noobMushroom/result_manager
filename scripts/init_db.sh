#!/usr/bin/env bash

set -x
set -eo pipefail
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=users}"
DB_PORT="${POSTGRES_PORT:=5432}"
REDIS_HOST=${REDIS_HOST:=localhost}
REDIS_PORT=${REDIS_PORT:=6379}
REDIS_PASSWORD=${REDIS_PASSWORD:=password}
if [[ -z "${SKIP_DOCKER}" ]]
then
docker run \
-e POSTGRES_USER=${DB_USER} \
-e POSTGRES_PASSWORD=${DB_PASSWORD} \
-e POSTGRES_DB=${DB_NAME} \
-p "${DB_PORT}":5432 \
-d postgres \
postgres -N 1000

docker run \
-p "${REDIS_PORT}:6379" \
-d redis \
redis-server --requirepass "${REDIS_PASSWORD}"
fi
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

until redis-cli -h "${REDIS_HOST}" -p "${REDIS_PORT}" -a "${REDIS_PASSWORD}" ping; do
  >&2 echo "Redis is still unavailable - sleeping"
  sleep 1
done
>&2 echo "Redis is up!"
REDIS_URL="redis://:${REDIS_PASSWORD}@${REDIS_HOST}:${REDIS_PORT}/0"

sqlx database create
sqlx migrate run
>&2 echo "Postgres has been migrated, ready to go!"
