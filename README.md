<div align="center">
  <img src="apps/desktop/src-tauri/icons/icon.png" width="144" height="144" alt="Wrestling Manager logo">

# Wrestling Manager

**Build the company. Book the show. Live with the consequences.**

A desktop management simulation about running a professional wrestling promotion.

![Development status](https://img.shields.io/badge/status-pre--alpha-C9973E?style=flat-square)
![Platform](https://img.shields.io/badge/platform-Windows-17283A?style=flat-square)
![Desktop](https://img.shields.io/badge/desktop-Tauri_2-17283A?style=flat-square)
![License](https://img.shields.io/badge/license-all_rights_reserved-17283A?style=flat-square)

</div>

> [!IMPORTANT]
> Wrestling Manager is in active pre-alpha development. The repository contains a playable
> foundation, not a finished game. Systems, save compatibility and presentation may change.

## The game

Wrestling Manager is an offline-first, single-player wrestling management game. You run the
creative and business sides of a promotion: assemble a roster, plan cards with road agents,
manage the audience across a live show and deal with the effects on talent, money and momentum.

The player books the intended result. The simulation decides how successfully the performers
deliver it. Attributes, fatigue, chemistry, morale, move familiarity, road-agent skill and the
state of the crowd all influence what happens between the bells. The long-term goal is a deep,
believable management game with the usability of a modern desktop sports simulation.

## Current playable build

The present development build includes:

- A native, fullscreen Windows application with persistent management navigation and shortcuts.
- A seeded fictional world with 40 wrestlers, individual attributes, condition and movesets.
- Six visible 0–100 rating groups with 37 sub-stats, nine Discipline Fits, style-specific Overall,
  Archetypes, Approach and Specialisations.
- Persistent personality, shared qualities, motivations, languages, hobbies and factual biographies
  in a Person & traits profile tab. Evidence-based exceptional-trait rules are implemented; their
  future media, wellness and sponsor systems do not yet produce events or apply effects.
- Show cards containing matches and angles, with ordering, editing and capacity rules.
- Road-agent planning, manual spots, control periods and protected-wrestler instructions.
- Deterministic, second-by-second show simulation with pause, playback speeds and live messages.
- A crowd that carries energy, trust, fatigue and expectations from one segment to the next.
- Match reports, worker development, injuries, chemistry, agent trust and show finances.
- Persistent SQLite careers, process-safe resume, verified migrations and news derived from play.
- A match-rule foundation for participant slots, opposing sides and unambiguous booked results.

Singles and no-disqualification singles are the currently playable match formats. Tag, trios,
multi-person and elimination structures can now be represented by the rules model, but their
runtime simulation and booking interface are upcoming work. The planned 2.5D presentation has
not been added; the live match currently uses a simulation event view.

See the [current ticket](docs/tickets/WM-022.md), [ticket index](docs/tickets/INDEX.md),
[ordered roadmap](docs/model-task-plan.md) and [milestones](docs/milestones.md) for the exact
development state.

## Run the development build

### Requirements

- Windows 10 or 11
- Node.js 24
- pnpm 11.19.0
- Rust 1.98 with `rustfmt` and `clippy`
- Microsoft C++ Build Tools with the Windows SDK
- Microsoft Edge WebView2 Runtime

[Tauri's Windows prerequisites](https://v2.tauri.app/start/prerequisites/) cover the native
toolchain. The included PowerShell scripts locate a standard Visual Studio and Rust installation.

```powershell
npm.cmd install --global pnpm@11.19.0
pnpm.cmd install --frozen-lockfile
pnpm.cmd dev
```

The first command is a one-time setup. Reopen an existing terminal after installing pnpm. Use
the `.cmd` suffix in PowerShell so Windows selects the executable launcher instead of a blocked
PowerShell shim.

Select **New career**, enter a lowercase save name and decimal world seed, then choose
**Create career**. Career files are stored at:

```text
%APPDATA%\com.wrestlingmanager.game\saves
```

Close the game before manually copying a career. A moved save must keep its original filename.

### Play the current loop

1. Open **Shows** and add a match or angle to the running order.
2. Choose the participants, planned winner and finish; optionally ask the road agent to plan it.
3. Save the instructions, complete the card and choose **Go on air**.
4. Select a viewing mode, control playback and send limited instructions through the agent.
5. Review the show report, worker changes, finances and news, then use **Continue**.

Useful controls:

| Input    | Action                                        |
| -------- | --------------------------------------------- |
| `F1–F6`  | Open the main management screens              |
| `Ctrl+K` | Search wrestlers, the promotion and next show |
| `F11`    | Toggle fullscreen                             |
| `Escape` | Open or close the game menu                   |

**Exit game** is available from the game menu. It waits for pending operations and warns before
discarding unsaved match instructions. `pnpm.cmd dev:web` runs the interface alone for frontend
work; careers and gameplay require the native desktop application.

## Technology

Wrestling Manager keeps the interface, simulation and save lifecycle separate so each can
evolve without making the renderer responsible for game results.

| Area              | Technology            | Responsibility                                       |
| ----------------- | --------------------- | ---------------------------------------------------- |
| Desktop shell     | Tauri 2               | Native window, commands and desktop lifecycle        |
| Interface         | React 19 + TypeScript | Management screens and live-show controls            |
| Simulation        | Rust                  | Match, show, audience and consequence rules          |
| Careers           | SQLite                | Portable world state, events, reports and migrations |
| Future match view | PixiJS 8              | 2D/2.5D rendering of simulation events               |

The simulation is seeded and independent of React, Tauri, SQLite and any future renderer. A
paused show stores its exact random-stream position and transient state, allowing deterministic
resume after restarting the application.

## Project structure

```text
apps/desktop/       React interface and Tauri command adapters
content/base/       Bundled fictional world and move definitions
crates/wm-domain/   Validated domain values and shared data contracts
crates/wm-sim/      World generation and match/show simulation
crates/wm-persistence/ SQLite career storage and migrations
packages/contracts/ Generated TypeScript contracts
docs/               Product, architecture, tickets and verification records
scripts/            Windows toolchain helpers and native smoke test
```

Start with the [vision](docs/vision.md), [architecture](docs/architecture.md),
[simulation design](docs/simulation.md), [domain model](docs/domain-model.md) and
[verification guide](docs/verification.md). Files under `sources/` are synced reference material
and must remain unchanged.

## Quality checks

```powershell
pnpm.cmd check
pnpm.cmd rust fmt --all -- --check
pnpm.cmd rust clippy --workspace --all-targets -- -D warnings
pnpm.cmd rust test --workspace
pnpm.cmd rust run -p wm-domain --bin export-contracts -- --check
pnpm.cmd build:desktop:debug
pnpm.cmd test:desktop
```

The native smoke test launches the real application against isolated saves and exercises career
creation, booking, live playback, restart/resume, reports and Continue. Disposable screenshots
and saves remain under the ignored `.artifacts/` directory.

`pnpm.cmd build:desktop` produces a release executable without an installer. Signing, installer
creation and public distribution are later milestones. Windows is the currently validated platform.

## Development policy

Work is organised as numbered `WM-###` tickets with one reviewable feature branch per ticket.
Each ticket records its scope, acceptance criteria, compatibility boundaries and verification.
Generated contracts must come from the Rust domain types, and gameplay changes must preserve
or explicitly version the save and deterministic simulation contracts.

This project does not currently accept external contributions. No open-source licence has been
granted; the code and original game content are all rights reserved. Third-party dependencies
remain subject to their respective licences.
