# Complete game product rules

Status: accepted by the owner on 13 September 2026.

This is the durable product baseline for Wrestling Manager. It records all 168 questionnaire decisions: 124 accepted as proposed, 43 accepted with the clarifications below and multiplayer explicitly removed. S8 is an additional owner-approved media/presentation amendment. These are destination rules, not claims about current implementation. Exact numeric balance, content volume and user-interface layouts still belong in bounded execution tickets and tests.

The rules preserve the causal separation between real life, kayfabe and presentation. Competitive games and real wrestling workflows are research inputs only; WM uses original terminology, interfaces, content and implementation.

## A — Product identity and scope

- **A1 — Primary game identity:** Deep booking and promotion-management simulator; presentation supports management rather than replacing it.
- **A2 — Default database:** WM ships a polished fictional database and can generate random, deeply interconnected fictional worlds without requiring seed entry. Advanced Setup may expose an optional reproducibility seed; real-world databases remain custom content.
- **A3 — Difficulty philosophy:** Change information, owner patience, financial tolerance and assistance—never give CPU companies hidden simulation cheats.
- **A4 — Game modes:** Standard and Sandbox use the same simulation. Guidance is an FM-style configurable onboarding, recommendation and automation layer—not a separate simulation mode.

## B — Opening, settings and player identity

- **B1 — Main menu:** Use a centred game menu: Continue Career, New Career, Load Career, Settings, Database & Editor, Credits and Exit. Database & Editor provides obvious import, create, copy and validation actions; save loading remains under Load Career.
- **B2 — Career terminology:** Replace Save Library with Careers and visual cards showing company, role, date, playtime and last event.
- **B3 — Settings categories:** Display, Interface, Gameplay, Simulation, Accessibility, Audio and Controls.
- **B4 — Global versus career settings:** Display, audio, accessibility and controls are global; difficulty, autosave, simulation detail and assistance belong to each career.
- **B5 — Player creation fields:** Player creation covers identity, age/date of birth, nationality, pronouns, portrait/avatar, biography, professional background, qualifications/licences, experience, editable personality traits and relevant prior relationships.
- **B6 — Player backgrounds:** Backgrounds include Former Wrestler, Creative Writer, Promoter, Business Executive, Coach/Agent, Broadcaster and Custom, with rare generated family or industry connections such as a nepotism route.
- **B7 — Background effects:** Background sets evidence-based starting reputation, knowledge, relationships and qualifications rather than magical permanent bonuses. Every Person, including the player, has context-bound industry/public perceptions such as credibility, respect, intimidation and notoriety; there is no single universal perception score.
- **B8 — Starting roles:** Owner-Booker, Hired Booker, General Manager and Commissioner-style role; unemployed careers can follow later.
- **B9 — Personal onboarding:** Onboarding combines FM-like role and organisation orientation with TEW-like wrestling context: personalised welcome messages, expectations, immediate tasks, advisor introductions and relevant company/world briefings.

## C — Career creation and world setup

- **C1 — Creation modes:** Quick Start, Guided Setup and Advanced Setup create the same valid career format.
- **C2 — Starting choices:** Take over an existing company, work for one in a selected role, or create a new company.
- **C3 — World sizes:** Offer Small, Standard, Large and Realistic world sizes. Realistic is the largest supported density intended to approximate a living real-world industry and carries clear hardware, processing-time and save-size guidance.
- **C4 — Simulation detail:** Use FM-like simulation detail: Standard or Detailed presets plus Advanced Setup controls for which regions, companies and competitions receive full detail; relevance may promote background entities automatically.
- **C5 — Geography:** Every supported region exists; detail changes density and simulation depth, not existence.
- **C6 — Database selection:** Shipped fictional database, installed custom database or editable copy with validation before creation.
- **C7 — Generation preview:** Preview estimated people, companies, staff, relationships, histories, regional coverage, processing load and save size. Generated entities must have coherent identities, careers, relationships and organisational context rather than filler records.
- **C8 — Seed and editing:** Automatic hidden seed with optional advanced override; foundational settings lock after start except audited Sandbox editing.

## D — Interface, navigation and accessibility

