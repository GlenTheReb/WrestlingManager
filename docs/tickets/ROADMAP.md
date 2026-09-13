# Wrestling Manager capability roadmap

Status: planning baseline. WM-001 is complete; all other entries are capability briefs and require
explicit owner permission before branch creation or implementation. They must be decomposed through
[the execution guide](EXECUTION-GUIDE.md); playable order is governed by
[the milestone plan](../milestones.md), and every packet is linked from the
[capability index](INDEX.md). Priority and estimates are maintained in
[`docs/model-task-plan.md`](../model-task-plan.md), while unresolved behavior belongs in
[`docs/product-decisions.md`](../product-decisions.md).

## Definition of done for every capability slice

- The selected capability has a dedicated `WM-###.md` context packet. Its bounded execution ticket
  has exact scope, inspected files/symbols, current SDD heading/line references, intentionality
  answers and explicit acceptance checks before code changes begin.
- Domain rules have one canonical source; generated TypeScript contracts are regenerated rather
  than edited; invalid data fails without partial save mutation.
- Save/schema/engine changes are versioned, backed up and tested against existing and paused
  careers. Deterministic match/world behavior retains replay/resume fingerprints where applicable.
- UI work covers keyboard use, loading/error/empty states and normal/compact desktop layouts.
- Targeted tests, formatting, lint, type checks and applicable native smoke pass. The SDD records
  implemented behavior, verification, limitations and the next dependency pointer.
- No ticket can present proposed behavior as shipped or add a decorative control with no causal
  simulation, persistence or actionable explanation behind it.

## Phase A — Match and booking depth

### WM-001 — Match-rule core — complete

Detailed result: [WM-001](WM-001.md). Participant slots, match-local sides, participation/victory
rules and booked-result semantics are implemented without changing legacy singles saves.

### WM-002 — Tag and trios runtime

Reserved branch stem: `feature/wm-002-tag-trios-engine`. Depends on WM-001.

Outcome: simulate legal participants, corners, tags, team control, saves and coordinated actions
instead of treating extra workers as passive stamina slots.

Exit: deterministic chunk/resume parity; illegal tags/actions rejected; team-aware incidents,
reports and consequences; old singles and paused shows remain valid.

### WM-003 — Multi-person, elimination and timed entry

Reserved branch stem: `feature/wm-003-multi-person-engine`. Depends on WM-002.

Outcome: support more than two sides/active workers, fall/elimination history, battle-royal and
rumble-style timed entry, gauntlets, handicap structures and surviving participants.

Exit: winner/result invariants hold across one-fall and elimination formats; entry/elimination
order persists; AI/manual instructions target valid active workers; replay/resume is deterministic.

### WM-004 — Expanded match-plan persistence and booking UI

Reserved branch stem: `feature/wm-004-team-booking-ui`. Depends on WM-002–003.

Outcome: expose sides, slots, legal participants, entrants, fall/elimination rules and booked
results through one accessible editor and one versioned persisted plan.

Exit: no duplicate legacy/new source of truth; migration is backup-safe; drag/reorder/edit retains
identities; invalid plans never reach the runtime; singles workflow remains fast.

### WM-005 — Composable stipulation rules

Reserved branch stem: `feature/wm-005-stipulation-rules`. Depends on WM-003–004.

Outcome: implement weapons, tables, ladders, cages/enclosures, escape, falls locations, no-DQ,
hardcore/deathmatch, iron-person/scoring, last-person-standing and tournament modifiers through
composable rules rather than brand-name engine forks.

Exit: every modifier changes legal plans, agent advice, causality, safety, crowd response and
reporting; combinations validate; unsupported combinations fail clearly.

### WM-006 — Rule-aware road-agent planning and live adaptation

Reserved branch stem: `feature/wm-006-road-agent-rules`. Depends on WM-005.

Outcome: make agents plan match structure, team roles, legal saves/interference, risk and crowd
management for the chosen rules, consult affected workers, then translate player-authorised live
changes into legal future beats. A finish or winner changes only through explicit player authority.

Exit: suggestions and worker responses are reviewable; sequence-derived referee opportunities govern
delivery; automatic beats preserve manual intent; immutable history, future replanning, overload,
imperfect compliance and refusal behavior are deterministic and tested.

