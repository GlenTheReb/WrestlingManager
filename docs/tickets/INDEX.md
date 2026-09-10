# Wrestling Manager capability index

Use [the execution guide](EXECUTION-GUIDE.md) before implementing any capability. The milestone plan,
not numeric order, decides which thin slice creates the next playable outcome. Every capability file
records destination context; a separate execution ticket is still required before code changes.

## Match and booking

- [WM-001 — Match-rule core](WM-001.md) — complete
- [WM-002 — Tag and trios runtime](WM-002.md)
- [WM-003 — Multi-person, elimination and timed entry](WM-003.md)
- [WM-004 — Expanded match-plan persistence and booking UI](WM-004.md)
- [WM-005 — Composable stipulation rules](WM-005.md)
- [WM-006 — Rule-aware road-agent planning](WM-006.md)
- [WM-007 — Move chains and finish execution](WM-007.md)
- [WM-008 — Crowd, safety and consequence balance](WM-008.md)
- [WM-009 — Advanced booking workflow](WM-009.md)
- [WM-010 — Moveset and learning content](WM-010.md)

## Company/world foundations and product delivery

- [WM-011 — Company identity and product foundation](WM-011.md)
- [WM-012 — Company/world relationship foundation](WM-012.md)
- [WM-013 — Presentation event contract](WM-013.md)
- [WM-014 — Text-led arena and show viewer](WM-014.md)
- [WM-015 — Save and compatibility hardening](WM-015.md)
- [WM-016 — Visual identity and complex-screen design](WM-016.md)
- [WM-017 — Navigation and accessible workflows](WM-017.md)
- [WM-018 — Regression, balance, performance and soak](WM-018.md)
- [WM-019 — Release candidate and project cleanup](WM-019.md)

## People and creative

- [WM-020 — Wrestler stat hierarchy](WM-020.md) — implemented and verified; uncommitted
  - [WM-020.1 — Canonical wrestler-rating system implementation](WM-020.1.md) — execution record
- [WM-021 — Roster discovery and filtering](WM-021.md)
- [WM-022 — Wrestler identity and personality](WM-022.md)
- [WM-023 — Wrestler relationships and interactions](WM-023.md)
- [WM-024 — Wrestler profile and career hub](WM-024.md)
- [WM-025 — Real-life and kayfabe separation](WM-025.md)
- [WM-026 — Teams, stables, managers and authority](WM-026.md)
- [WM-027 — Championships, divisions, rankings and tournaments](WM-027.md)
- [WM-028 — Storylines, creative planner and pre-booking](WM-028.md)
- [WM-029 — Assisted/manual angle creator](WM-029.md)

## Company, media and living world

- [WM-030 — Ownership, goals and company hierarchy](WM-030.md)
- [WM-031 — Contracts, promises and talent operations](WM-031.md)
- [WM-032 — Staff, facilities, scouting and development](WM-032.md)
- [WM-033 — Calendar, events and live operations](WM-033.md)
- [WM-034 — Broadcasting, sponsors and production](WM-034.md)
- [WM-035 — Company office and actionable inbox](WM-035.md)
- [WM-036 — Media, press, social and random events](WM-036.md)
- [WM-037 — Alliances, company groups and takeovers](WM-037.md)
- [WM-038 — Rival-company AI and CPU show simulation](WM-038.md)
- [WM-039 — Global search and CPU-show viewer](WM-039.md)
- [WM-040 — Legacy, records and honours](WM-040.md)
- [WM-041 — Native custom content and optional asset packs](WM-041.md)
- [WM-042 — Company finance, budgets and investment](WM-042.md)

## Context maintenance rule

When a capability changes, update its packet, affected accepted/open product decisions, roadmap card
and milestone relationship in the same documentation change. When code changes, also update the SDD
and execution-ticket handoff. Do not duplicate canonical schema details across all capability files;
link to the owning ticket and record only the dependency contract.
