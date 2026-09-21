# Photo Publisher — Phase 1.1

Phase 1.1 establishes and hardens the public contract layer for the Photo Publisher project.

## Contents

- JSON Schema Draft 2020-12 contracts
- Project contract v1
- Gallery manifest contract v1
- Rust contract validator
- Valid and invalid fixtures
- Automated tests
- GitHub Actions CI

## Validate locally

Requirements:

- Rust toolchain (stable)

Run:

```bash
cargo test --workspace --all-targets
```

Validate an individual document:

```bash
cargo run -p photo-publisher-contract-validator -- schemas/project.schema.json fixtures/valid/project.valid.json
```

## Contract rules

The schemas are public interfaces. The project contract contains no credentials or secrets. Gallery photo IDs must be unique, and gallery sequence numbers are one-based.

Breaking changes after contract stabilization require a new major contract version.
