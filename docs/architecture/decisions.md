# Architecture Decisions (ADR)

## ADR-001: Offline Vocabulary Snapshots
We pin `schemaorg-current-https.jsonld` locally to guarantee deterministic, reproducible builds without network dependencies at compile time or runtime.

## ADR-002: Dynamic vs Typed Model Foundation
`schemaorg-core` provides `DynamicEntity` which handles arbitrary `@type` and extra properties, while `schemaorg-vocabulary` provides generated Rust structs for standard Schema.org types.

## ADR-003: Conservative Hydration Engine
Hydration fills missing `@type` fields on nested objects only when the property's Schema.org `rangeIncludes` contains a single unambiguous entity type, avoiding ungrounded assumptions.
