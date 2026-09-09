# Wrestling Manager

An offline-first wrestling management prototype for Windows. The current playable loop
includes 40 seeded wrestlers, move repertoires, road-agent planning, match/angle booking,
deterministic live shows, reports, development, show finances and daily Continue.
This is an early simulation foundation, not the full management game described in the vision.

## Run on Windows

Use Node 24, pnpm 11.19.0, stable Rust with rustfmt/clippy, Microsoft C++ Build Tools with
the Windows SDK, and WebView2. [Tauri's prerequisites](https://v2.tauri.app/start/prerequisites/)
describe the native installation. The supplied Windows launch scripts discover the
installed Developer PowerShell and the standard user-local Rust installation.

```powershell
npm.cmd install --global pnpm@11.19.0
pnpm.cmd install --frozen-lockfile
pnpm.cmd dev
```

The first command installs pnpm for your normal terminal; Codex's private copy is not
automatically available in PowerShell. It is a one-time setup step. If your terminal was
already open before installation, reopen it. The `.cmd` suffix selects the Windows launcher.

Select **New career**, choose a lower-case save name and a decimal world seed, then
**Create career**. The promotion office reads UWF's saved name, region, date and funds.
Return to **Save library** to reopen it. Creation saves automatically. Each career is a
portable `*.sqlite3` file under `%APPDATA%\com.wrestlingmanager.game\saves` on Windows.
Close the game before copying save files. Use the same filename when moving a save.

Open **Shows**, choose participants and a winner, optionally ask the agent to plan, then
**Add to running order**. Add angles or further matches and choose **Go on air**. Playback
starts paused. Use the viewing-mode menu and agent channel, then open the post-show report.
**Continue** advances one day after today's show is complete. F11 toggles fullscreen,
F1–F6 change screens and Ctrl+K searches people, the promotion and the next show.
**News & inbox** shows actual show results, business and medical updates. Read state persists,
and articles link to the related wrestler or historical show. **Exit game** is in the game menu;
it waits for pending operations and offers a discard choice when instructions are unsaved.
Unsaved planner edits are local: choose **Save instructions** before changing screens.
Version 1/2 careers receive a verified `.sqlite3.v1.bak` or `.v2.bak` before upgrading to schema 3.

`pnpm dev:web` starts the interface in a browser for development; local gameplay requires
the desktop shell. Browser mode has no fake save backend.

## Build and verify

```powershell
pnpm check
pnpm rust fmt --all -- --check
pnpm rust clippy --workspace --all-targets -- -D warnings
pnpm rust test --workspace
pnpm rust run -p wm-domain --bin export-contracts -- --check
pnpm build:desktop:debug
pnpm test:desktop
```

`pnpm build:desktop:debug` builds a native executable with embedded frontend assets in
`target/debug/wm-desktop.exe`. The smoke test launches that real application, uses
Playwright with WebView2, creates an isolated test save, books a show and exercises
live instructions, process restart/resume, reports and Continue. Screenshots and disposable test data remain under
ignored `.artifacts/`. The debug-only save-directory override is disabled in release builds.

`pnpm build:desktop` builds a release executable without an installer. Installer creation,
signing and distribution are later milestones. Nothing is automatically published.

On non-Windows platforms, use the native `cargo` and filtered Tauri commands in place of
the PowerShell wrappers after installing that platform's prerequisites. Only Windows is
validated in this session.

## Repository map

- `apps/desktop`: React interface and native Tauri command adapters.
- `crates/wm-domain`: validated values and canonical transfer types.
- `crates/wm-persistence`: bundled SQLite schema and save lifecycle.
- `crates/wm-sim`: seeded generation, plan validation and second-based match/show simulation.
- `content/base`: editable bundled fictional world and move definitions.
- `packages/contracts`: generated TypeScript types; do not edit generated.ts manually.
- `docs`: product design, architecture, decisions, acceptance criteria and verification.
- `scripts`: Windows toolchain wrappers and the native smoke test.

The content pack is compiled into this prototype; runtime mod importing and PixiJS presentation
remain future work. `sources/` is read-only synced reference material.
`SDD.md` is an intentionally ignored local architectural reference; ask the owner before
recreating it in another checkout.

Start with [vision](docs/vision.md), [milestones](docs/vertical-slice.md),
[architecture](docs/architecture.md), [domain model](docs/domain-model.md),
[verification](docs/verification.md) and the [developer guide](docs/developer-guide.md).

No open-source licence has been selected. This scaffold does not grant a distribution
licence for the game's code or content. Dependency licences remain their own.