### WM-007 — Move chains and finish execution

Reserved branch stem: `feature/wm-007-match-sequences`. Depends on WM-006.

Outcome: model openings, transitions, counters, reversals, limb work, callbacks, escalation,
signature setups, referee communication opportunities and an explicit legal closing sequence that
can be amended without rewriting performed history.

Exit: the final move comes from the winner's legal repertoire; counters/failed moves have causal
recovery; body damage matters; no generic finish bypasses the planned result.

### WM-008 — Crowd, safety and long-term consequence balance

Reserved branch stem: `feature/wm-008-show-consequences`. Depends on WM-007 and the accepted WM-020 mapping.

Outcome: tune audience cohorts, regional/product expectations, pacing, fatigue, injuries,
development, wear, confidence, chemistry and show flow against representative seasons.

Exit: separate report dimensions remain; causal explanations are visible; representative cards
produce believable tradeoffs; short/long-save property and soak evidence replaces ad-hoc tuning.

### WM-009 — Advanced booking workflow

Reserved branch stem: `feature/wm-009-booking-workflow`. Depends on WM-004 and WM-007.

Outcome: persistent drafts, autosave/recovery, card-wide undo/redo, advanced phase timeline,
templates, duplicate segment, move drag/drop and suggestions-only agent review.

Exit: navigation/OS close protect dirty work; recovery never overwrites a newer revision; quick and
advanced workflows operate on the same plan; keyboard and interaction regressions pass.

### WM-010 — Moveset and learning content

Reserved branch stem: `feature/wm-010-moveset-library`. Depends on WM-007.

Outcome: expand regular, aerial, power, submission and situational moves; signatures/finishers;
proficiency, style fit, learning and paired contact metadata without presentation coupling.

Exit: content validates at build/import; generated workers receive coherent repertoires; moves have
simulation effects and legal contexts; balance samples cover major styles and body sizes.

## Phase B — People, identity and profile

### WM-020 — Wrestler stat hierarchy

Detailed proposed ticket: [WM-020](WM-020.md). Exactly six visible 0–100 base groups summarize
approved sub-stats; canonical values serve generation, simulation, saves, search and profiles.

### WM-022 — Wrestler identity and personality

Detailed proposed ticket: [WM-022](WM-022.md). Personality traits, languages, hobbies, background
and biography are deterministic, persisted and causal only where an approved rule says so.

### WM-023 — Relationships and player interactions

Detailed proposed ticket: [WM-023](WM-023.md). Directional personal relationships, memories and
contextual player actions are recorded without conflating real friendship with kayfabe stories.

### WM-021 — Roster discovery

Detailed proposed ticket: [WM-021](WM-021.md). Paged search, composable include/exclude filters,
sorting, comparisons and saved views cover stats, identity, relationships, availability and history.

### WM-024 — Person profile and career hub

Reserved branch stem: `feature/wm-024-person-profile-hub`. PD-133 is accepted; delivery uses one
execution ticket with internal phases and consumes canonical people/career sources as they exist.

Outcome: an original FM-inspired Person information hub with portrait, six-group stats/sub-stats, biography, personality,
languages, hobbies, condition, morale, contracts, promises, relationships, teams/stables, companies,
titles, awards, moveset, injuries, old matches, recent news and contextual interactions.

Exit: the Overview combines ratings/current state with a bounded recent-media rail; every section
deep-links to its source history/action; long lists page; only genuinely unknown/private fields are
qualified; profile state survives navigation; no single universal overall rating.

## Phase C — Kayfabe and creative production

### WM-025 — Real-life and kayfabe separation

Reserved branch stem: `feature/wm-025-kayfabe-model`. Depends on WM-022–023 and accepted PD-106;
its core model is implemented and verified locally and precedes full WM-024 profile integration.
Owner review and an explicitly authorised commit remain.

Outcome: separate a person, their current/previous characters, gimmicks, names, alignment, masks,
presentation and public claims from contracts, personal relationships and real health.

Exit: turns/character changes do not rewrite personal history; news/angles declare their reality
plane; permissions and timelines prevent contradictory active identities.

