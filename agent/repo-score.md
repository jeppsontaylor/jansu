# jankurai Repo Score

- Standard: `jankurai`
- Auditor: `0.7.0`
- Schema: `1.5.0`
- Paper edition: `2026.05-ed7`
- Target stack ID: `rust-ts-vite-react-postgres-bounded-python`
- Target stack: `Rust core + TypeScript/React/Vite + PostgreSQL + generated contracts + exception-only Python AI/data service`
- Repo: `.`
- Run ID: `1778019815`
- Started at: `1778019815`
- Elapsed: `861` ms
- Scope: `full`
- Raw score: `62`
- Final score: `60`
- Decision: `advisory`
- Minimum score: `85`
- Caps applied: `vibe-placeholders-in-product-code, fallback-soup-in-product-code, future-hostile-dead-language-in-product-code, severe-duplication-in-product-code, secret-like-content-detected, authz-or-data-isolation-gap, input-boundary-gap, no-agent-friendly-exception-pattern, missing-agent-readable-docs, rust-bad-behavior, sql-bad-behavior, docker-bad-behavior`

## Hard Rule Caps

| Rule | Max Score | Applied |
| --- | ---: | --- |
| `no-root-agent-instructions` | 75 | no |
| `no-one-command-setup-or-validation` | 70 | no |
| `no-deterministic-fast-lane` | 65 | no |
| `no-security-lane-on-high-risk-repo` | 60 | no |
| `generated-contracts-or-public-api-drift-untested` | 80 | no |
| `python-direct-product-truth-or-db-ownership` | 72 | no |
| `no-secret-or-dependency-scanning-in-ci` | 78 | no |
| `no-jankurai-audit-lane-in-ci` | 82 | no |
| `jankurai-required-tool-ci-evidence-gap` | 88 | no |
| `non-optimal-product-language-found` | 74 | no |
| `too-much-python-in-product-surface` | 72 | no |
| `boundary-reclassification-evidence-gap` | 72 | no |
| `vibe-placeholders-in-product-code` | 68 | yes |
| `fallback-soup-in-product-code` | 70 | yes |
| `future-hostile-dead-language-in-product-code` | 64 | yes |
| `severe-duplication-in-product-code` | 70 | yes |
| `generated-zone-mutation-risk` | 76 | no |
| `direct-db-access-from-wrong-layer` | 66 | no |
| `missing-web-e2e-lane` | 82 | no |
| `missing-rendered-ux-qa-lane` | 84 | no |
| `prompt-injection-risk` | 78 | no |
| `overbroad-agent-agency` | 65 | no |
| `secret-like-content-detected` | 60 | yes |
| `false-green-test-risk` | 76 | no |
| `destructive-migration-risk` | 70 | no |
| `authz-or-data-isolation-gap` | 78 | yes |
| `input-boundary-gap` | 78 | yes |
| `agent-tool-supply-chain-gap` | 78 | no |
| `release-readiness-gap` | 80 | no |
| `missing-rust-property-or-integration-tests` | 82 | no |
| `no-agent-friendly-exception-pattern` | 76 | yes |
| `missing-agent-readable-docs` | 80 | yes |
| `streaming-runtime-drift` | 78 | no |
| `rust-bad-behavior` | 72 | yes |
| `sql-bad-behavior` | 72 | yes |
| `typescript-bad-behavior` | 72 | no |
| `docker-bad-behavior` | 72 | yes |
| `python-bad-behavior` | 72 | no |
| `ci-bad-behavior` | 70 | no |
| `git-bad-behavior` | 70 | no |

## Dimensions

| Dimension | Weight | Score | Weighted | Evidence |
| --- | ---: | ---: | ---: | --- |
| Ownership and navigation surface | 13 | 61 | 7.93 | root `AGENTS.md` present; owner map present |
| Contract and boundary integrity | 13 | 88 | 11.44 | contract surface found; generated contract artifacts found |
| Proof lanes and test routing | 12 | 100 | 12.00 | one-command setup/validation lane found; deterministic fast lane found |
| Security and supply-chain posture | 12 | 66 | 7.92 | secret or dependency scan tooling found; provenance/SBOM tooling found |
| Code shape and semantic surface | 12 | 0 | 0.00 | largest authored code file: jansu-sans-io/tests/decode.rs (6177 LOC); code file exceeds 500 LOC |
| Data truth and workflow safety | 8 | 85 | 6.80 | database surface present; structured db boundary manifest present |
| Observability and repair evidence | 8 | 48 | 3.84 | observability libraries or patterns found; diagnostic shaping hints found |
| Context economy and agent instructions | 7 | 71 | 4.97 | root `AGENTS.md` present; root `AGENTS.md` stays short |
| Jankurai tool adoption and CI replacement | 7 | 17 | 1.19 | control-plane files present; applicable=14 |
| Python containment and polyglot hygiene | 4 | 100 | 4.00 | no Python files in scope |
| Build speed signals | 4 | 60 | 2.40 | build acceleration markers found; targeted test/build commands found |

## Rendered UX QA

- Web surface: `false`
- Layered UX lane: `true`
- Missing: `none`

## Tool Adoption

- Control plane present: `true`
- Applicable tools: `14`
- Configured: `5`
- CI evidence: `0`
- Artifact verified: `0`
- Replaced count: `0`
- Missing CI evidence: `audit-ci, proof-routing, proofbind, proofmark-rust, security, ci-bad-behavior, git-bad-behavior, contract-drift, rust-witness, authz-matrix, input-boundary, agent-tool-supply, release-readiness, cost-budget`

| Tool | Category | Mode | Status | Replaced | Artifacts |
| --- | --- | --- | --- | --- | --- |
| `audit-ci` | `audit` | `auto` | `configured` | `manual repo scoring, ad hoc score gates` | `agent/repo-score.json, agent/repo-score.md` |
| `proof-routing` | `proof` | `auto` | `configured` | `ad hoc proof lane selection, manual proof receipts` | `agent/repo-score.json, agent/repo-score.md, target/jankurai/repair-queue.jsonl` |
| `proofbind` | `proof` | `auto` | `missing` | `manual changed-surface routing, ad hoc proof obligation lists` | `target/jankurai/proofbind/surface-witness.json, target/jankurai/proofbind/obligations.json` |
| `proofmark-rust` | `proof` | `auto` | `missing` | `line-only coverage review, manual in-diff mutation review` | `target/jankurai/proofmark/proofmark-receipt.json, target/jankurai/proofmark/proof-receipt.json` |
| `security` | `security` | `auto` | `configured` | `gitleaks, dependency review, SBOM/provenance` | `target/jankurai/security/evidence.json` |
| `ci-bad-behavior` | `security` | `auto` | `missing` | `mutable workflow refs, secret echo/debug workflow checks, non-blocking security scans` | `crates/jankurai/tests/fixtures/language_bad_behavior/ci/` |
| `git-bad-behavior` | `audit` | `auto` | `missing` | `destructive git automation, force-push release scripts, hidden stash-based state` | `crates/jankurai/tests/fixtures/language_bad_behavior/git/` |
| `ux-qa` | `ux` | `auto` | `not_applicable` | `playwright, axe-core, visual baselines` | `target/jankurai/ux-qa.json` |
| `db-migration-analyze` | `db` | `auto` | `not_applicable` | `manual migration review` | `target/jankurai/migration-report.json` |
| `contract-drift` | `contract` | `auto` | `configured` | `handwritten contract drift checks, openapi diff` | `agent/repo-score.json, agent/repo-score.md` |
| `rust-witness` | `rust` | `auto` | `configured` | `manual witness graphing` | `target/jankurai/rust/witness-graph.json` |
| `vibe-coverage` | `audit` | `auto` | `not_applicable` | `manual vibe-coding coverage spreadsheet` | `target/jankurai/vibe-coverage.json, target/jankurai/vibe-coverage.md` |
| `authz-matrix` | `security` | `auto` | `missing` | `manual authz matrix review` | `agent/repo-score.json, agent/repo-score.md` |
| `input-boundary` | `security` | `auto` | `missing` | `manual unsafe sink review` | `agent/repo-score.json, agent/repo-score.md` |
| `agent-tool-supply` | `security` | `auto` | `missing` | `manual MCP/tool trust review` | `agent/repo-score.json, agent/repo-score.md` |
| `release-readiness` | `release` | `auto` | `missing` | `manual launch checklist` | `agent/repo-score.json, agent/repo-score.md` |
| `cost-budget` | `release` | `auto` | `missing` | `manual spend review` | `agent/repo-score.json, agent/repo-score.md` |

## Boundary manifest (ingested)

- Path: `agent/boundaries.toml`
- Stack: `rust-ts-vite-react-postgres-bounded-python` · version: `0.4.0`
- Queue path counts — adapter: `2`, event_contract: `1`, generated_type: `1`, client_marker: `7`, streaming_exception: `1`
- Content fingerprint: `sha256:441612192807b4437d9dd11275108b76f007606932c6a9a66974759b958db6d4`

## Boundary Reclassifications

No audited runtime boundary reclassifications declared.

## Findings

1. `medium` `shape` `.`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:shape` `soft` confidence `0.76`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: `Code shape and semantic surface` scored 0 below the standard floor of 85
   Fix: split large or ambiguous authored code into smaller semantic modules with focused tests
   Rerun: `just fast`
   Fingerprint: `sha256:2cb5c7678012e6ba6927ea08a8b4d8df6b4c96301e3a8e312e64bd78d2380fbe`
   Evidence: largest authored code file: jansu-sans-io/tests/decode.rs (6177 LOC), code file exceeds 500 LOC, code file exceeds 1000 LOC, duplicate code block marker found
2. `medium` `security` `.github/workflows/jankurai.yml`
   Rule: `HLT-016-SUPPLY-CHAIN-DRIFT`
   Check: `HLT-016-SUPPLY-CHAIN-DRIFT:security` `soft` confidence `0.76`
   Route: TLR `Security, secrets, agency`, lane `security`, owner `ops`
   Docs: `docs/audit-rubric.md#top-level-risk-mapping`
   Reason: `Security and supply-chain posture` scored 66 below the standard floor of 85
   Fix: wire secret, dependency, provenance, and workflow scans into an operational CI lane
   Rerun: `just security`
   Fingerprint: `sha256:eb6acb55678ae606bd1e3575fe3aa79da832aa766b88714c480c15225d48b3aa`
   Evidence: secret or dependency scan tooling found, provenance/SBOM tooling found, security lane present, canonical security lane wrapper present
3. `medium` `context` `AGENTS.md`
   Rule: `HLT-015-CONTEXT-SETUP-GAP`
   Check: `HLT-015-CONTEXT-SETUP-GAP:context` `soft` confidence `0.76`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `docs/agent-native-standard.md`
   Reason: `Context economy and agent instructions` scored 71 below the standard floor of 85
   Fix: keep root guidance short and route durable detail through agent-readable manifests and docs
   Rerun: `just fast`
   Fingerprint: `sha256:7871aad486ed815c8f56845e244c2cfc73025a990af53b510881c2c6b474c9cd`
   Evidence: root `AGENTS.md` present, root `AGENTS.md` stays short, machine-readable routing artifacts present, thin IDE/agent adapters are present
4. `medium` `proof` `Justfile`
   Rule: `HLT-018-PERF-CONCURRENCY-DRIFT`
   Check: `HLT-018-PERF-CONCURRENCY-DRIFT:proof` `soft` confidence `0.76`
   Route: TLR `Verification`, lane `fast`, owner `workspace`
   Docs: `docs/testing.md`
   Reason: `Build speed signals` scored 60 below the standard floor of 85
   Fix: add fast deterministic build/test targets, caches, and narrow proof lanes for agent iteration
   Rerun: `just fast`
   Fingerprint: `sha256:313bc78bbf6c49756ec5ada9e94dab2beeed53a1b51dfbba860001ff03dd24fc`
   Evidence: build acceleration markers found, targeted test/build commands found, CI cache hint found
5. `medium` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `soft` confidence `0.76`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: `Ownership and navigation surface` scored 61 below the standard floor of 85
   Fix: tighten owner/test maps and root routing until agents can localize ownership without inference
   Rerun: `just fast`
   Fingerprint: `sha256:3483f72abb28f0f546bb878136fcc9a3fff7c139a7f985bdc14b419fd7686f3b`
   Evidence: root `AGENTS.md` present, owner map present, test/proof routing map present, root `README.md` routes to workspace layout
6. `high` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `hard` confidence `0.88`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: path `.cursor/rules/jansu-master-plan.mdc` has no owner-map route
   Fix: add the narrowest stable prefix for this path to `agent/owner-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:115c66db5f596afa9da57da6b76f2bf98a17de0d2fb39f61a950963fd2b5fa28`
   Evidence: .cursor/rules/jansu-master-plan.mdc
7. `high` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `hard` confidence `0.88`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: path `.gitleaks.toml` has no owner-map route
   Fix: add the narrowest stable prefix for this path to `agent/owner-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:03e34894ef22396f19be77d61959819475e4353f1817e48ac6a76f0c6756bb33`
   Evidence: .gitleaks.toml
8. `high` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `hard` confidence `0.88`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: path `README-jankurai-scaffold.md` has no owner-map route
   Fix: add the narrowest stable prefix for this path to `agent/owner-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:90323ef79fcfad5f26ead925bde75e95e67ef920623a25c99b8b840cc920b969`
   Evidence: README-jankurai-scaffold.md
9. `high` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `hard` confidence `0.88`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: path `about.hbs` has no owner-map route
   Fix: add the narrowest stable prefix for this path to `agent/owner-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:7e50262a74b82ca94518947d0fc9d06859d0c09fad2089f38f5260529e870850`
   Evidence: about.hbs
10. `high` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `hard` confidence `0.88`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: path `about.toml` has no owner-map route
   Fix: add the narrowest stable prefix for this path to `agent/owner-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:48340ade6c9d526fd9eb1b8171d862fefde8eb808d34b950c3a5bc63d270b58f`
   Evidence: about.toml
11. `high` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `hard` confidence `0.88`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: path `assets/jansu_header.png` has no owner-map route
   Fix: add the narrowest stable prefix for this path to `agent/owner-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:461fa79e332acde8fa9b12917c86056ffc065312ddda3cb91d43fb471ff97cac`
   Evidence: assets/jansu_header.png
12. `high` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `hard` confidence `0.88`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: path `codebook.toml` has no owner-map route
   Fix: add the narrowest stable prefix for this path to `agent/owner-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:84e3709cd26dc8cc6f92adf3ea377b9855353433835d18c59ca6cadc64216c54`
   Evidence: codebook.toml
13. `high` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `hard` confidence `0.88`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: path `command-plain.properties` has no owner-map route
   Fix: add the narrowest stable prefix for this path to `agent/owner-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:3a10948e9c052eb4260da017612319a7b35bd0e91f947680378c429c5e969bfd`
   Evidence: command-plain.properties
