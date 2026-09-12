# Wrestling Manager product decision register

This register prevents provisional ideas from becoming accidental mechanics. “Accepted” means the
owner has confirmed the behavior. “Open” means implementation must not invent the answer. Decisions
record product rules; implementation evidence and architecture remain in the SDD and ticket handoffs.

## Accepted decisions

| ID     | Decision                                                                                                                                                                               | Consequence                                                                                                                                                                                                    |
| ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| PD-001 | Wrestlers expose exactly six base stat groups, each an integer from 0 to 100, with explanatory sub-stats.                                                                              | There is no context-free universal rating; the canonical catalogue is PD-008 and the current-Archetype derivation is PD-101.                                                                                   |
| PD-002 | Real-life personal state, kayfabe state and presentation are separate causal layers.                                                                                                   | Turns, angles and dialogue cannot silently rewrite contracts, health or personal relationships.                                                                                                                |
| PD-003 | The show viewer is text-led and simulation-driven, using scene cards, commentary, crowd state, smart lighting and restrained effects.                                                  | Theme songs, titantrons and full wrestler animation are not planned core features.                                                                                                                             |
| PD-004 | Native custom databases/scenarios are first-class; portraits are optional and sound/video packs are optional late layers.                                                              | Simulation and saves must work without assets or a network service.                                                                                                                                            |
| PD-005 | CPU companies use compatible rules and leave inspectable history.                                                                                                                      | Difficulty may change information or assistance, not grant unexplained simulation exceptions.                                                                                                                  |
| PD-006 | Important ratings, reviews, alerts and consequences must expose their causes.                                                                                                          | Decorative totals and unsupported dashboard metrics are rejected.                                                                                                                                              |
| PD-007 | Competitive games are research references, not specifications to copy.                                                                                                                 | Names, interface layouts, content and rules require an independent WM rationale.                                                                                                                               |
| PD-008 | The canonical wrestler stat catalogue is the six-group model recorded below.                                                                                                           | Generation, saves, simulation, search and profiles must use these exact concepts rather than inventing UI-local ratings.                                                                                       |
| PD-009 | Every human uses one Person record; wrestler, commentator, manager, booker, producer, trainer and other jobs are roles supported by independent Profession Skills and Role Experience. | Every Person has canonical wrestling ratings so an unexpected in-ring transition is possible, but profession ability stays separate and unsupported public knowledge is shown honestly rather than fabricated. |
| PD-101 | Wrestler group scores, Discipline Fits, Archetypes, current-Archetype Overall and style-development rules use the accepted deterministic model below.                                  | WM-020 may be decomposed for implementation; WM-008 owns later balance calibration without collapsing style, popularity and profession ability together.                                                       |
| PD-102 | Search uses a dedicated persistent workspace and shared category framework; established workers' publicly demonstrated ability is exact in current-era saves.                          | WM-021 builds the framework and Worker Finder; private potential, medical, contract and creative information remains protected while later tickets register their entity-specific discovery data.              |
| PD-106 | Person, Character and dated Character Tenure are separate; wrestling surfaces lead with the active ring identity while real identity remains appropriately scoped.                     | WM-025 must land before the final profile/Finder identity integration; names, aliases, gimmicks, alignments, masks and changes follow the accepted character-and-presentation rules.                           |
| PD-112 | Match instructions use pre-match worker agreement and sequence-derived referee communication opportunities; accepted live changes rebuild only the unsimulated match future.           | WM-006–009 and WM-013–014 follow the live-direction rules; WM-023/031/034 consume relationship, morale and broadcast consequences, while renderers remain non-authoritative.                                   |
| PD-132 | A future optional retro 3D broadcast viewer uses PlayCanvas Engine v2 through `@playcanvas/react`, driven only by canonical presentation events.                                       | WM-043 is conditional post-core work; WebGL2 is required, WebGPU is optional, text/2.5D remains complete and VPG stays reference/research only with no GPL code or unverified assets.                          |
| PD-133 | Every Person uses one persistent profile/career hub with a dense Overview, role-aware depth, contextual actions, attributed opinions and red-to-green rating readability.              | WM-024 follows `person-profile-hub-rules.md`; it includes underlying wrestling ability for non-wrestlers and composes moveset permissions, development and history from their owning systems.                  |