- **D1 — Main navigation:** Primary navigation exposes Home/Office, Booking, Roster, Talent/Scouting, Development/Training, Company, Finances and World through a clear hierarchy. Search, Inbox, Calendar and Settings remain globally reachable; specialist functions live in coherent hubs rather than random buttons.
- **D2 — Screen positioning:** Use and visually balance the available window; no unexplained top-left clustering.
- **D3 — Home screen:** Actionable office with assistant priorities, inbox, calendar, next event, company condition and deadlines.
- **D4 — Text selection:** Disable selection in game chrome; keep inputs, reports, dialogue and intentionally copyable text selectable.
- **D5 — Navigation continuity:** Back restores the prior screen, filters, selected entity, tab and scroll position.
- **D6 — Visual direction:** Dense FM-like hierarchy with original wrestling character, icons, portraits, colour and typography.
- **D7 — Colour meaning:** Comparable values use accessible low-to-high colour scales and text. WM-specific colours and original icons communicate categories, status, urgency and actions without relying on colour alone.
- **D8 — Context while working:** One consistent Reference Drawer supplies roster, teams, titles, stories and history.
- **D9 — Help:** Short hover/focus explanations plus a searchable handbook for deeper mechanics.

## E — Roster, Talent Search and Global Search

- **E1 — Company Roster:** Dedicated page for company personnel grouped and filtered by brand, role, status and availability.
- **E2 — Talent Search:** Separate recruitment and world-person discovery workspace.
- **E3 — Global Search entry:** Ctrl+K opens a fast category-aware bar that can expand into a persistent full Search page.
- **E4 — Search categories:** Global Search uses an FM-like large category selector with grouped live results for People, Companies, Teams, Stables, Titles, Storylines, Shows, Events, Venues, Contracts, News and game screens, plus a dedicated persistent page for advanced filtering and sorting.
- **E5 — Search interaction:** Grouped live results, keyboard navigation, recent searches and filters in the expanded page.
- **E6 — Knowledge rules:** Public facts are exact; private contract, medical and creative facts respect player knowledge.
- **E7 — Roster actions:** Inspect, compare, shortlist, talk, book, assign, review contract and open development where permitted.

## F — Names, identities and generated content

- **F1 — Generated ring-name variety:** Use legal-style names, nicknames, mononyms, masked identities and culturally appropriate stage names.
- **F2 — One-word names:** One-word ring names are fully supported and generated where believable, alongside the broad real-world variety seen across major and regional wrestling without forcing every identity into the same naming pattern.
- **F3 — Legal versus ring identity:** Generated wrestlers often—but not always—start with different legal and ring names.
- **F4 — Duplicate public names:** Prevent same-company active duplicates; show company, nationality or authorised legal-name context for global ambiguity.
- **F5 — Custom worker creation:** Full pre-career database editor; in-career creation belongs to audited Sandbox Editor.

## G — Company, product and audience

- **G1 — Product evolution:** Declared product guides the company; repeated delivered booking changes perceived product. Qualified assigned advisors—chosen from appropriate staff by role and skill—explain drift, forecast reactions and recommend adjustments.
- **G2 — Audience strictness:** Preferences are soft reactions; excellent execution can overcome imperfect product fit.
- **G3 — Audience groups:** Data-driven casual, hardcore, family, traditional, youth, regional and other cohorts mixed by company and market.
- **G4 — Fan loyalty:** Loyal fans may attend mediocre weekly shows, while satisfaction affects growth, buzz, merchandise and major-event conversion.
- **G5 — Company dimensions:** Size, reach, prestige, momentum, finances and recognition remain separate.
- **G6 — Brands:** Optional scopes for rosters, titles, shows, staff and identity—not accidental separate companies.
- **G7 — Culture:** Culture emerges from leadership, policies and repeated behaviour and affects morale, recruitment, discipline and risk tolerance. Company scale, market and culture also govern plausible advertising, sponsorship, celebrity collaboration, media, streaming and other revenue opportunities owned by their business systems.

## H — Booking and show creation

