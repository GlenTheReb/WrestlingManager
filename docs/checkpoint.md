# Wrestling Manager checkpoint — 13 September 2026

Current handoff: [WM-021.1 — Current-data Talent Search foundation](tickets/WM-021.1.md) is
merged through PR #5 at `28f6a26`. [WM-025 — Real-life and kayfabe separation](tickets/WM-025.md)
is implemented, review-fixed and merged through PR #6 at `61d2ec1`. Owner-authorised
[WM-024.1 — Person profile and career hub](tickets/WM-024.1.md) is implemented, review-fixed and
verified locally on `feature/wm-024-person-profile-hub`; an explicit commit remains.
WM-001–043 have context packets linked by
[the capability index](tickets/INDEX.md). [Playable milestones](milestones.md) govern vertical slices;
[the execution guide](tickets/EXECUTION-GUIDE.md), [template](tickets/TICKET-TEMPLATE.md) and
[decision register](product-decisions.md) govern model-ready implementation handoffs.

WM-023 adds directional affinity/respect/trust/tension, dated memories, management rapport, four
daily attention points, interaction prerequisites/cooldowns and short authored contextual replies.
State and outcomes are persisted in schema 8 / engine 0.7.0. The worker profile exposes the system,
and completed conversations create private People-inbox records. Personal state remains separate
from match chemistry, company sentiment, contracts and kayfabe.

## Latest implementation

- A compact quick profile now expands into one full Person hub without unmounting Talent Search,
  News or the originating workspace. Overview, Attributes, Character, Career, Matches & Appearances,
  Relationships, Contract, Development and Media compose only existing canonical facts.
- WorkerProfile now returns current company context, show-aware bounded appearance rows, one next
  booking and up to eight worker-linked news items. It does not add save state or duplicate history.
- Match history exposes show/date, recorded participant names, outcome, duration, six objective
  performance dimensions and reasons. Current-style Overall stays contextual and separate from
  popularity; future domains remain hidden instead of appearing as fake working controls.
- Local SVG icons, compact/full layouts, accessible value labels and responsive scrolling give the
  profile a clearer sports-management information hierarchy without an icon or network dependency.

- Schema 8 adds canonical Person, Character, dated company/brand Tenure, public alias, gimmick,
  alignment intent, identity-knowledge, complete audience-response evidence and revisioned change-plan
  records. Current generated names safely seed both legal and initial ring identity on migration.
- Talent Search and booking labels lead with the active ring name; permitted former aliases are
  searchable with explicit match reasons, while private aliases and legal names are excluded.
- The profile adds a Character tab with a wrestling-language creative brief, worker response,
  readiness/risk/advice, explicit public launch, dated identity history and character-only
  retirement/revival. No character action mutates permanent wrestling ratings or Person retirement.
- Local SVG icons, semantic chips, a stronger profile header and an accessible weak-red-to-elite-green
  rating scale improve information hierarchy without adding an icon dependency. WM-016 retains the full
  application-wide visual pass and WM-024 the complete FM-style career hub.

- Dedicated Talent Search replaces the shallow worker bar and capped Ctrl+K overlay without claiming
  to be the future company Roster or Global Search. SQLite now performs filtering, sorting, counting
  and 25/50/100-row pagination; Rust hydrates only the returned page.
- A compact filter-icon toggles the grouped left drawer. Choice filters are searchable, support
  explicit include/exclude states and appear as individually removable chips. Training lineage is an
  advanced background criterion rather than a primary search control.
- Configurable columns, four-worker comparison, complete saved views, multiple named shortlists and
  one blacklist are integrated. Working state survives navigation; durable state survives restart.
- Schema 8 / engine 0.7.0 owns the new identity model; schema 7's rebuildable public discovery
  projection and saved-list storage remain intact.
  Canonical workers remain authoritative and every worker write refreshes the projection.
- Regional popularity, contracts, companies, history and other entities remain with their domain
  owners; the Finder does not fabricate fields for systems that do not exist.

