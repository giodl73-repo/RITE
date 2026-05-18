# Pulse 01 - Repo foundation and intake

## Intent

Create RITE as a Knowledge Systems repo with a minimal validation core and seed
fixture for ritual and meaning-under-pressure records.

## Work

- Add README and product plan.
- Add Rust workspace with `rite-core` and `rite-cli`.
- Add `fixtures\seed-rite.json`.
- Add repo-local wave, pulse, and research skills.
- Update TRACKER dependency intake and submodule records.

## Validation

```powershell
cargo fmt --check
cargo test
cargo run -p rite-cli -- validate fixtures\seed-rite.json
git grep -n "RITE" -- README.md PRODUCT_PLAN.md context\waves\PHASES.md
```
