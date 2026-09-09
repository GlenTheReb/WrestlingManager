# Simulation contract — planned, not implemented

The foundation stores a seed and engine version but does not simulate matches, shows or
calendar days. This document constrains the next milestones.

## Inputs and outcomes

A match accepts the booked plan, conditions, people, referee/agent, audience cohorts,
story context, production and broadcast time. The booked result is locked by default.
Any enabled exceptional change needs a rare justified condition and an explanatory event;
randomness alone cannot reverse the winner. Delegation is an explicit plan choice.

Performers choose action intentions using repertoire, positioning, fatigue, restrictions,
cooperation, recent repetition and allowed freedom. Resolve execution, opponent response,
selling, referee awareness, crowd response and physical consequences. Flexible phases
support different performance structures without enforcing one universal match template.

## Event and report separation

Emit ordered timed events with stable identities. Rendering, textual commentary and instant
simulation consume the same result stream. Future 1×/2×/4× speed, key-moment skipping and
live instructions have explicit semantics: instruction receipt occurs at a logical tick,
possibly delayed or misunderstood according to context, never at a renderer frame boundary.

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

Property-test locked winners, legal references, valid conditions and once-only effects.
Golden fixtures pin representative output digests and engine versions. Balance batches
compare cohort/style distributions instead of maximising one rating. Summary simulation
for distant events must be calibrated against the detailed path. Decades-long soak tests
track invariants, size and timings. None of these simulation gates has been run yet.
