# PD-103 — Personality descriptions and exceptional trait rules

Status: Implemented as WM-022 rule version 1 following the owner's combined implementation instruction.
Exact thresholds are initial tuning defaults, not established balance. Producers and gameplay effects
remain owned by their management tickets. This document supersedes earlier chat examples.

Implementation conventions: JSON field IDs are camelCase; Rust field identifiers use snake_case.
Policy evidence is evaluated separately by company and visibility; public and confidential cases do
not combine into a public risk. Identity generation is fictional, independently seeded and does not
infer personality from nationality. Imported legacy adjectives are not converted into invented ratings.
There is no arbitrary incident-entry command or manual trait editor in this increment.

## Model and terminology

All people, including non-wrestling staff, use the same identity vocabulary. There are eight personality
ratings and nine shared qualities. Numeric values are integers from 0 through 100; unknown is a separate
value, never zero or 50. There is no personality Overall. A Personality Description is presentation;
Archetype remains reserved for wrestling style.

| Personality   | Exclusive responsibility                           | Low / high meaning                                          |
| ------------- | -------------------------------------------------- | ----------------------------------------------------------- |
| Ambition      | Desire for career advancement                      | Content with current position / strongly advancement-driven |
| Sociability   | Desire to engage socially                          | Reserved / outgoing                                         |
| Empathy       | Consideration of other people's needs              | Self-focused / considerate                                  |
| Loyalty       | Weight given to existing attachments when deciding | Independent / attached                                      |
| Ego           | Desire for recognition and special status          | Unassuming / status-conscious                               |
| Integrity     | Honesty and keeping promises                       | Expedient / principled                                      |
| Temperament   | Control of anger when frustrated                   | Quick-tempered / even-tempered                              |
| Outspokenness | Willingness to express a position                  | Guarded / outspoken                                         |

| Shared quality        | Responsibility                                       |
| --------------------- | ---------------------------------------------------- |
| Communication         | Conveying and understanding information              |
| Leadership            | Coordinating and influencing other people            |
| Creativity            | Producing useful original ideas                      |
| Personal Adaptability | Adjusting to changed working or living circumstances |
| Organisation          | Planning and managing responsibilities               |
| Professionalism       | Meeting workplace obligations reliably               |
| Work Ethic            | Effort invested in preparation and improvement       |
| Stress Management     | Coping with sustained pressure and workload          |
| Teaching              | Helping others learn                                 |

These qualities include conduct and effort, so the interface must not call all nine "abilities".
In-Ring Adaptability remains a separate existing wrestling skill. Performing a promo and communicating
privately also remain distinct. Willingness to accept change is not a second learning-speed bonus.
Work Ethic must not duplicate Professionalism in future training formulas: attendance/compliance and
effort are separate inputs, each consumed once. PD-009's profession-rating weights are unchanged;
WM-032 must define relevant per-role inputs before extending that calculation.

Motivations are one primary and up to two different secondary priorities: Achievement, Fame, Money,
Stability, Belonging, Wrestling Craft, Entertainment, Creative Influence, Exploration, Legacy.
They describe desired outcomes, not ability or morality. Ambition describes pursuit intensity.
Target-specific loyalty, trust, friendship and respect belong to relationships; current confidence,
morale and stress belong to temporary state. Character courage, cheating and betrayal belong to kayfabe.

## Personality naming version 1

The first naming catalogue uses the eight personality axes above plus Professionalism and Work Ethic.
Other qualities appear in their own profile section. Labels describe tendencies, not diagnoses or
claims of wrongdoing. "Free Spirit", "Manipulative" and "Bad Person" are not inferred from these values.

| Stable field ID, in tie-break order | Value 0–30                    | Value 70–100     |
| ----------------------------------- | ----------------------------- | ---------------- |
| professionalism                     | Inconsistent with obligations | Dependable       |
| work_ethic                          | Low training effort           | Hard-working     |
| ambition                            | Content with current position | Ambitious        |
| sociability                         | Reserved                      | Outgoing         |
| empathy                             | Self-focused                  | Considerate      |
| loyalty                             | Independent                   | Loyal            |
| ego                                 | Unassuming                    | Status-conscious |
| integrity                           | Expedient                     | Principled       |
| temperament                         | Quick-tempered                | Even-tempered    |
| outspokenness                       | Guarded                       | Outspoken        |

Algorithm:

1. Select the supplied ratings the viewer is entitled to know. A source-unknown field and a hidden
   field are both ineligible; never derive a public label from a hidden number.
2. Only the explicit low/high rules above produce candidates. Values 31–69 produce no candidate.
3. Rank eligible candidates by distance from 50, descending; break equal distances by the table order.
4. Display the first two as comma-separated descriptors, or the sole descriptor if only one qualifies.
5. When no candidate qualifies and all ten inputs are known, display "No pronounced tendencies".
   Otherwise display "Personality not yet established". With partial inputs, accompany any descriptor
   with "Partial assessment" so it does not imply complete knowledge.
