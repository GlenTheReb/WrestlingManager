# Wrestling Manager checkpoint — 9 September 2026

Current handoff: [WM-020 — Wrestler stat hierarchy](tickets/WM-020.md) is implemented and verified,
committed as `2677ffd` and pushed on `feature/wm-020-wrestler-rating-system`. WM-001–042 have context packets linked by
[the capability index](tickets/INDEX.md). [Playable milestones](milestones.md) govern vertical slices;
[the execution guide](tickets/EXECUTION-GUIDE.md), [template](tickets/TICKET-TEMPLATE.md) and
[decision register](product-decisions.md) govern model-ready implementation handoffs.

The documentation plan is committed as `67a0c0c` and pushed on the same branch. The earlier
foundation record is retained below for context.

WM-020 adds the canonical six-group/37-sub-stat 0–100 system, nine Discipline Fits, Approach,
Specialisations, curated Archetypes, current-style Overall, deterministic content conversion,
schema-4 migration including paused shows, explicit simulator adapters, generated contracts and
rating inspection in roster/search/profile screens. Full roster filtering, full FM-style profiles,
training operations and profession ratings remain in WM-021/024/032.

## Latest implementation

- WM branding replaces UWF in the app identity; the promotion keeps its company identity.
- Exit game closes the native application, waits for pending mutations and protects dirty
  planner instructions with a Return/Discard choice. Navigation and Alt+F4 guards remain future.
- News & inbox provides real results, business and medical articles, unread badge, categories,
  saved read state and links to the exact show/profile. F6 opens it.
- Save schema 3 adds news with verified v1/v2 backups and preserves paused matches. Old
  completed shows are backfilled from real reports, without resimulation or duplicate articles.
- Selected panels and controls are rounded; news uses an editorial layout. A clickable match
  timeline locates the exact beat input. No new match families or 2.5D renderer in this increment.

Latest verification: 25 Rust tests and 10 frontend tests passed, plus type/lint/Clippy/contract
checks and the native debug build. Native smoke passed news read/badges/report links, timeline
focus, actual Exit and the original match/show workflow. Isolated save:
`.artifacts/desktop-smoke-4rdKfO`. News (1920x1080) and booking (1280x720) screenshots inspected.

The [future task/model/thinking table](model-task-plan.md) is a manual recommendation. This planning
pass ran directly on GPT-5.6 Sol at High, without delegation. Sol High remains the normal complex
implementation ceiling; Astra requires a concrete unresolved blocker and owner approval.

## Latest planning direction

- WM-002–010 deepen team and multi-person matches, rules, road-agent logic, move/finish structure,
  crowd consequences, booking tools and personal movesets.
- WM-020 now supplies exactly six visible 0–100 stat groups and the approved style model. WM-021–024
  still own rich identity/relationships, TEW-scale discovery and the FM-style career/profile hub.
- WM-025–029 separate real life from kayfabe and add teams/stables, championships, storylines,
  pre-booking and a structured assisted/manual angle director.
- WM-011–012 and WM-030–042 build company governance, auditable finance, contracts, training,
  events/house shows, broadcast, office/inbox, journalists/social media, rival AI, inspectable CPU
  shows, corporate networks and historical honours.
- WM-013–019 and WM-041 deliver a renderer-neutral text-led show viewer, desktop coherence,
  compatibility/performance evidence, native database/scenario tools and final release packaging.

The numbered roadmap entries are capability briefs, not multi-week implementation branches. Each
must be decomposed into a two-to-five-day execution ticket with exact inspected code ownership,
intentionality answers, cross-system impact, acceptance, verification and a stop condition. M1 now
targets a repeatable management week from thin people/company/contract/finance/profile/office slices
alongside the existing singles loop; match/team and presentation depth then advance together.

Theme songs, titantrons and full wrestler animation are deliberately outside the current direction.
Smart lighting/effects, text, scene cards, portraits, commentary and crowd state carry presentation.
Portrait packs are optional; sound/video packs come last and remain optional. Product catalogues such
as stat names/weights and exact relationship rules still require owner approval inside their ticket.

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
build. Real player saves are under `%APPDATA%\com.wrestlingmanager.game\saves`. The WM-020 code
and roadmap documentation are committed and pushed; generated `output/` and `tmp/` artifacts remain local.

## SDD entry points and implementation record

Read the ignored root SDD.md first. Exact heading references for this checkpoint:

- Current checkpoint and execution pointer: SDD.md:3
- Confirmed direction: SDD.md:66; moveset requirements: SDD.md:114
- Implemented player loop: SDD.md:123
- Architecture and file ownership: SDD.md:166
- Commands and data contracts: SDD.md:192
- Content and world generation: SDD.md:206
- Simulation contract: SDD.md:221
- Persistence and upgrade safety: SDD.md:268
- Desktop layout and editing safety: SDD.md:300
- Verification at checkpoint: SDD.md:326
- Known limitations: SDD.md:344
- Resume order: SDD.md:358
- Latest WM/news implementation and verification: SDD.md:411
- Manual model selection plan: SDD.md:444
- WM-001 match-rule core: SDD.md:453
- Planned wrestler depth backlog: SDD.md:526
- Complete planned product architecture: SDD.md:555
- Model-ready planning contract: SDD.md:605

These sections supersede the foundation-era implemented-status descriptions in older docs.
The appended final-evidence section records the actual native result. Developer concepts,
three interview questions/answers and command explanations are in developer-guide.md under
“Playable-loop checkpoint: engineering concepts” and its following sections.
