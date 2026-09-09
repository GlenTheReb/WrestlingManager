# WM future task and model plan

Recommendations requested on 9 September 2026. This is a manual selection guide, not
permission to route, spawn agents or change the current model. Current instruction: direct
GPT-6 Astra at High. The task assignments and thinking levels below are engineering judgments,
not measured WM benchmarks or guaranteed usage savings. The [official model catalogue](https://developers.openai.com/api/docs/models)
positions Astra for the hardest work, Sol for complex professional work, Terra for a balance
of intelligence and cost, and Luna for cost-sensitive work. Subscription allowance consumption
must not be inferred directly from API pricing.

## Ticket roadmap

WM-001 is implemented and verified on its ticket branch; it is not yet merged.
See [acceptance criteria and handoff](tickets/WM-001.md). Remaining tickets are planned.

| Priority | Ticket | Task                                                            | Branch                                 | Model         | Thinking |
| -------: | ------ | --------------------------------------------------------------- | -------------------------------------- | ------------- | -------- |
|        1 | WM-001 | Match-rule core: participant slots, teams and result semantics  | `feature/wm-001-match-rule-core`       | GPT-6 Astra   | High     |
|        2 | WM-002 | Tag and trios simulation                                        | `feature/wm-002-tag-trios-engine`      | GPT-6 Astra   | High     |
|        3 | WM-003 | Multi-person and elimination simulation                         | `feature/wm-003-multi-person-engine`   | GPT-6 Astra   | High     |
|        4 | WM-004 | Team, participant and elimination booking UI                    | `feature/wm-004-team-booking-ui`       | GPT-5.6 Sol   | High     |
|        5 | WM-005 | Rules modules: weapons, cage, ladder and timed entry            | `feature/wm-005-stipulation-rules`     | GPT-6 Astra   | High     |
|        6 | WM-006 | Road-agent planning and live adaptation for expanded rules      | `feature/wm-006-road-agent-rules`      | GPT-6 Astra   | High     |
|        7 | WM-007 | Move chains, counters, limb work and finish execution           | `feature/wm-007-match-sequences`       | GPT-6 Astra   | High     |
|        8 | WM-008 | Crowd/show flow and long-term development balance               | `feature/wm-008-show-consequences`     | GPT-6 Astra   | High     |
|        9 | WM-009 | Advanced booking timeline, drafts, card-wide undo and templates | `feature/wm-009-booking-workflow`      | GPT-5.6 Sol   | High     |
|       10 | WM-010 | Expand moveset content against the established schema           | `feature/wm-010-moveset-library`       | GPT-5.6 Terra | Medium   |
|       11 | WM-011 | Company creation, calendar, contracts, scouting and finances    | `feature/wm-011-company-foundations`   | GPT-5.6 Sol   | High     |
|       12 | WM-012 | Rival-company decisions and evolving world simulation           | `feature/wm-012-world-simulation`      | GPT-6 Astra   | High     |
|       13 | WM-013 | 2.5D event contract, paired rigs and animation architecture     | `feature/wm-013-match-presentation`    | GPT-6 Astra   | High     |
|       14 | WM-014 | First PixiJS ring, camera and animation slice                   | `feature/wm-014-pixi-match-view`       | GPT-5.6 Sol   | High     |
|       15 | WM-015 | Save migrations, compatibility and difficult engine bugs        | `feature/wm-015-save-hardening`        | GPT-6 Astra   | High     |
|       16 | WM-016 | WM visual identity and complex screen design                    | `feature/wm-016-desktop-identity`      | GPT-6 Astra   | Medium   |
|       17 | WM-017 | Approved screens, filters and navigation                        | `feature/wm-017-management-navigation` | GPT-5.6 Terra | Medium   |
|       18 | WM-018 | Focused tests and routine bug fixes                             | `feature/wm-018-regression-suite`      | GPT-5.6 Terra | Medium   |
|       19 | WM-019 | Documentation and mechanical cleanup                            | `feature/wm-019-project-cleanup`       | GPT-5.6 Luna  | Low      |

## Branch and ticket rules

Each ticket has one branch, one reviewable purpose and explicit acceptance criteria before
implementation starts. Keep a branch small enough to test and merge independently. If a ticket
reveals two independent outcomes, split it into a new `WM-###` ticket rather than extending its
scope. Every implementation branch starts from the current integration branch after its parent
ticket has merged; do not stack unrelated work on an old feature branch.

Use Conventional Commits. A typical commit subject is `feat(match): add tag-team participant
roles` for WM-001. Put the ticket identifier in the commit body and pull-request description
until a GitHub issue tracker is connected. The branch name is the source of truth for now.

Do not create all branches upfront. WM-001 started directly from the published
`feature/initial-skeleton` baseline (`3d74c7a`) at the owner's request. Before integrating
finished tickets, establish a stable integration branch (`main`) from that baseline and merge
each finished ticket through review. Creating or publishing that branch is not part of WM-001.

Use higher-than-High thinking only for a concrete unresolved architectural or debugging
problem. Keep related work in one session to avoid repeated context reconstruction. A cheaper
model is efficient only if it can finish the bounded task correctly; repeated repair loops can
cost more. Final art and animation authoring also require an asset pipeline; choosing a language
model alone does not produce a consistent finished library of paired wrestling animations.
