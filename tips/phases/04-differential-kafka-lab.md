# Phase 04 - Differential Kafka Lab

Parallelism: MCP-PARALLEL

Depends on: Phase 01 - Compatibility Contract

Can run with: Phase 02, Phase 03, Phase 05, and Phase 06 after the Phase 01 ledger shape is stable.

Goal: Build an Apache Kafka vs Jansu differential lab that runs the same client and protocol workloads against both systems, then records response, error, offset, metadata, timing, and lifecycle differences.

Current code anchors:
- `compose.yaml`, `justfile`, and `README.md` already support local Jansu, PostgreSQL, S3/MinIO, Prometheus, and Kafka CLI-style workflows.
- `jansu-broker/tests/` contains broker behavior tests for auth, consumer groups, fetch, list offsets, metadata, produce/fetch, topics, transactions, and policies.
- `jansu-sans-io/tests/` contains protocol tests and proptest coverage.
- `fuzz/` is available for protocol and malformed input expansion.
- `docs/sarama.md` documents a Go/Sarama path that can become a client fixture.
- `jansu-perf/` can become the seed for workload drivers and measurement.

Implementation steps:
- Add a test harness that can start Apache Kafka 4.2 and Jansu side by side with isolated bootstrap addresses.
- Create common workload descriptions for produce, fetch, list offsets, metadata, admin, consumer groups, idempotence, and transactions.
- Run Java client and librdkafka first, then franz-go and Sarama where practical.
- Capture ApiVersions negotiation, request versions chosen by clients, response errors, offsets, timestamps, high watermarks, group states, and timeout behavior.
- Add Kafka CLI fixtures for `kafka-topics`, `kafka-configs`, `kafka-get-offsets`, `kafka-console-producer`, `kafka-console-consumer`, and `kafka-consumer-groups`.
- Store differential results as CI artifacts and feed pass/fail status back to the Phase 01 ledger.
- Make the harness reusable by later phases instead of one-off scripts.

Tests:
- Add a smoke test that creates a topic, produces records, consumes records, and commits offsets against both Kafka and Jansu.
- Add an ApiVersions diff test that compares Jansu advertised APIs to the Phase 01 ledger and Kafka 4.2 reference behavior.
- Add negative differential tests for unsupported versions, unknown topics, invalid partitions, bad config names, and timeout boundaries.
- Add client matrix jobs that can be run in CI or manually when external tools are not installed.

Acceptance gate: CI can compare Jansu behavior against real Kafka for each advertised API and publish a result that updates or validates the compatibility ledger.

Do not do:
- Do not rely only on Rust unit tests for compatibility claims.
- Do not make a harness that only tests the happy path.
- Do not require external services without documenting exact versions and startup commands.
- Do not let differences be ignored without an explicit ledger entry.

Fresh session handoff: Start with one end-to-end workload using Java client or Kafka CLI against Kafka 4.2 and Jansu. Make result capture boring and repeatable before adding more clients.

## Completion policy added 2026-05-04

Phase 04 owns the differential infrastructure, artifact schema, Kafka 4.2 reference
target, and compatibility-ledger proof plumbing.

A semantic mismatch for an unadvertised API does not block Phase 04 completion. It
must be recorded as evidence for the owning phase. An advertised API mismatch does
block Phase 04 and must fail the differential lab.

## Supplement 2026-05-05 — Produce differential no longer ignored

- `jansu-broker/tests/differential_lab.rs::differential_produce_round_trip_for_advertised_api` runs whenever `JANSU_DIFFERENTIAL=1` (same gating as other external tests); the old `#[ignore]` was removed so ledger-listed Produce differential proof and the documented `cargo test -p jansu-broker --test differential_lab` command stay honest. Phase 08–owned ListOffsets/Fetch read evidence remains in the same file under separate tests.
