# Phase Logs

Phase logs are the durable activity record for phase-backed work. The manifest in `phase-logs/index.json` is the source of truth for phase docs, canonical logs, and attempt-log directories. Agent behavior is defined by `AGENTS.md`, `MASTER_PLAN.md`, `AUDIT.md`, `tips/phases`, and this directory.

## Manifest Format

Each manifest entry tracks one `tips/phases/*.md` file with these fields:

- `phase`: two-digit phase number as a string.
- `phase_file`: phase document path.
- `status`: one of `legacy-unlogged`, `legacy-complete`, or `in-progress`.
- `log_file`: canonical phase-log path. Every phase has one.
- `attempt_log_dir`: directory for per-attempt logs for the phase.
- `ledger_api_keys`: ledger API keys owned by that phase.
- `primary_paths`: the main files touched or depended on while doing the phase work.
- `verification_commands`: commands used to verify the phase.
- `residual_risks`: the remaining work or gaps left after the phase.

`legacy-complete` is for older phases that already have logs, but those logs are still in the historical freeform format.
`legacy-unlogged` is for older phase docs that only have a placeholder canonical log.
`in-progress` is for a phase that is being actively tracked with the current structured log format.

## Attempt Logs

Every phase has an attempt directory at `phase-logs/attempts/NN-slug/`. Cross-phase audit or setup work uses `phase-logs/attempts/cross-phase/`.

Attempt logs use this path format:

- `phase-logs/attempts/NN-slug/YYYYMMDD-HHMMSS-agent.md`
- `phase-logs/attempts/cross-phase/YYYYMMDD-HHMMSS-agent.md`

Each attempt log must include these headings:

1. `Agent`
2. `Prompt`
3. `Phase Or Audit Item`
4. `Files Read`
5. `Files Changed`
6. `Tests Added`
7. `Verification Commands`
8. `Outcome`
9. `Residual Risks`
10. `Next Recommended Action`

Each parallel agent must use its own attempt log and a separate `CARGO_TARGET_DIR=/tmp/jansu-verify-<agent-or-phase>`.

## Structured Log Format

Structured logs use this heading order:

1. `Date`
2. `Phase File`
3. `Intent`
4. `Files Touched`
5. `Ledger Rows Touched`
6. `Tests Added`
7. `Verification Commands`
8. `Outcomes`
9. `Residual Risks`

Format rules:

- `Files Touched` lists one repo-relative path per bullet.
- `Ledger Rows Touched` lists one `api_key` per bullet.
- `Tests Added` lists fully qualified test names.
- `Verification Commands` lists one shell command per bullet.
- `Outcomes` and `Residual Risks` are short bullet summaries.

Current legacy logs remain readable as historical records. New work should use the structured headings above and should append or clarify rather than remove phase detail.