- Relationship generation is deterministic and directional without perturbing the existing world
  random stream. Stable ID-based neutral baselines are lazy; only changed or remembered links are
  stored, and at most two shared-school memories per worker give some starting links factual context.
- Interactions are validated server-side, protected by request IDs and revisions, and resolved in
  one SQLite transaction with exact relationship/morale/confidence deltas, history, domain-event and
  inbox effects. Colleague search is server-paged and bounded.
- Only tension decays automatically on the first and fifteenth; positive relationship dimensions
  change only through causal events. The internal event boundary supports later gameplay producers.
- The company/product/audience rules requested alongside this work are documented for future WM-011;
  they are not incorrectly presented as implemented gameplay.

WM-024.1 local verification: 90 Rust and 21 React tests pass, with warning-denying Clippy,
generated-contract regeneration, formatting, ESLint, strict TypeScript, production web build, native
debug build and isolated desktop smoke. The 1920×1080 quick, Overview, Character and Attributes
screenshots were inspected; navigation, scrolling and primary actions remain usable.

Final WM-025.1 review verification: 90 Rust and 17 React tests pass, with warning-denying Clippy,
generated-contract drift, formatting, ESLint, strict TypeScript, production web build, native debug
build and isolated desktop smoke. Review fixes enforce company-context identity selection, one active
identity per person/company context, revision-safe character status changes, bounded creative input,
complete proposed-plan display, refreshed form state and the red-to-green rating scale. The loaded
1920×1080 Character and planner screenshots were inspected without clipping or a blocked action.

Final WM-021.1 review verification: 83 Rust and 16 React tests pass, along with formatting, ESLint, strict
TypeScript, production web build, generated-contract drift, warning-denying Clippy and native debug
build. The isolated native smoke passed and its loaded 1920×1080 Talent Search screenshot was
inspected. See the root SDD and [WM-021.1](tickets/WM-021.1.md) for exact coverage and boundaries.

The [future task/model/thinking table](model-task-plan.md) is a manual recommendation. This planning
pass ran directly on GPT-5.6 Sol at High, without delegation. Sol High remains the normal complex
implementation ceiling; Astra requires a concrete unresolved blocker and owner approval.

## Latest planning direction

PD-102 is accepted in [the search and discovery rules](search-and-discovery-rules.md), and PD-106 is
accepted in the [person, character and presentation rules](character-and-presentation-rules.md).
[WM-021.1](tickets/WM-021.1.md) implements and validates the persistent current-data Talent Search
foundation, saved views/lists, comparison and restoration. PD-106 is now decision-complete, including
rename timing, contextual gimmick effects, multiple/secret identities, character retirement and
separate alignment intent/audience perception. The combined [WM-025.1 execution ticket](tickets/WM-025.1.md)
is now the merged implementation record from PR #6 at `61d2ec1`. PD-133 is accepted and its final
defaults are locked in the [Person profile and career hub rules](person-profile-hub-rules.md).
[WM-024.1](tickets/WM-024.1.md) is the verified local current-data hub. Later domains register their
own searchable/profile facts; accepted PD-134 rules D8 and E1–E7 own the complete Entity Hub and
contextual Booking Reference Drawer design.

PD-132 accepts a spike-gated [retro 3D broadcast direction](retro-broadcast-engine-rules.md)
and [WM-043 capability](tickets/WM-043.md): an original PlayCanvas Engine v2 low-poly viewer through
`@playcanvas/react`, with WebGL2 required and WebGPU optional. A canonical WM skeleton, verified
licensed/original modular assets, paired attacker/receiver animations and WM-owned move recipes are
driven only by WM-013 events. The text/2.5D viewer remains complete. A one-arena/two-wrestler/five-move
feasibility spike must prove 60 FPS on documented ordinary-PC hardware before longer work. PD-134
makes the proven WM-043 exit a commercial-release requirement, but it does not change the current
next ticket or block earlier management milestones.

