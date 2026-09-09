# Current verification

The playable-loop checkpoint is recorded in [checkpoint.md](checkpoint.md) and the root
SDD.md. The latest increment passed 25 Rust tests, 10 UI tests and the actual native
booking/live/restart/report workflow plus news/read links, timeline focus and native Exit
on 9 September 2026. The foundation record below is historical.

# Foundation verification

## Scope

Windows walking skeleton: a new UWF save, typed native queries, promotion overview and
reopening. These results do not establish a playable wrestling simulation or the full
vertical slice. See vertical-slice.md for the remaining acceptance criteria.

## Environment

Windows x64; Node 24.19.0; pnpm 11.19.0; Rust 1.98.1 stable with rustfmt/clippy;
MSVC 14.51.36231 and Windows SDK 10.0.26100.0; WebView2 152.0.4191.66.
Rust was installed into the standard user-local location during this session without
changing the global PATH. The supplied scripts discover it and the existing C++ tools.

Dependency selection used current package registry metadata and official Tauri/Vite
documentation. TypeScript 6.0.3 is within the lint parser's supported range; the latest
TypeScript major was outside it. The lint stack uses 8.69.0, satisfying the repository's
24-hour dependency maturity setting. Lockfiles record the actual resolved graph.

## Final checks

| Check                                      | Result                                                                                                      |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| Dependency installation                    | Passed                                                                                                      |
| ESLint                                     | Passed                                                                                                      |
| Strict TypeScript checking                 | Passed                                                                                                      |
| Vitest / React Testing Library             | 6 tests passed                                                                                              |
| Vite production frontend build             | Passed; 69 modules, approximately 239 kB JavaScript before compression                                      |
| Rust workspace tests                       | 16 passed: 5 domain/property, 10 SQLite integration, 1 blocking adapter                                     |
| Rust Clippy                                | Passed for workspace/all targets with warnings denied                                                       |
| Rust formatting                            | Passed                                                                                                      |
| Generated Rust → TypeScript contract drift | Passed                                                                                                      |
| Native Windows debug build                 | Passed with embedded frontend assets                                                                        |
| Native Playwright/WebView2 smoke           | Passed: create, overview, page reload, duplicate rejection, process restart and SQLite load; no page errors |
| Visual inspection                          | Normal-width and 900 px overview screenshots inspected; no horizontal overflow at 900 px                    |

Initial setup failures were resolved without weakening tests: the contracts workspace
name was aligned, dependency versions were selected within the maturity window, the
Windows script was corrected to forward Cargo arguments and find the existing toolchain,
and the required native application icon was generated from an original SVG. Database
tests also caught SQLite's unqualified `current_date` function; the query now qualifies
the column. Review added rejection tests for mismatched save identity and invalid dates.
The desktop library has a distinct name to prevent Windows debug-symbol collisions with
the executable.

The native test used seed 18446744073709551615 and verified it survived the entire
interface/native/database path exactly. It checked opening cash as GBP 250,000 after a
process restart. This is a persistence/precision test, not a random-world determinism test.
The executable is target/debug/wm-desktop.exe; screenshots are in ignored .artifacts/.

## Outstanding product work

Terminal setup follow-up: installed pnpm 11.19.0 into the standard user npm prefix because
the initial build used Codex's private pnpm. Confirmed the executable resolves using the
persisted Windows user/machine PATH and that frozen-lockfile installation succeeds.
README now includes this setup. No game code changed.

No generated roster, calendar advancement, booking, simulation, PixiJS presentation,
scouting, social feed or show consequences yet. No migration from a prior released schema,
backup rotation, recovery UI, mod-pack importer, installer/signing or measured hardware
performance budget. No decades-long or balance harness can meaningfully run until the
simulation exists. CI is configured but has not been executed on a hosted runner. Release
configuration, installer packaging and other operating systems have not been tested here.

The current library lists up to 100 compatible saves and omits files it cannot recognise.
Recovery/status entries and page controls remain planned. Player saves should not be
renamed independently of their internal identity.

## Implementation record and next entry point

No commits, push or publication. The branch is feature/initial-skeleton. Read-only sources
and the supplied AGENTS.md were preserved. SDD.md is ignored by agreement.

Relevant local SDD headings: Current status and execution pointer (line 3), Product
boundaries (line 11), Initial architecture and command contract (line 18), Persistence
and safety boundaries (line 40), Verification and handoff (line 57).
The next small milestone is a validated fictional content pack, deterministic starting
people and a tested one-day Continue transaction, followed by a virtualised roster.