14. `high` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `hard` confidence `0.88`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: path `command-scram-256.properties` has no owner-map route
   Fix: add the narrowest stable prefix for this path to `agent/owner-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:94c30e8adb44d7be1a93ced5802c76f60390c6e046c2d4202cec8124fdb3b333`
   Evidence: command-scram-256.properties
15. `high` `context` `agent/owner-map.json`
   Rule: `HLT-003-OWNERLESS-PATH`
   Check: `HLT-003-OWNERLESS-PATH:context` `hard` confidence `0.88`
   Route: TLR `Context/setup`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#ownership-boundaries`
   Reason: path `command-scram-512.properties` has no owner-map route
   Fix: add the narrowest stable prefix for this path to `agent/owner-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:be725edfd40b55efccc60b96f6719ed45607e2960bce6c0b54937e95567f1db9`
   Evidence: command-scram-512.properties
16. `critical` `security` `agent/repo-score.json:407`
   Rule: `HLT-010-SECRET-SPRAWL`
   Check: `HLT-010-SECRET-SPRAWL:security` `hard` confidence `0.95`
   Route: TLR `Security, secrets, agency`, lane `security`, owner `agent`
   Docs: `docs/audit-rubric.md#top-level-risk-mapping`
   Reason: secret-like value or credential material appears in repository text
   Fix: remove and rotate the credential, add local and CI secret scanning, and scan transcripts/artifacts/MCP config for related exposure
   Rerun: `just security`
   Fingerprint: `sha256:fad480ea4214eb74a7fa7eca8f86e88eaba6525bad0d05188b656dd834ddd649`
   Evidence: "id": "no-security-lane-on-high-risk-repo",
17. `medium` `proof` `agent/repo-score.json:1594`
   Rule: `HLT-027-HUMAN-REVIEW-EVIDENCE-GAP`
   Check: `HLT-027-HUMAN-REVIEW-EVIDENCE-GAP:proof` `soft` confidence `0.88`
   Route: TLR `Repair`, lane `audit`, owner `agent`
   Docs: `docs/testing.md`
   Matched term: `review evidence`
   Reason: proof and review claims need receipts
   Fix: attach raw CI logs, review receipts, and replayable commands instead of accepting claims or summaries
   Rerun: `just score`
   Fingerprint: `sha256:fe00ae823c72aef0591eb84616473827d54708317fcabf9f2ebb6f53f0fb83ad`
   Evidence: "\"Evidence: \\\"about\\\": \\\"The principal filter, or null to accept all principals.\\\" },\""
18. `high` `proof` `agent/test-map.json`
   Rule: `HLT-004-UNMAPPED-PROOF`
   Check: `HLT-004-UNMAPPED-PROOF:proof` `hard` confidence `0.88`
   Route: TLR `Verification`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#proof-lanes`
   Reason: path `.gitignore` has no test-map proof route
   Fix: add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:8fe7edc74ce7873d59c9b6ccb57a19bca48fe9589c015113fb5028c2cc245992`
   Evidence: .gitignore
19. `high` `proof` `agent/test-map.json`
   Rule: `HLT-004-UNMAPPED-PROOF`
   Check: `HLT-004-UNMAPPED-PROOF:proof` `hard` confidence `0.88`
   Route: TLR `Verification`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#proof-lanes`
   Reason: path `.gitleaks.toml` has no test-map proof route
   Fix: add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:5631afc028b47d640f6590988664750401fed6fde37735935a82834f5c2209e9`
   Evidence: .gitleaks.toml
20. `high` `proof` `agent/test-map.json`
   Rule: `HLT-004-UNMAPPED-PROOF`
   Check: `HLT-004-UNMAPPED-PROOF:proof` `hard` confidence `0.88`
   Route: TLR `Verification`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#proof-lanes`
   Reason: path `README.md` has no test-map proof route
   Fix: add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:c4a25759c16b05da63ece6c953b801c311e0df2a29f91df6cc92c21e69fa6cc7`
   Evidence: README.md
21. `high` `proof` `agent/test-map.json`
   Rule: `HLT-004-UNMAPPED-PROOF`
   Check: `HLT-004-UNMAPPED-PROOF:proof` `hard` confidence `0.88`
   Route: TLR `Verification`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#proof-lanes`
   Reason: path `about.hbs` has no test-map proof route
   Fix: add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:12463f159742b2b3659b908b1e86db561e50f606b7562d165e94a222225ec412`
   Evidence: about.hbs
22. `high` `proof` `agent/test-map.json`
   Rule: `HLT-004-UNMAPPED-PROOF`
   Check: `HLT-004-UNMAPPED-PROOF:proof` `hard` confidence `0.88`
   Route: TLR `Verification`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#proof-lanes`
   Reason: path `about.toml` has no test-map proof route
   Fix: add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:9d9110118db17ecdf8473ea1b9a3d3267cf26cfc058afdaeaa25325464dc4c87`
   Evidence: about.toml
23. `high` `proof` `agent/test-map.json`
   Rule: `HLT-004-UNMAPPED-PROOF`
   Check: `HLT-004-UNMAPPED-PROOF:proof` `hard` confidence `0.88`
   Route: TLR `Verification`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#proof-lanes`
   Reason: path `assets/jansu_header.png` has no test-map proof route
   Fix: add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:cb70d6fdffb1dfea55949bf656c9e2f633b6f29dc814482b29b3c99b0eb541f6`
   Evidence: assets/jansu_header.png
24. `high` `proof` `agent/test-map.json`
   Rule: `HLT-004-UNMAPPED-PROOF`
   Check: `HLT-004-UNMAPPED-PROOF:proof` `hard` confidence `0.88`
   Route: TLR `Verification`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#proof-lanes`
   Reason: path `codebook.toml` has no test-map proof route
   Fix: add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:60ee2989e50206e4f2b33dc67966dcc3c3f59b40ca2c1ffd85b667e87cbf115f`
   Evidence: codebook.toml
25. `high` `proof` `agent/test-map.json`
   Rule: `HLT-004-UNMAPPED-PROOF`
   Check: `HLT-004-UNMAPPED-PROOF:proof` `hard` confidence `0.88`
   Route: TLR `Verification`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#proof-lanes`
   Reason: path `command-plain.properties` has no test-map proof route
   Fix: add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:18d673235bbdf742566b2791c41ee4422db78972c4be2868c525f2840082f602`
   Evidence: command-plain.properties
26. `high` `proof` `agent/test-map.json`
   Rule: `HLT-004-UNMAPPED-PROOF`
   Check: `HLT-004-UNMAPPED-PROOF:proof` `hard` confidence `0.88`
   Route: TLR `Verification`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#proof-lanes`
   Reason: path `command-scram-256.properties` has no test-map proof route
   Fix: add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:2274e9d4c1c447331bb8c5922dd3eed799cb27c019e3af32324103d6029461e9`
   Evidence: command-scram-256.properties
27. `high` `proof` `agent/test-map.json`
   Rule: `HLT-004-UNMAPPED-PROOF`
   Check: `HLT-004-UNMAPPED-PROOF:proof` `hard` confidence `0.88`
   Route: TLR `Verification`, lane `fast`, owner `agent`
   Docs: `agent/JANKURAI_STANDARD.md#proof-lanes`
   Reason: path `command-scram-512.properties` has no test-map proof route
   Fix: add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Rerun: `just fast`
   Fingerprint: `sha256:ab63fd0e4be79e1900c456d1cc0da9938e67cf22c33cad58fec22998c796ac92`
   Evidence: command-scram-512.properties
28. `high` `security` `compose.yaml:97`
   Rule: `HLT-032-DOCKER-BAD-BEHAVIOR`
   Check: `HLT-032-DOCKER-BAD-BEHAVIOR:security` `hard` confidence `0.95`
   Route: TLR `Security, secrets, agency`, lane `security`, owner `ops`
   Docs: `docs/testing.md`
   Matched term: `docker.port.public-db-admin`
   Reason: the port mapping is publicly reachable without a local bind
   Fix: bind the port to localhost or keep it on an internal-only network
   Rerun: `just security`
   Fingerprint: `sha256:0ee732caa0bb38efb95069a404edd6c02777dc350a1ee8fdcbe4e272ff594fe5`
   Evidence: detector=docker.port.public-db-admin, path=compose.yaml, line=97, proof_window=None, snippet=- LAKEKEEPER__PG_DATABASE_URL_READ=postgresql://postgres:postgres@db:5432/postgres
29. `high` `security` `compose.yaml:98`
   Rule: `HLT-032-DOCKER-BAD-BEHAVIOR`
   Check: `HLT-032-DOCKER-BAD-BEHAVIOR:security` `hard` confidence `0.95`
   Route: TLR `Security, secrets, agency`, lane `security`, owner `ops`
   Docs: `docs/testing.md`
   Matched term: `docker.port.public-db-admin`
   Reason: the port mapping is publicly reachable without a local bind
   Fix: bind the port to localhost or keep it on an internal-only network
   Rerun: `just security`
   Fingerprint: `sha256:f7345b96cf712bb97ef59c4441f19974646c35d860397e0a02f61865f79926d1`
   Evidence: detector=docker.port.public-db-admin, path=compose.yaml, line=98, proof_window=None, snippet=- LAKEKEEPER__PG_DATABASE_URL_WRITE=postgresql://postgres:postgres@db:5432/postgres
30. `high` `security` `compose.yaml:120`
   Rule: `HLT-032-DOCKER-BAD-BEHAVIOR`
   Check: `HLT-032-DOCKER-BAD-BEHAVIOR:security` `hard` confidence `0.95`
   Route: TLR `Security, secrets, agency`, lane `security`, owner `ops`
   Docs: `docs/testing.md`
   Matched term: `docker.port.public-db-admin`
   Reason: the port mapping is publicly reachable without a local bind
   Fix: bind the port to localhost or keep it on an internal-only network
   Rerun: `just security`
   Fingerprint: `sha256:7584765832ae47043a5d45a51da95f8375690a5b902b9906a16a9293a36b4d49`
   Evidence: detector=docker.port.public-db-admin, path=compose.yaml, line=120, proof_window=None, snippet=- LAKEKEEPER__PG_DATABASE_URL_READ=postgresql://postgres:postgres@db:5432/postgres
31. `high` `security` `compose.yaml:121`
   Rule: `HLT-032-DOCKER-BAD-BEHAVIOR`
   Check: `HLT-032-DOCKER-BAD-BEHAVIOR:security` `hard` confidence `0.95`
   Route: TLR `Security, secrets, agency`, lane `security`, owner `ops`
   Docs: `docs/testing.md`
   Matched term: `docker.port.public-db-admin`
   Reason: the port mapping is publicly reachable without a local bind
   Fix: bind the port to localhost or keep it on an internal-only network
   Rerun: `just security`
   Fingerprint: `sha256:ab67b0a956fb0b0ca50058292cb0cd8608e24db34a655710d789000f2b7003d3`
   Evidence: detector=docker.port.public-db-admin, path=compose.yaml, line=121, proof_window=None, snippet=- LAKEKEEPER__PG_DATABASE_URL_WRITE=postgresql://postgres:postgres@db:5432/postgres
32. `high` `exceptions` `crates/domain`
   Rule: `HLT-017-OPAQUE-OBSERVABILITY`
   Check: `HLT-017-OPAQUE-OBSERVABILITY:exceptions` `hard` confidence `0.88`
   Route: TLR `Repair`, lane `observability`, owner `tools`
   Docs: `agent/JANKURAI_STANDARD.md#repair-receipts`
   Reason: no agent-friendly exception/error pattern was detected
   Fix: define a typed exception surface with purpose, reason, common fixes, docs_url, and repair_hint so the next rerun is local
   Rerun: `just score`
   Fingerprint: `sha256:538667a01e35d8e91eae100627364816dd225911862fa2fa1578642af63d4af8`
   Evidence: route repair work to the next agent, opaque failures slow local debugging and reruns, add a typed repair hint; name the common fixes; point at the local docs URL, docs/testing.md
33. `medium` `docs` `docs/`
   Check: `HLT-000-SCORE-DIMENSION:docs` `soft` confidence `0.76`
   Route: TLR `Context/setup`, lane `audit`, owner `standard`
   Reason: agent-readable documentation is incomplete
   Fix: add concise docs for architecture, boundaries, tests, generated zones, and audit rules; route them from root `AGENTS.md`
   Rerun: `just score`
   Fingerprint: `sha256:517269e227b26a69e8747d0d8cdb8a8d03582550b0de9e4c7c2b9287d8cee256`
   Evidence: docs/architecture.md or docs/boundaries.md, docs/testing.md
34. `medium` `observability` `docs/testing.md`
   Rule: `HLT-017-OPAQUE-OBSERVABILITY`
   Check: `HLT-017-OPAQUE-OBSERVABILITY:observability` `soft` confidence `0.76`
   Route: TLR `Repair`, lane `observability`, owner `standard`
   Docs: `agent/JANKURAI_STANDARD.md#repair-receipts`
   Reason: `Observability and repair evidence` scored 48 below the standard floor of 85
   Fix: add structured errors, telemetry, and repair receipts that tell the next agent where to rerun proof
   Rerun: `just score`
   Fingerprint: `sha256:e72a2f1b6604c37c9b364d5e6f883ae554786aa2e1ec2fd2a868f2387a043e11`
   Evidence: observability libraries or patterns found, diagnostic shaping hints found, repair receipts or raw artifact language found, no agent-friendly exception pattern found
35. `medium` `release` `docs/testing.md`
   Rule: `HLT-026-COST-BUDGET-GAP`
   Check: `HLT-026-COST-BUDGET-GAP:release` `soft` confidence `0.88`
   Route: TLR `Verification`, lane `release`, owner `standard`
   Docs: `docs/testing.md`
   Matched term: `budget`
   Reason: unbounded paid work needs budgets and stop conditions
   Fix: add explicit budgets, quotas, stop conditions, and kill-switch evidence for paid or unbounded operations
   Rerun: `just check`
   Fingerprint: `sha256:edd248b7afc24b644107205fa5b84a88103ac4b622009ff9f19b779de8798f59`
   Evidence: cost surface found without budget/stop-condition policy