### WM-026 — Teams, stables, managers and authority figures

Reserved branch stem: `feature/wm-026-groups-and-managers`. Depends on WM-025.

Outcome: permanent tag teams, trios, stables/factions, leaders, managers/valets and on-screen
authority roles with membership history, chemistry, loyalty and brand/division eligibility.

Exit: match-local sides remain distinct; memberships are date-effective; breakups/turns preserve
history; group context affects legal creative options without forcing real relationships.

### WM-027 — Championships, divisions, rankings and tournaments

Reserved branch stem: `feature/wm-027-titles-and-rankings`. Depends on WM-026.

Outcome: title eligibility/prestige, contender pictures, defenses/vacancies, divisions, tournament
brackets, company/world rankings and complete lineages.

Exit: title changes derive from results once; eligibility is rule-based; rankings explain movement;
historic reigns, brackets and records survive company/character changes.

### WM-028 — Storylines, creative planner and pre-booking

Reserved branch stem: `feature/wm-028-storyline-planner`. Depends on WM-025–027.

Outcome: plan feuds/arcs, participants, objectives, beats, target events, future matches/angles,
turns, promises, title paths and unresolved hooks across the calendar.

Exit: plans are guidance rather than predetermined outcomes; actual events advance or disrupt them;
creative memory surfaces callbacks; pre-booking detects availability/rule conflicts early.

### WM-029 — Assisted/manual angle creator

Reserved branch stem: `feature/wm-029-angle-director`. A minimal scene contract can support M2;
full capability depends on WM-007 and WM-028.

Outcome: create timed, paced scenes through locations, participant roles, dialogue, entrances,
interruptions, crowd work, reveals, attacks, interference, saves, injury selling and exits. Assisted
mode proposes an editable structure; manual mode controls every beat.

Exit: one structured scene contract drives simulation, recap and live presentation; manual text
overrides templates safely; participant skills/context affect execution; no runtime network model,
theme-song, titantron or full-animation dependency.

## Phase D — Company, media and living world

### WM-011 — Company identity and product foundation

Reserved branch stem: `feature/wm-011-company-foundations`. A foundation slice follows WM-020 and
PD-105; later character/angle integrations consume WM-025 and WM-029 without blocking company basics.

Outcome: custom company creation plus name/branding, founding history, headquarters, product,
audience, culture, size/prestige/momentum, brands, divisions, geography and market reach.

Exit: product preferences cause audience/worker/business tradeoffs; brands have scoped rosters,
shows/titles/staff; growth thresholds explain benefits/costs; UWF becomes ordinary seeded content.

### WM-030 — Ownership, goals and company hierarchy

Reserved branch stem: `feature/wm-030-company-governance`. Depends on WM-011.

Outcome: owners, boards, executives, general managers, bookers, booking teams, inner circles,
figureheads and culture-dependent authority with budgets, goals, trust and career-role resignation.

Exit: permissions differ by company/role; owner goals are measurable; conflicts produce choices;
hierarchy changes preserve history and never silently seize player control.

### WM-042 — Company finance, budgets and investment

Reserved branch stem: `feature/wm-042-company-finance`. Depends on WM-011 and WM-030.

Outcome: an auditable ledger for cash, wages, appearance fees, ticketing, travel, venues,
production, facilities, training, medical costs, merchandise, advertising, sponsorship, media
rights, loans/debt, taxes where configured, investments, acquisitions and one-off adjustments.

Exit: every balance change has a dated source and entity; monthly reports reconcile to the ledger;
budgets/forecasts explain committed versus available cash; difficulty can simplify presentation
without changing accounting truth; bankruptcy/recovery rules are explicit and tested.

### WM-031 — Contracts, promises and talent operations

Reserved branch stem: `feature/wm-031-talent-operations`. A basic negotiation slice depends on WM-022,
WM-030 and WM-042; full relationship effects consume WM-023.

Outcome: offers/renewals, negotiation, wages/bonuses, appearance terms, exclusivity, options,
merchandise cuts, creative control, release/non-compete terms, loans, time off, pushes, promises,
absences, suspensions, medical/wellness and configurable fictional drug testing.

