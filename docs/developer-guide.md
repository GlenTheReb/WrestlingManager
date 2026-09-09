# Learning from the walking skeleton

## News and desktop identity increment

The news reader demonstrates a projection: a saved show report is transformed into articles
without rerunning the match. `crates/wm-persistence/src/news.rs` writes those articles inside
the same completion transaction as the financial result. A unique source key identifies each
cause (for example, one results article per show), so retries cannot multiply headlines.
`News.tsx` reads the projection and changes only the read flag. It never invents simulation facts.

Two interview questions from this change:

1. **Why save articles rather than generate them each time the screen opens?** Persistent
   identity makes unread state, links and paging reliable, and keeps historical wording stable.
   Generating on read can reduce storage, but needs equally stable identity and a policy for
   wording changes. Here a small materialised projection is simpler and inexpensive.
2. **Why does adding news require migration tests when match simulation did not change?** A
   schema upgrade touches existing careers, including paused matches. The integration test
   restores schema 2 around a live snapshot, upgrades it and compares the entire returned live
   state. It then completes the show twice and verifies article counts, finances and read state.
   The reusable strategy is to test both preservation of old state and the new behaviour.

Commands used: `pnpm.cmd rust test --workspace` passed 25 Rust tests, including migration and
projection checks; `pnpm.cmd test` passed 10 UI tests; `pnpm.cmd build:desktop:debug` embeds the
latest UI; `pnpm.cmd test:desktop` exercises actual native news links, badges, timeline focus
and Exit, plus the original show workflow. These are safe to rerun with isolated test careers.
The detailed argument meanings and failure interpretation remain in Commands used below.

## One complete path before many screens

The useful result is a career that survives closing the program. App.tsx starts a user
operation; api.ts sends a typed command; commands.rs moves storage work off the interface
thread; SaveRepository validates and commits a SQLite file; the returned overview renders
the stored state. Each boundary has a narrow responsibility and can be tested separately.

This is a walking skeleton: deliberately little product behaviour connected through the
real architecture. It catches installation, serialisation and persistence problems before
they are hidden beneath dozens of screens. The next feature should extend this same path
with one coherent roster/day action, not add disconnected layers across the whole game.

## Data ownership and precision

Zustand stores selection/navigation in navigation.ts. React Query stores replaceable cached
answers. SQLite owns persistent state. This distinction prevents two mutable copies of
cash or the calendar disagreeing after a failed save. A query cache can be invalidated;
the authoritative save must be changed only through validated commands.

Rust's u64 can hold more exact integers than a JavaScript number. Passing a large seed as
a number would round it before Rust receives it. A decimal string preserves every digit;
Seed parses it at the trusted boundary. Money uses integer pence and an explicit safe
range, avoiding binary floating-point rounding such as fractions of a penny.

## Transactions and file publication

Creating a promotion spans several rows: metadata, state and a history event. A transaction
makes them succeed or fail together. That alone does not protect an existing filename from
being overwritten. The repository therefore creates a temporary file alongside the final
save, commits/closes it, then publishes without replacement. Same-directory creation avoids
a cross-filesystem move. A competing creation must fail rather than replace the winner.

Read-only loading first establishes that a file is a supported game database. Unknown
versions are not an invitation to recreate tables. Migration backups, interrupted-write
recovery and a player-facing recovery UI still need separate implementation and tests.

## Developer learning: interview questions

### 1. Why keep promotion data out of Zustand if React already displays it?

Because display state and domain state have different lifetimes and failure behaviour.
Selection is temporary interface state. Promotion cash must survive process exit and
participate in transactions with its history. If both stores were writable, a failed IPC
call could leave the screen showing money that never reached disk. Here the interface
sends commands, receives compact DTOs and invalidates cached queries when needed. This
single authority makes failure recovery and future multi-entity updates easier to reason
about. For a new feature, identify who owns each value before choosing a state library.

### 2. Why test a full process restart when a component test already passes?