36. `high` `data` `etc/initdb.d/010-schema.sql:28`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:f8a23181eadfc5b579e835f3c961bff218161cef6aff388860681adeae6fa5a9`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=cluster int references cluster (id) on delete cascade not null,
37. `high` `data` `etc/initdb.d/010-schema.sql:41`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:6d639508a08310387dd1456610a08eb6ccb6d01d2ccd0acda509b2f07986c833`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topic int references topic (id) on delete cascade not null,
38. `high` `data` `etc/initdb.d/010-schema.sql:64`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:b276d60b44229165462d078c5d38b1c89b27f187ad4a35a81b6bb0c01c592ba1`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topic int references topic (id) on delete cascade,
39. `high` `data` `etc/initdb.d/010-schema.sql:84`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:6ad4d7eb4f63a3f60234017c357fb60b95f1c27ce59ac382d8cbb45aa99eeddb`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition int references topition (id) on delete cascade,
40. `high` `data` `etc/initdb.d/010-schema.sql:93`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:6ad4d7eb4f63a3f60234017c357fb60b95f1c27ce59ac382d8cbb45aa99eeddb`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition int references topition (id) on delete cascade,
41. `high` `data` `etc/initdb.d/010-schema.sql:103`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:b276d60b44229165462d078c5d38b1c89b27f187ad4a35a81b6bb0c01c592ba1`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topic int references topic (id) on delete cascade,
42. `high` `data` `etc/initdb.d/010-schema.sql:126`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:6ad4d7eb4f63a3f60234017c357fb60b95f1c27ce59ac382d8cbb45aa99eeddb`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition int references topition (id) on delete cascade,
43. `high` `data` `etc/initdb.d/010-schema.sql:170`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:fca7a38ef532bf56dfff41dc746d711bcc8744869ee27ab3545f0c9a9b541d0a`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=foreign key (topition, offset_id) references record (topition, offset_id) on delete cascade,
44. `high` `data` `etc/initdb.d/010-schema.sql:182`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:f8a23181eadfc5b579e835f3c961bff218161cef6aff388860681adeae6fa5a9`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=cluster int references cluster (id) on delete cascade not null,
45. `high` `data` `etc/initdb.d/010-schema.sql:200`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:1338d6cb67fffc2a9441b19c7bc98bd06c33d237d1207dc39205ee4cdab6850f`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=consumer_group int references consumer_group (id) on delete cascade,
46. `high` `data` `etc/initdb.d/010-schema.sql:229`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:1338d6cb67fffc2a9441b19c7bc98bd06c33d237d1207dc39205ee4cdab6850f`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=consumer_group int references consumer_group (id) on delete cascade,
47. `high` `data` `etc/initdb.d/010-schema.sql:230`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:6ad4d7eb4f63a3f60234017c357fb60b95f1c27ce59ac382d8cbb45aa99eeddb`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition int references topition (id) on delete cascade,
48. `high` `data` `etc/initdb.d/010-schema.sql:264`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:f8a23181eadfc5b579e835f3c961bff218161cef6aff388860681adeae6fa5a9`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=cluster int references cluster (id) on delete cascade not null,
49. `high` `data` `etc/initdb.d/010-schema.sql:271`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:0be2e5503924ff9e8b7d984cc6fe3ac7159c5409323b99e0dff43b1a9d88a5d2`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=producer bigint references producer (id) on delete cascade,
50. `high` `data` `etc/initdb.d/010-schema.sql:291`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:8fcaee27841bbacf1b0ea4e30c2a82b228d61b1a69db0ba9f6e174f3b00409f8`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=producer_epoch int references producer_epoch (id) on delete cascade,
51. `high` `data` `etc/initdb.d/010-schema.sql:292`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:6ad4d7eb4f63a3f60234017c357fb60b95f1c27ce59ac382d8cbb45aa99eeddb`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition int references topition (id) on delete cascade,
52. `high` `data` `etc/initdb.d/010-schema.sql:321`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:a8acbadf9260a51831744cc757a633295955cae27629a1e9af0f84a97bbf4810`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=cluster int references cluster (id) on delete cascade,
53. `high` `data` `etc/initdb.d/010-schema.sql:324`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:0be2e5503924ff9e8b7d984cc6fe3ac7159c5409323b99e0dff43b1a9d88a5d2`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=producer bigint references producer (id) on delete cascade,
54. `high` `data` `etc/initdb.d/010-schema.sql:340`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:8d189a58e081da3edd55b4dafaf23786c4cf59e57fec2acf06152947c3562303`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=transaction bigint references txn (id) on delete cascade,
55. `high` `data` `etc/initdb.d/010-schema.sql:341`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:8fcaee27841bbacf1b0ea4e30c2a82b228d61b1a69db0ba9f6e174f3b00409f8`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=producer_epoch int references producer_epoch (id) on delete cascade,
56. `high` `data` `etc/initdb.d/010-schema.sql:385`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:332bc254eb8fb01ce057b4cfa8a89a792b6e9e11208ff3d7b60bf90ad5c4b71b`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=txn_detail int references txn_detail (id) on delete cascade,
57. `high` `data` `etc/initdb.d/010-schema.sql:386`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:6ad4d7eb4f63a3f60234017c357fb60b95f1c27ce59ac382d8cbb45aa99eeddb`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition int references topition (id) on delete cascade,
58. `high` `data` `etc/initdb.d/010-schema.sql:416`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:acccee382e7b58ff269d87b9cebd76b155f323465cfd123141e02b8ebcd24c30`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=txn_topition int references txn_topition (id) on delete cascade,
59. `high` `data` `etc/initdb.d/010-schema.sql:459`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:332bc254eb8fb01ce057b4cfa8a89a792b6e9e11208ff3d7b60bf90ad5c4b71b`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=txn_detail int references txn_detail (id) on delete cascade,
60. `high` `data` `etc/initdb.d/010-schema.sql:460`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:1338d6cb67fffc2a9441b19c7bc98bd06c33d237d1207dc39205ee4cdab6850f`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=consumer_group int references consumer_group (id) on delete cascade,
61. `high` `data` `etc/initdb.d/010-schema.sql:470`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:c29d57b1eecd43bffd11a9fd46dae06f0c20a9118221d5408edc9720eeda1d27`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=offset_commit int references txn_offset_commit (id) on delete cascade,
62. `high` `data` `etc/initdb.d/010-schema.sql:471`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:6ad4d7eb4f63a3f60234017c357fb60b95f1c27ce59ac382d8cbb45aa99eeddb`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition int references topition (id) on delete cascade,
63. `high` `data` `etc/initdb.d/010-schema.sql:565`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:f8a23181eadfc5b579e835f3c961bff218161cef6aff388860681adeae6fa5a9`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=cluster int references cluster (id) on delete cascade not null,
64. `high` `data` `etc/initdb.d/011-offset-retention-patch.sql:19`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:f4b02e37f6cb81c8df0502f86e3b3a1ee011e0649a582148c4cd97e778acd443`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition int references topition (id) on delete cascade,
65. `high` `vibe` `fuzz/fuzz_targets/generate_seeds.rs:1`
   Check: `HLT-000-SCORE-DIMENSION:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `unmapped`
   Reason: duplicated product code block detected
   Fix: extract the duplicated behavior behind one named boundary and add focused tests before changing behavior
   Rerun: `just fast`
   Fingerprint: `sha256:01b7d37b633f01e1b05fffb5e152e610abf7b8d318fbd02f93623534e76daee7`
   Evidence: duplicate block also appears at fuzz/fuzz_targets/generate_seeds.rs:1
66. `high` `vibe` `jansu-auth/src/handshake.rs:49`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:8f005d7a6bea03e308262a0a747332216e4a686057335833c1b19b74ab571867`
   Evidence: jansu-auth/src/handshake.rs:49, future-hostile/dead-language term `stale` appears
67. `high` `vibe` `jansu-auth/src/lib.rs:111`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: fallback soup detected in product code
   Fix: collapse fallback chains into explicit typed states with bounded retry policy, telemetry, and documented repair guidance
   Rerun: `just fast`
   Fingerprint: `sha256:9cd6138ad65ebf74395e19d8c51f9f5e19cdfe97350f98ca9576fb06723642d7`
   Evidence: jansu-auth/src/lib.rs:111 .unwrap_or_default()
68. `high` `security` `jansu-broker/src/broker.rs:19`
   Rule: `HLT-022-AUTHZ-ISOLATION-GAP`
   Check: `HLT-022-AUTHZ-ISOLATION-GAP:security` `hard` confidence `0.88`
   Route: TLR `Business truth`, lane `db`, owner `tools`
   Docs: `docs/audit-rubric.md#top-level-risk-mapping`
   Matched term: `admin`
   Reason: authz/data isolation requires negative proof evidence
   Fix: add owner/non-owner authorization tests or RLS evidence for the touched data boundary
   Rerun: `just fast`
   Fingerprint: `sha256:78c775741b3cd91a923b3acc5b2946c16b5c4f106d27fc28edc290120ac50ea1`
   Evidence: coordinator::group::{Coordinator, administrator::Controller},
69. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:1411`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a7f631790f25cf942817bd501c0f00894c894efcf23c0cb8569f9927d7f4f6ee`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:1411, future-hostile/dead-language term `stale` appears
70. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:1413`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:240dc8e78c1d066734030e788920d2b5a4f12cdf56c61efc867bc9ca7e28c9e5`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:1413, future-hostile/dead-language term `old` appears
71. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:1418`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:e96d20cab44aca6c7039d8a8e4bc901aa799bf20951a0cc9bf12c3136e7e5399`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:1418, future-hostile/dead-language term `stale` appears
72. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:1422`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b50aeaa553cd77a7672dffd08aa3fbef4aca1f0107da412105a239bf9085b2f3`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:1422, future-hostile/dead-language term `stale` appears
73. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:1521`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:0e6cf2d39c288bea488d8df68398a859703a4db3ea9d73953eb64b9a0e70f1b2`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:1521, future-hostile/dead-language term `stale` appears
74. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:1555`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:557575d10e777994f4be0f355cc649d3040df0651f4827302f6d5e5df7bdf513`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:1555, future-hostile/dead-language term `stale` appears
75. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:2207`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:39ef3104b7757e8b9afc89d69de1aa0f3979288d02237f47112a055f97009c86`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:2207, future-hostile/dead-language term `stale` appears
76. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:2303`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:bbb2ceca570190b542fb52c2dc98b09630cee6c448b463748728725b6954e772`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:2303, future-hostile/dead-language term `old` appears
77. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:2336`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:ba202fdc756b67a2171aa022cc36bf6968c620ff8d554ef5575372c36964cb8a`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:2336, future-hostile/dead-language term `stale` appears
78. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:2349`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:d26166fd3135747d2d5079bcc2d0c1d17b0fdb162af58095cb405492553c6994`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:2349, future-hostile/dead-language term `stale` appears
79. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:2872`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:70b3667f2c22556a0f9e12ac7b82c511d16ed7d960dcac7036647fb992f3fc56`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:2872, future-hostile/dead-language term `stale` appears
80. `high` `vibe` `jansu-broker/src/coordinator/group/administrator/tests.rs:2876`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:1d4dc68c1496c61204e4f15b345c870dbc06a1a5e571bbeea615fc556f6468fc`
   Evidence: jansu-broker/src/coordinator/group/administrator/tests.rs:2876, future-hostile/dead-language term `stale` appears
81. `high` `vibe` `jansu-broker/tests/auth.rs:583`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: product code contains TODO/stub/unimplemented/unreachable placeholder markers
   Fix: replace placeholders with implemented behavior, typed unsupported-state errors, or a tracked exception record with docs
   Rerun: `just fast`
   Fingerprint: `sha256:bcad104a7c3ef02f240e54ea90b94859448e9f1a0d0f0bd5a91e2d21708a64b6`
   Evidence: jansu-broker/tests/auth.rs:583 unimplemented!()
82. `high` `vibe` `jansu-broker/tests/cg_dynamic.rs:774`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:211ed6529d77b772ba4b1f842027c0d55adbe03caee543fbffa423045ca5b52c`
   Evidence: jansu-broker/tests/cg_dynamic.rs:774, future-hostile/dead-language term `old` appears
83. `high` `vibe` `jansu-broker/tests/cg_dynamic.rs:786`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:fa9c802ae6568b9ae437956ccc14c3f42e6eb3e02096eb6975c057feaf16be72`
   Evidence: jansu-broker/tests/cg_dynamic.rs:786, future-hostile/dead-language term `stale` appears
84. `high` `vibe` `jansu-broker/tests/compatibility_contract.rs:88`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `legacy` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:0728698ce13f273c9d36743202dcc93cee1293ba87302c5af4553a748e58090a`
   Evidence: jansu-broker/tests/compatibility_contract.rs:88, future-hostile/dead-language term `legacy` appears
85. `high` `vibe` `jansu-broker/tests/compatibility_contract.rs:89`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `legacy` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:854f837a280c60fb804c677d68aed5d31d52b9548d5c2c45a4e1ee9b67b5f84f`
   Evidence: jansu-broker/tests/compatibility_contract.rs:89, future-hostile/dead-language term `legacy` appears
86. `high` `vibe` `jansu-broker/tests/compatibility_contract.rs:103`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `compat` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a504b9c0537576af6edaf695cf64dcf8b345cb59c1cd255cd402cb8efdda146c`
   Evidence: jansu-broker/tests/compatibility_contract.rs:103, future-hostile/dead-language term `compat` appears
87. `high` `vibe` `jansu-broker/tests/compatibility_contract.rs:891`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `legacy` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:6f783f4d4205fb5088600082a66e3ceccd2a44e5633c9619996baef7719e60aa`
   Evidence: jansu-broker/tests/compatibility_contract.rs:891, future-hostile/dead-language term `legacy` appears
88. `high` `vibe` `jansu-broker/tests/compatibility_contract.rs:892`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `legacy` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:519e5409c7034f4294e65b4423551d6adbd8cb33870484541c1261bf513ff176`
   Evidence: jansu-broker/tests/compatibility_contract.rs:892, future-hostile/dead-language term `legacy` appears
89. `high` `vibe` `jansu-broker/tests/differential_lab.rs:501`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:3e81c5c154c2142a1d48aee688daab4fc3ab1fb21fb459458577413a3f3fe194`
   Evidence: jansu-broker/tests/differential_lab.rs:501, future-hostile/dead-language term `stale` appears
90. `high` `vibe` `jansu-broker/tests/differential_lab.rs:1016`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `placeholder` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:d8b20ad2bb6d870b07037eac28e821320d39a434fb6bffea23d6f091639329cc`
   Evidence: jansu-broker/tests/differential_lab.rs:1016, future-hostile/dead-language term `placeholder` appears
91. `high` `vibe` `jansu-cli/src/cli/perf.rs:103`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:28d6eeb8107b5cff46177c5091679a4a6e15603bfb974ea13d065d9bbb79bc07`
   Evidence: jansu-cli/src/cli/perf.rs:103, future-hostile/dead-language term `todo` appears
92. `high` `security` `jansu-cli/src/cli/user.rs:83`
   Rule: `HLT-029-RUST-BAD-BEHAVIOR`
   Check: `HLT-029-RUST-BAD-BEHAVIOR:security` `hard` confidence `0.95`
   Route: TLR `Security, secrets, agency`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#top-level-risk-mapping`
   Matched term: `rust.unsafe.zeroed`
   Reason: all-zero validity was not proven
   Fix: construct the type with a valid initializer instead of zeroing it
   Rerun: `just fast`
   Fingerprint: `sha256:d474ee3c5cc8815f1908f3e00fbb39dc51ffaa7332b454bb251b6a02e8dd1f19`
   Evidence: detector=zeroed, proof-window=NearbySafetyComment, snippet=let mut buf = BytesMut::zeroed(32);
