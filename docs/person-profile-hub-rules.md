# Person profile and career hub rules

Status: PD-133 accepted by the owner on 12 September 2026. These rules make WM-024
decision-complete; they do not authorise implementation.

## Player promise

One lively information hub answers who a person is, what they can do, how they are doing, what has
happened to them and what the player can legitimately do next. It combines Football Manager-style
at-a-glance usefulness with wrestling-specific depth without copying another game's layout or terms.

## Profile shell and navigation

- Every Person uses the same profile shell. Wrestler, commentator, manager, referee, road agent,
  booker, trainer and other professions add role-relevant panels rather than separate person records.
- A quick side panel supports reference while booking. **Full Profile** opens a persistent page and
  returns to the exact prior workspace, filters, tab and scroll position.
- The full navigation is Overview, Attributes, Character, Career, Matches & Appearances,
  Relationships, Contract, Development and Media. Tabs appear only when they have real canonical
  information or a clearly explained unavailable state.
- One primary action and an organised contextual **Actions** menu replace scattered button walls.
  Full worker comparison remains available in addition to quick comparison.

## Overview as the person's information hub

The first page is dense but readable. It leads with portrait, active ring identity, company/brand,
status, alignment, current Character, age and relevant biographical facts. The centre shows the six
wrestling groups and important sub-stats, current-Archetype Overall, Discipline/role fit, regional
popularity, momentum, morale, condition, contract and next-booking information. A bounded side rail
shows recent social posts, news, form, alerts and important relationship or availability changes.

Deeper evidence, histories and editing workflows live behind their owning tabs. Overview cards link
to those destinations instead of duplicating their state.

## Ratings, roles and knowledge

- Every Person has canonical wrestling sub-stats, including people currently used only as
  commentators, managers or other non-wrestling roles. An untrained person normally has appropriately
  weak Ringcraft/Fundamentals but may have transferable Movement, Physicality, Psychology or
  Entertainment ability. Exceptional athletic transitions are possible but rare and require the
  same training, safety and evidence rules as anybody else.
- Profession Skills and Role Experience remain independent. The profile switches role context and
  never treats commentary, booking or managing ability as wrestling ability.
- Current-era public footage and the internet make established workers' demonstrated ability broadly
  knowable. Exact values are shown for the player's roster and well-documented people. Only genuinely
  obscure, private, medical, contractual, potential or unevidenced facts use estimates, confidence or
  **Unknown**; uncertainty is never artificial busywork.
- Wrestling values use one accessible quality scale: weak red, developing amber, capable yellow,
  strong light green and elite green. Group headings and icons provide category structure; hue never
  changes a value's meaning and colour is never the only cue. Thresholds are user-configurable later.
- Popularity is a separate 0–100 regional measure. Momentum, marketability, company exposure,
  reputation and current-Archetype Overall remain distinct.
- Objective match/performance dimensions and subjective journalist opinions are displayed separately.
  Every review names its outlet/reporter and can explain the reporter's taste, credibility and main
  reasons so the player never mistakes an opinion for simulation truth.

## Identity, personal record and career history

- Wrestling surfaces lead with the company/date-correct ring name. Legal name, previous aliases,
  masks and company-specific Characters remain scoped by PD-106 knowledge rules.
- Personal and billed facts are separate: date of birth/age, nationality, residence, languages,
  hobbies, pronouns and biography do not overwrite billed hometown, height, weight or presentation.
- The career timeline pages company and brand tenures, contracts, roles, Characters, title reigns,
  awards, injuries, absences, retirements, returns and important incidents without rewriting history.
- Retirement is a state, not deletion. Where health and willingness allow, the Actions menu may open
  discussions for an appearance, angle, one-off match, short stint or comeback.

## Matches, appearances and media

- Match history supports date/company/opponent/result/finish/stipulation/title/duration filters and
  includes crowd response, performance dimensions, incidents and attributed journalist ratings where
  those facts exist. Angles, interviews and other appearances share a searchable career timeline and
  can be filtered into focused views.
- News, public social posts, interviews, rumours and highlights form the Media tab and overview rail.
  Private communication stays in the inbox or interaction history.
- All related people, companies, shows, titles, reports and articles deep-link without destroying
  current profile or booking context.

## Moveset and protected-move etiquette

- The profile treats the moveset as a serious working repertoire: finishers, secondary finishes,
  signatures, common offence, counters, team moves and situational moves show proficiency, safety,
  risk, style fit, setup/recipient requirements, legal contexts, recent usage and learning state.
- A generic move family and a worker's named/presented version are separate. Protection normally
  applies to a signature presentation or finish usage, not to every mechanically similar move.
- A move association may be Open, Associated, Protected or Restricted, with a claimant, company or
  legacy custodian, strength, territory/context and documented exceptions. Custom databases configure
  these facts; they are not inferred from copyrighted real-world names.
- The player can request permission through the same text-chat interaction system. Permission may be
  granted for one match, one programme, a time window or permanently; the claimant may negotiate the
  user, frequency, finish status or presentation, suggest an alternative, or refuse.
- Booking without permission is not always mechanically forbidden: it may cause claimant anger,
  relationship damage, locker-room tension, negative media/fan comparisons or company discipline when
  the breach becomes known. Separate safety/proficiency rules can block a move that cannot be performed
  responsibly. The interface explains both etiquette and safety rather than merging them.
- Retired or deceased claimants may leave a company-, family- or legacy-custodian rule. Otherwise the
  association becomes social history, not an impossible permission prompt.

WM-010 owns repertoire/move facts and permission claims; WM-023 owns relationship consequences;
WM-024 displays them and initiates the contextual interaction; WM-031 may add contractual creative
control; WM-032 owns training.

## Development and interaction

- Development uses a small number of realistic concurrent focus plans, not unlimited training of
  every stat. A plan selects a role, style, move family or tightly related skill cluster; age, body,
  health, base ability, trainer, facilities, schedule, learning capacity and potential constrain pace
  and ceiling. Staff show ranges and confidence, never guaranteed gains.
- The conversation interface is a chronological text chat. Player prompts and person replies remain
  visibly paired. Available topics and responses depend on context, personality, relationship,
  morale, status, promises and earlier exchanges; deterministic authored rules remain the baseline,
  with no required runtime language model.

## Safety, performance and delivery

- Historical snapshots are immutable. Current facts change only through commands owned by their
  domain; editor changes are available only in editor mode and are visibly identified.
- Histories, feeds and comparisons use indexed server-side filtering, stable sorting and bounded
  pagination. The client never loads an entire career or world to construct a profile.
- Loading, empty, error, stale, private and unknown states are explicit. Actions explain permission,
  cooldown and eligibility failures.
- WM-024 is delivered as one execution ticket/branch/review cycle with internal phases. It composes
  existing canonical data and typed extension points; it does not invent fake functioning contract,
  title, training, social or move systems before their owning capabilities exist.

## Research basis

- The TEW IX handbook supports one worker record with multiple possible roles, 0–100 skills,
  contextual worker conversations, regional popularity, health, organic biography and move sets.
  WM retains the useful breadth but makes moves causal and the interface contextual.
- Football Manager profile patterns support a strong overview, quick visual attribute reading,
  recent-form evidence, comparison and context-preserving action menus. WM uses these principles in an
  original wrestling-specific shell.

References: [TEW IX Player's Handbook](https://hansmellman.github.io/tew-ix-players-handbook/),
[Football Manager 2024 Mobile feature overview](https://www.footballmanager.com/features/football-manager-2024-mobile-new-features-revealed).