6. Each descriptor exposes its contributing field and assessment provenance without revealing hidden
   values. Use the same rule and tie-break everywhere, with no random naming or extra modifiers.

Examples: "Hard-working, dependable"; "Reserved, loyal"; "Outspoken, status-conscious".
The descriptor is derived, not separately editable. Modders edit source values and provenance.
Refresh after a persisted assessment/personality change or knowledge change, not every mood fluctuation.
The naming function has no mutation or random-number calls. PD-102 still owns discovery/scouting rules;
this function consumes an already-authorised view and does not decide what scouting reveals.

Display bands elsewhere: Very low 0–19, Low 20–39, Moderate 40–60, High 61–80, Very high 81–100.
Naming deliberately requires 70/30 to describe a tendency. High is not universally desirable.
No exact-number display is required in ordinary play; the editor can expose authored values.

## Evidence and event evaluation

Flow: source system records an event; the trait evaluator reads eligible evidence; it evaluates a
versioned rule; a changed trait and its explanation are recorded together. News subsequently reports
that change if the viewer is allowed to see it. A news story is never itself another triggering event.

Every source event supplies: stable event ID, incident/project ID, subject Person ID, event kind,
occurred-on game date, optional company/project scope, source owner, factual outcome, visibility,
and any correction/retraction reference. A source event's meaning is defined by its owning capability.
Do not introduce a generic "bad incident" or automatically treat allegations, criticism or rumours
as verified misconduct. Sexual allegations/investigations are excluded from procedural content.

Every trait rule supplies: stable ID/version, eligible event kinds/outcomes, scope, distinct-incident
count rule, observation window, gain/escalation/removal conditions, visibility, evidence requirements,
downstream effect channel and owner. No active effect exists until its owning gameplay consumer ships.

Evaluation contract:

- Evaluate on committed relevant events, corrections and scheduled window-expiry dates, using game
  time. At date D a W-day window includes events where D-W < occurred-on <= D.
- Count one underlying incident/project once even if reported by multiple systems. Corrected records
  replace their prior contribution; unsubstantiated events contribute nothing to conduct-based rules.
- Replay the same ordered evidence and rule version to obtain the same state. Repeating a delivered
  event does not add counts or another history entry. Use event ID as a tie-break on equal dates.
- Commit evidence, evaluated state and transition history atomically in a future persistence slice;
  save/reload, interrupted evaluation and stale requests must not duplicate transitions.
- Lifecycle history identifies rule/version, contributing IDs, scope, prior/new state and reason.
  Correction history remains visible to authorised viewers instead of silently deleting the record.
- Public reputation rules use public evidence. Confidential workplace evidence can inform authorised
  company decisions but cannot leak through public descriptors, media or sponsor reactions.
- Missing historical coverage means "insufficient evidence", not proof of incident-free conduct.
  Time without work does not prove successful rehabilitation or earn positive reliability labels.
- Manual DB entries carry an as-of date, provenance and explicit expiry/permanence policy. Default
  to review-required on missing expiry; do not silently erase them because the save has no old events.
- Rules bias decisions through their specific consumers. No automatic hiring ban or deterministic
  future wrongdoing follows from a trait. Personality only affects specifically approved event rules.

## Initial exceptional-trait catalogue

The numerical examples below are initial proposals for fixtures. Producers and actual financial,
relationship or performance weights belong to later capability tickets and remain unimplemented.

| Trait and scope                  | Proposed gain / level rule                                                                                                                                                                                                                 | Removal or change                                                                                                                                                                                                         | Owner                                 |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------- |
| Media Liability, person          | 3 distinct verified public media-obligation incidents in 540 days: Active; 5: Elevated. Eligible kinds: unexcused missed media booking or confirmed confidentiality breach. Opinion, criticism and refusal of mistreatment are ineligible. | Recompute rolling count: below 5 becomes Active if at least 3; below 3 becomes Inactive. Label means recent evidence, not moral rehabilitation.                                                                           | WM-036, consuming WM-031 obligations  |
| Wellness Compliance Risk, person | 2 distinct confirmed company-policy breaches in 730 days: Active; 3: Elevated. Only eligible under the actual policy in force at the event date.                                                                                           | Rolling count below 3 reduces level; below 2 becomes Inactive. No inference about present health, addiction or treatment success. Visibility follows evidence.                                                            | WM-031; PD-134 N5–N7 policy direction |
| Sponsor-Friendly, person         | 3 distinct completed external campaigns with positive partner feedback in 730 days. Repeated feedback for one campaign counts once.                                                                                                        | Below 3 expires the endorsement. Campaign reliability is a separate fact from a sponsor's product-specific risk decision.                                                                                                 | WM-034                                |
| Company Icon, person-company     | At least 1,825 cumulative days of service, 3 distinct qualifying major career milestones for that company, and company-audience recognition at least 80/100 at acquisition.                                                                | Historical distinction persists after departure. Display former-company scope; current employability/reputation remains separate. Milestone catalogue and recognition measure must be defined by owner before activation. | WM-040, consuming WM-030/history      |
| Crossover Celebrity, person      | 2 distinct completed external projects in 1,095 days and outside-wrestling recognition at least 70/100. Guest appearances without outcome/recognition evidence do not qualify.                                                             | Becomes historical when count drops below 2 or recognition below 60. Acquisition and retention thresholds differ to avoid flicker. Recognition measure must be defined before activation.                                 | WM-036                                |

