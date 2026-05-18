# RITE Quality Rubric

Score each promoted RITE artifact 0-4 on each axis. A fixture is ready for
downstream prototype use only when every axis is at least 3.

| Axis | 0 | 2 | 4 |
|---|---|---|---|
| Meaning pressure | Ritual is flavor text. | A rite or taboo is named. | Sacred obligation changes what choices are legitimate, forbidden, costly, or trusted. |
| Obligation clarity | No obligation is visible. | A duty or ban is named. | Required action, forbidden action, offering, mourning duty, access rule, or status change is explicit. |
| Place/calendar grounding | Meaning floats free of time/place. | A place or timing is named. | Sacred place, threshold, season, crisis, mourning period, or calendar event activates the pressure. |
| Downstream boundary | RITE owns everything. | Downstream users are named. | RITE keeps ritual evidence separate from BANISH rules, LUCIA interpretation, CANON identity, PORTO maps, and CERES economics. |
| Evidence readiness | No source or fixture support. | Seed evidence exists. | Evidence refs, ids, and fixture records can be validated and packed downstream. |

## BANISH nature-rite gamepack gate

The BANISH nature-rite pack should score **3+** on RITE's meaning/obligation axis
before a game moves from scenario pack to prototype planning. The gate is passed
when ritual is not flavor text: it must change what choices are legitimate,
forbidden, costly, trusted, or mourned.

Validation:

```powershell
cargo run -p rite-cli -- validate fixtures\consumers\banish-nature-rite-gamepack.json
```