Exit: parties negotiate from goals/leverage; clauses are enforced; every promise has scope/due
date/outcome; contract overview exposes risk; sensitive systems are configurable and non-exploitative.

### WM-032 — Staff, facilities, scouting and development

Reserved branch stem: `feature/wm-032-training-and-development`. Depends on WM-020, WM-022, WM-011,
WM-030–031 and WM-042.

Outcome: hire/develop road agents, trainers, scouts, doctors and producers; invest in office,
production, medical and training facilities; run individual plans, mentoring, schools/academies,
try-outs, newgens, call-ups, loans/excursions, retirement and staff conversion.

Exit: workload, age, potential, trainer quality, facilities, matches and recovery create explainable
development; scouting uncertainty is preserved; academy decisions have real financial/roster cost.

### WM-033 — Calendar, events and live operations

Reserved branch stem: `feature/wm-033-events-and-touring`. Depends on WM-011, WM-031–032 and WM-042.

Outcome: real calendar; recurring TV/special/premium/developmental/house events; tours; venues,
travel, weather, capacity, local demand, ticket prices, preshow/postshow, birthdays and scheduling.

Exit: calendars detect conflicts; attendance/gate derives from causal inputs; house shows can auto,
agent-direct or play lightly and affect reps/chemistry/conditioning/local popularity without TV weight.

### WM-034 — Broadcasting, sponsors and production

Reserved branch stem: `feature/wm-034-broadcast-business`. Depends on WM-011, WM-033 and WM-042.

Outcome: TV/streaming/PPV deals, networks, territories/coverage, time slots, ratings targets,
exclusivity, renewals, commentary teams/languages, advertising, sponsors, merchandise and production.

Exit: every deal has obligations and tradeoffs; ratings/revenue/coverage are explained; sponsor and
network trust respond to actual events; production quality costs money and changes expectations.

### WM-035 — Company office and actionable inbox

Reserved branch stem: `feature/wm-035-office-command-centre`. A thin M1 office consumes WM-030–031
and WM-042; later slices integrate WM-032–034 and beyond.

Outcome: an alert-driven office with company snapshot, assistant priorities, quick diary/e-mail/
roster/shortlist/blacklist, birthdays, deadlines, finances, next events, quick actions and deep-linked
inbox.

Exit: alerts deduplicate and rank urgency; messages identify cause, deadline and actions; resolved
items remain historical; no generic dashboard metric exists without an explanation or route to act.

### WM-036 — Media, press, social and random events

Reserved branch stem: `feature/wm-036-media-and-events`. Depends on WM-023, WM-028, WM-011 and
relevant WM-030–035 facts.

Outcome: journalists/critics, show/match reviews, interviews, press conferences/media scrums,
rumours, fan/critic social streams, public statements, controversies, random industry/backstage/
travel/business events and celebrities as restricted world characters.

Exit: content derives from stored facts or seeded event rules; outlets and fan cohorts have distinct
biases; player answers have visible consequences; random events have preconditions/cooldowns and
never fabricate canonical results.

### WM-012 — Company/world relationship foundation

Reserved branch stem: `feature/wm-012-world-foundation`. A schema/event slice follows WM-011 and
WM-042; full completion consumes WM-031–034.

Outcome: canonical companies, ownership edges, dated world events, markets, eras, popularity,
company-to-company sentiment and common decision/report contracts for player and CPU companies.

Exit: world state is deterministic, date-effective and queryable; all actors use compatible rules;
history can explain current ownership, relationships, rankings and market position.

### WM-037 — Alliances, company groups and takeovers

Reserved branch stem: `feature/wm-037-company-networks`. Depends on WM-012, WM-030–034 and WM-042.

Outcome: alliances, working agreements, talent trades/loans, co-promotion, parent/child/sister and
developmental companies, investments, mergers, acquisitions, takeovers, conglomerates, rating wars,
counter-programming and configurable competitive/dirty tactics.

Exit: authority/finance/contract ownership remain explicit; transactions preserve histories;
agreements enforce rights; aggressive actions carry legal, financial, relationship and reputation risk.