93. `high` `security` `jansu-cli/src/cli/user.rs:88`
   Rule: `HLT-029-RUST-BAD-BEHAVIOR`
   Check: `HLT-029-RUST-BAD-BEHAVIOR:security` `hard` confidence `0.95`
   Route: TLR `Security, secrets, agency`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#top-level-risk-mapping`
   Matched term: `rust.unsafe.zeroed`
   Reason: all-zero validity was not proven
   Fix: construct the type with a valid initializer instead of zeroing it
   Rerun: `just fast`
   Fingerprint: `sha256:2db32e4b761c181c90c1dd3953584c87ca743a0a4fa6c0648481d7a132a4b537`
   Evidence: detector=zeroed, proof-window=NearbySafetyComment, snippet=let mut buf = BytesMut::zeroed(64);
94. `high` `security` `jansu-cli/src/cli/user.rs:125`
   Rule: `HLT-029-RUST-BAD-BEHAVIOR`
   Check: `HLT-029-RUST-BAD-BEHAVIOR:security` `hard` confidence `0.95`
   Route: TLR `Security, secrets, agency`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#top-level-risk-mapping`
   Matched term: `rust.unsafe.zeroed`
   Reason: all-zero validity was not proven
   Fix: construct the type with a valid initializer instead of zeroing it
   Rerun: `just fast`
   Fingerprint: `sha256:6bc45337c0ccbdacb9c8100031ee17b6b376b47f9ac4514b0c48e40ef60b50d7`
   Evidence: detector=zeroed, proof-window=NearbySafetyComment, snippet=let mut salt = BytesMut::zeroed(Self::DEFAULT_SALT_LEN);
95. `high` `vibe` `jansu-client/src/lib.rs:311`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:818359831b3c9de8932f9018a555c14ba6e07554823bc17dc1bf89a0bb2539f2`
   Evidence: jansu-client/src/lib.rs:311, future-hostile/dead-language term `temporary` appears
96. `high` `vibe` `jansu-model/src/lib.rs:328`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `deprecated` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:7588de33ed685b5ffe735a5706036848a24eb22fb984d3bcf00575b109614ba6`
   Evidence: jansu-model/src/lib.rs:328, future-hostile/dead-language term `deprecated` appears
97. `high` `vibe` `jansu-model/src/lib.rs:329`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `deprecated` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:be4bd167313ccceec91a38915c7f73168d6e44d182c0850467a7cf67d5c47f77`
   Evidence: jansu-model/src/lib.rs:329, future-hostile/dead-language term `deprecated` appears
98. `high` `vibe` `jansu-model/src/lib.rs:349`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `deprecated` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:403a787a830e86409c4783938141921bc655d226b5abf9a74b41797693cae0a8`
   Evidence: jansu-model/src/lib.rs:349, future-hostile/dead-language term `deprecated` appears
99. `high` `vibe` `jansu-model/src/lib.rs:350`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `deprecated` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:fe8400e7dfdfedf1e75c9077ab79c0d44eb3341eca825a9c8172a9ab8d2b3931`
   Evidence: jansu-model/src/lib.rs:350, future-hostile/dead-language term `deprecated` appears
100. `high` `vibe` `jansu-model/src/lib.rs:367`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `deprecated` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:6a93be6a061ef23826ea3f27894603a8e21864ab5a8fda7d3dca5be74bdcf2c5`
   Evidence: jansu-model/src/lib.rs:367, future-hostile/dead-language term `deprecated` appears
101. `high` `vibe` `jansu-model/src/lib.rs:1401`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `deprecated` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b5e76d8caef09812dedd900dd9468082df2f15789d26f6a588a1086d7ddfb16a`
   Evidence: jansu-model/src/lib.rs:1401, future-hostile/dead-language term `deprecated` appears
102. `high` `vibe` `jansu-model/src/lib.rs:1701`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `deprecated` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a592ab6e36e6bbb4952a6fedde4f623ac4336148b5d97b3f691e26f9b9f35101`
   Evidence: jansu-model/src/lib.rs:1701, future-hostile/dead-language term `deprecated` appears
103. `high` `vibe` `jansu-model/src/lib.rs:1750`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `deprecated` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:32d23a9e6b072a89286ea2bf8e419d510e8277355fc944eb45862a9eef279b50`
   Evidence: jansu-model/src/lib.rs:1750, future-hostile/dead-language term `deprecated` appears
104. `high` `vibe` `jansu-sans-io/src/acl.rs:20`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:1bed3c0f933949c94ed73ea3825e9ef37a14fc7a4e6f098349288f60faaecfc9`
   Evidence: jansu-sans-io/src/acl.rs:20, future-hostile/dead-language term `old` appears
105. `high` `vibe` `jansu-sans-io/src/acl.rs:113`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:eb623f4d3f17553cdf0fa21e00919740857ea723d6a642c1e21e38975bc7c1e3`
   Evidence: jansu-sans-io/src/acl.rs:113, future-hostile/dead-language term `old` appears
106. `high` `vibe` `jansu-sans-io/src/lib.rs:1172`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:5076b4172893dc63dd693bdfacbb60a7d916a04dbbff10b4b5871d25e1b1930d`
   Evidence: jansu-sans-io/src/lib.rs:1172, future-hostile/dead-language term `old` appears
107. `high` `vibe` `jansu-sans-io/src/lib.rs:1372`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `stale` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:4effb4d2c284a4cbc052796ef5dc4adac235e26a25cfa054b78a6b7299f8140b`
   Evidence: jansu-sans-io/src/lib.rs:1372, future-hostile/dead-language term `stale` appears
108. `high` `vibe` `jansu-sans-io/src/resource.rs:20`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:bad4263abfe6807091747ffc2208070d22abe5baff9f28277fce4417d4e89364`
   Evidence: jansu-sans-io/src/resource.rs:20, future-hostile/dead-language term `old` appears
109. `high` `vibe` `jansu-sans-io/src/ser.rs:1247`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:6206677214b59787575b0e5f27d624808b8ae353256735c17b948295c5dc1f14`
   Evidence: jansu-sans-io/src/ser.rs:1247, future-hostile/dead-language term `todo` appears
110. `high` `vibe` `jansu-sans-io/src/ser.rs:1251`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b2043d7d4b6547a009aad33d17d39aaf7dddf674b0ebde39dad3f1f5e1e6179a`
   Evidence: jansu-sans-io/src/ser.rs:1251, future-hostile/dead-language term `todo` appears
111. `high` `vibe` `jansu-sans-io/src/ser.rs:1264`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:3646174e265e25f970182b1c796c245fd5f98fbec5bd1fc469474d0d5bc16021`
   Evidence: jansu-sans-io/src/ser.rs:1264, future-hostile/dead-language term `todo` appears
112. `high` `vibe` `jansu-sans-io/src/ser.rs:1268`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:297dd8d8f4e9b6834f827ff505ea6a443f30c8c1c46d9f6c271abcaaa249c088`
   Evidence: jansu-sans-io/src/ser.rs:1268, future-hostile/dead-language term `todo` appears
113. `high` `vibe` `jansu-sans-io/src/ser.rs:1281`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:11ecb6afea61f84eb30a932ce58afaed200d22a98f5b78ee8320b21ed9964c3f`
   Evidence: jansu-sans-io/src/ser.rs:1281, future-hostile/dead-language term `todo` appears
114. `high` `vibe` `jansu-sans-io/src/ser.rs:1288`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:56c0dde8f4dd2ec9f2773486e69b2f8b09289727b9e9a6ba880d9a38d6280ae1`
   Evidence: jansu-sans-io/src/ser.rs:1288, future-hostile/dead-language term `todo` appears
115. `high` `vibe` `jansu-sans-io/src/ser.rs:1292`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:6453e1b315c4b42a051ee4d698b9652be5fc5adc5e05bf15dab8045f6598fbd6`
   Evidence: jansu-sans-io/src/ser.rs:1292, future-hostile/dead-language term `todo` appears
116. `high` `vibe` `jansu-sans-io/src/ser.rs:1327`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b402528297c3e2e72c1a864a34adc778f4cf812adafe1e2fc54868f0119a004f`
   Evidence: jansu-sans-io/src/ser.rs:1327, future-hostile/dead-language term `todo` appears
117. `high` `vibe` `jansu-sans-io/src/ser.rs:1331`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:1133b94a04c6408b40a2085b2ec48a9de12cecee4dd244600e4e7ed3adc6945c`
   Evidence: jansu-sans-io/src/ser.rs:1331, future-hostile/dead-language term `todo` appears
118. `high` `vibe` `jansu-schema/src/avro.rs:541`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a518ac91d3a5020b0e0fc93ce83b79e248a3d68c7e9e1346378a0a9344198333`
   Evidence: jansu-schema/src/avro.rs:541, future-hostile/dead-language term `todo` appears
119. `high` `vibe` `jansu-schema/src/avro.rs:569`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:554d7dc7c7485667a1aee9bd39642f712dfa3f2517eefa18b76dfb2681df4b10`
   Evidence: jansu-schema/src/avro.rs:569, future-hostile/dead-language term `todo` appears
120. `high` `vibe` `jansu-schema/src/avro.rs:593`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:e57515fac56edda93b9abe3db34f78a29cdaf182478588d10d0b284eca96fc83`
   Evidence: jansu-schema/src/avro.rs:593, future-hostile/dead-language term `todo` appears
121. `high` `vibe` `jansu-schema/src/avro.rs:595`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:ec620c6eccdbf4c19b51a93aaef4ef7bb2be3c8574ae4eeac7ce8b5d94f47179`
   Evidence: jansu-schema/src/avro.rs:595, future-hostile/dead-language term `todo` appears
122. `high` `vibe` `jansu-schema/src/avro.rs:596`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:64a34b3490622867db73239a2e15e99419ccbbe30a97c117f6f13ece3ee9922c`
   Evidence: jansu-schema/src/avro.rs:596, future-hostile/dead-language term `todo` appears
123. `high` `vibe` `jansu-schema/src/avro.rs:598`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:7a559a50bafe370a4ddaba04d579b1c71a000a4b05047355301639d753a6615a`
   Evidence: jansu-schema/src/avro.rs:598, future-hostile/dead-language term `todo` appears
124. `high` `vibe` `jansu-schema/src/avro.rs:599`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:889f9414347eefbd01589e808bf5cfaae5e0a298549e6c80db2223604c84a2ea`
   Evidence: jansu-schema/src/avro.rs:599, future-hostile/dead-language term `todo` appears
125. `high` `vibe` `jansu-schema/src/avro.rs:601`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:984f15085b13ffef3ab2213cfbdf9fba77bafff6f30b072932ea3a6fabf9371d`
   Evidence: jansu-schema/src/avro.rs:601, future-hostile/dead-language term `todo` appears
126. `high` `vibe` `jansu-schema/src/avro.rs:602`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:047d4bf9e97d92d65ac26e2ab57ac5efa8bd1014e0afc3381532feb3747fb1d0`
   Evidence: jansu-schema/src/avro.rs:602, future-hostile/dead-language term `todo` appears
127. `high` `vibe` `jansu-schema/src/avro.rs:603`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a54eebcbc0cc2e7cb9be453dcb2094e2f22b80f02bfb57066c4b6bfe9ec13c57`
   Evidence: jansu-schema/src/avro.rs:603, future-hostile/dead-language term `todo` appears
128. `high` `vibe` `jansu-schema/src/avro.rs:605`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:d4c21d8c42adfd3ed210c1f6bcda8069a76882f78a6cc721d0913705b42b75d2`
   Evidence: jansu-schema/src/avro.rs:605, future-hostile/dead-language term `todo` appears
129. `high` `vibe` `jansu-schema/src/avro.rs:606`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a13b7932e0d05feac517b82202a5092de986ad5b56c7971c8670458c1c4a9f42`
   Evidence: jansu-schema/src/avro.rs:606, future-hostile/dead-language term `todo` appears
130. `high` `vibe` `jansu-schema/src/avro.rs:607`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:686cb592c729faa7f8013db764c692fa715ebd34aa15c54b4fb572bae83fc67c`
   Evidence: jansu-schema/src/avro.rs:607, future-hostile/dead-language term `todo` appears
131. `high` `vibe` `jansu-schema/src/avro.rs:609`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:32737232562712096c3d9ee2e3aadf5126c942d894262588258a26b839354237`
   Evidence: jansu-schema/src/avro.rs:609, future-hostile/dead-language term `todo` appears
132. `high` `vibe` `jansu-schema/src/avro/arrow.rs:220`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:c023102f4ac6e9a716ea660a4944d6a69200170597ae43c7d2a96c49fbf24619`
   Evidence: jansu-schema/src/avro/arrow.rs:220, future-hostile/dead-language term `todo` appears
133. `high` `vibe` `jansu-schema/src/avro/arrow.rs:252`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:3d9fc2989fe616d89fb30cc81ba3bf635b4add296f3028dd7c178c64456d6ce5`
   Evidence: jansu-schema/src/avro/arrow.rs:252, future-hostile/dead-language term `todo` appears
134. `high` `vibe` `jansu-schema/src/avro/arrow.rs:328`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:dbcba0d696f12ebae478a45a503a0ac1105d821da9642c40438b00aa343cb119`
   Evidence: jansu-schema/src/avro/arrow.rs:328, future-hostile/dead-language term `todo` appears
135. `high` `vibe` `jansu-schema/src/avro/arrow.rs:361`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b34cb647d46792dd7b8261758b959fc71c7ddae82b8beb70d806f2eef0da2618`
   Evidence: jansu-schema/src/avro/arrow.rs:361, future-hostile/dead-language term `todo` appears
136. `high` `vibe` `jansu-schema/src/avro/arrow.rs:387`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:23e86ca82ba3ed501e76337de0778e918cec3e5ada62c249456a79de8c405b92`
   Evidence: jansu-schema/src/avro/arrow.rs:387, future-hostile/dead-language term `todo` appears
137. `high` `vibe` `jansu-schema/src/avro/arrow.rs:564`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:bad755ce5120849843493d5193ce8ab27719e2227c95cc71bd3c5a5c8e18b8d5`
   Evidence: jansu-schema/src/avro/arrow.rs:564, future-hostile/dead-language term `todo` appears
138. `high` `vibe` `jansu-schema/src/avro/arrow.rs:565`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:bec4906d1dd0bba5c18d69906b02b19cefe5701e668623bcfad80d3785efe64e`
   Evidence: jansu-schema/src/avro/arrow.rs:565, future-hostile/dead-language term `todo` appears