### PD-007 interface research and integration doctrine — clarified 11 September 2026

Every player-facing execution ticket follows the
[interface and integration rules](interface-and-integration-rules.md). It studies the relevant TEW IX
counterpart when one exists and a focused current wrestling article or workflow, then records how WM
makes the player's task clearer and faster without copying the reference. Interfaces preserve working
context, expose causes and connect canonical people, company, show and history systems instead of
forming isolated menus.

### PD-008 canonical wrestler stat catalogue

All fields are integers from 0 to 100.

- **Movement:** Acceleration, Ring Speed, Agility, Acrobatics, Jumping, Flexibility, Balance.
- **Physicality:** Strength, Stamina, Toughness, Recovery, Injury Resistance.
- **Ringcraft:** Technical Grappling, Chain Wrestling, Striking, Brawling, Submissions, Aerial
  Wrestling, Power Offense, Hardcore/Weapons, Countering.
- **Psychology:** Match Storytelling, Pacing, Selling, Crowd Reading, Match Calling, Adaptability.
- **Fundamentals:** Timing, Consistency, Safety, Bumping, Positioning, Cooperation.
- **Entertainment:** Presence, Microphone, Character Performance, Crowd Connection.

Derived outputs such as explosiveness, schedule resilience, fatigue, injury risk and style-specific
overall ratings are not canonical editable sub-stats. Look, popularity, menace, marketability and
reputation are separate concepts outside wrestling ability. Raw physical capability and wrestling
application remain separate: for example, Acrobatics differs from Aerial Wrestling, Strength from
Power Offense, and Technical Grappling from fluid multi-technique Chain Wrestling.

### PD-101 wrestler derivation, style and Overall — accepted 10 September 2026

- A visible base-group score is the arithmetic mean of that group's canonical sub-stats, rounded to
  the nearest integer. The sub-stats remain the source of truth.
- The nine Disciplines are Technical Grappling, Submission Wrestling, Lucha Libre, Aerial Wrestling,
  Striking, Power Wrestling, Brawling, Hardcore Wrestling and Shoot Fighting.
- `Discipline Fit = 55% Attribute Fit + 25% Moveset Readiness + 15% Discipline Mastery + 5% Proven
Performance`. WM-020 owns the accepted Attribute Fit matrix.
- With one Secondary, `Discipline Blend = 75% Primary + 25% Secondary`. With two, it is
  `70% Primary + 20% Secondary 1 + 10% Secondary 2`; without one, it is 100% Primary.
- `Current-Archetype Overall = 50% Discipline Blend + 20% Psychology base + 20% Fundamentals base
  - 10% Entertainment base + Proven Performance Modifier`. The modifier is an evidence-backed
    integer from -3 to +3; popularity, Look and booking position cannot supply it.
- The highest credible Fit is Primary. A Secondary needs Fit 65 or higher, must be within 12 points
  of Primary, and needs Moveset Readiness and Discipline Mastery of at least 60. At most two are
  retained. A worker is Developing when the best Fit is below 55.
- A new Primary must lead the current Primary by four points for 90 in-game days or eight matches.
  A lead of eight points may change it immediately. This prevents noisy Archetype churn.
- All-Rounder is exceptional: at least five Discipline Fits of 85, Moveset Readiness and Mastery of
  80 in all five, and no more than five points between the first and fifth Fits.
- Approach is stored independently across Tempo (Methodical/Balanced/Fast-Paced), Structure
  (Traditional/Escalating/Spot-Driven), Presentation (Sport-Realistic/Dramatic/Spectacle/Comedy),
  Contact (Light/Standard/Stiff) and Risk (Conservative/Balanced/Daredevil).