### WM-038 — Rival-company AI and CPU show simulation

Reserved branch stem: `feature/wm-038-world-simulation`. Depends on WM-028 and WM-033–037.

Outcome: CPU companies hire/release/develop talent, negotiate, set products/goals, schedule/book
shows, run storylines, respond to finances/competition, grow, decline, merge, launch or close.

Exit: decisions use observable goals/resources rather than omniscience; CPU shows produce canonical
results/history/news; bounded simulation scales across years; deterministic seeds permit diagnosis.

### WM-039 — Global search and CPU-show viewer

Reserved branch stem: `feature/wm-039-world-search-and-viewer`. Depends on WM-034 and WM-036–038.

Outcome: intuitive global search for workers, companies, shows, titles, teams, stories and news;
open any CPU card/result and watch quick recap, key highlights or available detailed event stream.

Exit: search is paged/filtered; visibility respects scouting/media knowledge; viewing never changes
simulation; stored events and summaries cannot contradict official results.

### WM-040 — Legacy, records and honours

Reserved branch stem: `feature/wm-040-history-and-honours`. Depends on WM-027 and WM-036–039.

Outcome: show history, company history, career timelines, alumni, legends, retirements, awards, end-of-year
honours, Hall of Fame, Top 100s, records, title/team histories and historic news navigation.

Exit: records derive from canonical events; corrections are auditable; histories survive names,
characters, company ownership and database upgrades; ranking criteria are visible.

## Phase E — Presentation, hardening and release

### WM-013 — Presentation event contract

Reserved branch stem: `feature/wm-013-presentation-contract`. A match slice follows WM-007; full
completion integrates WM-029, WM-034, WM-036 and WM-038.

Outcome: a renderer-neutral stream of match, angle, entrance, interference, dialogue, crowd,
commentary, lighting and production cues for player and CPU shows, including the original brief,
referee communication opportunity, instruction response and accepted future-plan amendment.

Exit: presentation cannot decide results; summaries and detailed playback use the same facts;
versioned cues degrade gracefully; simulation tests do not require a renderer.

### WM-014 — Text-led arena and show viewer

Reserved branch stem: `feature/wm-014-arena-presentation`. Depends incrementally on WM-013.

Outcome: smart arena lighting/effects, location/stage cards, typography, portraits where available,
crowd state, commentary and timed text that make matches and angles watchable without full animation.

Exit: Quick Sim, Extended Highlights and Realtime agree across the guaranteed 2.5D viewer and any
available 3D viewer; show defaults and per-segment overrides work; accessibility offers reduced
motion/audio; missing assets fall back cleanly.

### WM-043 — Optional retro 3D broadcast engine

Reserved branch stem: `feature/wm-043-retro-broadcast`. Conditional on accepted PD-132, a successful
go/no-go spike and stable representative WM-013 events; it never blocks WM-014 or the core game.

Outcome: an original PlayCanvas Engine v2 low-poly broadcast viewer integrated through
`@playcanvas/react`, using a canonical WM skeleton, modular wrestlers, verified licensed/original
paired attacker/receiver animations, WM-owned move recipes and automated Blender/GLB processing.
WebGL2 is required; WebGPU is optional. Arenas, crowds, lighting, effects and cameras replay canonical
entrances, matches and later structured scenes.

Live instructions rebuild only buffered future animation choices after Rust accepts an amendment.
Quick Sim hands off to contingencies; 3D Extended Highlights and 3D Realtime expose the same decisions
as 2.5D without letting animation callbacks communicate with workers or mutate simulation.

Exit: representative singles/tag shows replay the same facts as reports and text playback across body
types, speeds, missing assets and target hardware; the documented ordinary-PC preset sustains 60 FPS.
The viewer cannot decide results, and VPG remains reference/research only: no proprietary No Mercy/VPG
code, unverified asset, unlicensed likeness or required optional pack enters WM.

### WM-015 — Save and compatibility hardening

Reserved branch stem: `feature/wm-015-save-hardening`. This is continuous: every schema ticket owns
its migration, while final recovery/compatibility exit follows WM-040.

Outcome: documented compatibility policy, backup rotation/recovery, repair diagnostics, long-save
compaction/performance and migration coverage for every schema/engine version.