- **H1 — Show creation:** Show creation combines the useful depth of wrestling-booking workflows with FM-style clarity: company/brand, show family, date, venue, broadcast, duration, ticketing and production are visible, validated and progressively disclosed.
- **H2 — Running order:** The running order is a central visual card with clear segment blocks, timing, participants, purpose, story/title context, warnings and show flow. Workers, producers/agents and advisors provide contextual comments, concerns and actionable suggestions.
- **H3 — Match duration:** Whole-minute slider plus compact MM:SS field in five-second increments; no decimal half-minute interface.
- **H4 — Quick versus Advanced:** One workspace with progressive disclosure; Quick captures essentials and Advanced reveals structure and contingencies.
- **H5 — Purpose vocabulary:** Use a large but non-overlapping wrestling-language catalogue of structured producer/road-agent directions covering pace, structure, finish, protection, crowd work, risk, storytelling, interference, commercial timing and participant roles, with explanations and optional notes.
- **H6 — Purpose consequences:** Purpose shapes agent structure, worker expectations, crowd interpretation, move selection and report evaluation.
- **H7 — Worker information:** Selection shows availability, fatigue, morale, current stories, relationships and concerns.
- **H8 — Pre-match agreement:** Losses, humiliation, dangerous bumps, protected moves and late changes may require discussion or produce alternatives.
- **H9 — Reference Drawer:** Roster, Teams, Stables, Titles, Stories and previous meetings in one searchable contextual drawer.
- **H10 — Draft behavior:** Immediate local autosave, explicit accepted revisions, crash recovery and card-wide undo/redo.
- **H11 — Templates:** Save segments, structures and cards without silently retaining invalid unavailable worker assignments.

## I — Match formats, rules and simulation

- **I1 — Essential formats:** Singles, tag, trios, multi-person, elimination, battle royal, gauntlet, timed-entry, tournament and steel-cage presentation must be supported before full release. Gender eligibility is a separate configurable rule so every valid format supports women and mixed participation where company rules allow.
- **I2 — Stipulations:** Composable falls, weapons, environment, elimination, time, entry and victory rules instead of hundreds of hardcoded labels.
- **I3 — Result control:** The player chooses winner/result and may specify the exact finish. The assigned producer/agent fills legal unspecified detail, with the quality, suitability and reliability of that assistance determined by their profession skill and context.
- **I4 — Non-victory results:** Time draw, double count-out, double disqualification, stoppage, no contest and other rule-valid outcomes.
- **I5 — Referees and managers:** Referees and managers are assigned people whose skill and context affect execution without arbitrarily overriding booking. Company operations include a dedicated Staff hub and staffing expectations appropriate to company scale.
- **I6 — Simulation authority:** Deterministic Rust simulation owns results and consequences; presentation renders its facts.
- **I7 — Accidents and deviations:** Only through documented safety, fatigue, proficiency and communication causes, with explanations.
- **I8 — Quality measures:** Objective performance, audience response, journalist opinion and business success remain separate.

## J — Moves, sequences and training

- **J1 — Move model:** Move families plus variations, setups, targets, counters, risk and paired animation recipes.
- **J2 — Moveset size:** Broad known repertoire with smaller active signature, finisher and common-use sets.
- **J3 — Proficiency:** Move proficiency is person-specific and developed through training and use. Experience generally makes learning safer and faster, while rare exceptional newcomers may begin unusually capable when their athletic, background and adaptability evidence supports it.
- **J4 — Protected moves:** Protected-move permission is contextual. Asking is an interaction; unauthorised use can affect relationships and provoke grounded media, interview, podcast or social reactions. Execution is usually competent at professional levels, while rare sloppiness and botches follow ability, fatigue, risk and circumstance.
- **J5 — Sequence planning:** Support intuitive full sequence scripting: legal next actions and transitions depend on current participants, position, state and move outcome. Players may author every beat or specify key sequences while skilled agents fill valid transitions and alternatives.
- **J6 — Failed spots:** Produce recovery, substitution or visible breakdown based on skill and context.
- **J7 — Style transitions:** Require time, training, moveset changes and experience with ranged staff forecasts.

## K — Angles, promos and storylines

- **K1 — Angle creator:** The angle creator uses extensible typed beats and roles so players can construct any plausible wrestling scene—dialogue, entrances, interruptions, attacks, reveals, saves, betrayals, crowd work, props, location changes and exits—while simulation evaluates feasibility and consequences.
- **K2 — Assisted versus Manual:** Assisted proposes an editable scene; Manual gives beat-level control; both use one canonical format.
- **K3 — Dialogue:** Authored and template-assisted character-aware suggestions; runtime AI remains optional and nonessential.
- **K4 — Viewing angles:** Angles offer Quick Sim, 2.5D Realtime/Highlights and 3D Realtime/Highlights. The player may switch supported viewing modes or jump to the result during the segment without changing authoritative outcomes.
- **K5 — Storylines:** Storylines are optional creative containers with objectives, people, stakes, milestones and remembered events. Death is a respectful real-life world event; relationship betrayal belongs to the real-life layer when applicable. Serious misconduct is never treated as an entertaining angle or explicit scene.
- **K6 — Story detection:** Advisors may suggest linking repeated conflict, but player approval creates an official storyline.
- **K7 — Pre-booking:** Pre-booking stores flexible future intentions, dependencies, alternatives and planned sequences. Quick Add surfaces them; workers and agents can recommend or semi-automatically build sequences around participant strengths. Only communicated plans become promises.