- Archetype names are selected from a curated rule table: Primary supplies the noun, the strongest
  Secondary normally supplies the modifier, and a distinctive Approach may replace that modifier.
  Specialisations are badges, not name fragments. Custom databases may provide validated overrides.
- Training changes relevant sub-stats, Moveset Readiness, Discipline Mastery and moveset composition
  over time. The interface may preview a projected gain or loss, but staff quality affects forecast
  confidence rather than guaranteeing the outcome. Measurements constrain feasibility and safety;
  they do not provide free rating bonuses.

Affected capabilities: WM-008, WM-010, WM-020, WM-021, WM-024 and WM-032. Rejected alternatives:
one universal Overall, Attribute Fit as Overall, popularity bonuses to wrestling ability, body-size
bonuses, generated Archetype prose and instant style changes. Save compatibility requires stable
field IDs, versioned migration and retained historical source values; no existing field may be
silently reinterpreted.

### PD-102 search, worker discovery and knowledge — accepted 11 September 2026

The owner accepted the [search and discovery rules](search-and-discovery-rules.md): a dedicated
persistent Search Workspace; live, explainable text matching; composable include/exclude filters;
server-backed pagination; stable sorting; configurable columns; four-worker comparison; named saved
views and shortlists; one personal blacklist; and exact publicly demonstrated wrestling ability for
established current-era workers. Unknown values remain explicit for genuinely unobserved or private
information.

WM-021 owns the reusable query framework and current-data Talent Search foundation. WM-025 registers
active ring identities and aliases before the Worker Finder is complete. Later domain tickets register
companies, shows/events, contracts, storylines, titles, teams, venues, media and history without
duplicating search engines or exposing private CPU-company records. PD-125 later owns the full Entity
Hub and contextual Booking Reference Drawer design while preserving this search-state contract.

### PD-009 unified Person and profession rating — accepted 10 September 2026

Shared Person attributes are Communication, Leadership, Creativity, Adaptability, Organisation,
Professionalism, Stress Management, Teaching and Languages. Profession Skills are grouped by
Broadcasting, Creative, Coaching, Officiating, Scouting, Medical and Business work; role-specific
skills such as Ringside Management are allowed where they describe a real responsibility rather
than duplicate Entertainment.

`Role Rating = 65% relevant Profession Skills + 20% Shared Person attributes + 15% Role Experience`.
Actual performance may then consume workload, chemistry, product knowledge, language, pressure and
fatigue. Occasional guest work may use explicitly mapped transferable attributes, but regular work
develops its dedicated Profession Skills. WM-032 owns the shared catalogue and calculation kernel;
the capability responsible for each job owns its requirements, assignments and interface.

Every Person also has the canonical six-group wrestling sub-stats even when they have never held a
wrestling role. Untrained people receive evidence-based starting values—normally weak wrestling
application with any legitimate athletic, performance or background transfer retained—so a future
in-ring attempt uses real simulation state rather than generating ability on debut. Visibility is a
separate knowledge question: only genuinely unevidenced facts are estimated or unknown. Profession
ratings never substitute for wrestling ratings, and a rare fast transition still requires
proficiency, safety, moveset and training evidence.

### PD-103 personality and exceptional-trait direction — design prepared 10 September 2026

The owner accepted the recommendation to retain eight personality ratings and nine shared qualities,
clarify overlaps, use evidence for exceptional traits and support unknown information. The concrete
[naming and event rules](personality-and-trait-rules.md) and [WM-022.1 execution ticket](tickets/WM-022.1.md)
are implemented together following the owner's WM-022 instruction. Exact thresholds are rule-version-1
defaults, not balance evidence; downstream management effects remain future work. This supersedes chat examples and speculative
real-worker personality ratings.

PD-009 vocabulary amendment: add Work Ethic separately from Professionalism, call shared Adaptability
Personal Adaptability, and treat Languages as structured identity data rather than a ninth numeric
quality. WM-022 defines the shared vocabulary; WM-032 retains profession calculation and training
ownership. The accepted 65/20/15 role formula is unchanged. The accepted PD-102 knowledge rules
govern what discovery and scouting interfaces may reveal; unknown inputs and suppression of hidden
values are required from the outset. WM-032 owns scouting mechanics.

