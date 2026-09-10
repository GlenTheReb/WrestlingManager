# Wrestling Manager product decision register

This register prevents provisional ideas from becoming accidental mechanics. “Accepted” means the
owner has confirmed the behavior. “Open” means implementation must not invent the answer. Decisions
record product rules; implementation evidence and architecture remain in the SDD and ticket handoffs.

## Accepted decisions

| ID     | Decision                                                                                                                                                                               | Consequence                                                                                                                                                                                                             |
| ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| PD-001 | Wrestlers expose exactly six base stat groups, each an integer from 0 to 100, with explanatory sub-stats.                                                                              | There is no context-free universal rating; the canonical catalogue is PD-008 and the current-Archetype derivation is PD-101.                                                                                            |
| PD-002 | Real-life personal state, kayfabe state and presentation are separate causal layers.                                                                                                   | Turns, angles and dialogue cannot silently rewrite contracts, health or personal relationships.                                                                                                                         |
| PD-003 | The show viewer is text-led and simulation-driven, using scene cards, commentary, crowd state, smart lighting and restrained effects.                                                  | Theme songs, titantrons and full wrestler animation are not planned core features.                                                                                                                                      |
| PD-004 | Native custom databases/scenarios are first-class; portraits are optional and sound/video packs are optional late layers.                                                              | Simulation and saves must work without assets or a network service.                                                                                                                                                     |
| PD-005 | CPU companies use compatible rules and leave inspectable history.                                                                                                                      | Difficulty may change information or assistance, not grant unexplained simulation exceptions.                                                                                                                           |
| PD-006 | Important ratings, reviews, alerts and consequences must expose their causes.                                                                                                          | Decorative totals and unsupported dashboard metrics are rejected.                                                                                                                                                       |
| PD-007 | Competitive games are research references, not specifications to copy.                                                                                                                 | Names, interface layouts, content and rules require an independent WM rationale.                                                                                                                                        |
| PD-008 | The canonical wrestler stat catalogue is the six-group model recorded below.                                                                                                           | Generation, saves, simulation, search and profiles must use these exact concepts rather than inventing UI-local ratings.                                                                                                |
| PD-009 | Every human uses one Person record; wrestler, commentator, manager, booker, producer, trainer and other jobs are roles supported by independent Profession Skills and Role Experience. | On-screen Entertainment can be shared across roles, historical wrestling ability may coexist with a later staff career, and people without evidenced wrestling ability receive no fabricated wrestler stats or Overall. |
| PD-101 | Wrestler group scores, Discipline Fits, Archetypes, current-Archetype Overall and style-development rules use the accepted deterministic model below.                                  | WM-020 may be decomposed for implementation; WM-008 owns later balance calibration without collapsing style, popularity and profession ability together.                                                                |

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
ownership. The accepted 65/20/15 role formula is unchanged. PD-102 still owns scouting discovery;
unknown inputs and suppression of hidden values are required from the outset.

## Open decisions that block specific implementation

| ID     | Required decision                                                                                                               | Blocks                                                               |
| ------ | ------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| PD-102 | Visibility model for known, estimated, hidden and stale wrestler information.                                                   | WM-021, WM-024 and WM-032 scouting interfaces.                       |
| PD-103 | Rule version 1 and identity registries implemented in WM-022; balance remains unproven.                                         | Downstream event producers and effects require their owning systems. |
| PD-104 | Personal relationship categories, bounds, memories, decay and player interaction catalogue.                                     | WM-023.                                                              |
| PD-105 | Product, audience, culture, company-size and regional market taxonomies.                                                        | WM-008, WM-011 and business/world balance.                           |
| PD-106 | Character/gimmick/alignment terminology and rules for public versus private knowledge.                                          | WM-025 and creative systems.                                         |
| PD-107 | Financial difficulty layers, currency display, tax/debt detail and bankruptcy/recovery philosophy.                              | WM-042 balance and interface.                                        |
| PD-108 | Contract clause catalogue, negotiation cadence and acceptable sensitive wellness/drug-testing presentation.                     | WM-031.                                                              |
| PD-109 | First-release database editor scope and compatibility promise for third-party packs.                                            | WM-041 public format.                                                |
| PD-110 | First commercial/release milestone: required world size, content volume and supported career length.                            | Final scope for WM-018–019 and release claims.                       |
| PD-111 | Match-format rules: tag legality, fall/elimination/entry semantics, stipulation composition and quick/advanced booking control. | WM-002–005.                                                          |
| PD-112 | Road-agent authority, advice detail, live-instruction catalogue and player control over sequence/finish recovery.               | WM-006–007.                                                          |
| PD-113 | Draft autosave/recovery, undo checkpoint and reusable-template policy.                                                          | WM-009.                                                              |
| PD-114 | Move taxonomy, naming, repertoire sizes, visibility and learning/proficiency cadence.                                           | WM-010.                                                              |
| PD-115 | Playable company roles, governance permissions, owner-goal families and dismissal/resignation continuation.                     | WM-030.                                                              |
| PD-116 | Calendar granularity, event families, travel/weather depth, pricing and house-show automation.                                  | WM-033.                                                              |
| PD-117 | Broadcast/ratings abstraction, media territories, sponsor sensitivity, merchandise and production depth.                        | WM-034.                                                              |
| PD-118 | Office urgency, notification, snooze and assistant automation authority.                                                        | WM-035.                                                              |
| PD-119 | Outlet/fan taxonomy, press-response design, rumour uncertainty, random-event philosophy and celebrity limits.                   | WM-036.                                                              |
| PD-120 | Agreement, talent-trade, corporate-control and competitive-tactic catalogue and risk model.                                     | WM-037.                                                              |
| PD-121 | CPU-company difficulty philosophy, decision horizon, simulation-detail tiers and lifecycle target rates.                        | WM-038.                                                              |
| PD-122 | Historical event/detail retention, search knowledge/ranking and presentation-stream compatibility.                              | WM-013 and WM-039.                                                   |
| PD-123 | Title prestige, divisions, rankings, tournaments, awards, Top 100 and Hall of Fame criteria.                                    | WM-027 and WM-040.                                                   |
| PD-124 | Storyline state/objectives, angle-beat catalogue, dialogue storage and assisted-versus-manual authority.                        | WM-028–029.                                                          |
| PD-125 | Viewer timing/density/effects, WM visual direction, navigation hierarchy, shortcuts and accessibility defaults.                 | WM-014 and WM-016–017.                                               |
| PD-126 | Save backup/recovery retention, supported upgrade horizon, repair limits and engine-version policy.                             | WM-015.                                                              |
| PD-127 | Supported hardware/world/career performance budgets and representative balance ranges.                                          | WM-018.                                                              |
| PD-128 | Release channel/version/signing and supported Windows/installer policy.                                                         | WM-019.                                                              |
| PD-129 | Permanent group types, overlapping membership, chemistry/loyalty dimensions and manager/authority permissions.                  | WM-026.                                                              |
| PD-130 | World market/era granularity, company-relationship dimensions and long-save event retention.                                    | WM-012.                                                              |

## Decision record format

When the owner resolves an item, move it to Accepted and record: decision text, date, affected
tickets, alternatives rejected and migration/compatibility consequences. If later evidence changes
it, add a superseding ID rather than rewriting history invisibly.