PD-134 accepts the complete [game product baseline](game-product-rules.md): all 168 questionnaire
decisions are durable destination rules, former PD-107–131 blockers are resolved, multiplayer is out
of scope and exact balance/content constants remain with their execution tickets.

PD-112 accepts [pre-match and live match direction](live-match-direction-rules.md): most workers
cooperate, while contextual safety, creative, trust, morale, status, relationship or personality
causes can produce concern or refusal. Instructions reach wrestlers through sequence-derived referee
opportunities, and accepted changes deterministically rebuild only the unsimulated future. Quick Sim
uses contingencies; Extended Highlights and Realtime 2.5D/3D can expose live decisions.

All future player-facing tickets must follow the
[interface and integration rules](interface-and-integration-rules.md): inspect the relevant TEW IX
counterpart and focused current wrestling workflow, improve it for the player's actual task, preserve
navigation state and connect canonical people/company/show/history systems. The latest play review
also records sortable roster columns in WM-021, a transcript-style relationship conversation
follow-up, detailed match history in WM-024, contextual news imagery in WM-036 and richer venue-aware
show reports across WM-008/013/033.

PD-131 adds company-dependent temporary alumni/legend returns and pre-event fictional media rumours:
WM-040 owns legacy status, WM-031 the dated return agreement, WM-033 the event, WM-028/029 creative
use, WM-036 reporting and WM-042 financial impact. A rumour is a provenance-labelled claim, never an
automatic leak or forced surprise.

- WM-002–010 deepen team and multi-person matches, rules, road-agent logic, move/finish structure,
  crowd consequences, booking tools and personal movesets.
- WM-020 supplies the six visible 0–100 stat groups; WM-022 supplies personal identity/personality;
  WM-023 supplies relationships/interactions. WM-021 supplies discovery infrastructure; WM-025 now
  supplies Person/Character/ring identity before WM-024 completes the FM-style profile hub.
- WM-026–029 add teams/stables, championships, storylines,
  pre-booking and a structured assisted/manual angle director.
- WM-011–012 and WM-030–042 build company governance, auditable finance, contracts, training,
  events/house shows, broadcast, office/inbox, journalists/social media, rival AI, inspectable CPU
  shows, corporate networks and historical honours.
- WM-013–019 and WM-041 deliver a renderer-neutral text/2.5D show viewer, desktop coherence,
  compatibility/performance evidence, native database/scenario tools and final release packaging.
  Spike-gated WM-043 later adds the original retro 3D broadcast viewer after its go/no-go spike.

The numbered roadmap entries are capability briefs, not multi-week implementation branches. Each
must be decomposed into a two-to-five-day execution ticket with exact inspected code ownership,
intentionality answers, cross-system impact, acceptance, verification and a stop condition. M1 now
targets a repeatable management week from thin people/company/contract/finance/profile/office slices
alongside the existing singles loop; match/team and presentation depth then advance together.

Theme songs and titantrons remain outside the direction. Smart lighting/effects, text, scene cards,
portraits, commentary and crowd state carry the guaranteed presentation. Full wrestler animation is
not an early management-milestone dependency, but the low-poly WM-043 viewer is a commercial-release
gate. Portrait packs are optional; sound/video and eventual 3D content packs remain optional. PD-134
locks the destination product catalogues; execution tickets still calibrate exact constants and content.

## Earlier foundation inventory (historical)

| Area       | Current implementation                                                                                                                                           |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Desktop    | Fullscreen native Tauri app; F11 window toggle; dense persistent left navigation; profile/menu overlays; Ctrl+K people/promotion/show search                     |
| Careers    | Real SQLite creation/load, exact 64-bit seeds, verified v1 upgrade backup, schema 2                                                                              |
| People     | 40 seeded wrestlers, biographies, age/size/style, attributes/condition, actual moves/proficiency, three agents                                                   |
| Booking    | Three work areas; match/angle running order; edit/remove/reorder/drag/drop; brief/sequence tabs; templates; local undo/redo; directional warnings                |
| Road agent | Chooses legal repertoire spots around fixed user instructions, preserving winner and manual beats; attributes influence execution and live delivery              |
| Match/show | One-second Rust transitions; fatigue, execution mistakes/improvisation, injuries, chemistry and continuous crowd cohorts; booked results stay locked             |
| Watching   | Full, quick, highlights, instant; 1x/2x/4x/8x; commentary/stamina/crowd; delayed agent/referee instructions; persisted live restart/resume                       |
| Aftermath  | Separate report dimensions, worker development/wear, history, chemistry, popularity/momentum, show gate/costs, result-driven media, next show and daily Continue |