A component test replaces IPC and proves UI behaviour under controlled responses. It
cannot prove command registration, native serialisation, save paths or persistence.
The native smoke test creates through the actual WebView2/Tauri bridge and restarts the
application before loading. That destroys all in-memory state, so success is evidence
that the disk boundary works. Keep both kinds: component tests are cheaper and isolate
failures; native tests cover integration risks with fewer, meaningful scenarios.

### 3. What does a transaction solve, and what does it leave unsolved here?

It prevents partially applied relational changes: there cannot be an accepted new career
with metadata but no promotion or creation event. It does not itself prevent filename
collisions, guarantee backups or define migration policy. No-clobber publication protects
the filename; version checks protect interpretation. Future backups and recovery need
their own acceptance tests. The reusable strategy is to enumerate failure points from
request validation through commit, publication and response instead of treating “save”
as one indivisible operation.

## Commands used

Run these from the repository root. They are safe to rerun; build and test commands may
write generated output, caches or isolated test files, but do not overwrite player careers.

- `pnpm install --frozen-lockfile` installs the exact resolved frontend dependency graph.
  “Frozen” means disagreement with package manifests fails instead of rewriting the lock.
  A clean exit establishes installation, not application correctness.
- `pnpm check` runs formatting, linting, strict type checking, React tests and a production
  frontend build. The stages stop on failure. Lint catches suspect code; type checking
  catches invalid shapes; tests check behaviour. None substitutes for a native test.
- `pnpm rust fmt --all -- --check` checks every Rust package against rustfmt conventions.
  The second `--` forwards `--check` to rustfmt; it reports changes instead of applying them.
  Without that last flag, it formats the code. The wrapper supplies Windows compiler paths.
- `pnpm rust clippy --workspace --all-targets -- -D warnings` checks all workspace packages
  and their tests/examples with Rust's linter. `-D warnings` makes warnings fail the check,
  keeping the accepted baseline clean. It is a static check, not a test run.
- `pnpm rust test --workspace --locked` compiles and runs domain, property, persistence and
  adapter tests. `--locked` prevents dependency re-resolution. Property tests sample input
  spaces; SQLite tests use temporary databases and check independent reopening and rejection.
- `pnpm rust run -p wm-domain --bin export-contracts -- --check` runs the selected generator
  executable and verifies that checked-in TypeScript exactly matches Rust's declarations.
  Remove `--check` after intentionally changing a DTO to regenerate it, then type-check
  callers. This prevents a handwritten frontend type from drifting away from native data.
- `pnpm build:desktop:debug` compiles frontend assets and embeds them in a native debug
  executable. The `custom-protocol` feature serves the built files without a development
  server. Debug mode permits an isolated test save directory and WebView2 inspection.
- `pnpm test:desktop` runs the actual create/reload/duplicate/restart/load workflow using
  Playwright against Windows WebView2. A successful exit covers the connected path; the
  screenshots also need visual inspection. It does not measure long-save performance or
  prove simulation features that have not been written.

When a check fails, locate the first relevant error and decide which boundary it belongs
to before changing code. Do not weaken a test merely to clear the failure. Rerun the
affected check, then perform the full agreed validation once near completion.

## Playable-loop checkpoint: engineering concepts

The new loop separates intent, execution and presentation. Booking.tsx submits a MatchPlan;
runtime.rs turns that intent into events; LiveShow.tsx displays them. This lets a player lock
an outcome while execution remains uncertain, and lets different playback modes share one
source of truth. A renderer must never decide a move succeeds merely because an animation ends.

State that affects future decisions must be explicit. Session includes the RNG position,
planned beats, stamina, crowd, pending instructions and reports. A resumed match must preserve
all of these, not just the displayed clock. The chunk-equivalence test compares events and
complete final state, so it catches invisible drift that a winner-only assertion would miss.

Persistence combines transactional writes with expected-state checks. SQLite transactions in
career.rs keep a chunk's events and consequences together. expectedTick stops two requests
from accidentally advancing the same starting state twice; unique ledger/history entries and
a completion guard prevent duplicate effects. These solve different problems and work together.

