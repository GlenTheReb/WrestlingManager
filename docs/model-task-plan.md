# Wrestling Manager implementation plan

This is the capability-level scope and completion map for the complete product direction discussed on
9 September 2026. It covers the existing match engine, Football Manager-style people depth,
TEW-scale company operations, structured creative tools, a living industry, text-led show
presentation and native custom content. The rows are capability briefs, not automatically
implementation-sized branches. Detailed outcomes are in [the capability roadmap](tickets/ROADMAP.md),
vertical delivery order is in [the playable milestones](milestones.md), and every coding slice must
follow [the ticket execution guide](tickets/EXECUTION-GUIDE.md).

This plan is not permission to create branches, implement, delegate, commit or publish. GPT-5.6
Sol at High is the default ceiling for complex implementation. Astra may be proposed only after
a concrete architectural or debugging blocker survives focused Sol investigation and still
requires the owner's explicit approval. Terra fits bounded UI/data work and Luna fits mechanical
documentation. These recommendations are engineering judgments, not measured WM benchmarks.

## Product doctrine

- Simulate causes, not decorative scores. Every important rating, alert, review and consequence
  must link back to the people, plans, audience, business rules or events that produced it.
- Keep **real life**, **kayfabe** and **presentation** separate. Personal relationships and
  contracts are real-world state; characters, turns and storylines are kayfabe; dialogue,
  lighting and commentary present either without rewriting them.
- The office is an actionable command centre: assistant alerts, inbox, diary, quick actions and
  deep links into the exact worker, company, contract, event or story that needs attention.
- Match and angle presentation is text-led and simulation-driven. Smart arena lighting, effects,
  scene cards, crowd state and commentary are planned; theme songs, titantrons and full wrestler
  animation are not part of the current product direction.
- Native custom databases are a first-class requirement. Portrait packs are optional; sound and
  video packs are later optional presentation layers and can never affect simulation correctness.
- CPU promotions obey compatible rules and produce inspectable history. The player can search for
  any company or show and watch a stored condensed or detailed presentation where data exists.
- Each ticket has one reviewable outcome, preserves deterministic resume/save guarantees, updates
  the SDD and provides tests appropriate to its risk before it can be called complete.

## Capability completion map

This table is not a 42-branch serial queue. “Depends on” describes what the complete capability must
ultimately integrate with. Foundation slices can begin earlier where their capability packet and
[milestone plan](milestones.md) say so. The milestone promise determines execution priority; the
capability packet is decomposed into small execution tickets before code changes.

WM-001 is implemented, verified and merged through pull request #1 at `ba706ff`.
See [WM-001 acceptance criteria and handoff](tickets/WM-001.md) and the completed
[WM-020 execution record](tickets/WM-020.1.md). WM-022 and WM-023 are merged. PD-102 is accepted, so
WM-021.1 implements the current-data Talent Search foundation and is undergoing review; see its
[canonical search rules](search-and-discovery-rules.md). Later-domain search registration remains
with the systems that own those facts. Other tickets remain planned.