### PD-104 directional relationships and interactions — accepted 11 September 2026

The owner approved directional Affinity, Respect, Trust and Tension; dated memories with retained
history and bounded tension decay; context-sensitive player conversations; explicit cooldowns and
management-attention costs; and lightweight personalised replies without a runtime language model.
The concrete [relationship and interaction rules](relationship-and-interaction-rules.md) are rule
version 1 and are implemented by WM-023. Kayfabe, match chemistry, company sentiment and contracts
remain separate. WM-030 must later connect current-management history to playable Person roles.

### PD-105 company product, audience, culture, scale and markets — accepted 11 September 2026

The owner approved the [company/product/market rules](company-product-market-rules.md): declared,
delivered and core product identity; company/brand/show scope; soft audience expectations; evidence-
based audience estimates; dynamic regional affinity; organisation culture; seven scale labels with
separate reach/prestige/momentum/finance; global moddable geography; local saturation; tiered world
simulation; and a read-only shipped database with future pre-game and marked in-game editors.
This decision is design input only and does not claim WM-008, WM-011 or the world/business systems are
implemented.

### PD-106 person, character and presentation — accepted 11 September 2026

The owner approved the [person, character and presentation rules](character-and-presentation-rules.md):
separate Person, Character and dated Character Tenure records; active ring names on wrestling
surfaces; appropriately scoped real names; searchable known aliases; context-bound gimmick,
alignment and mask history; and a proposal/preparation/debut workflow for identity changes. WM-025
owns the canonical model and precedes final WM-024 profile work. Schema 8 now implements the core
record/change model and WM-021 Finder identity integration; later company and audience systems add
their own context/evidence producers rather than duplicating it.

Decision completion accepted 12 September 2026: active character changes are proposed by the company
and may be accepted, negotiated or refused; preparation uses an advisor range and explicit public
debut. Abrupt exposed renames risk continuity/acceptance but never lower wrestling ability. Gimmicks
change contextual delivery and reception rather than permanent stats, and players control a creative
brief rather than numerical bonuses. Multiple-company identities may be open; a simultaneous secret
identity needs credible concealment and Private/Rumoured/Public knowledge. Character retirement is
separate from Person retirement. Face/Heel/Tweener/Unaligned is the complete intent catalogue;
perceived role, reaction, intensity, acceptance and intent match are derived separately.

Popularity clarification accepted 11 September 2026: worker popularity is a 0–100 regional fact,
not wrestling ability or a bonus to current-style Overall. The interface leads with the market
relevant to the current decision and exposes the regional breakdown on demand. Momentum,
marketability, reach and contextual drawing power remain separate.

### PD-133 person profile and career hub — accepted 12 September 2026

The owner approved the complete [person profile and career hub rules](person-profile-hub-rules.md).
WM-024 is one persistent, role-aware information hub for every Person: a dense overview leads with
identity and wrestling ratings, relevant state and a recent media rail; deeper tabs own evidence and
history. Values use a weak-red to elite-green quality scale, with labels/icons and accessible text.
All people retain underlying wrestling ratings, while genuinely obscure or private knowledge remains
honestly qualified. Contextual actions use an organised FM-inspired menu and chronological text-chat
interaction without copying another game's layout.

The same decision defines a serious moveset surface and protected-move etiquette: generic move
families remain distinct from named signature presentations; permission can be scoped and negotiated;
an unauthorised use may create social, company or media consequences without being confused with a
separate safety/proficiency block. WM-010/023/031/032 own the causal move, relationship, contractual
and training data that WM-024 composes. Development allows only a small number of realistic focuses
and gives ranged staff forecasts rather than guaranteed improvement.

### PD-132 optional retro 3D broadcast direction — accepted 12 September 2026

