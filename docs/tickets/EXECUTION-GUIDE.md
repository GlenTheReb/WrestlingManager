# Ticket execution guide

This guide turns the Wrestling Manager roadmap into bounded, intentional implementation work.
The numbered WM roadmap entries are **capability briefs**. Except for completed WM-001, they are
not permission to implement and are usually too large for one branch. Before code changes, the
selected capability must be decomposed into an execution ticket that follows
[the execution-ticket template](TICKET-TEMPLATE.md).

## Source-of-truth order

When two documents appear to disagree, use this order and stop for an owner decision when the
conflict changes behavior:

1. The owner's latest explicit decision in the current conversation.
2. Implemented behavior and invariants recorded in root `SDD.md`.
3. Accepted entries in [`docs/product-decisions.md`](../product-decisions.md).
4. The selected `WM-###.md` capability brief and its accepted dependency handoffs.
5. [`ROADMAP.md`](ROADMAP.md) for broad scope and [`docs/model-task-plan.md`](../model-task-plan.md)
   for sequencing, estimates and model recommendations.
6. Competitive references as inspiration only. They never authorize copied names, screens,
   mechanics or assets.

Proposed text is not implemented fact. A missing rule is an open decision, not permission to invent
one during coding.

## Smallest useful context packet

The implementing model reads only:

1. `SDD.md` Current checkpoint plus the exact headings named by the execution ticket.
2. This guide and the selected capability brief.
3. Accepted product decisions named by that brief.
4. Only the dependency handoff sections and code paths explicitly named by the execution ticket.
5. Focused source symbols found by tracing the current execution path.

Do not reread the entire repository, every roadmap ticket or all of `SDD.md`. If an implementation
discovers a new cross-system invariant, record it in the SDD and the capability handoff so the next
model does not have to rediscover it.

## Capability brief versus execution ticket

A capability brief explains the complete destination, boundaries and cross-system contract. An
execution ticket is normally two to five focused engineering days and delivers one reviewable
behavior. A capability may need several sequential execution tickets, for example domain model,
persistence/migration, simulation behavior, query surface, interface and balance evidence.

An execution ticket is ready only when it has:

- one player-visible or architecture-enabling outcome;
- explicit included and excluded behavior;
- exact domain types or concepts it owns, consumes and emits;
- accepted decisions, with no behavior-changing question hidden in implementation notes;
- file/symbol ownership established from the current repository rather than guessed paths;
- migration and compatibility impact;
- deterministic, unit, integration and native checks appropriate to its risk;
- a stop condition that prevents opportunistic adjacent features.

Use a `feature/` branch named for the execution outcome only after current owner authorization.
Do not create one long-lived branch for an entire multi-week capability.

## Intentionality gate

Before implementation, answer these questions in the execution ticket:

1. **Player decision:** What new informed choice, understanding or creative control does this add?
2. **Canonical cause:** Which stored facts and rules produce the result?
3. **Consequence:** Which state changes, and where can the player inspect why?
4. **Layer:** Is each field simulation fact, real-life social state, kayfabe state or presentation?
5. **Reuse:** Which existing contract is extended instead of creating a parallel source of truth?
6. **Absence:** What happens when information, scouting knowledge or optional assets are missing?
7. **Abuse case:** How could invalid data, stale revisions, mods or repeated actions break it?

If a feature cannot answer the first three, it is probably decorative. If it cannot answer the
fourth or fifth, it is likely to create incoherent state. Stop and revise the ticket.

## Anti-hallucination rules

- Never add a field, score, modifier, random event or control only because another game has it.
- Every requirement is labelled owner-confirmed, implemented invariant, accepted design decision or
  proposal. Proposals cannot silently become mechanics.
- Do not invent exact weights, thresholds, probability tables, tax/legal rules, medical outcomes,
  content taxonomies or AI behavior. Record an open decision or use a clearly approved fixture.
- Do not display a control until its command, validation, persistence, consequence and failure state
  exist. Disabled previews must say they are planned.
- Do not use presentation text to create canonical facts. News, commentary and reviews must cite
  stored events or deterministic rules.
- Do not couple simulation correctness to portraits, sound, video, a renderer or network service.
- Do not claim broad ticket completion from one thin slice. Update the capability checklist and
  leave the remaining slices explicit.

## Cross-system contracts

Every execution ticket records its impact on this matrix:

| Contract     | Required question                                                           |
| ------------ | --------------------------------------------------------------------------- |
| Domain       | What is the single canonical type and stable identity?                      |
| Persistence  | Does the save schema change, and how do old/paused careers upgrade?         |
| Simulation   | Does deterministic behavior or its fingerprint change intentionally?        |
| Commands     | What bounded query/mutation exists, with revision and validation behavior?  |
| Interface    | What loading, empty, error, unknown and keyboard states exist?              |
| History      | Which dated event explains the change later?                                |
| AI           | Can CPU companies use the same rule without privileged information?         |
| Modding      | Are IDs, catalogues and validation stable enough for future custom content? |
| Presentation | Which facts/cues are exposed without deciding outcomes?                     |
| Safety       | Can the action duplicate, overwrite, exploit or reveal hidden information?  |

Write “no impact” with a reason; never leave a row implicit.

## Handoff contract

At the end of an execution ticket, its record must state:

- actual files and canonical types changed;
- behavior now implemented, separately from remaining capability scope;
- migrations, compatibility rules and deterministic-version effects;
- commands/tests run and exact results;
- limitations, rejected alternatives and new accepted/open decisions;
- exact current SDD heading/line references;
- the smallest safe next execution ticket.

The SDD remains the architectural source of truth. Ticket records preserve local reasoning and
evidence; they do not become competing architecture documents.
