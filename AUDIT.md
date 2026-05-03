# AUDIT

`AUDIT.md` is the canonical audit backlog for correctness, compatibility, logging, and verification debt. Status values are `open`, `in-progress`, `blocked`, and `resolved`. Every item must point to a phase or to `cross-phase`.

## Open

- `AUDIT-001` status: `open`; phase: `cross-phase`; area: agent context. Keep `AGENTS.md`, `MASTER_PLAN.md`, `AUDIT.md`, tool-specific pointer files, `phase-logs/README.md`, and `phase-logs/index.json` aligned as the command surface evolves.
- `AUDIT-002` status: `open`; phase: `cross-phase`; area: rename fallout. Continue checking for user-facing legacy project-name references that should now be `jansu`, without touching historical paths or unrelated generated content.
- `AUDIT-003` status: `open`; phase: `07`; area: produce exactness. Produce remains unadvertised until stable client or differential proof is recorded in the compatibility ledger.
- `AUDIT-004` status: `in-progress`; phase: `08`; area: fetch/list-offsets. Leader epoch truthfulness fixed in ListOffsetsService (no longer hardcoded 0). API 23 routed through broker but remains unadvertised. SQLite, DynoStore, and SlateDB now initialize epoch 0 and persist storage-driven epoch transitions on produce; Fetch response current leader enrichment is covered by focused broker tests. Remaining: broker ListOffsets wire-level mixed-partition decoding failure, differential/client proof, API 23 advertisement proof, and truncation/recovery/read-committed integration.
- `AUDIT-006` status: `open`; phase: `12`; area: transactions. Verify transactional offset commits, producer epoch fencing, abort visibility, and end-to-end EOS semantics.
- `AUDIT-007` status: `open`; phase: `13`; area: retention and compaction. Verify delete-records, retention, compaction, tombstones, and storage-engine certification.
- `AUDIT-008` status: `open`; phase: `14`; area: security. Verify ACL and quota behavior before exposing compatibility claims for secured deployments.
- `AUDIT-009` status: `open`; phase: `15`; area: modern metadata. Verify cluster, controller, topic partition, broker registration, and Kafka 4.2 metadata semantics.
- `AUDIT-010` status: `open`; phase: `16`; area: ecosystem. Add durable Java, librdkafka, franz-go or Sarama, CLI, performance, and migration proof where phase scope requires it.

## In Progress

- `AUDIT-015` status: `in-progress`; phase: `10`; area: consumer group hardening. DynoStore `delete_groups` fixed to use `head()` before `delete()` (InMemory object_store returns Ok for non-existent keys). Three `#[ignore]` annotations removed from `delete_unknown_consumer_group` tests. Three failure-mode unit tests added: `leave_unknown_member_returns_per_member_error`, `sync_with_wrong_generation_returns_illegal_generation`, `heartbeat_with_stale_generation_returns_illegal_generation`. Ledger failure_modes upgraded from `untested` to `partial` for all Phase 10 APIs. PG migrations remain outstanding.

## Blocked

- `AUDIT-012` status: `blocked`; phase: `04`; area: differential Kafka lab. Full differential certification is blocked until a stable local Kafka harness and client matrix are available in the repo.

## Resolved

- `AUDIT-005` status: `resolved`; phase: `10`; area: consumer groups. Postgres-backed coordinator proof (`cg_dynamic`/`cg_static` `pg` modules with `just ci`), storage offset round-trip and `maintain` expiry cleanup (`jansu-storage/tests/consumer_offsets.rs` postgres tests), init schema `leader_epoch_history` in `etc/initdb.d/010-schema.sql`, `PgStorage::maintain` fixed to run `consumer_offset_delete_expired.sql` via `prepare_execute`, ledger proofs extended for phase-owned API keys; see `phase-logs/10-consumer-groups-offsets.md.log`.
- `AUDIT-013` status: `resolved`; phase: `01`; area: compatibility contract. Initial ledger and compatibility-contract tests exist for protocol coverage, route coverage, advertised API versions, and proof-backed compatibility claims.
- `AUDIT-014` status: `resolved`; phase: `02`; area: API version truth. ApiVersions advertisement is ledger-gated and tested by the compatibility contract.
- `AUDIT-011` status: `resolved`; phase: `cross-phase`; area: phase logs. Every phase now has a canonical log, an attempt directory, and manifest coverage enforced by compatibility-contract tests.