useHistory.ts keeps undo snapshots in memory, bounded to fifty edits. Saving remains explicit.
That is simple and avoids database writes on every keystroke, but undo cannot survive closing
the planner. A future draft store must address navigation and recovery deliberately.

## Developer learning: questions from the playable loop

### 1. Why is a seed alone insufficient to resume the same match?

A seed reproduces a random sequence from its beginning. A partially played match has already
consumed part of that sequence. Restoring only the seed restarts randomness and produces new
future events. Session therefore stores seed, stream identity and word position as well as all
mutable match/show state. The test runs 347 individual ticks, serialises and restores, then
finishes in chunks of eight and compares the complete result with an instant simulation.

### 2. How would you prevent a double-click or retried request from paying twice?

Use business invariants in the backend rather than relying on a disabled button. advance_show
checks expectedTick inside an immediate transaction; completed shows return their stored result.
The ledger's one-show uniqueness is another defence. UI disabling improves usability, but only
the backend can protect callers from separate windows, retries and stale cached state.

### 3. What is the tradeoff in a shared live snapshot versus writing every tick separately?

A snapshot captures all state needed to continue and is straightforward to restore. Committing
one transaction per requested chunk reduces database overhead compared with one per second.
The tradeoff is serialising transient state and ensuring every causal field is included. It
should remain bounded to active-show people, while world identity and histories stay relational.
For long saves, profile the snapshot and event tables before inventing a more elaborate system.

## Commands used for this checkpoint

- `pnpm.cmd rust test --workspace` runs all Rust packages, including the new deterministic
  simulation and career migration/restart tests. `--workspace` includes every member; a nonzero
  exit means a build or assertion failed. It is safe to rerun; tests use temporary careers.
- `pnpm.cmd test` runs six React workflow tests with mocked IPC. This checks UI behaviour and
  request shapes, but cannot prove SQLite or native integration. No player saves are changed.
- `pnpm.cmd build:desktop:debug` type-checks and builds the UI, then embeds it in wm-desktop.exe.
  Run it before testing the packaged native UI; otherwise the executable can contain older
  assets. A successful build proves compilation, not playability. Safe when the target test
  executable is closed; avoid simultaneous Cargo builds because they share a build lock.
- `pnpm.cmd test:desktop` launches only its owned executable process, using an isolated save
  under .artifacts. It books a match and angle, sends an instruction, restarts, resumes and
  verifies the result/report/date through WebView2 and real native queries. Reruns create new
  temporary careers; they do not overwrite normal player saves. Test selectors must match
  the actual accessible names; selector failures are not evidence of simulation failures.
- `pnpm.cmd lint` checks frontend and test-script code. `pnpm.cmd exec prettier --check <paths>`
  checks formatting without editing; `--write` applies formatting. Both are safe to rerun.
- The Rust format, Clippy and generated-contract commands explained above remain the same.
  All commands should be run from the repository root. Use the .cmd suffix in PowerShell
  when executable resolution or script execution policy would otherwise select pnpm.ps1.

## WM-001 developer learning

The outcome is one shared definition of match membership and results, used by booking
validation and simulator construction. `crates/wm-domain/src/match_rules.rs` owns the rules;
`crates/wm-sim/src/planning.rs` checks which valid formats this engine can actually execute.
`runtime.rs` now returns a success or rule error when constructing a session. `career.rs`
propagates that error before writing a runtime snapshot inside its existing transaction.

### 1. Why separate a worker, a participant slot and a side?

A worker is a person in the world. A slot is their place in this match. A side is the group
competing together for this match, even if they have no permanent team record. Storing the
result as “array element 0 wins” would change its meaning when the interface reordered the
participants. Stable slot/side IDs preserve those references. A substitution can change a
slot's worker while leaving its role in the booking intact; the UI should make that inherited
booking visible. Permanent teams alone would make ad-hoc alliances unnecessarily difficult.

