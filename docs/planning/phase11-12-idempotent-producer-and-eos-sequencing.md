# Phase 11 → Phase 12 — Sequencing and exit criteria

`MASTER_PLAN.md` requires **Phase 11 (idempotent producer)** to complete before **Phase 12 (transactions / EOS)**. Do not run parallel refactors on producer state and transaction state.

## Phase 11 — Idempotent producer ([`tips/phases/11-idempotent-producer.md`](../../tips/phases/11-idempotent-producer.md))

**Storage contract (design first)**

- Durable map: `(producer_id, epoch)` → metadata; per `(topic, partition)` sequence window, last offset, fencing markers.
- Recovery after broker/storage restart must reload the same windows.

**Produce path**

- `ProduceService` consults storage **before** append for idempotent batches; exact errors: `OutOfOrderSequenceNumber`, `DuplicateSequenceNumber`, `UnknownProducerId`, `InvalidProducerEpoch`, `ProducerFenced`.

**Tests**

- Storage conformance: sequence gaps, duplicates, epoch bump, unknown producer.
- Broker: extend `pg_init_producer.rs`, `txn.rs` patterns for idempotent-only flows before full txn.
- Java / librdkafka idempotent retries — attach to Phase 04 harness when ready.

**Exit:** `enable.idempotence=true` survives retries and restarts with Kafka-equivalent errors (Phase 11 acceptance gate).

## Phase 12 — EOS ([`tips/phases/12-transactions-eos.md`](../../tips/phases/12-transactions-eos.md))

**Depends on:** Phase 11 durable producer state feeding the same storage abstractions transaction code uses (`AUDIT-006`).

**Scope**

- Transactional offset commit, abort markers, read_committed visibility coordination with Phase 08 Fetch/ListOffsets where specified.

**Exit:** End-to-end EOS proof in broker + storage tests + ledger rows for txn APIs; no duplicate producer-state paths.

## Verification

- Use isolated `CARGO_TARGET_DIR=/tmp/jansu-verify-phase11` (and phase12) per `AGENTS.md`.
- Update `phase-logs/11-*.md.log` / `12-*.md.log` and `AUDIT.md` when closing items.