## L — Teams, stables, managers and authority

- **L1 — Group types:** Tag teams, trios, stables/factions, manager-client groups, alliances and kayfabe authority units.
- **L2 — Overlapping membership:** Allowed where roles do not conflict.
- **L3 — Group qualities:** Chemistry, experience, cohesion, loyalty, momentum and audience recognition remain separate.
- **L4 — Leadership and history:** Formal/informal leaders with succession, conflict, expulsion and reunion history.
- **L5 — Alternate line-ups:** Freebird and alternate combinations use explicit membership and title-eligibility rules.

## M — Championships, divisions and tournaments

- **M1 — Title types:** Singles, tag, trios, division, regional, brand, annual trophy and custom championships.
- **M2 — Prestige:** Contextual lineage, holder quality, booking consistency, match importance and audience recognition.
- **M3 — Rankings:** Optional per company/division and configurable as strict, advisory or disabled.
- **M4 — Tournaments:** Brackets, round robin, leagues, cups and custom stages with scheduling and tie-breakers.
- **M5 — Vacancies and interim titles:** Explicit dated states and reasons preserved in lineage.
- **M6 — Awards and Hall of Fame:** Awards and Hall of Fame selections use explainable criteria and keep company, regional, outlet and global honours distinct. A person may hold honours from multiple organisations and markets.

## N — Contracts, promises and morale

- **N1 — Negotiation:** Negotiations are asynchronous offers and counters with deadlines, leverage, priorities, agents and relevant meetings. Plausible leaks, strategic return rumours and public reception emerge from access, intent and evidence rather than exposing private plans as fact.
- **N2 — Contract forms:** Written, per-appearance, handshake, exclusive, non-exclusive, developmental, loan/excursion and temporary return.
- **N3 — Clauses:** Wages, bonuses, merchandise, creative control, exclusivity, options, release, outside work and booking protections.
- **N4 — Promises:** Explicit subject, scope, deadline and evidence for pushes, titles, roles, time off, training or creative direction.
- **N5 — Morale:** Explained history driven by booking, communication, money, relationships, workload, promises, safety and culture.
- **N6 — Sensitive systems:** Sensitive systems are fictional, configurable and evidence-based. Serious misconduct or safeguarding allegations may exist as restrained organisational risks with provenance, uncertainty, investigation, legal/media and employment consequences; WM does not recreate real allegations, depict abuse or turn sexual violence into entertainment.
- **N7 — Absence and retirement:** Injury, leave, suspension, outside work, partial retirement and retirement are distinct; plausible returns remain possible.

## O — Staff, training, scouting and development

- **O1 — Staff roles:** Staffing covers the real operational roles useful to play: producers/agents, coaches, scouts, doctors, physios, creative staff, broadcasters, referees and business, legal, media and production specialists. Exact roles are consolidated where responsibilities overlap and scale with the company.
- **O2 — Workload:** Staff cannot cover unlimited people or shows; overload reduces attention and forecast reliability.
- **O3 — Training:** Limited simultaneous focuses with facilities, coaches, recovery and diminishing returns.
- **O4 — Potential:** Potential is a hidden dynamic range influenced by body, background, adaptability, age, coaching, opportunity, injury and major life events. It can improve or decline and may be sharply reduced, but is not a fixed destiny score.
- **O5 — Scouting:** Public demonstrated ability is exact; potential, private personality, health and intent need appropriate knowledge.
- **O6 — Academies and newgens:** Development includes schools, try-outs, trainees, academies, call-ups, excursions, failures, staff conversion and optional competition/reality-series formats similar in purpose to televised try-out programmes.

## P — Calendar, events, venues and touring