## Earlier foundation verification (historical)

25 Rust tests and 6 frontend tests passed. TypeScript/production frontend/native debug builds,
ESLint, Clippy and generated-contract checks passed. Native smoke completed creation, roster
moves, agent booking, match and angle, live instructions, process restart/resume, locked result,
reports and Continue, plus a 1280x720 layout check. Normal and compact screenshots inspected.

This verifies a connected prototype, not depth, realism or long-save balance. The latest smoke
uses .artifacts/desktop-smoke-SDvNnE; it never touches normal player careers.

## Earlier foundation limits (news/exit changes superseded above)

Only singles/no-DQ matches and simple angles. No granular phase-dragging/finish engine, full
agent structure intelligence, persistent planner drafts, card-wide undo, or custom template
library. Unsaved planner edits disappear when leaving the editor. The feed shows the latest
80 events; archival replay/paging is absent. Fixed UWF creation, weekly schedule and simple
finances are placeholders for richer management systems, not complete versions of them.

No contracts negotiation, custom company creation, scouting/development centres, evolving
newgen life cycles, rival AI, titles/storylines, full calendar, inbox, production/broadcasting,
full social ecosystem, runtime mod importer, portraits or PixiJS ring presentation. WM-020 now has
an engine-0.3 golden snapshot; no installer, long-save soak or gameplay balancing evidence yet. Search, local undo and
drag/drop compile but do not yet have dedicated interaction regression tests.

## Files and launch

Run `pnpm.cmd dev` from the project root, or open `target/debug/wm-desktop.exe` after the debug
build. Real player saves are under `%APPDATA%\com.wrestlingmanager.game\saves`. WM-021.1 and its
roadmap documentation are merged; generated `output/` and `tmp/` artifacts remain local.

## SDD entry points and implementation record

Read the ignored root SDD.md first. Exact heading references for this checkpoint:

- Current checkpoint and execution pointer: SDD.md:3
- Confirmed direction: SDD.md:103; moveset requirements are within that section
- Implemented player loop: SDD.md:161
- Architecture and file ownership: SDD.md:207
- Commands and data contracts: SDD.md:244
- Content and world generation: SDD.md:268
- Simulation contract: SDD.md:285
- Persistence and upgrade safety: SDD.md:332
- Desktop layout and editing safety: SDD.md:384
- Verification at checkpoint: SDD.md:433
- Known limitations: SDD.md:488
- Resume order: SDD.md:516
- WM identity and news increment: SDD.md:576
- Manual model selection plan: SDD.md:609
- WM-001 match-rule core: SDD.md:618
- Planned wrestler depth backlog: SDD.md:691
- PD-102 search/discovery design: SDD.md:739
- PD-106 Person/Character design: SDD.md:762
- PD-133 Person profile design: SDD.md:789
- PD-132 release-gated retro 3D design: SDD.md:789
- PD-134 complete game product baseline: SDD.md:833
- PD-135 between-segment social interstitials: SDD.md:850
- Complete planned product architecture: SDD.md:850
- Model-ready planning contract: SDD.md:926
- WM-022 combined implementation: SDD.md:980
- WM-023 combined implementation: SDD.md:1028
- PD-112 match planning and live direction: SDD.md:1072

These sections supersede the foundation-era implemented-status descriptions in older docs.
The appended final-evidence section records the actual native result. Developer concepts,
three interview questions/answers and command explanations are in developer-guide.md under
“Playable-loop checkpoint: engineering concepts” and its following sections.
