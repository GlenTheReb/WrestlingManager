# Simulation contract

The current prototype implements a deterministic one-second match/show slice. This document records
the broader contract that later match, direction, presentation and world milestones must preserve.

## Inputs and outcomes

A match accepts the booked plan, conditions, people, referee/agent, audience cohorts, story context,
production and broadcast time. The booked result is locked by default. Per
[PD-112](live-match-direction-rules.md), an explicit player-authorised live amendment may change a
future finish or winner after required communication and worker responses; an agent or random roll
cannot do so autonomously. Delegation is an explicit plan choice.

Performers choose action intentions using repertoire, positioning, fatigue, restrictions,
cooperation, recent repetition and allowed freedom. Resolve execution, opponent response,
selling, referee awareness, crowd response and physical consequences. Flexible phases
support different performance structures without enforcing one universal match template.

## Event and report separation

Emit ordered timed events with stable identities. Rendering, textual commentary and instant
simulation consume the same result stream. Playback speed, key-moment skipping and live instructions
have explicit semantics: instruction receipt occurs at a logical tick representing a credible
sequence-derived referee opportunity, possibly delayed, misunderstood or refused according to
context, never at a renderer frame boundary.

Report execution, psychology/pacing, engagement, storyline advancement, presentation,
protection, safety, originality/memorability, broadcast quality and commercial effect
separately. Different cohorts and stakeholders can disagree. Explanations cite recorded
causes, such as fatigue, chemistry, preparation or repetition, without printing equations.

## Determinism and consequences

Canonical order, version-pinned ChaCha streams, stable seed derivation and controlled
numeric representations are prerequisites. Commit the show's history and consequences
atomically and guard against applying the same show twice. Replaying/rendering a completed
show is a read operation. Social templates derive from committed facts and do not alter
canonical outcomes.

## Validation gates

Property-test locked winners except for recorded player-authorised amendments, legal references,
immutable executed history, deterministic future replanning, valid conditions and once-only effects.
Golden fixtures pin representative output digests and engine versions. Balance batches
compare cohort/style distributions instead of maximising one rating. Summary simulation
for distant events must be calibrated against the detailed path. Decades-long soak tests
track invariants, size and timings. The current slice has focused determinism tests; the complete
future gate has not yet been run.
