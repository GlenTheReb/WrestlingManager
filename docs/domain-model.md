# Initial domain model

## Implemented boundary

The Rust domain crate defines SaveId, Seed, a validated new-game specification and the
compact IPC transfer types. Database access is not a domain responsibility. The initial
save contains one promotion; it is not yet the generated world described in the vision.

| Value/entity   | Meaning and invariant                                                                                                                                                                                                                                            |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| SaveId         | Local filename-safe identity, 1–48 lower-case ASCII letters/digits/hyphens; starts with a letter/digit. Names cannot escape the saves directory.                                                                                                                 |
| Seed           | Canonical decimal text representing 0 through 18,446,744,073,709,551,615. No whitespace, signs or leading zeroes except zero itself.                                                                                                                             |
| Promotion      | Stable id `uwf`, Ultimate Wrestling Federation, initial region United Kingdom, founded 2026-01-01. Names are display fields, not foreign keys.                                                                                                                   |
| Money          | Integer pence in GBP, avoiding floating-point rounding. Initial cash is 25,000,000 pence; this skeleton constrains it to a non-negative JavaScript-safe integer. Future liabilities require an explicit ledger design, not casually relaxing unrelated balances. |
| Save metadata  | Seed, engine version, schema version and file identity. Unsupported formats cannot be silently interpreted as current saves.                                                                                                                                     |
| Creation event | Append-only historical fact committed with the new promotion. Describes completed creation, not a request that might fail.                                                                                                                                       |

JavaScript numbers cannot exactly represent all 64-bit integers. Keep seeds as strings
across the interface boundary; parse and validate in Rust. Promotion cash can be a number
only because its allowed range is explicitly bounded.

## Initial aggregate and commands

Creating a career is an aggregate transaction: metadata, UWF and its initial event either
all become visible or none do. A read returns a promotion overview, never the entire
database. Load must find an existing save and validate its identity and versions before
returning state. The save library has bounded results; large in-world lists will use
explicit pagination rather than this small career list API.

## Planned aggregates

These are design boundaries, not implemented entities:

- **Person and careers:** identity, biography, background, wrestling identity and staff
  qualifications. Membership and contracts are explicit relationships. Age is derived from
  birth date. Ability differs from fatigue, morale, momentum and scouting knowledge.
- **Employment:** contract/negotiation, dates, guarantees, fees, rights and promises;
  availability constraints apply to player and rivals alike.
- **Creative programme:** storyline stakes, character goals, established facts, audience
  knowledge, unresolved questions and planned/completed beats. Heat alone is insufficient.
- **Show:** schedule, venue, broadcast window and ordered segments. Match/angle plans are
  intentions; results and emitted events are immutable performance history.
- **Match plan:** legal participants/roles, locked outcome, modular finish/stipulations,
  purpose, duration, protection, freedom and medical restrictions. The booked winner must
  be a legal participant; non-victory outcomes do not invent a winner.
- **Championship:** eligibility, title reigns and lineage; changes reference completed
  results and never silently overwrite a previous reign.
- **Audience:** representative regional cohorts with separate awareness, affinity,
  trust, excitement and fatigue, plus remembered expectations and repeated material.
- **People management:** relationships, cliques, injuries, medical assessments, training,
  grievances and promises. All incidents have causal context.
- **Business:** venues/markets, media and sponsor agreements, budgets and financial ledger.
- **Information/history:** scouting evidence, news/social facts, career records, awards,
  results and event history with stable source references.

## Attribute semantics to preserve

Physical attributes describe capability/endurance; in-ring attributes describe executing,
cooperating and communicating a performance; entertainment attributes describe character
and audience work; professional attributes describe judgement and behaviour. Use 1–20
with scouting ranges where appropriate. Define each field's effects when its consuming
system arrives; do not populate an unused generic attribute bag. Correlate generated
attributes with coherent backgrounds and opportunities.

## Planned invariants and tests

Assert legal contracts, references and match outcomes; locked results independent of
rendering; atomic consequences; chronological histories; bounded conditions; exact ledger
arithmetic; no duplicate application of a completed show; deterministic ordered output.
Property tests exercise broad input spaces, while scenario tests demonstrate concrete
behaviour. A golden seed must check a stable expected digest, not just compare two runs.