139. `high` `vibe` `jansu-schema/src/avro/arrow.rs:566`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:bd3e8b15909e069dc7816f991edbc79c717c33a5936791ff36314b9fba72403c`
   Evidence: jansu-schema/src/avro/arrow.rs:566, future-hostile/dead-language term `todo` appears
140. `high` `vibe` `jansu-schema/src/avro/arrow.rs:588`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:2f3b98aa5c14c8379477786aa0e9e5af379c25ea6663a8ad2df6783a1c9b2752`
   Evidence: jansu-schema/src/avro/arrow.rs:588, future-hostile/dead-language term `todo` appears
141. `high` `vibe` `jansu-schema/src/avro/arrow.rs:589`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:5a0af089b96a508da6e63737f2341b232744b5e1cb7f69b12f1e9b20e92ae362`
   Evidence: jansu-schema/src/avro/arrow.rs:589, future-hostile/dead-language term `todo` appears
142. `high` `vibe` `jansu-schema/src/avro/arrow.rs:590`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:aa3d46788c34f988d99d4f85f01017a1874a64da11f76b51069c47282f1a8a18`
   Evidence: jansu-schema/src/avro/arrow.rs:590, future-hostile/dead-language term `todo` appears
143. `high` `vibe` `jansu-schema/src/avro/arrow.rs:591`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:8a302bb37666e5bf0d2bfa1e9566fe6bc9dffda575efc3452f27ea0ab6cf0a75`
   Evidence: jansu-schema/src/avro/arrow.rs:591, future-hostile/dead-language term `todo` appears
144. `high` `vibe` `jansu-schema/src/avro/arrow.rs:647`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:239e69a10cfe716f9316a3e05ca5755478e35b00c8bc6bded0a1f655f5a63cb7`
   Evidence: jansu-schema/src/avro/arrow.rs:647, future-hostile/dead-language term `todo` appears
145. `high` `vibe` `jansu-schema/src/avro/arrow.rs:648`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:ad808a570773a7ce5e64f7e9defb5e677ea24490b49ebee85f106708ee339504`
   Evidence: jansu-schema/src/avro/arrow.rs:648, future-hostile/dead-language term `todo` appears
146. `high` `vibe` `jansu-schema/src/avro/arrow.rs:649`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:4209021b8415a09554a0d24fbe4f19daf78af5094ef450607b16ab3e3314b132`
   Evidence: jansu-schema/src/avro/arrow.rs:649, future-hostile/dead-language term `todo` appears
147. `high` `vibe` `jansu-schema/src/avro/arrow.rs:650`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:bbd586cb1c98a59cadf8c973546c155e94a822831c3d3fe3e895e3e71867abfc`
   Evidence: jansu-schema/src/avro/arrow.rs:650, future-hostile/dead-language term `todo` appears
148. `high` `vibe` `jansu-schema/src/avro/arrow.rs:651`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:1cda46451ab417f172468464e8eb2996a1d30e06431601f42cbf6c10df3855fe`
   Evidence: jansu-schema/src/avro/arrow.rs:651, future-hostile/dead-language term `todo` appears
149. `high` `vibe` `jansu-schema/src/avro/arrow.rs:652`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:3771a2dbdb47321f25ed9b28d212e3604f86ca42f9995ce23716bd1dd4337f32`
   Evidence: jansu-schema/src/avro/arrow.rs:652, future-hostile/dead-language term `todo` appears
150. `high` `vibe` `jansu-schema/src/avro/arrow.rs:653`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:7c8d2d024c3a651d866b7aed2d586acfb11b8b79bb7073f9a523f995222cc7ea`
   Evidence: jansu-schema/src/avro/arrow.rs:653, future-hostile/dead-language term `todo` appears
151. `high` `vibe` `jansu-schema/src/avro/arrow.rs:656`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:900f781e2178bf228b275c16788a69d7aa58c35f24f24e789ead231882fd743a`
   Evidence: jansu-schema/src/avro/arrow.rs:656, future-hostile/dead-language term `todo` appears
152. `high` `vibe` `jansu-schema/src/avro/arrow.rs:743`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:5c3b230b5349b54be891c6792e0a359a1ee555a991c6918ec382d7cf44cb41be`
   Evidence: jansu-schema/src/avro/arrow.rs:743, future-hostile/dead-language term `todo` appears
153. `high` `vibe` `jansu-schema/src/avro/arrow.rs:751`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:0a274c1141e6195796bbd62fccfd9f5fcd73e5057260ad62ad6cb2245fa79881`
   Evidence: jansu-schema/src/avro/arrow.rs:751, future-hostile/dead-language term `todo` appears
154. `high` `vibe` `jansu-schema/src/avro/arrow.rs:752`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:2b82d52065e5387f7dd7a4df9b65b3ec5944cedbc0baff1373c6ce0c97200ba4`
   Evidence: jansu-schema/src/avro/arrow.rs:752, future-hostile/dead-language term `todo` appears
155. `high` `vibe` `jansu-schema/src/avro/arrow.rs:753`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a71adc1cc73a6a8e58418bab6a16dd6cf04d5bc570ed8ce93f3ebccb64c1ef6b`
   Evidence: jansu-schema/src/avro/arrow.rs:753, future-hostile/dead-language term `todo` appears
156. `high` `vibe` `jansu-schema/src/avro/arrow.rs:790`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:ad73f1e1fd461f891ae02795945b937fc5edc34cc1ea2883711aeec5e8a5a16f`
   Evidence: jansu-schema/src/avro/arrow.rs:790, future-hostile/dead-language term `todo` appears
157. `high` `vibe` `jansu-schema/src/avro/arrow.rs:791`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:bdd7d469b8dd100ffa4b6d3749563b65d888e62b05626368ee4e46c2618a4f0b`
   Evidence: jansu-schema/src/avro/arrow.rs:791, future-hostile/dead-language term `todo` appears
158. `high` `vibe` `jansu-schema/src/avro/arrow.rs:792`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:62f2b3204c29c41967394c876341f22720ed47a89a83d24828423013e9ca1821`
   Evidence: jansu-schema/src/avro/arrow.rs:792, future-hostile/dead-language term `todo` appears
159. `high` `vibe` `jansu-schema/src/avro/arrow.rs:793`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:08629827ec00dd3c7d0a0ef13e7f2285329a6e7e132bc5baeb73fb4e69fc25c6`
   Evidence: jansu-schema/src/avro/arrow.rs:793, future-hostile/dead-language term `todo` appears
160. `high` `vibe` `jansu-schema/src/avro/arrow.rs:796`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:0d4519e03860f37657c5034652887f8b1b13c2a49ffdacb565015efcf6f92209`
   Evidence: jansu-schema/src/avro/arrow.rs:796, future-hostile/dead-language term `todo` appears
161. `high` `vibe` `jansu-schema/src/avro/arrow.rs:929`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b20436baee9fd0ebfa2838d36cf20d632e3547a3ad598c494144407af229f1e6`
   Evidence: jansu-schema/src/avro/arrow.rs:929, future-hostile/dead-language term `todo` appears
162. `high` `vibe` `jansu-schema/src/avro/arrow.rs:986`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:6f9443c9b598fec65d7a256f8358b7db6b0a06543e4274e2f84a23da366190de`
   Evidence: jansu-schema/src/avro/arrow.rs:986, future-hostile/dead-language term `todo` appears
163. `high` `vibe` `jansu-schema/src/avro/arrow.rs:1019`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:d20835014964e918101bef755436596cc3dc69c4e6eaac131a53ddfcfb010570`
   Evidence: jansu-schema/src/avro/arrow.rs:1019, future-hostile/dead-language term `todo` appears
164. `high` `vibe` `jansu-schema/src/avro/arrow.rs:1022`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:3e93d60048a0154ab26a1b135b6f687cb8e5317156a9a3aa869a9dffa682dc8a`
   Evidence: jansu-schema/src/avro/arrow.rs:1022, future-hostile/dead-language term `todo` appears
165. `high` `vibe` `jansu-schema/src/avro/arrow.rs:1055`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:536bd86b1e1afa0bd696965c3819927148b22fa1ea1ae0c36d67ee04c8f35410`
   Evidence: jansu-schema/src/avro/arrow.rs:1055, future-hostile/dead-language term `todo` appears
166. `high` `vibe` `jansu-schema/src/avro/arrow.rs:1058`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b074670942e6e51e3c8168f9026a3b98746341d9e90cd2184981f4c971b4815c`
   Evidence: jansu-schema/src/avro/arrow.rs:1058, future-hostile/dead-language term `todo` appears
167. `high` `vibe` `jansu-schema/src/avro/arrow.rs:1061`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:cdf35689d0e01efb6930a0db6bfbda53da97dee9573159e6774c4b882e612451`
   Evidence: jansu-schema/src/avro/arrow.rs:1061, future-hostile/dead-language term `todo` appears
168. `high` `vibe` `jansu-schema/src/avro/arrow.rs:1064`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a674be9d9a9ad96c6c2f2487aae3e51c74cdad0fa09d87620c932af757a66813`
   Evidence: jansu-schema/src/avro/arrow.rs:1064, future-hostile/dead-language term `todo` appears
169. `high` `security` `jansu-schema/src/avro/arrow.rs:1430`
   Rule: `HLT-023-INPUT-BOUNDARY-GAP`
   Check: `HLT-023-INPUT-BOUNDARY-GAP:security` `hard` confidence `0.88`
   Route: TLR `Security, secrets, agency`, lane `security`, owner `ops`
   Docs: `docs/audit-rubric.md#top-level-risk-mapping`
   Matched term: `string sql`
   Reason: input handling risk needs deterministic negative tests
   Fix: replace unsafe sinks with typed schemas, parameterized APIs, allowlists, or sandboxed execution plus negative tests
   Rerun: `just security`
   Fingerprint: `sha256:d5675b925cb467255d5da4f31f1f1f57387a1d90592bc4ed1fb65fbca2ad83d0`
   Evidence: let df = ctx.sql(format!("select * from {topic}").as_str()).await?;
170. `high` `vibe` `jansu-schema/src/json.rs:162`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:7c1d1ad0a201890878187e1bda648442d598657f9967bd3f5b899445ad140dd3`
   Evidence: jansu-schema/src/json.rs:162, future-hostile/dead-language term `todo` appears
171. `high` `vibe` `jansu-schema/src/json.rs:169`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:fbe41d74388688571194573c5ba2cfc430937d3cd117c29421217f4d8865e204`
   Evidence: jansu-schema/src/json.rs:169, future-hostile/dead-language term `todo` appears
172. `high` `vibe` `jansu-schema/src/lake/delta.rs:945`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:104dc3e6bb33cc062aa4320513d5eb5f9c75141f2a822cefcdd09ae34974982d`
   Evidence: jansu-schema/src/lake/delta.rs:945, future-hostile/dead-language term `temporary` appears
173. `high` `vibe` `jansu-schema/src/lake/delta.rs:1045`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:463d80cb3fb42d9a86d8747751f3ab7d4ce6a5e4a2223d5319dab0ef1b2b64cb`
   Evidence: jansu-schema/src/lake/delta.rs:1045, future-hostile/dead-language term `temporary` appears
174. `high` `vibe` `jansu-schema/src/lake/delta.rs:1138`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:eb4b4ccaa3cda4c80b3b553b56da63e407707df8acf1684050c3acc806b0b060`
   Evidence: jansu-schema/src/lake/delta.rs:1138, future-hostile/dead-language term `temporary` appears
175. `high` `vibe` `jansu-schema/src/lake/delta.rs:1242`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a41fcfcef1643644031d343cc7fca8fa1b59f20dc202dedf01bb32db01699196`
   Evidence: jansu-schema/src/lake/delta.rs:1242, future-hostile/dead-language term `temporary` appears
176. `high` `vibe` `jansu-schema/src/lake/delta.rs:1356`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:726998d7e6ec19aeb209a7f681dfdf8c3f3d7169e8e653d252ee407299f85a2f`
   Evidence: jansu-schema/src/lake/delta.rs:1356, future-hostile/dead-language term `temporary` appears
177. `high` `vibe` `jansu-schema/src/lake/delta.rs:1470`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:84cad81d54ebd2f569e2bc7e118022bb49698495912ab43544fe1bf0922834a4`
   Evidence: jansu-schema/src/lake/delta.rs:1470, future-hostile/dead-language term `temporary` appears
178. `high` `vibe` `jansu-schema/src/lake/delta.rs:1574`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:e4a4394b10850a71f000a6463d2e777916ef806549e6e005454af55b9364cb78`
   Evidence: jansu-schema/src/lake/delta.rs:1574, future-hostile/dead-language term `temporary` appears
179. `high` `vibe` `jansu-schema/src/lake/delta.rs:1688`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:da2dabed5e80a0064db0060385c934e95cc8466fd07cd9b7438be5d77e9a625c`
   Evidence: jansu-schema/src/lake/delta.rs:1688, future-hostile/dead-language term `temporary` appears
180. `high` `vibe` `jansu-schema/src/lake/delta.rs:1769`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:27049cb27fc6d4dc8d8146c10d273292ed324865f885fc7070a5c46cffe877d8`
   Evidence: jansu-schema/src/lake/delta.rs:1769, future-hostile/dead-language term `temporary` appears
181. `high` `vibe` `jansu-schema/src/lake/delta.rs:1916`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a5d24a5e468267243b5234111b1e3cf47e4cf8e0764263009548cf5753808652`
   Evidence: jansu-schema/src/lake/delta.rs:1916, future-hostile/dead-language term `temporary` appears
182. `high` `vibe` `jansu-schema/src/lake/delta.rs:2006`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:52db7daaf442099e5eb9c7014a6c9d6b119a9faea067589514a5b2d80db56c52`
   Evidence: jansu-schema/src/lake/delta.rs:2006, future-hostile/dead-language term `temporary` appears
183. `high` `vibe` `jansu-schema/src/lake/delta.rs:2284`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:751fcb44e766a81194f0a66c9710039a82197e86ad6ec9f20e1f8498d37fc3e3`
   Evidence: jansu-schema/src/lake/delta.rs:2284, future-hostile/dead-language term `temporary` appears
184. `high` `vibe` `jansu-schema/src/lake/delta.rs:2421`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:f48701f8a4944ab765b2d1b50f96dd79933fa22106d6465c320de14d4df101cf`
   Evidence: jansu-schema/src/lake/delta.rs:2421, future-hostile/dead-language term `temporary` appears
185. `high` `vibe` `jansu-schema/src/proto.rs:240`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:fff38a330d23dcc80d84392764ea6a68bf1b7a443c2222bac8a6a69d67636a46`
   Evidence: jansu-schema/src/proto.rs:240, future-hostile/dead-language term `todo` appears
