# Attempt log

## Agent

Cursor (GPT-5.2)

## Prompt

Study tips and MASTER_PLAN; next logical phase; update logs; keep going until work is hardened and verified.

## Phase Or Audit Item

Phase **08** — `AUDIT-004` (ListOffsets differential coverage)

## Files Read

- `AGENTS.md`, `MASTER_PLAN.md`, `AUDIT.md`, `phase-logs/index.json`
- `tips/phases/08-fetch-list-offsets-leader-epoch.md`
- `jansu-broker/tests/differential_lab.rs`

## Files Changed

- `jansu-broker/tests/differential_lab.rs`
- `AUDIT.md`
- `docs/compatibility/kafka-4.2-ledger.json`
- `phase-logs/08-fetch-list-offsets-leader-epoch.md.log`
- `tips/phases/08-fetch-list-offsets-leader-epoch.md`

## Tests Added

- Extended `differential_listoffsets_latest_and_fetch_consume_after_produce` (Earliest + Latest wire checks; artifact fields).

## Verification Commands

- `cargo fmt --all --check`
- `cargo test -p jansu-broker --test differential_lab --all-features -- --nocapture`
- `just compatibility-contract`

## Outcome

- External differential read test now compares **ListOffsets v9 Earliest** and **Latest** vs Kafka 4.2; `ProduceListOffsetsFetchArtifact` includes earliest offset columns.
- Silenced workspace `unused_results` warnings on manual Tcp `read_exact` paths by binding the returned `usize` (wire helpers + ApiVersions v0 parser).

## Residual Risks

- External tier still requires `JANSU_DIFFERENTIAL=1` + Kafka 4.2. Timestamp / read_committed ListOffsets and broader Phase 08 backlog unchanged.

## Next Recommended Action

- Add timestamp-based ListOffsets differential once storage timestamps are aligned with Kafka reference, or pursue read_committed/LSO unit tests in `jansu-storage`.