- **P1 — Time progression:** Daily calendar; detailed time only for production, deadlines and live events.
- **P2 — Event families:** The calendar covers weekly TV, streaming, premium, specials, house, developmental, tours, press events and training days. Scheduling views show nearby and simultaneous competing broadcasts, with advisors and visual cues explaining likely audience, travel and brand-positioning tradeoffs.
- **P3 — Venues:** Geography, capacity, configurations, cost, availability, ownership, production limits and local history.
- **P4 — Travel and weather:** Meaningful scheduling, cost, fatigue and cancellation risk without transport micromanagement.
- **P5 — Ticket pricing:** Recommended range plus manual override; attendance uses demand, card, market, venue and value.
- **P6 — House shows:** Auto-run, agent-directed or lightly playable; affect experience, chemistry, fatigue, local popularity and finances.
- **P7 — Conflict checking:** Explain contract, venue, travel, broadcast and worker conflicts before confirmation.

## Q — Broadcasting, production, sponsors and finance

- **Q1 — Media deals:** TV, streaming, premium-event and regional distribution with territories, slots, reach, revenue and obligations.
- **Q2 — Ratings:** Audience size, share, retention and demographic response rather than one unexplained total.
- **Q3 — Sponsors and advertisers:** Sponsors and advertisers are persistent fictional organisations with preferences, targets, sensitivities, obligations and trust; custom databases may define their own brands, territories, deals and assets.
- **Q4 — Merchandise:** Worker, team, title and company products driven by popularity, reach, momentum, design and licensing.
- **Q5 — Production:** Lighting, staging, commentary, cameras, staffing and reliability with cost and expectation tradeoffs.
- **Q6 — Finance complexity:** Auditable accounting underneath; Standard summaries and Advanced budgets/forecasts.
- **Q7 — Currency:** Company-local display, configurable preferred currency and recorded historical exchange rates.
- **Q8 — Taxes and legal detail:** Abstract regional operating costs initially—not a tax-accounting simulator.
- **Q9 — Financial failure:** Financial failure progresses through warnings, cost-cutting, financing, administration and possible closure. Closed promotions remain historically searchable and may be legitimately revived; Sandbox may disable terminal failure.

## R — Office, inbox and assistant

- **R1 — Inbox categories:** Owner/Board, Talent, Creative, Contracts, Medical, Events, Finance, Media and World.
- **R2 — Message design:** Formal email and an informal phone/text conversation screen are separate interfaces over the same canonical communication history. Messages show sender, reason, evidence, urgency, deadline and actions without flavour spam.
- **R3 — Assistant authority:** Recommend and prepare; no major irreversible decisions without delegated authority.
- **R4 — Delegation:** FM-like delegation allows policy boundaries per system, person and urgency, with responsible staff, review cadence, exception alerts and the ability to reclaim control.
- **R5 — Diary:** Events, deadlines, birthdays, contracts, promises, media obligations and follow-ups.
- **R6 — Personalised writing:** Deterministic authored structures using sender personality, relationship, role and facts.

## S — Media, social networks and random events

- **S1 — Media ecosystem:** Persistent fictional journalists, critics, outlets, fan communities, workers, companies and celebrities.
- **S2 — Reporter differences:** Region, access, credibility, reach, preferences, quirks and rating format.
- **S3 — Social generation:** Bounded fact-based authored composition with distinct voices, replies and deduplication.
- **S4 — Press conferences:** Contextual questions and tone/content responses with relationship, morale, sponsor and story consequences.
- **S5 — Rumours:** Confidence-labelled claims with provenance; private plans never appear as confirmed facts.
- **S6 — Random events:** Eligibility, warning where appropriate, choices, cooldowns and consequences—no arbitrary punishment lottery.
- **S7 — Celebrities:** Restricted world Persons with fame, availability, skills and roles; fame does not grant wrestling competence.
- **S8 — In-show social interstitials:** Between show segments, the viewer may present a short, timestamped social-media rail or interstitial containing relevant fan, journalist, worker and company posts reacting to completed segments or discussing credible upcoming rumours. Posts must already exist in the canonical media timeline, respect knowledge/provenance and never reveal private plans as fact. The player can choose a show-level density/default, collapse or skip an interstitial, and continue immediately; Quick Sim, 2.5D and 3D use the same generated posts.

## T — Ownership, competition and CPU companies

- **T1 — Governance:** Owners, boards, executives, booking teams, permissions, budgets, trust and measurable goals.
- **T2 — CPU rules:** Same economic, contract, booking and development rules with scalable decision detail.
- **T3 — Competition:** Competition includes ratings battles, counter-programming, bidding, talent movement, public criticism, dirty tricks and espionage where plausible. Costs, discovery risk, legality, reputation and relationship consequences prevent consequence-free sabotage.
- **T4 — Agreements:** Alliances, working agreements, loans, trades, co-promotion and developmental/parent/sister structures.
- **T5 — Corporate change:** Investment, merger, acquisition, takeover, closure and new formation with preserved history.
- **T6 — Simulation detail:** Full for player/relevant companies, condensed elsewhere, promoted automatically when relevance grows.

