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
| PD-132 | A future retro 3D broadcast viewer uses PlayCanvas Engine v2 through `@playcanvas/react`, driven only by canonical presentation events.                                                | WM-043 remains spike-gated; WebGL2 is required, WebGPU is optional, text/2.5D remains complete and VPG stays reference/research only with no GPL code or unverified assets.                                    |
| PD-133 | Every Person uses one persistent profile/career hub with a dense Overview, role-aware depth, contextual actions, attributed opinions and red-to-green rating readability.              | WM-024 follows `person-profile-hub-rules.md`; it includes underlying wrestling ability for non-wrestlers and composes moveset permissions, development and history from their owning systems.                  |
| PD-134 | The complete 168-item game-product baseline is accepted, including its owner clarifications and removal of multiplayer.                                                                | `game-product-rules.md` is the destination contract for future tickets and resolves former PD-107–131 blockers without claiming those capabilities are implemented.                                            |
| PD-135 | Social posts may appear as optional, skippable interstitials between show segments, using only timestamped canonical media events.                                                     | WM-036 generates and owns the posts; WM-013 carries interstitial events; WM-014 and WM-043 present them without changing simulation or forcing a reading pause.                                                |

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
duplicating search engines or exposing private CPU-company records. Accepted PD-134 rules D8 and
E1–E7 own the full Entity Hub and contextual Booking Reference Drawer design while preserving this
search-state contract.

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

### PD-133 person profile and career hub — accepted 12 September 2026; execution defaults locked 13 September 2026

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

The final execution defaults keep current-style Overall contextual, use one combined WM-024.1
delivery, and forbid player-facing “not implemented” placeholders. Unfinished domains stay hidden;
implemented domains use honest in-world empty states.

### PD-132 retro 3D broadcast direction — accepted 12 September 2026; release requirement amended 13 September 2026

The owner approved the [retro broadcast engine rules](retro-broadcast-engine-rules.md): an original
PlayCanvas Engine v2 low-poly viewer integrated through `@playcanvas/react` in WM's existing
React/TypeScript/Vite/Tauri client. WebGL2 is the required baseline and WebGPU is optional. The viewer
is driven by the renderer-neutral event contract and retains a complete text/2.5D fallback. One
canonical WM skeleton, modular models, reproducible Blender/GLB processing, paired attacker/receiver
animations and WM-owned move recipes provide the content path. Virtual Pro Grappler remains
reference/research only; its GPL code, unverified assets, proprietary No Mercy material and unlicensed
likenesses/branding are excluded.

WM-043 begins only after a bounded go/no-go spike can consume stable WM-013 facts. It is a broadcast
viewer, not playable combat. The management game and guaranteed text/2.5D viewer are built first, but
a viable 3D viewer is now a commercial-release requirement: release waits for the capability rather
than silently dropping it. The spike must prove sustained 60 FPS on a documented ordinary-PC baseline
in the real Tauri/WebView2 application. Exact purchased assets, licences and the post-spike production
commitment remain evidence gates inside WM-043 rather than assumptions in current implementation.

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

### PD-119 social-feed direction — accepted 13 September 2026

WM-036 will provide bounded procedural social posts from persistent fan, journalist, worker, company,
celebrity and outlet accounts. Authored compositional writing plus account voice, knowledge, bias,
relationships, region and event facts must produce realistic variation without runtime AI. Workers and
company accounts can be asked to promote, respond, tease, apologise or stay silent; personality,
morale, relationship and media skill affect compliance and consequences. Independent posts, deletion
memory, replies and uncertain rumours are supported. Rules S1–S7 in the complete product baseline
govern outlets, press, rumours, random events and celebrities; exact content counts and balance remain
execution-ticket calibration rather than open product design.

Reporter-generation direction accepted 11 September 2026: custom-database reporters and outlets are
retained when supplied; deterministic world generation fills only the missing media ecosystem for a
new save. Generated reporters receive persistent identities, careers, regions, outlet roles,
credibility/reach, wrestling preferences and a small curated set of visible quirks. A reporter may use
a star scale, ten-point score, letter grade or prose-only review, backed by one normalized assessment
for simulation effects. Individual taste and access can produce legitimate disagreement, but reviews
cannot invent or overwrite official match facts. Exact generation counts, distributions and quirk
catalogue are calibrated and tested by WM-036.

Booking-reaction direction accepted 11 September 2026: reporters may publish articles, short social
posts and replies about any completed public booking unit—matches, promos, angles, debuts, turns,
interferences, title changes, storyline developments and whole shows/events. They may praise, mock or
strongly criticise it in their established voice. Each subject uses an appropriate rubric: match
execution is not reused as angle quality, and a show review is not a simple average of match scores.
Posts must be grounded in observed public facts, respect each reporter's access and knowledge, and
remain bounded and deduplicated. Private plans cannot leak as fact; uncertain information uses the
separate rumour system.

