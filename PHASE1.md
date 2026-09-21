# Phase 1 — Contract Layer

## Definition of done

- [x] Project schema v1 present
- [x] Gallery schema v1 present
- [x] Draft 2020-12 validation
- [x] Valid fixtures
- [x] Invalid fixtures
- [x] Automated Rust tests
- [x] Minimal developer documentation

## Test matrix

| Case | Expected |
|---|---|
| Valid project | pass |
| Valid gallery | pass |
| Invalid project ID | fail |
| Gallery preview width = 0 | fail |
| Unexpected gallery property | fail |

## Next phase

Phase 2 should implement the provider-agnostic Publisher Core only after the contract layer is reviewed and accepted.
