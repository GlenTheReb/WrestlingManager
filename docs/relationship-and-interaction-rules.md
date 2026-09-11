# PD-104 — Relationship, memory and interaction rules

Status: Rule version 1 implemented by WM-023 following owner approval on 11 September 2026.
These rules supersede exploratory chat examples. They are initial gameplay defaults, not long-save
balance evidence.

## Boundaries

Personal relationships record one person's real-life view of another person. They are directional:
A may trust B while B distrusts A. They are not match chemistry, a character feud, storyline heat,
worker-to-company sentiment or contract satisfaction. Those systems may produce explicit dated
relationship events later, but they cannot silently reuse or overwrite this state.

Until WM-030 creates playable people and company roles, worker conversations use a separate
worker-to-current-management relationship. It represents how the worker experiences the player's
management, not affection for the company. WM-030 must preserve or deliberately migrate that history
when it introduces the player's persistent Person identity and dismissal/role changes.

## Relationship state

Every directional relationship contains four independent integer values:

| Dimension | Range    | Responsibility                                    |
| --------- | -------- | ------------------------------------------------- |
| Affinity  | -100–100 | Personal warmth or dislike                        |
| Respect   | -100–100 | Professional regard or dismissal                  |
| Trust     | -100–100 | Confidence that the other party will act reliably |
| Tension   | 0–100    | Current unresolved strain or active conflict      |

Management uses the same stored dimensions but presents Affinity as **Rapport**. No universal
relationship Overall exists. The ordinary interface exposes descriptive bands and an explained
summary rather than exact internal values. The future editor may expose exact values.

Initial worker relationships are generated without consuming the established world random stream.
Every pair has a stable directional, ID-based neutral baseline that does not change when a worker's
ratings develop. Only relationships with events or memories need stored rows. World generation stores
at most two deterministic shared-school links per worker, preventing quadratic save growth while
giving some starting links factual context. Nationality never determines morality or hostility.
Shared-school memories describe common background, not invented friendship or a claim that the
workers trained together at the same time.

## Memories, change and decay

A relationship change requires a validated event containing a stable ID, subject, other person,
game date, kind, factual summary, signed impacts, salience, source and optional active-until date.
Exact repeated delivery is idempotent. A reused event ID with different facts is rejected; a new event
based on a stale relationship revision is also rejected. Event, state and memory commit together.

History is never deleted when an influence expires. Active-until controls whether a memory remains a
current influence; the interface keeps the historical fact and identifies inactive influence.
Affinity, respect and trust do not drift toward neutral merely because time passes. Unresolved tension
eases by one point on the first and fifteenth of each game month. Future event-owning tickets must
define their mappings explicitly; a booked match or kayfabe betrayal is not automatically a real
personal incident.

## Player interaction catalogue

The current company has four management-attention points per game day. Cooldowns prevent mechanical
spamming; they are not a claim that people cannot speak informally between represented conversations.

| Interaction            | Cost | Availability and repeat rule                              |
| ---------------------- | ---: | --------------------------------------------------------- |
| Introduce yourself     |    1 | Once for the current management relationship              |
| Check in               |    1 | Seven-day cooldown                                        |
| Praise recent work     |    1 | A performance in the last 28 days; fourteen-day cooldown  |
| Offer encouragement    |    1 | Morale or confidence below 60; fourteen-day cooldown      |
| Ask for creative input |    1 | Fourteen-day cooldown                                     |
| Discuss a colleague    |    1 | Select another roster member; seven-day cooldown          |
| Clear the air          |    2 | Management tension at least 20; twenty-eight-day cooldown |

The server recalculates availability inside an immediate transaction. Disabled interface controls are
convenience, not authority. A request supplies a unique ID and expected management revision so exact
retries return the original outcome while stale or conflicting requests cannot apply twice.
Colleague choices come from a server-paged, searchable query capped at 32 rows per request; the
profile never embeds the entire roster as an interaction-option payload.

## Lightweight dialogue rules

Dialogue uses authored response families rather than a network or runtime language model. Resolution
reads only documented inputs: the selected interaction, relevant personality/shared-quality values,
current management rapport/trust/tension, morale or confidence prerequisites, recent work and any
selected colleague relationship. Unknown identity values are neutral inputs, not invented knowledge.

The deterministic result selects Warm, Open, Guarded or Defensive tone and one of several authored
phrasings using stable worker/action/history context. Responses can mention the worker, recent
performance or selected colleague. Effects update management rapport/respect/trust/tension and
bounded morale or confidence. The exact clamping-aware numeric deltas are stored with the result;
plain-language effects are derived from those values and validated on reload. The result records its
factors, appears in conversation history and creates a private People inbox note. No dialogue text
creates an unrecorded factual event.

## Verification fixtures

Executable tests cover bounded deterministic generation, asymmetric state, sparse persistence,
paged context targets, once-only introduction, cooldowns, company-wide daily attention, stale writes,
idempotent retries, conflicting IDs, structured morale/confidence effects, directional event updates,
memory history, schema-5 migration and interrupted-upgrade retry, backup creation and invalid
stored-state rejection. Future long-save work must tune distributions, decay and management-effect
strength before treating these values as balanced.