186. `high` `vibe` `jansu-schema/src/proto.rs:365`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:6a4b32d365ab54216c857d48f1961c32381cd121fcbb47bd26faba296665100e`
   Evidence: jansu-schema/src/proto.rs:365, future-hostile/dead-language term `todo` appears
187. `high` `vibe` `jansu-schema/src/proto.rs:559`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:c907e2a86efce5f3b492065e66cb904a47ac4959262102d4f09326f60fb0e13c`
   Evidence: jansu-schema/src/proto.rs:559, future-hostile/dead-language term `todo` appears
188. `high` `vibe` `jansu-schema/src/proto.rs:614`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:41e492094a5e2520890d095933a471f90ce86ecdd180f40d155f96bd2d63d98d`
   Evidence: jansu-schema/src/proto.rs:614, future-hostile/dead-language term `todo` appears
189. `high` `vibe` `jansu-schema/src/sql.rs:33`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:ec494ec0bd95da83295be85532ff0e93c8765376d7ea8396926a2a2fb61a6525`
   Evidence: jansu-schema/src/sql.rs:33, future-hostile/dead-language term `todo` appears
190. `high` `vibe` `jansu-schema/src/sql.rs:42`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:f9fbc401d68e5e16c3ad65ca67f067e44e8910444784e18605546756a1aaae23`
   Evidence: jansu-schema/src/sql.rs:42, future-hostile/dead-language term `todo` appears
191. `high` `data` `jansu-storage/src/ddl/020-consumer-group.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:edff3c67c24176acb2888edbb992ffb9410e0be1e5cb6a5b523fb4e8f750ab64`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=cluster integer references cluster (id) on delete cascade not null,
192. `high` `data` `jansu-storage/src/ddl/020-producer.sql:19`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:fa6bbda19c2beea4dc2e929c1df901f4a1e08372a82717baed2763121652d97a`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=cluster int references cluster (id) on delete cascade not null,
193. `high` `data` `jansu-storage/src/ddl/020-scram-credential.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:e2e8dd86badacc424592a375612c1a5b5dd8aac947954841ef9300aeac36501b`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=cluster integer references cluster (id) on delete cascade not null,
194. `high` `data` `jansu-storage/src/ddl/020-topic.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:3a96bfe0d00636025f0e33a3d6b171ef04cba87c11191cdaf7a6b2fc9453d524`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=cluster int references cluster (id) on delete cascade not null,
195. `high` `data` `jansu-storage/src/ddl/030-consumer-group-detail.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:c61b55cd2e082b2fafe4f77985e22b38ae11f70b821bca8edf4f760bca0b0707`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=consumer_group int references consumer_group (id) on delete cascade,
196. `high` `data` `jansu-storage/src/ddl/030-producer-epoch.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:c9d359a278759eae4bf963f5e15121645137f654b7732200a50f1dab2e5daa04`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=producer integer references producer (id) on delete cascade,
197. `high` `data` `jansu-storage/src/ddl/030-topic-configuration.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:c66e5fe096e606dd604f55fe3363491897319afbb6c7a384d77b9c0bcc061ca0`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topic int references topic (id) on delete cascade,
198. `high` `data` `jansu-storage/src/ddl/030-topition.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:29eaa1069e6ee5c899ec053e7b4b118975a0d7ccf7ea5b3652fdf65dcfb17b8c`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topic integer references topic (id) on delete cascade,
199. `high` `data` `jansu-storage/src/ddl/030-txn.sql:19`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:ef57d6394efec4c3edea17c557518e70e2cd912a467d3d3a2cc73d65ece02cc3`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=cluster integer references cluster (id) on delete cascade,
200. `high` `data` `jansu-storage/src/ddl/030-txn.sql:21`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:c66f8d4bad35c5ec9b7b210edcd861481c27d611c5fe4de8b4d7866b856b7848`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=producer integer references producer (id) on delete cascade,
201. `high` `data` `jansu-storage/src/ddl/030-virtual-topic.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:53148ee9ca1ecfe9cfd97756c859e0fa7aa94eede5dd6e63089b34e113eab3e4`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topic integer references topic (id) on delete cascade,
202. `high` `data` `jansu-storage/src/ddl/040-consumer-offset.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:373263fe835d8d68561ffbeab7f2be16a473d4cfc9b3a7b5b0581636488371e9`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=consumer_group integer references consumer_group (id) on delete cascade,
203. `high` `data` `jansu-storage/src/ddl/040-consumer-offset.sql:18`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:ffbb5f2c0d952c823d8ab568d0fdea28de113281579b25af569b740d75322bfa`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition integer references topition (id) on delete cascade,
204. `high` `data` `jansu-storage/src/ddl/040-header.sql:22`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:204950cd231f01782bffe7c6b61da1fc0be3e452a3e4cca580a8f9537ffbee65`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=foreign key (topition, offset_id) references record (topition, offset_id) on delete cascade
205. `high` `data` `jansu-storage/src/ddl/040-leader-epoch-history.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:c443b4e8e144e37db6c1c82e71e09d1f69bac3d56516da3ae5a0332444c39702`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition integer references topition (id) on delete cascade,
206. `high` `data` `jansu-storage/src/ddl/040-producer-detail.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:bc81d8d35d7dc586c8ae14c1bdfc4881cdc3c090f7a88a0c82ab7ca9ca428cb7`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=producer_epoch integer references producer_epoch (id) on delete cascade,
207. `high` `data` `jansu-storage/src/ddl/040-producer-detail.sql:18`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:d3f3f9c4fec59f131bf895d4fab701e616c696d78ee51a1262e1ea069a1f11d8`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition integer references topition (id) on delete cascade,
208. `high` `data` `jansu-storage/src/ddl/040-record.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:f305eb47d4694b29abbc350119b53350faf303c9b8d59f522838678a1659453f`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition integer references topition (id) on delete cascade,
209. `high` `data` `jansu-storage/src/ddl/040-watermark.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:3f1200c20eceb7b7e290bb32f5dbbb2b8d3f73ccdbe2afd146203cbef869d096`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition integer references topition (id) on delete cascade,
210. `high` `data` `jansu-storage/src/ddl/050-txn-offset-commit.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:332324e50732b8467258c4fc20a562e8f4b9652c9473dc6f43d3b8a13a3ee5a4`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=txn_detail integer references txn_detail (id) on delete cascade,
211. `high` `data` `jansu-storage/src/ddl/050-txn-offset-commit.sql:18`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:f17695f1226f652a4fd8d0e96fc2c9aa92841726db223dffba05b879a8def765`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=consumer_group integer references consumer_group (id) on delete cascade,
212. `high` `data` `jansu-storage/src/ddl/050-txn-topition.sql:19`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:ed9d7ef5bdb01d7a44349a079bc3230c89177bbc7e7f29b8781d2107b2cd17ac`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=txn_detail integer references txn_detail (id) on delete cascade,
213. `high` `data` `jansu-storage/src/ddl/050-txn-topition.sql:20`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:7d5d74ae397da7b5163ba002072e2b4d7faafd6d4d29bbd32a0fb3a5a62f2e3e`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition integer references topition (id) on delete cascade,
214. `high` `data` `jansu-storage/src/ddl/060-txn-offset-commit-tp.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:b8eab459e7994f39e3c37c1e2d443daba60936790ead3a5d7f43bb803d967473`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=offset_commit integer references txn_offset_commit (id) on delete cascade,
215. `high` `data` `jansu-storage/src/ddl/060-txn-offset-commit-tp.sql:18`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:8f2360d9ff8e8499a405dc20c0d5922836624677fc29ef17bd8d2aafcf118a07`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition integer references topition (id) on delete cascade,
216. `high` `data` `jansu-storage/src/ddl/060-txn-produce-offset.sql:17`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:22f483e56b1be7dd06fd1662026c7254b19cd5dfc2000a456cec0a785fa5ecc6`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=txn_topition integer references txn_topition (id) on delete cascade unique,
217. `high` `vibe` `jansu-storage/src/dynostore.rs:243`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b6febde9c32e44b345b723d3dacced7af67593ad3e315c8e9242180ea57a3b0d`
   Evidence: jansu-storage/src/dynostore.rs:243, future-hostile/dead-language term `todo` appears
218. `high` `vibe` `jansu-storage/src/dynostore.rs:244`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:00ed810860108bf7dceaaa48296b6f705f3b4e2b89acd52a5056fc74e57fcbb0`
   Evidence: jansu-storage/src/dynostore.rs:244, future-hostile/dead-language term `todo` appears
219. `high` `vibe` `jansu-storage/src/dynostore.rs:645`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:595fa7723fba9dafeaec55fe6989e9155388829cd679d3c542cc09cc82841255`
   Evidence: jansu-storage/src/dynostore.rs:645, future-hostile/dead-language term `todo` appears
220. `high` `vibe` `jansu-storage/src/dynostore.rs:1825`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:697e379ce5f358aea0e07e7054869c4cea4a018414f04e3809403c1ad35bdcbc`
   Evidence: jansu-storage/src/dynostore.rs:1825, future-hostile/dead-language term `todo` appears
221. `high` `vibe` `jansu-storage/src/dynostore.rs:2184`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:814af32bd88a21d2ead2351f3c15fd723e771d0b42fea92973f00314768a5d9d`
   Evidence: jansu-storage/src/dynostore.rs:2184, future-hostile/dead-language term `todo` appears
222. `high` `vibe` `jansu-storage/src/dynostore.rs:2435`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:74a8768cab2e0ecc54cfe74791cc765a052d9d2fdd6a7b4c73babecaadb82300`
   Evidence: jansu-storage/src/dynostore.rs:2435, future-hostile/dead-language term `todo` appears
223. `high` `vibe` `jansu-storage/src/limbo.rs:202`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:0dc8e8a304f3a47d429b116b7adbd79b06aa695fba44a5e67b1e4dc9a9126e3f`
   Evidence: jansu-storage/src/limbo.rs:202, future-hostile/dead-language term `todo` appears
224. `high` `vibe` `jansu-storage/src/limbo.rs:1144`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:30a1bf58f30846877ac9a8a7880f77585f4f3d4f47a43dec3b74238a1ff01ac0`
   Evidence: jansu-storage/src/limbo.rs:1144, future-hostile/dead-language term `todo` appears
225. `high` `vibe` `jansu-storage/src/limbo.rs:1284`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b1fda096588d6d88f8ddab0e904803e52a65802acd54af9e8a4b9698ae284f42`
   Evidence: jansu-storage/src/limbo.rs:1284, future-hostile/dead-language term `todo` appears
226. `high` `vibe` `jansu-storage/src/limbo.rs:1426`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:8dd6c936fac633b261711fbc6bf5646ed697b38c42f2797a5929dccc6e40fa9e`
   Evidence: jansu-storage/src/limbo.rs:1426, future-hostile/dead-language term `todo` appears
227. `high` `vibe` `jansu-storage/src/limbo.rs:1427`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:7ba215dbfac7f87a3a19f4dc5c1cdba2ed951d4a67b140f27cf141b9596aa062`
   Evidence: jansu-storage/src/limbo.rs:1427, future-hostile/dead-language term `todo` appears
228. `high` `vibe` `jansu-storage/src/lite.rs:1360`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:8a00bd1819d8d33c7e69a7a5eb545ca97ee1748238ac785eacf7751bf68cb3b9`
   Evidence: jansu-storage/src/lite.rs:1360, future-hostile/dead-language term `temporary` appears
229. `high` `vibe` `jansu-storage/src/lite.rs:1361`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:4b2331c90942f6ad519b2b5e931dcb5fab9e672e3bcbfaad57646ce85cd0b3ff`
   Evidence: jansu-storage/src/lite.rs:1361, future-hostile/dead-language term `temporary` appears
230. `high` `vibe` `jansu-storage/src/lite.rs:1362`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:979dc94c3a7c2ee135ddd5830bd073229111d82cd7a2b6a3f5e8661f7ff287ea`
   Evidence: jansu-storage/src/lite.rs:1362, future-hostile/dead-language term `temporary` appears
231. `high` `vibe` `jansu-storage/src/lite.rs:1364`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:94870f1b1e3bccaed38b025dd424aa5e93de5b4ff6a2e94cf81a91fc1993682c`
   Evidence: jansu-storage/src/lite.rs:1364, future-hostile/dead-language term `temporary` appears
232. `high` `vibe` `jansu-storage/src/lite.rs:1370`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `temporary` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:13aaea201d7fc5934f9895848c2f6aacf9de452f2c5b87e4ef8eae8da95d3ab5`
   Evidence: jansu-storage/src/lite.rs:1370, future-hostile/dead-language term `temporary` appears
233. `high` `vibe` `jansu-storage/src/lite.rs:2555`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:664c5fb0c900e660ef59715de646be49e4b32d573ef803469ac0610dd531319a`
   Evidence: jansu-storage/src/lite.rs:2555, future-hostile/dead-language term `todo` appears
234. `high` `vibe` `jansu-storage/src/lite.rs:2690`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:53aae280c7e09c4e9192c50573e5df3e333ef36e95b2d7d063e99286c13fb38d`
   Evidence: jansu-storage/src/lite.rs:2690, future-hostile/dead-language term `todo` appears
235. `high` `vibe` `jansu-storage/src/lite.rs:2691`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:e2ce22b399b28ac579bcba14154e0312d4c1d4439d8036c820d2ebbd6a6ab187`
   Evidence: jansu-storage/src/lite.rs:2691, future-hostile/dead-language term `todo` appears
236. `high` `data` `jansu-storage/src/lite/policy_compact_delete.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:ba1294c71defe1af5cde3d43868682e51acdddd066030b6ec7270538b73947a9`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from record
237. `high` `data` `jansu-storage/src/lite/policy_delete.sql:51`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:1bf0911bbade1a46ff0a24303d8d1c0c2133a9c01dc4c726067f97c98a9d94dd`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from record
238. `high` `vibe` `jansu-storage/src/pg.rs:1815`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:aa1ee3945a2b476f959051a36a5dc13d6f35a644b61994bf01bd4681c7e13f7c`
   Evidence: jansu-storage/src/pg.rs:1815, future-hostile/dead-language term `todo` appears
239. `high` `vibe` `jansu-storage/src/pg.rs:1816`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:246d1969a87ddb8e619ed8554035d97369ed94dd77d28746d722bdd938a27087`
   Evidence: jansu-storage/src/pg.rs:1816, future-hostile/dead-language term `todo` appears
240. `high` `vibe` `jansu-storage/src/service.rs:1595`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:377cea3db75643e30439273d6c11e824c50db716444776746768d0216e993875`
   Evidence: jansu-storage/src/service.rs:1595, future-hostile/dead-language term `todo` appears
241. `high` `vibe` `jansu-storage/src/service.rs:1599`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b1e8578e6980a51f90e37e93a9faaebe528d7cccf009eb60bc28f76613bbf168`
   Evidence: jansu-storage/src/service.rs:1599, future-hostile/dead-language term `todo` appears