The owner approved the [retro broadcast engine rules](retro-broadcast-engine-rules.md): an original
PlayCanvas Engine v2 low-poly viewer integrated through `@playcanvas/react` in WM's existing
React/TypeScript/Vite/Tauri client. WebGL2 is the required baseline and WebGPU is optional. The viewer
is driven by the renderer-neutral event contract and retains a complete text/2.5D fallback. One
canonical WM skeleton, modular models, reproducible Blender/GLB processing, paired attacker/receiver
animations and WM-owned move recipes provide the content path. Virtual Pro Grappler remains
reference/research only; its GPL code, unverified assets, proprietary No Mercy material and unlicensed
likenesses/branding are excluded.

WM-043 begins only after a bounded go/no-go spike can consume stable WM-013 facts. It is a broadcast
viewer, not playable combat, and cannot delay the management game. The spike must prove sustained
60 FPS on a documented ordinary-PC baseline in the real Tauri/WebView2 application. Exact purchased
assets, licences and the post-spike production commitment remain evidence gates inside WM-043 rather
than assumptions in current implementation.

### PD-112 match planning and live direction — accepted 12 September 2026

The owner approved the [match planning and live direction rules](live-match-direction-rules.md).
Workers normally cooperate, while specific safety, creative-control, trust, morale, status,
relationship and personality causes can produce concern, negotiation, imperfect compliance or
refusal before or during a match. Backstage instructions reach active wrestlers through the referee
only at credible opportunities produced by the executed move/sequence state, never through a fixed
timer or renderer callback.

Rust commits an immutable executed prefix and deterministically rebuilds only the unsimulated suffix
after a delivered, accepted instruction. The player may issue none, one or many; excessive or
contradictory direction creates contextual cognitive, cohesion and safety risk rather than an
artificial instruction cap. Player-authorised changes may include sequences, interference, finish or
winner, but essential physical participants must be informed and consequences remain explainable.
Quick Sim uses pre-booked contingencies; Extended Highlights can pause at management decisions; 2.5D
and 3D Realtime expose continuous live direction while sharing the same canonical facts.

### PD-119 social-feed direction — partially accepted 11 September 2026

WM-036 will provide bounded procedural social posts from persistent fan, journalist, worker, company,
celebrity and outlet accounts. Authored compositional writing plus account voice, knowledge, bias,
relationships, region and event facts must produce realistic variation without runtime AI. Workers and
company accounts can be asked to promote, respond, tease, apologise or stay silent; personality,
morale, relationship and media skill affect compliance and consequences. Independent posts, deletion
memory, replies and uncertain rumours are supported. Exact outlet taxonomy, press rules, random-event
catalogue and celebrity limits remain open under PD-119.

Reporter-generation direction accepted 11 September 2026: custom-database reporters and outlets are
retained when supplied; deterministic world generation fills only the missing media ecosystem for a
new save. Generated reporters receive persistent identities, careers, regions, outlet roles,
credibility/reach, wrestling preferences and a small curated set of visible quirks. A reporter may use
a star scale, ten-point score, letter grade or prose-only review, backed by one normalized assessment
for simulation effects. Individual taste and access can produce legitimate disagreement, but reviews
cannot invent or overwrite official match facts. Exact generation counts, distributions and quirk
catalogue remain part of the open PD-119 balance/content decision.

Booking-reaction direction accepted 11 September 2026: reporters may publish articles, short social
posts and replies about any completed public booking unit—matches, promos, angles, debuts, turns,
interferences, title changes, storyline developments and whole shows/events. They may praise, mock or
strongly criticise it in their established voice. Each subject uses an appropriate rubric: match
execution is not reused as angle quality, and a show review is not a simple average of match scores.
Posts must be grounded in observed public facts, respect each reporter's access and knowledge, and
remain bounded and deduplicated. Private plans cannot leak as fact; uncertain information uses the
separate rumour system.

### PD-131 temporary alumni returns and event rumours — partially accepted 11 September 2026

