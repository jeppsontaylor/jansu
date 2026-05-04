# Phase 07 — Session 2: Advertisement Completion

## Agent

Antigravity (Gemini)

## Prompt

Complete Phase 07 Produce Exactness to 100% — implement remaining gaps, fix bugs, add tests, advertise Produce, and update ledger/contract.

## Phase Or Audit Item

Phase 07 / AUDIT-003

## Files Read

- AGENTS.md
- MASTER_PLAN.md
- AUDIT.md
- phase-logs/index.json
- tips/phases/07-produce-exactness.md
- tips/jansu_phases_tips/phase-07/tip1.txt
- tips/jansu_phases_tips/phase-07/tip2.txt
- phase-logs/07-produce-exactness.md.log
- jansu-broker/src/service.rs
- jansu-broker/tests/produce_exactness.rs
- jansu-broker/tests/compatibility_contract.rs
- jansu-storage/src/service/produce.rs
- jansu-storage/tests/produce_exactness.rs
- jansu-sans-io/src/record/deflated.rs
- jansu-sans-io/src/lib.rs
- jansu-service/src/stream.rs
- docs/compatibility/kafka-4.2-ledger.json

## Files Changed

- jansu-sans-io/src/record/deflated.rs (Snappy encode support)
- jansu-service/src/stream.rs (frame-size bug fix)
- jansu-storage/src/service/produce.rs (batch-size validation)
- jansu-broker/src/service.rs (Produce advertisement)
- jansu-broker/tests/produce_exactness.rs (replaced negative test, added acks=0 storage proof)
- jansu-broker/tests/compatibility_contract.rs (Produce in expected APIs)
- jansu-storage/tests/produce_exactness.rs (Snappy in matrix, oversized record, acks=0)
- docs/compatibility/kafka-4.2-ledger.json (API key 0 advertised)
- AUDIT.md (AUDIT-003 resolved)
- phase-logs/07-produce-exactness.md.log (Session 2 appended)

## Tests Added

- jansu-storage/tests/produce_exactness.rs::oversized_record_is_rejected_without_append
- jansu-storage/tests/produce_exactness.rs::acks_zero_is_valid_and_appends
- jansu-broker/tests/produce_exactness.rs::api_versions_advertises_produce_after_phase_07
- jansu-broker/tests/produce_exactness.rs::produce_acks_zero_real_storage_appends_but_suppresses_wire

## Verification Commands

```
env CARGO_TARGET_DIR=/tmp/jansu-verify-phase07 cargo test -p jansu-storage --test produce_exactness --no-default-features --features dynostore  # 16 passed
env CARGO_TARGET_DIR=/tmp/jansu-verify-phase07 cargo test -p jansu-broker --test produce_exactness --all-features  # 5 passed
env CARGO_TARGET_DIR=/tmp/jansu-verify-phase07 cargo test -p jansu-broker --test compatibility_contract --all-features  # 15 passed
env CARGO_TARGET_DIR=/tmp/jansu-verify-phase07 cargo test -p jansu-sans-io --all-features --lib -- deflated  # 8 passed
```

## Outcome

All 44 tests pass (16 storage + 5 broker produce + 15 contract + 8 deflated). Phase 07 is complete.

## Residual Risks

- v12/v13 topic-ID Produce not implemented (protocol codegen change).
- Differential Java/librdkafka proof deferred to Phase 04/AUDIT-012.
- DynoStore lake-sink watermark ordering may need separate audit.

## Next Recommended Action

Focus on Phase 08 (Fetch/ListOffsets) or Phase 04 (Differential Kafka Lab) to build the differential harness needed for full client proof.
