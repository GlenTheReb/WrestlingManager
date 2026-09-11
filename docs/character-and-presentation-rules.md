# PD-106 — Person, character and presentation rules

Status: Accepted 11 September 2026. These rules unblock WM-025 and define the identity boundary used
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
- Masks, face coverings, billed locations and similar presentation facts belong to Character tenure.
  Unmaskings and reveals are explicit events; a mask never creates a second biological person.
- Existing pre-PD-106 saves migrate the legacy worker name into an initial active Character. The legal
  name remains unknown unless a canonical source already supplies it; migration must not invent one.

## Gimmick and alignment vocabulary

- A gimmick combines a player-facing name and short concept with structured presentation tags. Tags
  support simulation and search without forcing every character into one rigid preset catalogue.
- The supported alignment intent is **Hero**, **Villain**, **Tweener** or **Unaligned**. This describes
  intended wrestling presentation, not morality or personality. Audience perception is stored
  separately and may differ by region, company, segment or recent events.
- Gimmick fit is contextual: worker comfort, performance skills, company product, audience, momentum,
  freshness, consistency and execution all matter. No gimmick type is universally superior.
- Character popularity, personal popularity, company support and wrestler ability remain separate
  facts. Changing a name or alignment does not silently change any of them.

## Character-change workflow

1. The player proposes a rename, alignment turn, mask change, presentation adjustment or full reboot.
2. The worker and relevant staff respond using contract control, trust, personality, adaptability,
   concept fit, company culture and the size of the change.
3. Accepted changes receive preparation and intended debut dates. Training, rehearsal, creative work
   and production support can improve readiness; a rushed or forced change carries visible risk.
4. The change becomes public only through an explicit debut, announcement or completed show event.
5. Audience response and worker comfort settle over time. The history keeps the proposal, acceptance,
   launch and later adjustments.

Emergency changes are allowed when context permits, but they are not free instant edits. The interface
shows estimated preparation time, acceptance, likely gains/losses and the evidence behind the advice.

## Knowledge and interface rules

- Publicly performed names, alignments and presentation are normally exact. Unannounced plans,
  contract restrictions, worker objections and future turns remain private to authorised viewers.
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