In-show social interstitials accepted 13 September 2026: after a segment commits, WM may surface a
short feed rail between segments with relevant fan, journalist, worker or company posts. The posts
must have an in-world timestamp and provenance, react only to public completed facts or labelled
rumours, and use the same canonical media records in Quick Sim, 2.5D and 3D views. Show-level density,
collapse and skip controls keep the feed useful without interrupting the player's running order.

### PD-131 temporary alumni returns and event rumours — accepted 13 September 2026

The owner approved the [alumni return and rumour rules](alumni-return-and-rumour-rules.md): persistent
alumni/legend history, company-dependent date-bound return agreements, safe role-specific appearances,
custom-database support and fictional pre-event reporting whose claim may later prove accurate,
inaccurate or partial. WM-040 owns legacy status, WM-031 agreements, WM-033 events, WM-028/029
creative use, WM-036 media and WM-042 finance. Rules N1, S5 and U3–U4 lock the destination behavior;
exact eligibility weights, agreement variants, frequency and confidence thresholds remain ticket-level
balance constants.

### PD-134 complete game product baseline — accepted 13 September 2026

The owner accepted all 168 decisions in [the complete game product rules](game-product-rules.md):
124 as proposed, 43 with recorded clarifications and multiplayer explicitly removed. The baseline
covers product identity, onboarding, world generation, interface, search, names, company/audience
behavior, booking, matches, moves, angles, groups, titles, contracts, staff, events, business,
communications, media, governance, history, custom content, presentation, saves and release.

Guidance is an assistance layer rather than a separate simulation mode. Fictional worlds generate
without requiring a seed and include a profiled Realistic size. Player creation includes personality,
qualifications, background and contextual perception. The interface uses coherent management hubs,
global category search, a contextual booking drawer, formal email and informal phone conversations.
Sensitive misconduct is restrained, fictional and evidence-based, never an explicit scene or
entertainment mechanic. Careers are designed to continue indefinitely through scalable simulation
and archival history. A viable WM-043 3D viewer is required before commercial release, while its
spike and 60 FPS gate remain mandatory and the complete text/2.5D viewer remains independent.
PD-135 additionally adds skippable between-segment social interstitials without changing the 168
questionnaire count.

## Resolved product-decision blockers

PD-134 resolves former PD-107–131 at destination level. Implementation tickets still own numeric
balance, catalogues, migration and evidence; they may not reopen the product direction casually.

| Former IDs             | Locked rules                                                  | Primary capability owners              |
| ---------------------- | ------------------------------------------------------------- | -------------------------------------- |
| PD-107, PD-117         | Q1–Q9 finance, distribution, sponsors, production and failure | WM-034, WM-042                         |
| PD-108                 | N1–N7 contracts, promises, morale and sensitive systems       | WM-031                                 |
| PD-109                 | B1, C6 and V1–V5 databases, editor and safe packs             | WM-041                                 |
| PD-110, PD-127, PD-128 | W4 and X1–X12 performance, platform and release               | WM-018, WM-019, WM-043                 |
| PD-111                 | H4 and I1–I8 match formats, rules and authority               | WM-002–005                             |
| PD-113                 | H10–H11 draft recovery, undo and templates                    | WM-009                                 |
| PD-114                 | J1–J7 move taxonomy, repertoire and learning                  | WM-010                                 |
| PD-115                 | B8 and T1 governance, roles and authority                     | WM-030                                 |
| PD-116                 | P1–P7 calendar, events, venues and touring                    | WM-033                                 |
| PD-118                 | R1–R6 office, communication and delegation                    | WM-035                                 |
| PD-119                 | S1–S7 media, press, rumours, events and celebrities           | WM-036                                 |
| PD-120                 | T3–T5 competition, agreements and corporate change            | WM-037                                 |
| PD-121                 | C4 and T2/T6 scalable CPU simulation                          | WM-038                                 |
| PD-122                 | U1–U2 and W1–W2 history and presentation retention            | WM-013, WM-039                         |
| PD-123                 | M1–M6 and U3 awards, rankings and legacy                      | WM-027, WM-040                         |
| PD-124                 | K1–K7 storylines, angles and pre-booking                      | WM-028, WM-029                         |
| PD-125                 | D1–D9 and W1–W5 interface and presentation                    | WM-014, WM-016, WM-017                 |
| PD-126                 | X1–X3 save recovery and long-career policy                    | WM-015                                 |
| PD-129                 | L1–L5 group types and membership                              | WM-026                                 |
| PD-130                 | C4–C5, G1–G7, T2/T6 and U1–U2 world detail and retention      | WM-011, WM-012, WM-038                 |
| PD-131                 | N1, S5 and U3–U4 returns and rumours                          | WM-031, WM-033, WM-036, WM-040, WM-042 |

## Decision record format

When the owner resolves an item, move it to Accepted and record: decision text, date, affected
tickets, alternatives rejected and migration/compatibility consequences. If later evidence changes
it, add a superseding ID rather than rewriting history invisibly.
