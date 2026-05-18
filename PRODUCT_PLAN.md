# RITE Product Plan

## One-line product

RITE is the ritual and meaning-under-pressure knowledge system for portfolio
worlds.

## Problem

Survival, history, and economy systems often explain action through material
need alone. That misses the obligations that make people accept costs: mourning
rites, taboos, sacred calendars, vows, legitimacy tests, festivals, funerals,
pilgrimage, and places that cannot be treated as interchangeable terrain.

## Product promise

RITE turns cultural obligation into reviewable evidence artifacts. It lets a repo
say what rite exists, what obligation it imposes, which place or calendar moment
it binds, what taboo it creates, and which source or fixture supports the claim.

## Core objects

| Object | Meaning |
|---|---|
| Rite record | Stable record for a ritual practice, vow, funeral, festival, oath, pilgrimage, or legitimacy ceremony. |
| Obligation | Required, forbidden, costly, recurring, or status-changing action tied to a rite. |
| Taboo | A boundary that makes an action, place, material, time, or relationship forbidden or dangerous. |
| Sacred place | A site, route, threshold, burial ground, shrine, grove, river crossing, or civic place with ritual meaning. |
| Calendar event | Seasonal, lunar, annual, crisis, mourning, harvest, or succession timing that activates a rite. |
| Evidence reference | Source path, source id, and optional anchor supporting a record. |

## First wave

**Wave:** RITE Foundation

Goal: establish the repo, Rust core, CLI validator, first seed fixture, and
dependency-chain placement without prematurely turning ritual into downstream
mechanics or generic religion flavor.

Pulses:

1. **Repo foundation and intake** - scaffold docs, Rust workspace, skills, seed
   fixture, and TRACKER integration.
2. **Rite and obligation schema** - harden rite, obligation, and evidence rules.
3. **Taboo, place, and calendar schema** - harden sacred-place, taboo, calendar,
   and legitimacy categories.
4. **Consumer probes** - test one fixture each for BANISH, LUCIA, CANON, PORTO,
   CERES, and MAXIM.
5. **Publisher contract** - define CROP/PEBBLE/FLETCH/PROOF output boundaries.

### Foundation acceptance checks

| Pulse | Acceptance check |
|---:|---|
| 1 | Rust workspace builds, seed fixture validates, and TRACKER records RITE as a Knowledge Systems submodule. |
| 2 | Rite and obligation records distinguish sacred cost from ordinary preference or economics. |
| 3 | Taboo, place, and calendar records explain when meaning changes access, legitimacy, mourning, or social acceptance. |
| 4 | Consumer probes do not move BANISH mechanics, LUCIA interpretation, CANON identity, PORTO geography, CERES economics, or MAXIM references into RITE. |
| 5 | Publisher contract names generated artifacts and keeps fetch/cache/crop/render ownership outside RITE. |

## Dependency placement

RITE is a Knowledge Systems repo and source-corpus candidate. It may eventually
provide a runtime crate, but the first wave treats generated artifacts and
validation as safer than forcing downstream runtime linkage.

| System | Initial status | Reason |
|---|---|---|
| PROOF | Planned | Validate and render rite docs and generated status reports. |
| CROP | Planned | Index rite documents and crop ritual/meaning neighborhoods. |
| PEBBLE | Planned | Package portable rite bundles for AI/context transfer. |
| FLETCH | Planned | Publish fetchable rite packs and registries downstream. |
| RLINE | Deferred | Extract shared calendar/context primitives only after repeated consumer need is proven. |
| SLICE | Planned later | Query rite documents once selectors stabilize. |
| ROLES | Planned | Add ritual/source-custody/downstream-readiness review panels after schema stabilizes. |

## Non-goals

- No real-world theological authority or religious prescription.
- No BANISH rules ownership, LUCIA interpretation policy, CANON identity
  ownership, PORTO map ownership, or CERES price/cost ownership.
- No fetch/cache, graph-cut selection, portable pack, or rendering ownership.
- No shared RLINE extraction until at least two downstream repos need the same
  primitive.

## Validation commands

```powershell
cargo fmt --check
cargo test
cargo run -p rite-cli -- validate fixtures\seed-rite.json
git grep -n "RITE" -- README.md PRODUCT_PLAN.md context\waves\PHASES.md
```