## U — History, records, alumni and world knowledge

- **U1 — Permanent records:** Results, title changes, contracts, ownership and major career events remain permanently summarised.
- **U2 — Detailed retention:** Player and important shows retain detailed events; background shows retain compact summaries unless configured otherwise.
- **U3 — Alumni and legends:** Alumni and legend status is derived independently per company from real history and relationships. The same person may be alumni or a legend for multiple promotions at once.
- **U4 — Temporary returns:** One appearance, short run, non-wrestling role or conditional match based on health, relationship, money and fit.
- **U5 — Historical corrections:** Auditable editor corrections that never silently rewrite unrelated lineage.

## V — Custom databases and content packs

- **V1 — Editor scope:** Workers, staff, companies, relationships, titles, teams, events, products, moves, venues, journalists and history.
- **V2 — In-career editor:** Complete Sandbox Editor with audit history and Edited Career marker.
- **V3 — Pack order:** Database/scenario and portrait first; sound/video later; 3D packs only after WM-043 proves the format.
- **V4 — Package safety:** Data-only packages, stable IDs, versioned manifests, validation preview and no executable scripts.
- **V5 — Missing assets:** Clean fallbacks; optional assets never determine simulation correctness.

## W — Show presentation and 3D

- **W1 — Per-segment viewing:** Quick Sim, 2.5D Realtime, 2.5D Extended Highlights, 3D Realtime and 3D Extended Highlights where available.
- **W2 — Default viewer:** Complete guaranteed 2.5D/text viewer with global default and per-segment override.
- **W3 — 3D style:** Original low-poly, readable, animation-driven presentation inspired by N64-era clarity.
- **W4 — 3D release gate:** A viable PlayCanvas 3D viewer is required for commercial release, so release waits for it rather than silently dropping it. WM-043 still begins with a strict go/no-go feasibility spike proving paired-animation workflow and 60 FPS; the complete text/2.5D viewer remains mandatory and saves never depend on 3D.
- **W5 — Audio:** UI sounds, crowd ambience and effects eventually; licensed themes, recorded commentary and titantrons are not core.

## X — Saves, performance and release

- **X1 — Autosaves:** Rotating autosaves plus manual named saves; critical mutations remain transactional.
- **X2 — Recovery:** Preserve originals, verify backups and explain corruption or incompatibility.
- **X3 — Career length target:** Careers are designed to continue indefinitely with realistic ageing, retirement, new generations, organisational change and world history. Tiered simulation, archival summaries and lifecycle controls must prevent uncontrolled growth or major slowdown.
- **X4 — Standard world target:** Roughly 2,500–5,000 active people and 75–150 companies; Large expands after profiling.
- **X5 — Performance targets:** UI under 100 ms, normal searches under 200 ms, Standard day advancement usually under two seconds and presentation at 60 FPS on baseline hardware.
- **X6 — Commercial milestone:** Do not sell until the coherent management week, companies, contracts, office, finance, events, rivals, safe saves and the release-gated WM-043 3D viewer meet their acceptance thresholds.
- **X7 — Initial platform:** Windows 10/11 desktop first; Linux/macOS after stability and measured demand.
- **X8 — Distribution:** Steam first with optional direct build later and no always-online requirement.
- **X9 — Monetisation:** One-time purchase; no currencies, loot boxes, energy or paid statistical advantages.
- **X10 — Multiplayer:** Multiplayer is out of scope. Do not spend current design or engineering effort preserving a hypothetical multiplayer path.
- **X11 — Localisation:** English first, but player-facing text and content formats are localisation-ready.
- **X12 — Telemetry:** Completely opt-in crash/performance reporting; offline play remains complete.

## Implementation ownership

The capability map in [model-task-plan.md](model-task-plan.md) and the packets under [tickets](tickets/INDEX.md) own delivery. Before implementation, the selected execution ticket must cite the relevant rule IDs from this file, resolve only remaining balance constants needed for that slice, preserve save compatibility and record verification in the SDD. This baseline resolves the former product blockers PD-107 through PD-131 at destination level; it does not authorise implementation, branch creation, commits or publishing.
