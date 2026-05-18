# RITE

Ritual, taboo, sacred-place, calendar, mourning, oath, legitimacy, and cultural
obligation knowledge for portfolio systems.

RITE is a Knowledge Systems repo. It does not own theology, game mechanics,
historical interpretation, or canonical identity. It owns the evidence and
validation layer that lets downstream systems reason about meaning under
pressure: why people accept costs, keep vows, mourn losses, avoid taboos, gather
at sacred places, and treat some actions as legitimate or forbidden.

## Product thesis

Communities do not survive only through food, routes, prices, and identity. They
also survive through obligations that feel binding: rites, mourning practices,
taboos, calendars, vows, festivals, sacred places, and legitimacy tests. RITE
provides portable records for those meaning systems without turning them into
generic religion flavor or downstream mechanics.

## First consumers

| Consumer | Use |
|---|---|
| BANISH | Add meaning-under-pressure scenarios where sacred obligations change survival choices. |
| LUCIA | Connect ritual practice to people-history, mourning, legitimacy, vows, and cultural continuity. |
| CANON | Preserve stable rite, taboo, sacred-place, calendar, and scenario ids across fixtures. |
| PORTO | Ground sacred places, pilgrimage routes, forbidden crossings, and ritual access constraints. |
| CERES | Track ritual cost, offering, festival, burial, and obligation pressure without reducing meaning to price. |
| MAXIM | Provide reference-grounded concepts for rite, taboo, oath, legitimacy, and sacred obligation. |

## Initial scope

1. Define product-neutral Rust primitives for rites, obligations, taboos, sacred
   places, calendar events, evidence references, and validation findings.
2. Provide a CLI that validates RITE fixture records.
3. Establish the first wave/pulse process and research skill.
4. Document boundaries with BANISH, LUCIA, CANON, PORTO, CERES, MAXIM, PROOF,
   CROP, PEBBLE, FLETCH, RLINE, SLICE, and ROLES.
5. Defer simulation/runtime extraction until at least two consumers prove the
   same product-neutral ritual/meaning primitives.

## Non-goals

- RITE does not prescribe real-world religious truth or theology.
- RITE does not replace LUCIA people-history, CANON identity, PORTO geography, or
  CERES economics.
- RITE does not write BANISH game rules or scenario prose.
- RITE does not own fetch/cache, context cropping, rendering, or portable pack
  distribution.
- RITE does not force downstream repos to link Rust crates before artifact
  contracts are stable.

## Crates

| Crate | Role |
|---|---|
| `rite-core` | Rite, obligation, taboo, calendar, sacred-place, evidence, and validation primitives. |
| `rite-cli` | Validation CLI for RITE fixture records. |

## First validation

```powershell
cargo fmt --check
cargo test
cargo run -p rite-cli -- validate fixtures\seed-rite.json
git grep -n "RITE" -- README.md PRODUCT_PLAN.md context\waves\PHASES.md
```
