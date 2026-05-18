# Pulse 05: FLETCH registry seed

## Goal

Publish the first RITE-owned FLETCH registry for stable ritual/meaning fixtures,
consumer probes, rubric, and capability-map assets.

## Change

- Added `.fletch\registries\rite-foundation-assets.json`.
- Registered the seed fixture, BANISH nature-rite probe, quality rubric, and
  capability expansion document as repo-owned local file assets.

## Validation

```powershell
cargo fmt --check
cargo test
cargo run -p rite-cli -- validate fixtures\seed-rite.json
fletch registry validate --file .fletch\registries\rite-foundation-assets.json
```
