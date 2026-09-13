<div align="center">
  <img src="apps/desktop/src-tauri/icons/icon.png" width="144" height="144" alt="Wrestling Manager logo">

# Wrestling Manager

**Build the company. Book the show. Live with the consequences.**

An offline-first professional-wrestling management simulation for Windows.

![Development status](https://img.shields.io/badge/status-pre--alpha-C9973E?style=flat-square)
![Platform](https://img.shields.io/badge/platform-Windows-17283A?style=flat-square)
![Desktop](https://img.shields.io/badge/desktop-Tauri_2-17283A?style=flat-square)
![License](https://img.shields.io/badge/license-all_rights_reserved-17283A?style=flat-square)

</div>

> [!IMPORTANT]
> Wrestling Manager is an active pre-alpha. The repository contains a playable development
> foundation, not a finished or publicly supported release. Features, interfaces and save formats
> may change.

## What Wrestling Manager aims to be

Wrestling Manager is a single-player game about running the creative and business sides of a
wrestling promotion. The player chooses the intended result; the simulation determines how the
workers deliver it and what follows from their preparation, abilities, relationships, health,
audience and circumstances.

The design is built around four principles:

- **Causal simulation:** reports and consequences should explain what produced them.
- **Distinct people and characters:** real relationships, health and contracts stay separate from
  ring names, gimmicks, alignments and storylines.
- **Multiple valid styles:** there is no universally correct wrestler, product or booking formula.
- **Fast, connected management:** profiles, search, booking, news and company systems should preserve
  context and link to one another instead of behaving like isolated forms.

## What works today

The current development build supports a complete prototype loop:

1. Create or load a local career in the bundled fictional scenario.
2. Explore Talent Search and detailed wrestler profiles.
3. Build a show card containing singles matches and angles.
4. Set the winner, finish, purpose, pacing and risk; ask a road agent for suggestions or edit the
   sequence manually.
5. Run the show through the simulation-event view, change playback speed and send limited delayed
   instructions.
6. Inspect reports, media, worker development, injuries, chemistry, finances and history.
7. Advance the date and continue the career.

Implemented foundations include:

- A native Tauri desktop application with persistent local SQLite careers.
- Deterministic second-by-second show simulation, pause/resume and process-safe recovery.
- Six visible 0–100 wrestler rating groups, detailed sub-ratings, style fits, specialisations,
  archetypes and style-specific Overall values.
- Separate Person and Character records with dated ring identities, aliases, masks, gimmicks and
  Face/Heel/Tweener intent.
- Persistent personality, motivations, languages, hobbies, biographies, directional relationships,
  memories and deterministic player conversations.
- A multi-tab Person hub covering overview, ratings, character, career, matches, relationships,
  contract, development and media. Sections without an owning system use honest empty states.
- Server-paged Talent Search with combined filters, stable sorting, configurable columns, comparison,
  saved views, shortlists, a blacklist and navigation restoration.
- Match and angle running-order editing, bounded undo/redo, road-agent planning, manual spots, control
  periods and protected-worker instructions.
- Persistent crowd energy, trust, fatigue and expectations across a show.
- Match history with outcomes, duration, performance dimensions and explanations; event-derived news
  links back to relevant people and reports.
- Versioned Rust-to-TypeScript contracts and tested career migrations.

Singles and no-disqualification singles are the playable match formats. The domain model can
represent broader participant structures, but tag, trios, multi-person and elimination runtime
behavior is not implemented yet.

## Important work not yet implemented

The following are planned and ticketed, but should not be mistaken for current features:

- A finished main menu, settings suite, player-person creation, role-aware onboarding and advanced
  career/world setup.
- A dedicated company Roster workspace. The current Roster entry still reuses Talent Search.
- Multi-entity Global Search for companies, events, titles, teams, stories and news.
- Full contracts, scouting, training, staff, facilities, company finance, broadcasting, sponsors,
  venues, tours, house shows and rival-company simulation.
- Championships, teams, stables, storylines, tournaments and the full assisted/manual angle director.
- A complete text/2.5D show viewer. Live shows currently use a functional simulation-event view.
- The release-gated PlayCanvas Engine v2 retro 3D viewer. It remains a documented feasibility project;
  no production 3D match engine or approved asset library is present.
- Native database editing, custom database/asset-pack import, installer packaging and public release
  support.

See the [capability index](docs/tickets/INDEX.md), [implementation plan](docs/model-task-plan.md),
[playable milestones](docs/milestones.md) and [accepted product rules](docs/game-product-rules.md) for
the planned destination and current ticket status.

## Run the development build

### Requirements

- Windows 10 or 11
- Node.js 24
- pnpm 11
- The stable Rust toolchain with `rustfmt` and `clippy`
- Microsoft C++ Build Tools and Windows SDK
- Microsoft Edge WebView2 Runtime

[Tauri's Windows prerequisites](https://v2.tauri.app/start/prerequisites/) cover the native toolchain.
The repository scripts locate standard Visual Studio and Rust installations.

```powershell
npm.cmd install --global pnpm@11.19.0
pnpm.cmd install --frozen-lockfile
pnpm.cmd dev
```

The first command installs the pinned package-manager version and is normally needed only once. In
PowerShell, the `.cmd` suffix selects the executable launcher when script execution policy blocks the
PowerShell shim.

The native application stores careers under:

```text
%APPDATA%\com.wrestlingmanager.game\saves
```

Close the game before manually copying a career. Keep a copied career's original filename. The web-only
command `pnpm.cmd dev:web` is useful for interface work, but native commands and career gameplay require
`pnpm.cmd dev`.

### Current controls

| Input    | Action                              |
| -------- | ----------------------------------- |
| `F1–F6`  | Open implemented management screens |
| `Ctrl+K` | Open and focus Talent Search        |
| `F11`    | Toggle fullscreen                   |
| `Escape` | Open or close the game menu         |

The game menu includes **Exit game** and protects pending operations or unsaved match instructions.

## Architecture

Simulation, persistence, interface and presentation are deliberately separated:

| Area                  | Technology                   | Responsibility                                              |
| --------------------- | ---------------------------- | ----------------------------------------------------------- |
| Desktop shell         | Tauri 2                      | Native window, commands and lifecycle                       |
| Interface             | React 19 + TypeScript + Vite | Management workflows and live controls                      |
| Domain and simulation | Rust                         | Validation, match/show rules and deterministic consequences |
| Careers               | SQLite                       | World state, events, reports and migrations                 |
| Planned presentation  | Renderer-neutral events      | Complete text/2.5D fallback and spike-gated PlayCanvas 3D   |

Rust remains authoritative. React, Tauri, SQLite and any future renderer do not decide match results.
A paused show stores its random-stream position and transient state so the same engine version can
resume deterministically after restart.

```text
apps/desktop/             React interface and Tauri adapter
content/base/             Bundled fictional content and move definitions
crates/wm-domain/         Validated domain values and shared contracts
crates/wm-sim/            World generation and match/show simulation
crates/wm-persistence/    SQLite careers and migrations
packages/contracts/       Generated TypeScript contracts
docs/                     Product, architecture, tickets and verification
scripts/                  Windows toolchain and native smoke automation
```

Technical starting points: [architecture](docs/architecture.md), [simulation](docs/simulation.md),
[domain model](docs/domain-model.md), [verification](docs/verification.md) and
[ticket execution guide](docs/tickets/EXECUTION-GUIDE.md).

## Validation

```powershell
pnpm.cmd check
pnpm.cmd rust fmt --all -- --check
pnpm.cmd rust clippy --workspace --all-targets -- -D warnings
pnpm.cmd rust test --workspace
pnpm.cmd rust run -p wm-domain --bin export-contracts -- --check
pnpm.cmd build:desktop:debug
pnpm.cmd test:desktop
```

`pnpm.cmd check` covers formatting, linting, TypeScript, frontend tests and the production web build.
The native smoke test uses isolated disposable careers and exercises creation, booking, live playback,
restart/resume, reports and date advancement. `pnpm.cmd build:desktop` creates an unsigned release
executable without an installer; signing and distribution are later milestones.

## Project status and licensing

Work is tracked through `WM-###` capability and execution tickets. A feature is complete only when its
domain, persistence, interface, compatibility and verification obligations are satisfied; roadmap text
does not imply shipped behavior.

The project does not currently accept external contributions. No open-source licence has been granted.
The code and original game content are all rights reserved; third-party dependencies remain governed
by their own licences.
