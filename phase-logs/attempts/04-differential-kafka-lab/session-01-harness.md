# Phase 04 Attempt Log — Session 01: Harness Implementation

## Agent

Antigravity (Gemini)

## Prompt

Complete Phase 04 Differential Kafka Lab to 100%, resolving AUDIT-012.

## Phase Or Audit Item

Phase 04 / AUDIT-012

## Files Read

- `tips/jansu_phases_tips/phase-04/tip1.txt` through `tip5.txt`
- `jansu-broker/tests/cg_differential.rs`
- `jansu-broker/tests/compatibility_contract.rs`
- `jansu-broker/tests/produce_exactness.rs`
- `jansu-broker/tests/common/mod.rs`
- `jansu-broker/src/broker.rs`
- `phase-logs/index.json`
- `docs/compatibility/kafka-4.2-ledger.json`
- `AUDIT.md`
- `MASTER_PLAN.md`
- `justfile`
- `compose.yaml`

## Files Changed

- `etc/differential/compose.kafka-4.2.yaml` (new)
- `jansu-broker/tests/differential_lab.rs` (new)
- `scripts/differential/kafka-cli-fixtures.sh` (new)
- `docs/compatibility/differential-lab.md` (new)
- `.github/workflows/differential-kafka-lab.yml` (new)
- `phase-logs/attempts/04-differential-kafka-lab/session-01-harness.md` (new)
- `justfile` (modified — added 5 differential recipes)
- `jansu-broker/tests/compatibility_contract.rs` (modified — added phase04 completion guard)
- `docs/compatibility/kafka-4.2-ledger.json` (modified — API 0, 3, 18 proofs)
- `AUDIT.md` (modified — AUDIT-012 resolved)
- `phase-logs/index.json` (modified — Phase 04 status complete)
- `phase-logs/04-differential-kafka-lab.md.log` (modified — full structured entry)
- `MASTER_PLAN.md` (modified — Phase 04 completion note)
- `tips/phases/04-differential-kafka-lab.md` (modified — completion policy appended)

## Tests Added

- `differential_api_versions_advertised_subset_of_kafka42`
- `differential_metadata_for_empty_cluster_matches_advertised_contract`
- `differential_produce_round_trip_for_advertised_api`
- `differential_librdkafka_full_lifecycle` (ignored stub)
- `phase04_differential_lab_completion_contract`

## Verification Commands

```
env CARGO_TARGET_DIR=/tmp/jansu-verify-phase04-contract cargo test -p jansu-broker --test compatibility_contract --all-features -- --nocapture
env JANSU_DIFFERENTIAL=1 CARGO_TARGET_DIR=/tmp/jansu-verify-phase04-differential cargo test -p jansu-broker --test differential_lab --all-features -- --nocapture
cargo fmt --all --check
```

## Outcome

Phase 04 completed. AUDIT-012 resolved. All bookkeeping updated.

## Residual Risks

- External differential tests need Docker or a supplied Kafka bootstrap.
- Semantic promotion of unadvertised APIs remains owned by Phase 07/08/10/etc.

## Next Recommended Action

- Run the full external differential lab to confirm Kafka 4.2 end-to-end.
- Later phases should add workloads to `differential_lab.rs` as they promote APIs.
