# Phase 09 — AdminClient operation matrix (planning)

This matrix tracks **Kafka AdminClient** and **CLI** operations against Jansu routes, ledger rows, and proof status. Update cells as services and tests land; do not claim compatibility without `docs/compatibility/kafka-4.2-ledger.json` proof.

| Operation area | Primary APIs (api_key) | Service modules | Broker tests / CLI | Ledger / proof target |
|----------------|----------------------|-----------------|---------------------|------------------------|
| Create / delete topic | CreateTopics (19), DeleteTopics (20) | `create_topics`, `delete_topics` | `jansu-broker/tests/topic.rs`, `justfile` kafka-topics | Row per API, storage tiers |
| Metadata | Metadata (3) | `metadata` | `metadata.rs` | Already under contract |
| Describe / alter configs | DescribeConfigs (32), IncrementalAlterConfigs (75), legacy AlterConfigs if required | `describe_configs`, `incremental_alter_configs` | `describe_configs.rs` | Error paths for unknown config |
| Describe topic partitions | DescribeTopicPartitionsRequest (**75**) | `describe_topic_partitions` | `topic.rs`, metadata flows | Topic ID consistency |
| Delete records | DeleteRecords (21) | `delete_records` | `policy_compact_delete.rs` | Overlap Phase 13 |
| List reassignments | ListPartitionReassignments (46) | `list_partition_reassignments` | Low coverage — add CLI/negative tests | Safe-error vs supported |
| Create partitions | CreatePartitions (37) | (if routed) | Add when implemented | Ledger gated |
| Topic ID | Cross-cutting Metadata, Fetch, ListOffsets, DeleteTopics | All above + Phase 08 | Integration tests ensuring ID stable | Phase 08 + 09 joint |

## Next implementation passes

1. Fill **empty / gray** ledger cells for each admin API with `unit_tests` / `client_tests` pointers.
2. Add **negative** matrix row: invalid RF, unknown config, validation-only create, unauthorized (post Phase 14).
3. Wire **just** recipes mirroring CI for `kafka-configs`, `kafka-delete-records`, reassign smoke.

See [`tips/phases/09-topic-config-admin.md`](../../tips/phases/09-topic-config-admin.md) for acceptance gate and “Do not do” rules.