The owner approved the [alumni return and rumour rules](alumni-return-and-rumour-rules.md): persistent
alumni/legend history, company-dependent date-bound return agreements, safe role-specific appearances,
custom-database support and fictional pre-event reporting whose claim may later prove accurate,
inaccurate or partial. WM-040 owns legacy status, WM-031 agreements, WM-033 events, WM-028/029
creative use, WM-036 media and WM-042 finance. Exact eligibility weights, agreement catalogue,
frequency and rumour-confidence presentation remain open.

## Open decisions that block specific implementation

| ID     | Required decision                                                                                                               | Blocks                                            |
| ------ | ------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| PD-107 | Financial difficulty layers, currency display, tax/debt detail and bankruptcy/recovery philosophy.                              | WM-042 balance and interface.                     |
| PD-108 | Contract clause catalogue, negotiation cadence and acceptable sensitive wellness/drug-testing presentation.                     | WM-031.                                           |
| PD-109 | First-release database editor scope and compatibility promise for third-party packs.                                            | WM-041 public format.                             |
| PD-110 | First commercial/release milestone: required world size, content volume and supported career length.                            | Final scope for WM-018–019 and release claims.    |
| PD-111 | Match-format rules: tag legality, fall/elimination/entry semantics, stipulation composition and quick/advanced booking control. | WM-002–005.                                       |
| PD-113 | Draft autosave/recovery, undo checkpoint and reusable-template policy.                                                          | WM-009.                                           |
| PD-114 | Move taxonomy, naming, repertoire sizes, visibility and learning/proficiency cadence.                                           | WM-010.                                           |
| PD-115 | Playable company roles, governance permissions, owner-goal families and dismissal/resignation continuation.                     | WM-030.                                           |
| PD-116 | Calendar granularity, event families, travel/weather depth, pricing and house-show automation.                                  | WM-033.                                           |
| PD-117 | Broadcast/ratings abstraction, media territories, sponsor sensitivity, merchandise and production depth.                        | WM-034.                                           |
| PD-118 | Office urgency, notification, snooze and assistant automation authority.                                                        | WM-035.                                           |
| PD-119 | Remaining outlet taxonomy, press-response rules, rumour confidence, random-event catalogue and celebrity limits.                | WM-036 beyond the accepted social-feed direction. |
| PD-120 | Agreement, talent-trade, corporate-control and competitive-tactic catalogue and risk model.                                     | WM-037.                                           |
| PD-121 | CPU-company difficulty philosophy, decision horizon, simulation-detail tiers and lifecycle target rates.                        | WM-038.                                           |
| PD-122 | Historical event/detail retention, search knowledge/ranking and presentation-stream compatibility.                              | WM-013 and WM-039.                                |
| PD-123 | Title prestige, divisions, rankings, tournaments, awards, Top 100 and Hall of Fame criteria.                                    | WM-027 and WM-040.                                |
| PD-124 | Storyline state/objectives, angle-beat catalogue, dialogue storage and assisted-versus-manual authority.                        | WM-028–029.                                       |
| PD-125 | Viewer timing/density/effects, WM visual direction, navigation hierarchy, shortcuts and accessibility defaults.                 | WM-014 and WM-016–017.                            |
| PD-126 | Save backup/recovery retention, supported upgrade horizon, repair limits and engine-version policy.                             | WM-015.                                           |
| PD-127 | Supported hardware/world/career performance budgets and representative balance ranges.                                          | WM-018.                                           |
| PD-128 | Release channel/version/signing and supported Windows/installer policy.                                                         | WM-019.                                           |
| PD-129 | Permanent group types, overlapping membership, chemistry/loyalty dimensions and manager/authority permissions.                  | WM-026.                                           |
| PD-130 | World market/era granularity, company-relationship dimensions and long-save event retention.                                    | WM-012.                                           |
| PD-131 | Exact temporary-return agreement catalogue, eligibility weighting, rumour frequency/confidence and resolution effects.          | WM-031, WM-033, WM-036, WM-040 and WM-042.        |

## Decision record format

When the owner resolves an item, move it to Accepted and record: decision text, date, affected
tickets, alternatives rejected and migration/compatibility consequences. If later evidence changes
it, add a superseding ID rather than rewriting history invisibly.