242. `high` `vibe` `jansu-storage/src/service.rs:1606`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:7ddc4602b55c7394d8d2cb57f9c07c71d22baec764044bd2258e1bbfac1f1de8`
   Evidence: jansu-storage/src/service.rs:1606, future-hostile/dead-language term `todo` appears
243. `high` `vibe` `jansu-storage/src/service.rs:1613`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:d7bf133501f86f450b5d3ad4bf1f070095281f243d7c245af7cdb532d6b32fed`
   Evidence: jansu-storage/src/service.rs:1613, future-hostile/dead-language term `todo` appears
244. `high` `vibe` `jansu-storage/src/service.rs:1617`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:613898c0379bd2fc3b4ca593f2924c0882f0ed2f6ff226f33f5950d90fe7c966`
   Evidence: jansu-storage/src/service.rs:1617, future-hostile/dead-language term `todo` appears
245. `high` `vibe` `jansu-storage/src/service.rs:1621`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:baec921c749dfc1ec4ce571a7da7252bfb34ba74f3160366125aeadcd7e8c0d8`
   Evidence: jansu-storage/src/service.rs:1621, future-hostile/dead-language term `todo` appears
246. `high` `vibe` `jansu-storage/src/service.rs:1630`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:fd937d28a9bdaadf5cb0db618d5a8606b13b2885f29a36fa9158e1f62332b7e3`
   Evidence: jansu-storage/src/service.rs:1630, future-hostile/dead-language term `todo` appears
247. `high` `vibe` `jansu-storage/src/service.rs:1646`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:502cccfa5fa3945aeefd9d4f367a542ef747b2a1af742c04a08543909183553c`
   Evidence: jansu-storage/src/service.rs:1646, future-hostile/dead-language term `todo` appears
248. `high` `vibe` `jansu-storage/src/service.rs:1654`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:3a786d0dd8cd5c78f482eb763408d49c9ad5ed9cf7374ab3f78c5b123a543f90`
   Evidence: jansu-storage/src/service.rs:1654, future-hostile/dead-language term `todo` appears
249. `high` `vibe` `jansu-storage/src/service.rs:1663`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:555fcaa1a0c112da08ad69faf558025b9466600f8b67df22317292cc14bed9eb`
   Evidence: jansu-storage/src/service.rs:1663, future-hostile/dead-language term `todo` appears
250. `high` `vibe` `jansu-storage/src/service.rs:1671`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b2bae1c12fa1c07f459f0aebd8b2eabe59eedb3b6ba4bf692b35796de6166621`
   Evidence: jansu-storage/src/service.rs:1671, future-hostile/dead-language term `todo` appears
251. `high` `vibe` `jansu-storage/src/service.rs:1680`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b4a54cf2fcc631c826e773d3c6d98462e1c41a12b94001cf5a49ba1064870feb`
   Evidence: jansu-storage/src/service.rs:1680, future-hostile/dead-language term `todo` appears
252. `high` `vibe` `jansu-storage/src/service.rs:1689`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:06119403f333def66e170f799fb725ce7cfae07ea0699f06cd9dbcd885437050`
   Evidence: jansu-storage/src/service.rs:1689, future-hostile/dead-language term `todo` appears
253. `high` `vibe` `jansu-storage/src/service.rs:1696`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:4668930481e3f5a87849fc5c30f8d8cc3884a5840db6e2bc0d09410fc21d1244`
   Evidence: jansu-storage/src/service.rs:1696, future-hostile/dead-language term `todo` appears
254. `high` `vibe` `jansu-storage/src/service.rs:1700`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:69e4acb93d11b9fce1f9657e0cc72f1fbbc15cb5df7ff655d6828298704f7f4c`
   Evidence: jansu-storage/src/service.rs:1700, future-hostile/dead-language term `todo` appears
255. `high` `vibe` `jansu-storage/src/service.rs:1709`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:c7c037ad6133402aeced34c575fe9e49567cc415cc5071836f0c59bacde85811`
   Evidence: jansu-storage/src/service.rs:1709, future-hostile/dead-language term `todo` appears
256. `high` `vibe` `jansu-storage/src/service.rs:1717`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:6d6434ec687ac10a87a1c0001c05e98bc96e579dfd42fbffc675933636df600b`
   Evidence: jansu-storage/src/service.rs:1717, future-hostile/dead-language term `todo` appears
257. `high` `vibe` `jansu-storage/src/service.rs:1725`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:15303e1c14cb8b42a985a46a195c06ff7c9b7d9fe22c9def75f248848e4c5f77`
   Evidence: jansu-storage/src/service.rs:1725, future-hostile/dead-language term `todo` appears
258. `high` `vibe` `jansu-storage/src/service.rs:1734`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a043db49b2ac538322239b0b43fbcd79a9433d794d92e5cb7c235cf171100a55`
   Evidence: jansu-storage/src/service.rs:1734, future-hostile/dead-language term `todo` appears
259. `high` `vibe` `jansu-storage/src/service.rs:1738`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:7ea897dd474948cb3c0d403ec567e10624e5925771a570d26bcef350a64dc797`
   Evidence: jansu-storage/src/service.rs:1738, future-hostile/dead-language term `todo` appears
260. `high` `vibe` `jansu-storage/src/service.rs:1745`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:d9a01eed4940f5fe168c01798b7603b7ee895b547f745e56ce7c48ecaf06a432`
   Evidence: jansu-storage/src/service.rs:1745, future-hostile/dead-language term `todo` appears
261. `high` `vibe` `jansu-storage/src/service.rs:1753`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:827dbf24b7108f6d56494207d35144d67d79b88f7de4256bcbfcc2f99bfa485d`
   Evidence: jansu-storage/src/service.rs:1753, future-hostile/dead-language term `todo` appears
262. `high` `vibe` `jansu-storage/src/service.rs:1762`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:9c8704571a75517260ea33e49bc1f55ea733aa5e107b62569ea433c9a06c95c4`
   Evidence: jansu-storage/src/service.rs:1762, future-hostile/dead-language term `todo` appears
263. `high` `vibe` `jansu-storage/src/service.rs:1771`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:7c03772c5893c658bf7a43ffedd3b23fad4724320e63cdfd2a19f90c8942be71`
   Evidence: jansu-storage/src/service.rs:1771, future-hostile/dead-language term `todo` appears
264. `high` `vibe` `jansu-storage/src/service.rs:1781`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:5277e55c789885c1e10dec425c483a41829c2097a7998cd2fd82fc5d120a4410`
   Evidence: jansu-storage/src/service.rs:1781, future-hostile/dead-language term `todo` appears
265. `high` `vibe` `jansu-storage/src/service.rs:1791`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:bbdcc5c4e44fcd5a06c08004ef72b1f181d9ac997d2aad6e11dbeb0d25090658`
   Evidence: jansu-storage/src/service.rs:1791, future-hostile/dead-language term `todo` appears
266. `high` `vibe` `jansu-storage/src/service.rs:1798`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:f31e0ee440d05a874857b14f84e3afe42186dff464845614688f8ad74bfb0461`
   Evidence: jansu-storage/src/service.rs:1798, future-hostile/dead-language term `todo` appears
267. `high` `vibe` `jansu-storage/src/service.rs:1805`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:34207f5875010d8af590d94a3167e22b8f909c914f2aeb19b4299bc8883ed310`
   Evidence: jansu-storage/src/service.rs:1805, future-hostile/dead-language term `todo` appears
268. `high` `vibe` `jansu-storage/src/service.rs:1815`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:b4025c9fa2dd5eebb9fd7b555a86c1e0e5cd9ea4a0d7ca855ace746893304c7e`
   Evidence: jansu-storage/src/service.rs:1815, future-hostile/dead-language term `todo` appears
269. `high` `vibe` `jansu-storage/src/service.rs:1819`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:f5aacc5364721e4ca44ac444e5c91cf06a4ff7bedc52dbff8a09efefa4667eb4`
   Evidence: jansu-storage/src/service.rs:1819, future-hostile/dead-language term `todo` appears
270. `high` `vibe` `jansu-storage/src/service.rs:1823`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:6a71c8217e8c1368ac19faa5514a2ad4af34db959344b271ca84174208bb76ea`
   Evidence: jansu-storage/src/service.rs:1823, future-hostile/dead-language term `todo` appears
271. `high` `vibe` `jansu-storage/src/service.rs:1827`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:51c19a0ce05775594faece08209fc2cd1aa9416842c0cbd8b80020f38ab4839f`
   Evidence: jansu-storage/src/service.rs:1827, future-hostile/dead-language term `todo` appears
272. `high` `vibe` `jansu-storage/src/service.rs:1831`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:caeb68e9654a6716f8367bc16bc9eaeb39fbb5f0c205ed21322d4d0a6eb23390`
   Evidence: jansu-storage/src/service.rs:1831, future-hostile/dead-language term `todo` appears
273. `high` `vibe` `jansu-storage/src/slate/engine.rs:324`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:495aa1ebd54ab7d4a099a8f538d3a87fbbe288e7900e4a78867906e6256d6495`
   Evidence: jansu-storage/src/slate/engine.rs:324, future-hostile/dead-language term `old` appears
274. `high` `vibe` `jansu-storage/src/slate/engine.rs:325`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:621f72dc5232a64ffe685dd3ba8388c58f1b35ef9eef0b657a8ab24d87f9d4b0`
   Evidence: jansu-storage/src/slate/engine.rs:325, future-hostile/dead-language term `old` appears
275. `high` `vibe` `jansu-storage/src/slate/engine.rs:339`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:02971bfe521dbe7a66e3d206b35b572dc1ae3a3183c9fe56ad64637b29a51e8b`
   Evidence: jansu-storage/src/slate/engine.rs:339, future-hostile/dead-language term `old` appears
276. `high` `vibe` `jansu-storage/src/slate/engine.rs:375`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:84b46f0f6634dfc96a3e5ca69508f412bb6fb4f0b3d82e0086e19a5d81654c17`
   Evidence: jansu-storage/src/slate/engine.rs:375, future-hostile/dead-language term `old` appears
277. `high` `vibe` `jansu-storage/src/slate/storage.rs:158`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:47868af9ecf79ee4cdb7d4f5fbf093d3c4558baa7b9f46bc8cf077647a57e0dc`
   Evidence: jansu-storage/src/slate/storage.rs:158, future-hostile/dead-language term `todo` appears
278. `high` `vibe` `jansu-storage/src/slate/storage.rs:698`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:db2de448769a11c283e810529e06812cb5ed8eed70461fa102ebb5f88513462e`
   Evidence: jansu-storage/src/slate/storage.rs:698, future-hostile/dead-language term `todo` appears
279. `high` `vibe` `jansu-storage/src/slate/storage.rs:791`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:5c771a0c2d4abda3381a842e89fab628605037ceb583a4a5f27b1e93006e79f9`
   Evidence: jansu-storage/src/slate/storage.rs:791, future-hostile/dead-language term `todo` appears
280. `high` `vibe` `jansu-storage/src/slate/storage.rs:1348`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:cd3ce2f36025498e966c1b7d4ebfecab166905379937c81d035042c550e93618`
   Evidence: jansu-storage/src/slate/storage.rs:1348, future-hostile/dead-language term `todo` appears
281. `high` `vibe` `jansu-storage/src/slate/storage.rs:1423`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:e1c695fbcc5e6c33166345dc36c519ad2f4525e735849d8067eeee1f9392175f`
   Evidence: jansu-storage/src/slate/storage.rs:1423, future-hostile/dead-language term `todo` appears
282. `high` `vibe` `jansu-storage/src/slate/storage.rs:1512`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:f20f496a15178d395a5b01106dc21b20260e31249f48af85bdfdc3a978343bda`
   Evidence: jansu-storage/src/slate/storage.rs:1512, future-hostile/dead-language term `todo` appears
283. `high` `vibe` `jansu-storage/src/slate/storage.rs:1602`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:c445fd3fdf3fb9ffabb31bd10c4a541bfd9e9963e11b5801c1355d49518ce3c0`
   Evidence: jansu-storage/src/slate/storage.rs:1602, future-hostile/dead-language term `todo` appears
284. `high` `vibe` `jansu-storage/src/slate/storage.rs:1728`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:75650d7c511939855d23fb306f7dbd49e7be9dd6c4ac51326d588a65aa1f33cb`
   Evidence: jansu-storage/src/slate/storage.rs:1728, future-hostile/dead-language term `old` appears
285. `high` `vibe` `jansu-storage/src/slate/storage.rs:1807`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `old` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:71f4222dbd01e7dd51d776798d583ecb0a3757c7f4e6780ec992444928c996ee`
   Evidence: jansu-storage/src/slate/storage.rs:1807, future-hostile/dead-language term `old` appears
