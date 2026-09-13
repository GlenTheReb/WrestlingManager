# Desktop architecture

## Current walking skeleton

```text
React forms / overview / save library
       │ typed command requests                 ▲ compact generated DTOs
       ▼                                        │
Tauri command adapters ── spawn_blocking ── SaveRepository
                                                │
                                      validated Rust domain
                                                │
                                   bundled SQLite save file
```

apps/desktop/src contains React 19, strict TypeScript, semantic HTML and CSS Modules.
Design tokens provide a restrained desktop office style with keyboard focus and responsive
layout. There is no remote font or art download at runtime. Browser development explicitly
reports that local careers require the desktop application; there is no production mock.

The selected career and screen live in Zustand. TanStack Query caches overview/list DTOs
and invalidates the library after creation. Persistent promotion data is never copied into
Zustand. Save mutations are not automatically retried: a successful write followed by a
lost response must not cause an automatic second creation.

apps/desktop/src-tauri owns application paths, window settings and four asynchronous
commands. SQLite/filesystem work runs on Tauri's blocking pool. Each operation opens its
own connection; the interface does not hold database handles or locks. Errors are typed
and mapped to a small code/message response; structured tracing retains internal details.

crates/wm-domain owns validation and Rust transfer types. ts-rs produces the TypeScript
contract in packages/contracts; the exporter has a check mode for stale generated output.
crates/wm-persistence owns SQLite, migrations, portable save identity and initialisation.
The exact IPC shapes are in the generated contract and command adapters.

## Commands, queries and events

`create_game` changes state and returns an overview only after commit. `load_game` validates
an existing save. `get_promotion_overview` queries that saved state. `list_saves` returns a
bounded local career list. Save identifiers are validated in Rust before becoming paths.
The frontend cannot supply arbitrary paths or SQL. The current library is limited to
100 valid saves; page controls are a future interface extension.

The new-game event is a persisted fact. Future show/day commands must transactionally
update relational state and append their important events, then publish committed results
to the interface. Those events are historical context; this is not full event sourcing.
Periodic snapshots and idempotency identifiers will be added with longer-running commands.

## Save lifecycle

Saves live under Tauri's per-user application-data directory in `saves/*.sqlite3`.
One file contains the data and explicit metadata. A debug-only `WM_SAVE_DIR` override
allows native tests to use isolated disposable directories; release builds ignore it.

Create in a temporary file in the same directory, initialise schema and state in one
transaction, close SQLite and publish without overwriting an existing destination.
An incompatible or foreign file must be rejected without migration or rewriting. Loading
does not create absent files. Schema v1 is the first schema, so there are no authentic
earlier released fixtures. Migration backup/rotation/recovery policy must be implemented
before the first substantive upgrade; do not claim that infrastructure already exists.

## Boundaries that arrive with the playable loop

wm-content will validate mod packs and generate coherent fictional world data. wm-sim
will own calendar/show/match state transitions. wm-ai will select legal actions using
goals, scored options and recorded reasons. Add each crate with a working feature rather
than empty services. Shared UI packages likewise follow demonstrated reuse.

The guaranteed text/2.5D viewer will consume timed performance events and interpolate visual states;
conditional WM-043 may later consume the same contract through PlayCanvas Engine v2 and
`@playcanvas/react`. WebGL2 is its required baseline and WebGPU is optional. Neither viewer selects
moves or winners. Playback speed and visibility cannot affect simulation.
PD-112 live direction is also simulation-owned: Rust commits the performed prefix and may rebuild the
unsimulated suffix after credible referee delivery and worker response. A viewer discards only its
buffered future presentation and derives a replacement from new WM-013 events.
Install TanStack Table/Virtual when roster data is present; evaluate uPlot against the
then-current stack when analytics arrives. No unused renderer/chart dependencies now.

## Determinism and scale

Seed and engine versions are stored now; no random world generation exists yet. Before
generation, pin a ChaCha-family implementation and define seed-to-stream derivation,
ordering, integer arithmetic and replay input/version rules. Never use wall-clock time,
uncontrolled randomness or hash-map iteration as simulation input. A stored seed alone is
not sufficient to guarantee reproducible decades of simulation.

Query only needed pages/columns. Use audience cohorts, lower detail for distant shows and
explicit event retention/aggregation policy that preserves important history. Measure the
budgets in vertical-slice.md before adding parallelism.

## Dependencies and security

The lockfiles capture compatible npm and Rust dependency resolutions. Node 24 is the
development baseline; Vite 8 requires a modern Node version. TypeScript 6.0.3 was selected
because the current lint parser supports versions below 6.1. SQLite is compiled into the
application. Tauri's window has core permissions only, with no filesystem/shell/network
plugin exposed to JavaScript. The content security policy permits local assets and IPC.
No custom server, account, cloud API, Redis or external database is required.

Windows uses Microsoft's C++ tools, the SDK and WebView2. `scripts/cargo.ps1` discovers
Developer PowerShell to supply native compiler paths. Other operating systems need Tauri's
documented prerequisites; they are not verified by this first Windows session.

Primary references checked for this session: [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/),
[Tauri commands](https://v2.tauri.app/develop/calling-rust/),
[blocking work](https://docs.rs/tauri/latest/tauri/async_runtime/fn.spawn_blocking.html),
[Vite 8 requirements](https://vite.dev/guide/).