The reusable strategy is to distinguish identity from display order and from membership.
Choose references that survive the user operations you intend to support, then test those
operations. The generated-layout test reverses both lists and substitutes a wrestler before
checking the result and membership rules.

### 2. Why use a result enum instead of several optional winner fields?

An enum expresses mutually exclusive possibilities: a decision, a draw or a no contest.
Only a decision contains a winning side and a decisive method. `Option<String>` is still useful
for individual actors in a whole-side count-out or disqualification. Pinfall/submission require
both actors, and validation checks that they belong to opposing sides. Rust and the generated
TypeScript union express the same shape, reducing disagreement across the desktop boundary.

Types are not sufficient by themselves: JSON arrives at runtime and can contain contradictory
fields. A new test found that Serde's unit enum variants ignored extra fields even with the
unknown-field restriction. Empty struct variants (`Draw {}`) enforce rejection. Test actual
serialisation/deserialisation instead of assuming a compiler type proves external data is valid.

### 3. Why adapt old singles plans instead of migrating every save immediately?

`MatchDefinition::try_from(&MatchPlan)` reads the old plan and builds a validated, temporary
definition. The old plan remains the only persisted source of truth. This avoids both a risky
save-format change before expanded simulation exists and two editable winner fields drifting
apart. The tradeoff is a temporary adapter and a deliberate later migration when team plans
become persisted inputs. It is not a permanent justification for keeping the two-worker model.

Separate business validity from implementation capability: a three-team booking can make
sense while the current simulator cannot run it. The capability check rejects it explicitly.
This pattern supports gradual upgrades without either corrupting data or pretending unfinished
features work. WM-002/003 must add real runtime support before relaxing that check.

### 4. How do you prove a refactor preserved a seeded simulation?

First capture evidence before changing production code. WM-001 recorded fingerprints of a
347-second snapshot, its completed state and all emitted events from baseline `3d74c7a`.
The golden test checks those exact outputs after the change. Its FNV-1a hash is a compact
regression fingerprint, not a cryptographic guarantee. Semantic tests add meaningful assertions:
the booked winner stays fixed, all existing finishes work, chunk sizes and serialised resume
agree, and SQLite rejects invalid bookings without changing the card revision.

A golden test alone covers only one scenario and can fail for harmless serialisation changes.
Property tests explore broader inputs but cannot prove old output compatibility. Use both,
investigate differences, and never update a golden constant just to make a failing test pass.

## WM-001 commands used

- `pnpm.cmd rust test -p wm-domain -p wm-sim -p wm-persistence` selects the three affected Rust
  packages with repeated `-p` options. It runs their unit/integration tests and examples marked
  as documentation tests. A nonzero exit means compilation or an assertion failed; inspect the
  named failure before rerunning. Tests use temporary saves and are safe to rerun.
- `pnpm.cmd rust run -p wm-domain --bin export-contracts` runs the contract generator and
  rewrites `packages/contracts/src/generated.ts` from Rust definitions. Adding `-- --check`
  passes `--check` to that program instead of Cargo, checking drift without writing. Generate
  after editing types; check in validation. Both are safe to rerun.
- `pnpm.cmd rust clippy --workspace --all-targets -- -D warnings` checks every Rust package and
  target, including tests, with warnings treated as errors. It catches suspicious code; passing
  does not prove simulation correctness. It does not modify player saves.
- `pnpm.cmd check` runs formatting, frontend lint, type checks, UI tests and the frontend build.
  It verifies the generated contracts still fit the app. It is safe to rerun, but does not build
  the native executable or prove real desktop-to-database communication.
- `pnpm.cmd rust build -p wm-desktop --features custom-protocol` embeds the previously built
  frontend in the debug desktop executable. Run after `pnpm.cmd check` or `pnpm.cmd build` so
  the assets are current. `pnpm.cmd test:desktop` then exercises real booking, playback,
  restart/resume and results using isolated saves under `.artifacts`. It opens a temporary
  game window and closes only processes it owns; it is safe to rerun.
