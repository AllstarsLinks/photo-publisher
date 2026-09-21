# Photo Publisher — Phase 1

Phase 1 establishes the public contract layer for the Photo Publisher project.

## Scope

- JSON Schema Draft 2020-12 contracts
- Project contract v1
- Gallery manifest contract v1
- Rust validator
- Valid/invalid fixtures
- Automated tests

## Explicitly out of scope

No GUI, GitHub, Vercel, R2/S3, Lightroom integration, AI adapter, or publishing engine is implemented in this phase.

## Validate locally

Requirements:

- Rust toolchain (stable)

Run:

```bash
cargo test
```

Validate an individual document:

```bash
cargo run -p photo-publisher-contract-validator -- schemas/project.schema.json fixtures/valid/project.valid.json
```

## Contract rule

The schemas are public interfaces. Breaking changes require a new major schema version. Keep provider credentials and secrets out of `project.json`.
