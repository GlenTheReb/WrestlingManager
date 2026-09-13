# Delivery milestones and acceptance criteria

## Milestone 0: desktop walking skeleton — current session

- Launch a Windows Tauri application with a React interface.
- Create a named local career with a validated full-width unsigned 64-bit seed.
- Store UWF, the opening date, cash, schema/engine metadata and a creation event in SQLite.
- Reopen that career after closing the process; query an overview through typed commands.
- Render the returned promotion name, region, date, cash and seed. Do not simulate success
  with browser storage, hard-coded returned values or a fake browser backend.
- Reject duplicate names, missing saves, invalid inputs and incompatible files safely.
- Verify domain rules, database transactions/reopening, UI flows and a native smoke test.
- Establish formatting, linting, strict types, generated-contract checks and CI.

Final execution results belong in [verification.md](verification.md). Unfinished work
must remain labelled as such even if the interface or documentation has been scaffolded.

## Milestone 1: a seeded world and one day — next

Create wm-content and wm-sim only when implementing their first real behaviour. Validate
an editable base content pack. Pin ChaCha version and a stable stream-allocation scheme.
Generate UWF, two simplified competitors, around 40 UWF wrestlers, staff, free agents and
prospects. Implement a calendar, inbox and Continue command with atomic day advancement.

Acceptance: the same seed and engine produce the same canonical world digest; different
seeds vary coherent fictional people; invalid references/attributes are rejected; Continue
commits one day and its events exactly once. Query and virtualise the roster with TanStack
Table/Virtual. Profiles show six 0–100 wrestler groups with detailed sub-stats, condition, morale,
contracts and history; profession skills remain separate.
Scouting shows uncertain ranges and evidence about generated prospects.

## Milestone 2: book a complete card

Implement persisted shows, timed segment order, match and angle plans. A match editor
supports participants, winner/loser or non-victory result, finish, duration, purpose,
protection and performer freedom. Angles support participants and purpose. Validate
duplicates, availability, illegal winners/finishes and broadcast time constraints without
enforcing an arbitrary style quota. Card changes survive save/reload.

## Milestone 3: perform and review

Implement deterministic performance events from the booked plan. Locked outcomes must
hold over large property-test samples. Separate performance dimensions and emit causal
evidence. Add a renderer-neutral timeline and basic event view; changing playback speed,
skipping or hiding any current/future renderer cannot change the result. The complete text/2.5D
viewer and spike-gated WM-043 retro 3D capability remain later work.

Apply momentum, morale, fatigue, cohort popularity, money and storyline consequences
once, transactionally. Generate a fictional social feed from committed events using
authored templates. The end-to-end test creates a world, books a match and an angle,
runs the show, checks its result and consequences, closes/reopens the save and continues
into the following week. Only then is the initial playable vertical slice complete.

## Subsequent backlog

1. Transactional migration backups, backup rotation, recovery UI and real prior-version
   fixtures before upgrading player saves. Idempotent commands and periodic snapshots.
2. Rival strategy/booking, contracts and negotiations, staffing/delegation and medical care.
3. Audience memory, business commitments, story continuity and backstage relationships.
4. Development, schools, coherent youth generation and deeper scouting.
5. Historical queries, lineage, social ecosystems, external talent and analytics.
6. Validated pack import/editor, data licences, procedural portraits and content tooling.
7. Keyboard navigation polish, accessible large tables, global search and comparisons.
8. Distribution installers, code signing and supported Windows hardware verification.

## Reliability and performance gates

Target ordinary Windows 10/11 x64 PCs with integrated graphics and 8 GB RAM. At medium
scale (2,000–5,000 people, 20–50 promotions), measure cold launch under 3 s, idle memory
around/below 300 MB, Continue under 250 ms, ordinary week under 2 s and save/load under
2 s. Target 60 FPS presentation with lower-frequency deterministic simulation and event
interpolation. Add match balance batches and 10-, 20-, 30-year deterministic soak tests.
Watch invariants, storage growth and timing trends, not merely completion.

Distant minor events will use a calibrated summary path from the same rules. Never model
millions of fans individually. Profile before parallelising; stable ordering is mandatory
if concurrency is introduced. These targets have not been measured in this foundation.
