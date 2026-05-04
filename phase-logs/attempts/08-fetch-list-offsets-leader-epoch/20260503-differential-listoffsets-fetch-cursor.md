# Attempt log

## Agent

Cursor (GPT-5.2)

## Prompt

Study tips and MASTER_PLAN; next logical phase; update logs; keep going until work is hardened and verified.

## Phase Or Audit Item

Phase **08** — `AUDIT-004` (Fetch / ListOffsets / differential evidence)

## Files Read

- `AGENTS.md`, `MASTER_PLAN.md`, `AUDIT.md`, `phase-logs/index.json`
- `tips/phases/08-fetch-list-offsets-leader-epoch.md`
- `docs/compatibility/differential-lab.md`, `docs/compatibility/kafka-4.2-ledger.json`
- `jansu-broker/tests/differential_lab.rs` (existing harness)

## Files Changed

- `jansu-broker/tests/differential_lab.rs`
- `docs/compatibility/kafka-4.2-ledger.json`
- `AUDIT.md`
- `phase-logs/08-fetch-list-offsets-leader-epoch.md.log`
- `phase-logs/index.json`
- `phase-logs/attempts/08-fetch-list-offsets-leader-epoch/20260503-differential-listoffsets-fetch-cursor.md`
- `docs/compatibility/differential-lab.md`
- `tips/phases/08-fetch-list-offsets-leader-epoch.md`

## Tests Added

- `jansu-broker/tests/differential_lab.rs::differential_listoffsets_latest_and_fetch_consume_after_produce`

## Verification Commands

- `cargo check -p jansu-broker --test differential_lab --all-features`
- `cargo test -p jansu-broker --test differential_lab --all-features -- --nocapture`
- `just compatibility-contract`
- `cargo fmt --all --check`

## Outcome

- External differential tier now includes **ListOffsets v9 latest** HWM parity and **librdkafka consume-from-beginning** payload parity vs Kafka 4.2 after identical Produce, without advertising Fetch/ListOffsets.
- Ledger API keys **1** and **2** reference the new differential test; ListOffsets `failure_modes` set to `partial` with proof hook.
- Repaired `api_versions` v0 manual parse path for non-exhaustive `ApiVersion` and `error_code` setter drift; hardened `create_topics` result handling in both produce differential tests.

## Residual Risks

- External test requires `JANSU_DIFFERENTIAL=1` + Kafka 4.2 (not run in default CI without that flag).
- `AUDIT-004` backlog unchanged for read_committed/LSO, truncation, incremental Fetch, topic ID, API 23, Java/timestamp seek matrix.

## Next Recommended Action

- Run `JANSU_DIFFERENTIAL=1 cargo test -p jansu-broker --test differential_lab --all-features` locally or in CI when Docker is available; extend harness with timestamp seek when ready.