286. `high` `vibe` `jansu-storage/src/slate/storage.rs:1919`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `todo` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:15a3abd4d14b139c2575b3ffdf3f5df1f47f3cc5871a119f1c3dd2c29d35743c`
   Evidence: jansu-storage/src/slate/storage.rs:1919, future-hostile/dead-language term `todo` appears
287. `high` `data` `jansu-storage/src/sql/consumer_group_delete.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:b5cfd55c45fe01dc5c713716028d38c054198cd209129f96764b51d1167fc047`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from consumer_group
288. `high` `data` `jansu-storage/src/sql/consumer_group_detail_delete_by_cg.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:d45e618c6e1e38eb30c1868a28e9e6773c1951d3166825b40d1a3c213bb54e13`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from consumer_group_detail
289. `high` `data` `jansu-storage/src/sql/consumer_offset_delete_by_cg.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:03d29138964876f4a58cee06d10d1c6b37f83cca304d67cf4ddacb7d3a1d5814`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from consumer_offset
290. `high` `data` `jansu-storage/src/sql/consumer_offset_delete_by_topic.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:a6df48098e64565172f5291671db3c990a89f7c123d4e0b27563bcad96313c3c`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from consumer_offset
291. `high` `data` `jansu-storage/src/sql/consumer_offset_delete_expired.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:77a9666cc237e3cab3f23280df8487757cd77dc7dda80256225ddb372c8d13d6`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from consumer_offset
292. `high` `data` `jansu-storage/src/sql/header_delete_by_topic.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:e198e522cbef0ec41e3d12885415bc67ce616fd8e839943ccdf4215fad2acfd1`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from header
293. `high` `data` `jansu-storage/src/sql/pg_migration_phase10.sql:18`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `drop table`
   Reason: the migration can remove or rewrite data without local evidence of recovery
   Fix: split the change into a reviewed migration with rollback, backup, and row-count evidence
   Rerun: `just fast`
   Fingerprint: `sha256:a77bada0e7b350a772886973ca5223efd93fbf3a5404911ac3abdbf71262aa78`
   Evidence: detector=sql.migration.destructive-no-proof, proof-window=nearby-proof, snippet=topition int REFERENCES topition (id) ON DELETE CASCADE,
294. `high` `data` `jansu-storage/src/sql/policy_compact.sql:41`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:5e7f478668a60a4a36cbb7e8b35ddbb8209ab7a641095b66199f76746c838292`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from record
295. `high` `data` `jansu-storage/src/sql/policy_delete.sql:51`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:277052cba0dd24e943e72a470ff3eb8188d73577e24662da5a9282b4e422862b`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from record
296. `high` `data` `jansu-storage/src/sql/producer_detail_delete_by_topic.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:d98cdc7fdbf52744ad8b4b0839f12c016ce74e0b98015d3c371b8135627339c4`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from producer_detail
297. `high` `data` `jansu-storage/src/sql/record_delete_by_topic.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:569ece75279a2a431c947c24e1f1adf3913b7e4daf79a9f64cf09eee9afcdf14`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from record
298. `high` `data` `jansu-storage/src/sql/scram_credential_delete.sql:14`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:ff59f6329206b3cc255d831714ab4cfc1ef2ab8b24da57f6de4d447833cc0204`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from scram_credential
299. `high` `data` `jansu-storage/src/sql/topic_configuration_delete.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:3cd62a68482774f70deb5a135aae987d99913a1691c2ac7578daff5e1f83e42b`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from topic_configuration
300. `high` `data` `jansu-storage/src/sql/topic_configuration_delete_by_topic.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:7c4b9d1e45d087f200a9b208fac43740515419d23dc97c7dc658324afce1bd93`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from topic_configuration
301. `high` `data` `jansu-storage/src/sql/topic_delete_by.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:c59f81977050b41f65b316821ee8fc3977d94f41bbadae06ea3445f21b0e80ad`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from topic
302. `high` `data` `jansu-storage/src/sql/topition_delete_by_topic.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:bb4ddffee46d9e171a269b3c6c7de21fd13d11e00d856f75ccb9d8c8de6f12cd`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from topition
303. `high` `data` `jansu-storage/src/sql/txn_offset_commit_delete_by_txn.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:fe3adb6297868646de2d6898f00b95dbf1e584bf67d8d6a1376816eff5721667`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from txn_offset_commit
304. `high` `data` `jansu-storage/src/sql/txn_offset_commit_tp_delete_by_topic.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:4f7f4378ae7b0af7fe730ba74f37ab14915eb697aae772e591f1d868b2b4cbfe`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from txn_offset_commit_tp
305. `high` `data` `jansu-storage/src/sql/txn_offset_commit_tp_delete_by_txn.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:dd24affd16235ec6ef8e91c938827aca5e23a330c952c6b3c4d4d0e6e3080e2d`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from txn_offset_commit_tp
306. `high` `data` `jansu-storage/src/sql/txn_produce_offset_delete_by_topic.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:8d8181cd53f5838b45028b98618f06ba61a07c985bf57e7ae6d90389e9154799`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from txn_produce_offset
307. `high` `data` `jansu-storage/src/sql/txn_produce_offset_delete_by_txn.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:eaa2da354e511dfec1dab5543bdcbc6b0b2cb424549ce99963ac82466245f8d3`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from txn_produce_offset
308. `high` `data` `jansu-storage/src/sql/txn_topition_delete_by_topic.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:ad38f5fe2852ee74d5a06a5a3d30f79c1a2abe6b4683afd452dc73ba6387201b`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from txn_topition
309. `high` `data` `jansu-storage/src/sql/txn_topition_delete_by_txn.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:13df00a40f53eb882c753e15e76d8d9e297f5a63162877388c82d81d08302cba`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from txn_topition
310. `high` `data` `jansu-storage/src/sql/watermark_delete_by_topic.sql:16`
   Rule: `HLT-030-SQL-BAD-BEHAVIOR`
   Check: `HLT-030-SQL-BAD-BEHAVIOR:data` `hard` confidence `0.95`
   Route: TLR `Contracts/data`, lane `db`, owner `tools`
   Docs: `docs/testing.md`
   Matched term: `update/delete`
   Reason: the statement reaches a whole-table write path without a row filter
   Fix: add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Rerun: `just fast`
   Fingerprint: `sha256:a20c0d99c81dbd8083ce042983869b9ad72e4aff855a551289ffc6a0244e5d6a`
   Evidence: detector=sql.query.full-table-write, proof-window=where-clause, snippet=delete from watermark
311. `high` `vibe` `jansu-storage/tests/consumer_offsets.rs:202`
   Rule: `HLT-001-DEAD-MARKER`
   Check: `HLT-001-DEAD-MARKER:vibe` `hard` confidence `0.88`
   Route: TLR `Entropy`, lane `fast`, owner `tools`
   Docs: `docs/audit-rubric.md#future-hostile-language-rule`
   Reason: future-hostile/dead-language term `placeholder` appears in product/runtime code
   Fix: remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Rerun: `just fast`
   Fingerprint: `sha256:a913f6765e3b700cd926a03da58d661ca6fbef23d5e6c24511e447f7930a0f80`
   Evidence: jansu-storage/tests/consumer_offsets.rs:202, future-hostile/dead-language term `placeholder` appears

## Policy

- Policy file: `./agent/audit-policy.toml`
- Minimum score: `85`
- Fail on: `critical, high`

## Agent Fix Queue

1. `high` `HLT-022-AUTHZ-ISOLATION-GAP` `jansu-broker/src/broker.rs` - add owner/non-owner authorization tests or RLS evidence for the touched data boundary
   Route: `Business truth`/`db`
2. `high` `HLT-030-SQL-BAD-BEHAVIOR` `etc/initdb.d/010-schema.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
3. `high` `HLT-030-SQL-BAD-BEHAVIOR` `etc/initdb.d/011-offset-retention-patch.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
4. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/020-consumer-group.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
5. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/020-producer.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
6. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/020-scram-credential.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
7. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/020-topic.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
8. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/030-consumer-group-detail.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
9. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/030-producer-epoch.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
10. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/030-topic-configuration.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
11. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/030-topition.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
12. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/030-txn.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
13. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/030-virtual-topic.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
14. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/040-consumer-offset.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
15. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/040-header.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
16. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/040-leader-epoch-history.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
17. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/040-producer-detail.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
18. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/040-record.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
19. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/040-watermark.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
20. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/050-txn-offset-commit.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
21. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/050-txn-topition.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
22. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/060-txn-offset-commit-tp.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
23. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/ddl/060-txn-produce-offset.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
24. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/lite/policy_compact_delete.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
25. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/lite/policy_delete.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
26. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/consumer_group_delete.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
27. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/consumer_group_detail_delete_by_cg.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
28. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/consumer_offset_delete_by_cg.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
29. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/consumer_offset_delete_by_topic.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
30. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/consumer_offset_delete_expired.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
31. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/header_delete_by_topic.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
32. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/pg_migration_phase10.sql` - split the change into a reviewed migration with rollback, backup, and row-count evidence
   Route: `Contracts/data`/`db`
33. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/policy_compact.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
34. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/policy_delete.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
35. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/producer_detail_delete_by_topic.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
36. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/record_delete_by_topic.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
37. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/scram_credential_delete.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
38. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/topic_configuration_delete.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
39. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/topic_configuration_delete_by_topic.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
40. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/topic_delete_by.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
41. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/topition_delete_by_topic.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
42. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/txn_offset_commit_delete_by_txn.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
43. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/txn_offset_commit_tp_delete_by_topic.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
44. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/txn_offset_commit_tp_delete_by_txn.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
45. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/txn_produce_offset_delete_by_topic.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
46. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/txn_produce_offset_delete_by_txn.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
47. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/txn_topition_delete_by_topic.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
48. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/txn_topition_delete_by_txn.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
49. `high` `HLT-030-SQL-BAD-BEHAVIOR` `jansu-storage/src/sql/watermark_delete_by_topic.sql` - add a WHERE clause or prove the full-table rewrite with a local migration receipt
   Route: `Contracts/data`/`db`
50. `high` `HLT-004-UNMAPPED-PROOF` `agent/test-map.json` - add the narrowest stable prefix and runnable proof command to `agent/test-map.json`
   Route: `Verification`/`fast`
51. `medium` `HLT-018-PERF-CONCURRENCY-DRIFT` `Justfile` - add fast deterministic build/test targets, caches, and narrow proof lanes for agent iteration
   Route: `Verification`/`fast`
52. `medium` `HLT-026-COST-BUDGET-GAP` `docs/testing.md` - add explicit budgets, quotas, stop conditions, and kill-switch evidence for paid or unbounded operations
   Route: `Verification`/`release`
53. `high` `HLT-017-OPAQUE-OBSERVABILITY` `crates/domain` - define a typed exception surface with purpose, reason, common fixes, docs_url, and repair_hint so the next rerun is local
   Route: `Repair`/`observability`
54. `medium` `HLT-027-HUMAN-REVIEW-EVIDENCE-GAP` `agent/repo-score.json` - attach raw CI logs, review receipts, and replayable commands instead of accepting claims or summaries
   Route: `Repair`/`audit`
55. `medium` `HLT-017-OPAQUE-OBSERVABILITY` `docs/testing.md` - add structured errors, telemetry, and repair receipts that tell the next agent where to rerun proof
   Route: `Repair`/`observability`
56. `high` `HLT-003-OWNERLESS-PATH` `agent/owner-map.json` - add the narrowest stable prefix for this path to `agent/owner-map.json`
   Route: `Context/setup`/`fast`
57. `medium` `HLT-015-CONTEXT-SETUP-GAP` `AGENTS.md` - keep root guidance short and route durable detail through agent-readable manifests and docs
   Route: `Context/setup`/`fast`
58. `medium` `HLT-003-OWNERLESS-PATH` `agent/owner-map.json` - tighten owner/test maps and root routing until agents can localize ownership without inference
   Route: `Context/setup`/`fast`
59. `medium` `docs/` - add concise docs for architecture, boundaries, tests, generated zones, and audit rules; route them from root `AGENTS.md`
   Route: `Context/setup`/`audit`
60. `critical` `HLT-010-SECRET-SPRAWL` `agent/repo-score.json` - remove and rotate the credential, add local and CI secret scanning, and scan transcripts/artifacts/MCP config for related exposure
   Route: `Security, secrets, agency`/`security`
61. `high` `HLT-032-DOCKER-BAD-BEHAVIOR` `compose.yaml` - bind the port to localhost or keep it on an internal-only network
   Route: `Security, secrets, agency`/`security`
62. `high` `fuzz/fuzz_targets/generate_seeds.rs` - extract the duplicated behavior behind one named boundary and add focused tests before changing behavior
   Route: `Entropy`/`fast`
63. `high` `HLT-001-DEAD-MARKER` `jansu-auth/src/handshake.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
64. `high` `HLT-001-DEAD-MARKER` `jansu-auth/src/lib.rs` - collapse fallback chains into explicit typed states with bounded retry policy, telemetry, and documented repair guidance
   Route: `Entropy`/`fast`
65. `high` `HLT-001-DEAD-MARKER` `jansu-broker/src/coordinator/group/administrator/tests.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
66. `high` `HLT-001-DEAD-MARKER` `jansu-broker/tests/auth.rs` - replace placeholders with implemented behavior, typed unsupported-state errors, or a tracked exception record with docs
   Route: `Entropy`/`fast`
67. `high` `HLT-001-DEAD-MARKER` `jansu-broker/tests/cg_dynamic.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
68. `high` `HLT-001-DEAD-MARKER` `jansu-broker/tests/compatibility_contract.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
69. `high` `HLT-001-DEAD-MARKER` `jansu-broker/tests/differential_lab.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
70. `high` `HLT-001-DEAD-MARKER` `jansu-cli/src/cli/perf.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
71. `high` `HLT-029-RUST-BAD-BEHAVIOR` `jansu-cli/src/cli/user.rs` - construct the type with a valid initializer instead of zeroing it
   Route: `Security, secrets, agency`/`fast`
72. `high` `HLT-001-DEAD-MARKER` `jansu-client/src/lib.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
73. `high` `HLT-001-DEAD-MARKER` `jansu-model/src/lib.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
74. `high` `HLT-001-DEAD-MARKER` `jansu-sans-io/src/acl.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
75. `high` `HLT-001-DEAD-MARKER` `jansu-sans-io/src/lib.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
76. `high` `HLT-001-DEAD-MARKER` `jansu-sans-io/src/resource.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
77. `high` `HLT-001-DEAD-MARKER` `jansu-sans-io/src/ser.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
78. `high` `HLT-001-DEAD-MARKER` `jansu-schema/src/avro.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
79. `high` `HLT-001-DEAD-MARKER` `jansu-schema/src/avro/arrow.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
80. `high` `HLT-023-INPUT-BOUNDARY-GAP` `jansu-schema/src/avro/arrow.rs` - replace unsafe sinks with typed schemas, parameterized APIs, allowlists, or sandboxed execution plus negative tests
   Route: `Security, secrets, agency`/`security`
81. `high` `HLT-001-DEAD-MARKER` `jansu-schema/src/json.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
82. `high` `HLT-001-DEAD-MARKER` `jansu-schema/src/lake/delta.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
83. `high` `HLT-001-DEAD-MARKER` `jansu-schema/src/proto.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
84. `high` `HLT-001-DEAD-MARKER` `jansu-schema/src/sql.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
85. `high` `HLT-001-DEAD-MARKER` `jansu-storage/src/dynostore.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
86. `high` `HLT-001-DEAD-MARKER` `jansu-storage/src/limbo.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
87. `high` `HLT-001-DEAD-MARKER` `jansu-storage/src/lite.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
88. `high` `HLT-001-DEAD-MARKER` `jansu-storage/src/pg.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
89. `high` `HLT-001-DEAD-MARKER` `jansu-storage/src/service.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
90. `high` `HLT-001-DEAD-MARKER` `jansu-storage/src/slate/engine.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
91. `high` `HLT-001-DEAD-MARKER` `jansu-storage/src/slate/storage.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
92. `high` `HLT-001-DEAD-MARKER` `jansu-storage/tests/consumer_offsets.rs` - remove or rename the marker, implement the intended behavior, model a typed unsupported state, or move docs/generated/vendor/product-copy text into an allowlisted context
   Route: `Entropy`/`fast`
93. `medium` `HLT-001-DEAD-MARKER` `.` - split large or ambiguous authored code into smaller semantic modules with focused tests
   Route: `Entropy`/`fast`
94. `medium` `HLT-016-SUPPLY-CHAIN-DRIFT` `.github/workflows/jankurai.yml` - wire secret, dependency, provenance, and workflow scans into an operational CI lane
   Route: `Security, secrets, agency`/`security`
