# Wrestling Manager checkpoint — 9 September 2026

Latest increment completed after the owner resumed implementation. No routing, delegation,
commits, pushes or publication. The earlier foundation record is retained below for context.

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

The [future task/model/thinking table](model-task-plan.md) is a manual recommendation; the
current direct Astra High instruction remains in effect. No automatic routing enabled.

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
full social ecosystem, runtime mod importer, portraits or PixiJS ring presentation. No golden
snapshot, installer, long-save soak or gameplay balancing evidence yet. Search, local undo and
drag/drop compile but do not yet have dedicated interaction regression tests.

## Files and launch

Run `pnpm.cmd dev` from the project root, or open `target/debug/wm-desktop.exe` after the debug
build. Real player saves are under `%APPDATA%\com.wrestlingmanager.game\saves`. Source, docs and
binary remain local; no Git checkpoint commit has been created.

## SDD entry points and implementation record

Read the ignored root SDD.md first. Exact heading references for this checkpoint:

- Current checkpoint and execution pointer: SDD.md:3
- Confirmed direction: SDD.md:38; moveset requirements: SDD.md:86
- Implemented player loop: SDD.md:96
- Architecture and file ownership: SDD.md:139
- Commands and data contracts: SDD.md:165
- Content and world generation: SDD.md:179
- Simulation contract: SDD.md:191
- Persistence and upgrade safety: SDD.md:229
- Desktop layout and editing safety: SDD.md:256
- Verification at checkpoint: SDD.md:277
- Known limitations: SDD.md:294
- Resume order: SDD.md:307
- Latest WM/news implementation and verification: SDD.md:359
- Manual model selection plan: SDD.md:392

These sections supersede the foundation-era implemented-status descriptions in older docs.
The appended final-evidence section records the actual native result. Developer concepts,
three interview questions/answers and command explanations are in developer-guide.md under
“Playable-loop checkpoint: engineering concepts” and its following sections.
