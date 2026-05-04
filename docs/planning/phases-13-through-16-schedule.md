# Phases 13–16 — Schedule and audit mapping

Order follows [`MASTER_PLAN.md`](../../MASTER_PLAN.md) priority queue after Phase 12. Dependencies from MASTER_PLAN:

- **Phase 13** may proceed after Phase 06 storage contracts are stable; overlaps delete-records/retention with Phase 09 admin tests — coordinate file ownership.
- **Phase 14** may parallelize with 08/09/10 only when coordinator vs auth files are disjoint.
- **Phase 15** may overlap Phase 16 for documentation/harness-only work that does not change protocol routing.

| Phase | Tip | Primary `AUDIT` | Notes |
|-------|-----|-----------------|-------|
| 13 | [`tips/phases/13-retention-compaction-delete-records.md`](../../tips/phases/13-retention-compaction-delete-records.md) | `AUDIT-007` | Tombstones, compaction, storage certification |
| 14 | [`tips/phases/14-security-acls-quotas.md`](../../tips/phases/14-security-acls-quotas.md) | `AUDIT-008` | ACLs, quotas before secured compatibility claims |
| 15 | [`tips/phases/15-cluster-metadata-modern-kafka.md`](../../tips/phases/15-cluster-metadata-modern-kafka.md) | `AUDIT-009` | Controller, registration, Kafka 4.2 metadata |
| 16 | [`tips/phases/16-ecosystem-performance-ops-migration.md`](../../tips/phases/16-ecosystem-performance-ops-migration.md) | `AUDIT-010` | Java/librdkafka/franz-go/Sarama, perf, migration |

## Bookkeeping

Each phase start: canonical `phase-logs/NN-*.md.log`, `phase-logs/attempts/NN-slug/`, `phase-logs/index.json` verification row if commands change.

Cross-phase differential lab: [`tips/phases/04-differential-kafka-lab.md`](../../tips/phases/04-differential-kafka-lab.md) supplies harnesses; phase-owned APIs attach proof there when promoting ledger status.