Exit: corrupt/conflicting files never mutate; recovery is user-controlled and tested; old careers
upgrade once; multi-year saves remain bounded and inspectable.

### WM-016 — Visual identity and complex-screen design

Reserved branch stem: `feature/wm-016-desktop-identity`. Patterns evolve with proven screens; final
cross-screen completion consumes WM-014 and WM-035.

Outcome: a coherent WM design system for dense office, profile, roster, creative, event and world
screens with wrestling character and clear hierarchy rather than copied TEW/FM layouts.

Exit: reusable tokens/layouts handle compact/normal widths; status/action hierarchy is consistent;
screens pass screenshot review, contrast and typography checks.

### WM-017 — Navigation and accessible management workflows

Reserved branch stem: `feature/wm-017-management-navigation`. Built incrementally with proven screens;
final completion follows WM-016.

Outcome: Home/Booking/Talent/Company/World navigation, context menus, history/back behavior, command
search, saved filters, keyboard shortcuts, focus management and cross-entity deep links.

Exit: common tasks minimize repeated clicks; navigation preserves context/drafts; keyboard-only and
screen-reader paths cover core loops; global search and inbox links resolve correctly.

### WM-018 — Regression, balance, performance and soak suite

Reserved branch stem: `feature/wm-018-regression-suite`. Quality gates start early; final release
coverage consumes WM-015–017 and PD-110.

Outcome: deterministic/property/contract tests, representative season balance, CPU-world soak,
database scale, migration matrix, accessibility audit, profiling and local native release smoke.

Exit: published budgets and failure thresholds exist; flaky or non-reproducible failures are not
ignored; CI/local responsibilities are explicit; release-blocking regressions have fixtures.

### WM-041 — Native custom content and optional asset packs

Reserved branch stem: `feature/wm-041-custom-content`. Stable IDs and validation constrain every
ticket; public editor/import/export follows WM-015–018 and PD-109.

Outcome: native database/scenario editor and import/export for workers, companies, titles, events,
venues, moves, rules, stories, relationships and history. A versioned manifest supports optional
portrait packs first and optional local sound/video packs last.

Exit: validation reports exact errors before import; IDs/dependencies/conflicts are deterministic;
packages cannot escape their root or execute code; missing/unlicensed assets are never required;
saves snapshot required data and declare package/version provenance.

### WM-019 — Release candidate and project cleanup

Reserved branch stem: `feature/wm-019-project-cleanup`. Depends on WM-041 and accepted PD-110.

Outcome: Windows installer/uninstaller, application data/recovery documentation, licensing and
third-party notices, contributor/mod-author guidance, final public feature claims and mechanical cleanup.

Exit: clean-machine install/upgrade/uninstall smoke passes without deleting careers; release notes
match shipped behavior; no secrets/generated caches ship; remaining limitations are explicit.

## Product acceptance gates

These gates summarize [the playable milestones](../milestones.md); they do not require every listed
capability to be globally complete before useful slices of another capability begin.

### Gate 1 — Management week worth repeating

A thin people/company/contract/finance/profile/office loop creates informed decisions and explains
their consequences around the existing singles show. This proves the management fantasy early.

### Gate 2 — Matches and production worth watching

Representative tag/multi-person/rule structures, agents and finishes feed the first text-led match
and angle presentation. Depth expands from proven examples instead of an empty format catalogue.

### Gate 3 — People and creative memory worth managing

Stats, identities, relationships, characters, groups, titles and stories stay coherent across a
multi-month arc, with real-life and kayfabe state demonstrably separate.

### Gate 4 — Company worth operating

Governance, contracts, finance, development, events, broadcasting, media and the office reconcile
through one calendar, ledger and event history over a representative season.

### Gate 5 — Industry worth observing

CPU companies use compatible rules for years, produce inspectable shows and decisions, and create
searchable company/worker history without privileged information or unbounded simulation cost.

### Gate 6 — Product worth shipping and extending

Compatibility, recovery, performance, accessibility, native custom content and clean installation
meet published thresholds. Optional media packs cannot delay or destabilize the core release.
