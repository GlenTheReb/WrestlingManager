# PD-106 — Person, character and presentation rules

Status: Accepted and decision-complete 12 September 2026. These rules unblock WM-025 and define the identity boundary used
later by profiles, search, booking, creative history, media and custom databases.

## Player promise

A human being and the wrestling identities they portray are related but never interchangeable. Lists
show the identity relevant to the current wrestling context, while profiles and history let the player
understand the person, every known character and when each identity was used.

## Canonical records

- **Person** owns legal/real name when known, biography, languages, interests, personality, personal
  relationships, health, employment and profession skills.
- **Character** owns its stable ID, ring name, optional aliases, concept, visual presentation, mask
  state and audience alignment. A person may portray several characters over time or in different
  companies when their agreements allow it.
- **Character tenure** owns the company, optional brand, start/end dates, status and the presentation
  used during that period. History references the tenure/character that existed when an event happened.
- A rename or presentation adjustment can retain the same Character. A deliberate clean break may
  create a new Character; the interface explains the historical consequence before confirmation.
- Real-life state, kayfabe claims and production presentation remain separately typed. A storyline
  injury, marriage, firing or retirement cannot mutate real health, relationships or employment.

## Names, aliases and masks

- Roster, booking and wrestling results lead with the active ring name for the relevant company and
  date. The real/legal name appears only in appropriate personal/profile and editor contexts.
- Search matches the active ring name first, then known former names and aliases. Match explanations
  state which identity matched. Private or genuinely unknown legal names are not leaked.
- Name history is dated and never rewritten. Match, title and show history retains the identity shown
  to that audience at the time while still linking to the same Person.
- A rename before a character's public debut is normally low-risk. Renaming an actively presented
  character without a suitable absence or story reason may reduce character acceptance, recognition,
  momentum continuity and worker comfort; it never reduces wrestling ability. The expected penalty
  depends on exposure, name similarity, preparation and explanation rather than one universal number.
  An appropriately long or narratively credible break reduces that risk.
- Masks, face coverings, billed locations and similar presentation facts belong to Character tenure.
  Unmaskings and reveals are explicit events; a mask never creates a second biological person.
- Existing pre-PD-106 saves migrate the legacy worker name into an initial active Character. Because
  current WM-generated full names are canonical person names, schema-7 saves use that value as the
  initial legal name too. Future imports keep a genuinely unknown legal name unknown.

## Gimmick and alignment vocabulary

- A gimmick combines a player-facing name and short concept with structured presentation tags. Tags
  support simulation and search without forcing every character into one rigid preset catalogue.
- Curated tags drive simulation. Optional custom tags let the player describe unusual concepts but
  remain descriptive until mapped to a validated simulation tag; custom prose cannot grant a direct
  modifier. The player controls the creative brief—concept, tags, intended audience, realism,
  presentation intensity and abilities to emphasise—not arbitrary numerical bonuses.
- The supported alignment intent is **Face**, **Heel**, **Tweener** or **Unaligned**. This describes
  intended wrestling presentation, not morality or personality. Audience perception is stored
  separately and may differ by region, company, segment or recent events.
- Gimmick fit is contextual: worker comfort, performance skills, company product, audience, momentum,
  freshness, consistency and execution all matter. No gimmick type is universally superior.
- A gimmick never edits permanent worker stats. It affects contextual delivery, audience response,
  character acceptance, momentum/popularity opportunity, freshness and the risk of an unbelievable or
  confusing performance. Advice explains which inputs created the forecast; the eventual result
  follows performed evidence rather than a hidden fixed buff.
- Character popularity, personal popularity, company support and wrestler ability remain separate
  facts. Changing a name or alignment does not silently change any of them.

## Alignment intent and audience perception

- **Face**, **Heel**, **Tweener** and **Unaligned** are the complete player-set intent catalogue.
- Observed perception is derived separately for the relevant company, region and audience context. It
  records perceived role (**Face**, **Heel**, **Mixed** or **Unclear**), reaction (**Cheered**,
  **Booed**, **Mixed** or **Indifferent**), reaction intensity, character acceptance and whether the
  response matched company intent.
- Boos are not automatically failure: a villain receiving strong intended boos may be succeeding,
  while a hero receiving hostile boos is an intent mismatch. Popularity, momentum, drawing power and
  wrestling ability remain separate rather than being folded into perception.
- WM-025 records intent, the vocabulary and dated response evidence; it does not invent one universal
  numeric perception score before later regional audience/product systems can derive contextual views.

## Character-change workflow

1. The player proposes a rename, alignment turn, mask change, presentation adjustment or full reboot.
2. The company proposes the change; the worker may accept, negotiate or refuse using currently
   implemented trust, personality, adaptability, concept fit, company culture and change size.
   WM-031 later adds contractual creative-control constraints without replacing this workflow.
3. Accepted changes receive an advisor-estimated readiness range and a player-selected intended debut
   show/date. Training, rehearsal, creative work and production support can improve readiness; rushing
   remains allowed but carries visible comfort, execution and acceptance risk.
4. The change becomes public only through an explicit match, angle, promo, announcement or completed
   show event selected by the player; reaching a date cannot silently publish it.
5. Audience response and worker comfort settle over time. The history keeps the proposal, acceptance,
   launch and later adjustments.

Emergency changes are allowed when context permits, but they are not free instant edits. The interface
shows estimated preparation time, acceptance, likely gains/losses and the evidence behind the advice.

Character retirement is distinct from Person career retirement. A Person may stop using one Character,
continue under another and later revive a retired Character with a new dated tenure and public return.

## Multiple identities and public knowledge

- A Person may openly use different Characters in different companies; a separate ring name does not
  imply audiences believe they are different people.
- A genuinely secret alternate identity normally requires a mask or credible visual/voice concealment.
  Each relevant audience/company link is **Private**, **Rumoured** or **Public**. The simulation retains
  the canonical Person link while interfaces reveal only what the viewer is authorised to know.
- Exposure, recognisable body/style, publicity, leaks, unmasking and explicit reveals can move the link
  toward Rumoured or Public. Concealment and limited exposure may preserve privacy. A reveal is a dated
  event and never creates or merges biological people.
- Within one company a Person normally has one active public Character per brand context. A simultaneous
  secret/alternate Character requires an explicit flag and valid concealment/knowledge state; invalid
  overlaps fail validation rather than being silently accepted.

## Knowledge and interface rules

- Publicly performed names, alignments and presentation are normally exact. Unannounced plans,
  contract restrictions, worker objections and future turns remain private to authorised viewers.
- Known legal names are editable through custom-database/everything-editor workflows, not ordinary
  booking actions. The detailed personal profile may show an authorised known legal name; wrestling
  lists continue to lead with the active ring identity.
- The Worker Finder remains a Person-backed search foundation until WM-025 lands. Final integration
  must display active ring identity, search dated aliases and preserve the matched identity reason.
- The full worker profile is a navigable entity hub. It leads with active Character, company and
  current status, with separate Character/Presentation and Personal tabs plus dated career history.
- Quick previews may use a drawer or overlay, but they must link to the full profile and never replace
  its navigable history.

## Ownership and exclusions

WM-025 owns these records and transitions. WM-024 composes the full profile after the core WM-025
identity model exists. WM-026 owns teams/factions and on-screen group roles; WM-028 owns storylines;
WM-029 owns performed scenes/dialogue; WM-031 owns contractual creative control. PD-106 does not add
titles, teams, storylines, contracts or scripted angle execution.
