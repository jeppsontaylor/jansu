#!/usr/bin/env bash
# Phase 04 — Kafka CLI Fixture Runner
#
# Runs the same Kafka CLI workloads against any target bootstrap address.
# Each fixture writes structured JSON metadata under the artifact directory.
#
# Usage:
#   kafka-cli-fixtures.sh <bootstrap> <target-name> [artifact-dir]
#
# Examples:
#   kafka-cli-fixtures.sh 127.0.0.1:19092 kafka42 target/differential/cli
#   kafka-cli-fixtures.sh 127.0.0.1:9092  jansu   target/differential/cli

set -euo pipefail

BOOTSTRAP="${1:?usage: kafka-cli-fixtures.sh <bootstrap> <target-name> [artifact-dir]}"
TARGET="${2:?usage: kafka-cli-fixtures.sh <bootstrap> <target-name> [artifact-dir]}"
ARTIFACT_DIR="${3:-target/differential/cli}"

mkdir -p "${ARTIFACT_DIR}"

timestamp_ms() {
  python3 - <<'PY'
import time
print(int(time.time() * 1000))
PY
}

# Find Kafka CLI tools: check PATH, $KAFKA_HOME, or use docker exec
find_kafka_bin() {
  local tool="$1"

  if command -v "${tool}" >/dev/null 2>&1; then
    echo "${tool}"
    return 0
  fi

  if command -v "${tool}.sh" >/dev/null 2>&1; then
    echo "${tool}.sh"
    return 0
  fi

  if [[ -n "${KAFKA_HOME:-}" && -x "${KAFKA_HOME}/bin/${tool}.sh" ]]; then
    echo "${KAFKA_HOME}/bin/${tool}.sh"
    return 0
  fi

  # Fall back to docker exec against the running Kafka container
  if docker ps --format '{{.Names}}' 2>/dev/null | grep -q 'jansu-differential-kafka42'; then
    echo "docker exec jansu-differential-kafka42 /opt/kafka/bin/${tool}.sh"
    return 0
  fi

  echo "missing Kafka CLI tool: ${tool}" >&2
  return 127
}

run_fixture() {
  local name="$1"
  shift

  local ts
  ts="$(timestamp_ms)"
  local out="${ARTIFACT_DIR}/${TARGET}-${name}-${ts}.out"
  local err="${ARTIFACT_DIR}/${TARGET}-${name}-${ts}.err"
  local meta="${ARTIFACT_DIR}/${TARGET}-${name}-${ts}.json"

  set +e
  eval "$@" >"${out}" 2>"${err}"
  local status=$?
  set -e

  cat > "${meta}" <<JSON
{
  "schema_version": 1,
  "target": "${TARGET}",
  "fixture": "${name}",
  "bootstrap": "${BOOTSTRAP}",
  "exit_status": ${status},
  "stdout": "${out}",
  "stderr": "${err}",
  "timestamp_ms": ${ts}
}
JSON

  echo "fixture ${name}: exit=${status}"
  return 0
}

TOPIC="phase04-cli-${TARGET}-$(timestamp_ms)"

BROKER_API_VERSIONS="$(find_kafka_bin kafka-broker-api-versions)"
TOPICS="$(find_kafka_bin kafka-topics)"
GET_OFFSETS="$(find_kafka_bin kafka-get-offsets)"
PRODUCER="$(find_kafka_bin kafka-console-producer)"
CONSUMER="$(find_kafka_bin kafka-console-consumer)"
CONSUMER_GROUPS="$(find_kafka_bin kafka-consumer-groups)"

run_fixture broker-api-versions \
  "${BROKER_API_VERSIONS}" --bootstrap-server "${BOOTSTRAP}"

run_fixture topic-create \
  "${TOPICS}" --bootstrap-server "${BOOTSTRAP}" \
  --create --if-not-exists --topic "${TOPIC}" \
  --partitions 1 --replication-factor 1

run_fixture topic-describe \
  "${TOPICS}" --bootstrap-server "${BOOTSTRAP}" \
  --describe --topic "${TOPIC}"

printf 'k1\tv1\nk2\tv2\n' | run_fixture console-producer \
  "${PRODUCER}" --bootstrap-server "${BOOTSTRAP}" \
  --topic "${TOPIC}" \
  --property parse.key=true \
  --property key.separator=$'\t'

run_fixture get-offsets-earliest \
  "${GET_OFFSETS}" --bootstrap-server "${BOOTSTRAP}" \
  --topic "${TOPIC}" --time earliest

run_fixture get-offsets-latest \
  "${GET_OFFSETS}" --bootstrap-server "${BOOTSTRAP}" \
  --topic "${TOPIC}" --time latest

run_fixture console-consumer \
  "${CONSUMER}" --bootstrap-server "${BOOTSTRAP}" \
  --topic "${TOPIC}" \
  --from-beginning \
  --timeout-ms 10000 \
  --property print.key=true \
  --property print.offset=true \
  --property print.partition=true \
  --property print.value=true

run_fixture consumer-groups-list \
  "${CONSUMER_GROUPS}" --bootstrap-server "${BOOTSTRAP}" --list

echo "all fixtures complete for target=${TARGET}"
