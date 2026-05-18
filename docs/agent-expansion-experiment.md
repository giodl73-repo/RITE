# RITE Agent Expansion Experiment

## Goal

Use agents to expand RITE from a seed ritual/meaning schema into **50-100
candidate capability systems** while improving the rubric after each batch.

## Batch contract

Each agent batch proposes 10-20 candidate systems. Every candidate must include:

| Field | Requirement |
|---|---|
| Capability id | Stable `rite-capability:*` id. |
| Meaning pressure | Rite, taboo, sacred place, calendar, mourning, oath, legitimacy, pilgrimage, offering, or status change. |
| Consumer fit | At least one BANISH, LUCIA, CANON, PORTO, CERES, or MAXIM use. |
| Rubric delta | Which RITE rubric axis the capability improves. |
| Non-goal | What repo should own the adjacent mechanics or interpretation. |

## First 10 capability lanes

1. Mourning duties under crisis.
2. Sacred-place access constraints.
3. Taboo-breaking and legitimacy loss.
4. Oath-bound commons.
5. Ritual calendars and labor windows.
6. Pilgrimage route pressure.
7. Funeral attendance versus survival labor.
8. Offerings as costly public commitments.
9. Boundary processions and land claims.
10. Restitution vows after emergency desecration.

## Rubric improvement loop

After each batch, update [`QUALITY_RUBRIC.md`](..\QUALITY_RUBRIC.md) only when
the new candidates expose a repeated scoring gap. Do not add axes for one-off
ideas.
