# Attempt log

## Agent

Cursor (GPT-5.2)

## Prompt

Study tips and MASTER_PLAN; next logical phase; update logs; keep going until work is done, hardened, verified.

## Phase Or Audit Item

Phase **08** / **Phase 04 harness** — `AUDIT-004` + ledger honesty for API key 0 differential Produce

## Files Read

- `AGENTS.md`, `MASTER_PLAN.md`, `AUDIT.md`, `docs/compatibility/kafka-4.2-ledger.json`
- `jansu-broker/tests/differential_lab.rs`

## Files Changed

- `jansu-broker/tests/differential_lab.rs`
- `AUDIT.md`
- `docs/compatibility/differential-lab.md`
- `MASTER_PLAN.md`
- `phase-logs/08-fetch-list-offsets-leader-epoch.md.log`
- `tips/phases/08-fetch-list-offsets-leader-epoch.md`

## Tests Added

- None (`differential_produce_round_trip_for_advertised_api` **un-ignored**).

## Verification Commands

- `cargo fmt --all --check`
- `cargo test -p jansu-broker --test differential_lab --all-features -- --nocapture`
- `just compatibility-contract`

## Outcome

- Stale ignore removed from Produce differential test; `AUDIT-004` and `differential-lab.md` describe the full external suite; `MASTER_PLAN` Phase 04 bullet lists API keys 0,1,2,3,18 for ledger-backed harness work.

## Residual Risks

- Full external tier still needs Docker or `JANSU_DIFF_KAFKA_BOOTSTRAP`. `differential_librdkafka_full_lifecycle` remains ignored placeholder.

## Next Recommended Action

- Run `JANSU_DIFFERENTIAL=1` differential suite in CI or locally after Kafka 4.2 is up; continue Phase 08 read_committed / truncation / incremental Fetch per `AUDIT-004`.
