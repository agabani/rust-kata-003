#!/usr/bin/env bash
# set -x
set -eo pipefail

POSTGRES_USER="${POSTGRES_USER:=postgres}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:=password}"
POSTGRES_DB="${POSTGRES_DB:="rust-kata-003"}"
POSTGRES_PORT="${POSTGRES_PORT:=5432}"

REDIS_PORT="${REDIS_PORT:=6379}"

if [[ -z "${SKIP_DOCKER}" ]]; then
  POSTGRES_DOCKER_CONTAINER_ID=$(docker run \
    --health-cmd pg_isready \
    --health-interval 10s \
    --health-timeout 5s \
    --health-retries 5 \
    -e POSTGRES_USER="${POSTGRES_USER}" \
    -e POSTGRES_PASSWORD="${POSTGRES_PASSWORD}" \
    -e POSTGRES_DB="${POSTGRES_DB}" \
    -p "${POSTGRES_PORT}":5432 \
    -d \
    --rm \
    postgres \
    postgres -N 1000)

  REDIS_DOCKER_CONTAINER_ID=$(
    docker run \
      --health-cmd "redis-cli ping" \
      --health-interval 10s \
      --health-timeout 5s \
      --health-retries 5 \
      -p "${REDIS_PORT}":6379 \
      -d \
      --rm \
      redis
  )

  until [[ $(docker inspect --format="{{if .Config.Healthcheck}}{{print .State.Health.Status}}{{end}}" "$POSTGRES_DOCKER_CONTAINER_ID") == healthy ]]; do
    echo >&2 "postgres service is starting, waiting 1 seconds before checking again."
    sleep 1
  done

  until [[ $(docker inspect --format="{{if .Config.Healthcheck}}{{print .State.Health.Status}}{{end}}" "$REDIS_DOCKER_CONTAINER_ID") == healthy ]]; do
    echo >&2 "redis service is starting, waiting 1 seconds before checking again."
    sleep 1
  done
fi

echo >&2 "postgres is up and running on port ${POSTGRES_PORT}!"
echo >&2 "redis is up and running on port ${REDIS_PORT}!"


echo >&2 "postgres service is initializing."

export DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:${POSTGRES_PORT}/${POSTGRES_DB}
sqlx database create
sqlx migrate run

echo >&2 "postgres service is initialized!"
