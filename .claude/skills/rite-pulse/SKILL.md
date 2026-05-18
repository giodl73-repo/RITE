# RITE Pulse Skill

Use this skill for individual RITE implementation pulses.

## Pulse requirements

- State the meaning object being changed: rite, obligation, taboo, sacred place,
  calendar event, or evidence.
- Add or update a fixture when schema behavior changes.
- Run:

```powershell
cargo fmt --check
cargo test
cargo run -p rite-cli -- validate fixtures\seed-rite.json
```