No shared severity ladder: risks use Inactive/Active/Elevated; campaign endorsement expires; icons
are historical distinctions; celebrity has active/historical status. Keep historical achievement
separate from a current commercial endorsement.

"Reputational Risk" is a summary of relevant public risks, not an additional stacking trait.
A manually authored real-world reputation concern may contribute to that summary with dated public
provenance; it does not establish guilt or populate Empathy/Integrity scores. "Serious" cannot be
inferred solely from the existence of an allegation. Do not invent real-person misconduct procedurally.
Earlier Cena/Riddle numeric examples are illustrative chat guesses, not database calibration fixtures.

Effects are resolved once per decision channel. For commercial screening, a consumer evaluates the
underlying evidence/risk set once; it cannot subtract for Media Liability and then subtract again for
its Reputational Risk summary. Sponsor-Friendly does not cancel a safety/compliance concern. Declaring
which concern dominates or how multiple independent concerns combine is part of that consumer's ticket.

## Fictional review fixtures

Unspecified naming inputs below are known 50 unless explicitly unknown. Test numbers are design data.

| Worker / scenario               | Inputs or events                                   | Expected result                                              |
| ------------------------------- | -------------------------------------------------- | ------------------------------------------------------------ |
| Mara Vale                       | Work Ethic 95, Professionalism 90                  | Hard-working, dependable                                     |
| Sol Mercer                      | Professionalism 95, Work Ethic 20                  | Dependable, low training effort                              |
| Jules North                     | Sociability 10, Loyalty 90                         | Reserved, loyal; table order breaks equal distances          |
| Ari Storm                       | Outspokenness 95, Ego 80                           | Outspoken, status-conscious; no exceptional risk from scores |
| Ari speaks against mistreatment | Public criticism only                              | No Media Liability evidence                                  |
| Unknown newcomer                | All ten inputs unknown                             | Personality not yet established                              |
| Partially scouted newcomer      | Only Loyalty 90 known                              | Loyal; Partial assessment                                    |
| Hidden Integrity                | Hidden Integrity 5; visible inputs all 50          | No "Expedient" leak; Personality not yet established         |
| Neutral known profile           | All ten inputs 50                                  | No pronounced tendencies                                     |
| Boundary profile                | Ambition 69, then 70                               | No descriptor, then Ambitious                                |
| Duplicate coverage              | One eligible missed booking, five articles         | One incident; no trait                                       |
| Media escalation                | 3, then 5 distinct eligible incidents in 540 days  | Active, then Elevated                                        |
| Media correction                | Exactly 3 eligible incidents; one retracted        | Inactive with correction explanation                         |
| Window boundary                 | Three incidents, one exactly D-540                 | Only two count; Inactive                                     |
| Campaign repetition             | Three feedback messages for one campaign           | One campaign; no Sponsor-Friendly                            |
| Confidential wellness           | Two eligible private policy breaches               | Authorised company risk only; no public reputation signal    |
| Icon departure                  | Qualified icon leaves company A for B              | Historical icon of A; never automatically icon of B          |
| No-work interval                | No assignments and no complete historical coverage | No rehabilitation or positive reliability label              |

Before implementation, executable tests must cover these cases, threshold ties, invalid/missing IDs,
unknown values, out-of-range ratings, duplicate delivery, source corrections, expiry, viewer isolation,
save/reload and rule-version migration. The documentation-stage review does not establish gameplay balance.

## Remaining identity fields and delivery boundary

Language proficiency: Basic, Conversational, Fluent; Native is a separate background flag, not a
learned level above Fluent. Allow several native languages and unknown proficiency. This corrects
the earlier four-rung proposal without requiring extra language skill scores.
Hobbies: zero to five unique interests with Casual/Regular/Passionate involvement. Biography:
Authored/Organic/Hybrid; organic text uses stored facts and history, with unavailable facts omitted.
Exact hobby/language registries and factual biography fields are deferred to the identity-content slice.

WM-022.1 owns a pure personality-description kernel only. A later WM-022 slice can own the reusable
evidence evaluation contract after inspection of the history/persistence boundary. WM-023/031/034/036/040
own their event producers and domain-specific effects. Do not build all later systems inside WM-022.

Research basis: [TEW IX handbook](https://hansmellman.github.io/tew-ix-players-handbook/) for attribute
visibility/conflicts, goals and biography modes; [OOTP manual](https://manuals.ootpdevelopments.com/index.php?man=ootp21&page=player_personalities)
for approximate personality presentation. The rules and thresholds here are WM proposals, not claims
that another simulator uses these formulas.