| Reference order | Ticket | Outcome                                                                                           | Complete capability depends on                         | Route                 | Estimate   |
| --------------: | ------ | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------ | --------------------- | ---------- |
|               1 | WM-001 | Match-local participants, sides and result semantics — complete                                   | —                                                      | Completed             | —          |
|               2 | WM-002 | Tag/trios legal-worker, tag and team-control runtime                                              | WM-001                                                 | Sol/High              | 2–3 wk     |
|               3 | WM-003 | Multi-person, elimination and timed-entry runtime                                                 | WM-002                                                 | Sol/High              | 2–4 wk     |
|               4 | WM-004 | Expanded match-plan persistence and booking UI                                                    | WM-002–003                                             | Sol/High              | 2–3 wk     |
|               5 | WM-005 | Composable stipulation, location, weapon, escape and scoring rules                                | WM-003–004                                             | Sol/High              | 4–7 wk     |
|               6 | WM-006 | Team/rule-aware road-agent planning and live adaptation                                           | WM-005                                                 | Sol/High              | 2–4 wk     |
|               7 | WM-007 | Move chains, counters, limb work and explicit finish execution                                    | WM-006                                                 | Sol/High              | 3–5 wk     |
|               8 | WM-008 | Crowd/show flow, safety, fatigue and career-consequence balance                                   | WM-007                                                 | Sol/High              | 3–5 wk     |
|               9 | WM-009 | Persistent drafts, advanced timeline, card undo and templates                                     | WM-004, WM-007                                         | Sol/High              | 2–4 wk     |
|              10 | WM-010 | Personal movesets, signatures, finishers and learning content                                     | WM-007                                                 | Terra/Medium          | 1–2 wk     |
|              11 | WM-020 | Six 0–100 stat groups, Disciplines, Archetypes and current-style Overall — implemented and pushed | WM-001                                                 | Completed on Sol/High | —          |
|              12 | WM-022 | Personality, languages, hobbies, biography and trait evidence — implemented and pushed            | WM-001, PD-103                                         | Complete              | —          |
|              13 | WM-023 | Directional relationships, memories and player interactions — merged through PR #4                | WM-022                                                 | Completed on Sol/High | —          |
|              14 | WM-021 | Talent Search foundation — WM-021.1 implemented; final ring-identity integration follows WM-025   | WM-020, WM-022–023; WM-025 for full exit               | Sol/High              | foundation |
|              15 | WM-025 | Person/Character separation, ring identities, gimmicks, masks and alignment                       | WM-022–023, PD-106                                     | Sol/High              | 2–3 wk     |
|              16 | WM-024 | FM-style wrestler profile and career hub                                                          | WM-020–023, WM-025                                     | Sol/High              | 2–4 wk     |
|              17 | WM-026 | Permanent teams, stables, managers, authority figures and chemistry                               | WM-025                                                 | Sol/High              | 2–4 wk     |
|              18 | WM-027 | Championships, divisions, rankings and tournament history                                         | WM-026                                                 | Sol/High              | 3–5 wk     |
|              19 | WM-028 | Storylines, creative memory, future plans and pre-booking                                         | WM-025–027                                             | Sol/High              | 3–5 wk     |
|              20 | WM-029 | Assisted/manual angle creator with timed dialogue scene beats                                     | WM-007, WM-028                                         | Sol/High              | 4–7 wk     |
|              21 | WM-011 | Company identity, product, culture, brands, size and geography foundation                         | WM-020, PD-105                                         | Sol/High              | 3–5 wk     |
|              22 | WM-030 | Ownership, goals, hierarchy, booking team, inner circle and figurehead                            | WM-011                                                 | Sol/High              | 2–4 wk     |
|              23 | WM-042 | Auditable company finance, budgets, forecasts, debt and investment                                | WM-011, WM-030                                         | Sol/High              | 3–5 wk     |
|              24 | WM-031 | Contracts, negotiations, promises, absences, morale and wellness                                  | WM-023, WM-030, WM-042                                 | Sol/High              | 4–6 wk     |
|              25 | WM-032 | Staff, facilities, scouting, training, mentoring and academy pipeline                             | WM-020, WM-022, WM-011, WM-030–031, WM-042             | Sol/High              | 4–7 wk     |
|              26 | WM-033 | Calendar, events, venues, prices, tours, house/developmental shows                                | WM-011, WM-031–032, WM-042                             | Sol/High              | 4–6 wk     |
|              27 | WM-034 | TV/streaming/PPV, networks, coverage, commentary and production                                   | WM-011, WM-033, WM-042                                 | Sol/High              | 4–7 wk     |
|              28 | WM-035 | Company office, assistant, actionable inbox, diary and quick actions                              | WM-030–034, WM-042                                     | Terra/High            | 2–4 wk     |
|              29 | WM-036 | Journalists, reviews, press conferences, social media and random events                           | WM-023, WM-028, WM-011, WM-030–035                     | Sol/High              | 4–7 wk     |
|              30 | WM-012 | Company/world relationship and historical-event domain foundation                                 | WM-011, WM-031–034, WM-042                             | Sol/High              | 2–4 wk     |
|              31 | WM-037 | Alliances, battles, trades, child/sister companies and takeovers                                  | WM-012, WM-030–034, WM-042                             | Sol/High              | 4–7 wk     |
|              32 | WM-038 | Rival-company AI, evolving industry and CPU show simulation                                       | WM-028, WM-033–037                                     | Sol/High              | 5–8 wk     |
|              33 | WM-039 | Global search and condensed/full CPU-show viewing                                                 | WM-034, WM-036–038                                     | Sol/High              | 3–5 wk     |
|              34 | WM-040 | Show/company histories, alumni, legends, awards, rankings and Hall of Fame                        | WM-027, WM-036–039                                     | Sol/High              | 3–5 wk     |
|              35 | WM-013 | Renderer-neutral match/angle/show presentation event contract                                     | WM-007, WM-029, WM-034, WM-036, WM-038                 | Sol/High              | 2–3 wk     |
|              36 | WM-014 | Text-led arena viewer with lighting, effects, scene cards and crowd state                         | WM-013                                                 | Sol/High              | 3–5 wk     |
|              37 | WM-015 | Save recovery, migration/compatibility policy and long-save hardening                             | Continuous; final gate after WM-040                    | Sol/High              | 3–5 wk     |
|              38 | WM-016 | Cohesive WM visual identity and complex-screen design system                                      | WM-014, WM-035                                         | Sol/High              | 3–5 wk     |
|              39 | WM-017 | Full management navigation, context menus and accessible workflows                                | WM-016                                                 | Terra/High            | 3–5 wk     |
|              40 | WM-018 | Regression, property, balance, performance, accessibility and soak suite                          | Continuous; final gate after WM-015–017                | Terra/High            | 4–7 wk     |
|              41 | WM-041 | Native database editor, scenario import/export and optional asset packs                           | Stable IDs continuously; public tools after WM-015–018 | Sol/High              | 4–7 wk     |
|              42 | WM-019 | Installer, licensing, documentation and release-candidate cleanup                                 | WM-041                                                 | Terra/Medium          | 2–3 wk     |

## Capability and execution-ticket rules

WM-002–042 are destination-level capability briefs. Most are intentionally larger than one safe
branch. Before implementation, decompose the selected capability into a two-to-five-day execution
ticket using [`TICKET-TEMPLATE.md`](tickets/TICKET-TEMPLATE.md). Each execution ticket has one
reviewable purpose and explicit acceptance criteria. If it reveals two independent outcomes, split
them rather than extending scope. Every implementation branch starts from the current integration
branch after required dependencies have merged; do not stack unrelated work on an old branch.

Use the `feature/` branch prefix and Conventional Commits. Put the capability and execution-ticket
identifiers in the
commit body and pull-request description until a GitHub issue tracker is connected. The ticket
record is the source of truth for scope; branch creation still requires current owner permission.

Do not create all branches upfront. WM-001 started directly from the published
`feature/initial-skeleton` baseline (`3d74c7a`) at the owner's request. Before integrating
finished tickets, establish a stable integration branch (`main`) from that baseline and merge
each finished ticket through review. Creating or publishing that branch is not part of WM-001.

Before implementation, read the selected `docs/tickets/WM-###.md` context packet and create its
execution ticket with exact acceptance criteria, inspected file/symbol ownership, SDD heading/line
references and verification commands. Product rules must be accepted in
[`docs/product-decisions.md`](product-decisions.md), not guessed inside code. Existing detailed
records for WM-001 and WM-020–023 remain authoritative where they are more specific.

## Schedule estimate

These are capability-level engineering-time ranges for one developer working sequentially. They are planning
ranges rather than delivery promises; asset production, playtesting, review turnaround and
scope discovered during implementation can extend them.

| Capability area                           | Tickets                        |    Estimated engineering time |
| ----------------------------------------- | ------------------------------ | ----------------------------: |
| Match and booking depth                   | WM-002–010                     |                   21–37 weeks |
| People, identity and profile              | WM-020–024                     |                    9–17 weeks |
| Kayfabe and creative production           | WM-025–029                     |                   14–24 weeks |
| Company, media and living world           | WM-011–012, WM-030–040, WM-042 |                   47–80 weeks |
| Presentation contract and viewer          | WM-013–014                     |                     5–8 weeks |
| Hardening, interface, modding and release | WM-015–019, WM-041             |                   19–32 weeks |
| **Total**                                 | **WM-002–042**                 | **115–198 engineering weeks** |

The estimate assumes one developer working sequentially and excludes final content authoring,
licensed assets, community testing, marketing and review turnaround. The upper bound is more
credible than a short promise: this plan describes a multi-year simulation product. Each capability area
must contribute to playable milestones; [the milestone plan](milestones.md), not these horizontal
estimate buckets, governs delivery and integration.
