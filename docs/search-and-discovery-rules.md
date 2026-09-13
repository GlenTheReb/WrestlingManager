# PD-102 — Search, worker discovery and knowledge rules

Status: Accepted by the owner on 11 September 2026. This is the canonical product rule set for
WM-021 and the discovery interfaces that later capabilities register with it.

## Player promise

Wrestling Manager uses one dedicated Search Workspace rather than a capped dropdown or unrelated
search screens. It is fast enough for a large world, keeps the player's working context, and only
shows information the player could reasonably know.

WM-021 implements the shared query framework and current-data Talent Search foundation. WM-025 must
add active ring identities and dated aliases before the Worker Finder is complete. Later capability
tickets register their own searchable entity data and filters when those records exist; WM-021 must
not create placeholder companies, contracts, shows, storylines or history to make search appear
broader than the simulation.

## Knowledge and visibility

- An established current-era worker's publicly demonstrated wrestling ability is shown exactly when
  meaningful match footage exists. Employer, region and player scouting do not obscure observable
  ability.
- Public facts such as identity, known career history and announced employment are exact.
- Uncertainty remains meaningful for hidden potential, private personality evidence, medical state,
  private contract terms, unannounced creative plans and genuinely obscure rookies or newgens with
  too little evidence.
- Scouting improves projections, context and access to private or uncertain information; it does not
  manufacture fog around widely available footage.
- Uncertainty depends on evidence: recent footage, distribution/archives, worker activity and
  obscurity, report age, regional access and scout knowledge/languages. National wealth is never a
  direct accuracy penalty. A low-resource promotion may be harder to observe, but a well-documented
  worker in the same market remains accurately known.
- Other companies' private contracts and storylines are not globally exposed. Search may surface the
  public person, company or announcement without revealing the hidden record.
- Filters operate only on visible values. Unknown is a distinct state, never silently converted to
  zero, average or a hidden exact value.

## Search Workspace

- Search is a full persistent page, not a capped dropdown. The current Ctrl+K control opens Talent
  Search; WM-039 later upgrades the shell control to category-aware Global Search using the same
  workspace/state contract.
- Results update as the player types. Matching prioritises exact names, then aliases and direct field
  matches, then other indexed record text. Typo tolerance is deterministic and bounded.
- Non-obvious matches explain why they appeared, such as a former ring name, previous company or
  biography phrase.
- Query, category, filters, sort, columns, selection, page and scroll position survive profile
  navigation and return. Named saved views persist across sessions.
- Deep links and browser-style Back restore the exact prior search state.
- Category tabs and a searchable filter catalogue replace long walls of controls. Categories with no
  implemented data remain absent rather than disabled decoration.

## Query and filter semantics

- Different fields combine with AND; multiple selected values inside one field combine with OR.
- A field can be included or excluded. Version 1 does not expose arbitrary nested Boolean logic.
- Numeric ratings use exact minimum and maximum ranges. Quick filters appear as chips; advanced
  filters are searchable and every active rule is visible and individually removable.
- The Worker Finder covers identity and aliases; role and status; canonical stats, style and
  profession ability; known personality and presentation; condition and availability; public
  company/employment data; known career history; and visible relationships.
- Search and filters always combine. Clear/reset behavior, invalid combinations and empty states are
  explicit.
- The company Roster, world Talent Search and category-aware Global Search are separate player tasks
  that reuse shared query primitives. The Roster is scoped to current employment; Talent Search finds
  people; Global Search finds every registered entity and game area.

## Results, lists and scale

- Queries are server-backed and paged. The default page size is 50 and the supported maximum is 100;
  the desktop client never loads the whole world merely to filter it.
- Sorting is stable and deterministic, with record ID as the final tie-breaker. Relevance is the
  default for text search; an intentional domain sort is used when the query is empty.
- Players can choose columns and compare up to four workers without changing the underlying records.
- A saved view stores scope, query, filters, sort and columns. Players can maintain multiple named
  shortlists and one personal blacklist; membership is reversible and has no simulation effect by
  itself.
- Accessible keyboard navigation, focus management and readable result counts are required behavior,
  not optional polish.

## Shared discovery integration

The shared framework is designed to accept People, Companies, Brands, Show Series, individual Shows,
Events, Titles, Storylines, Teams/Stables, Venues, Broadcasters, Training Facilities, player-visible
Contracts and News/History as their owning capabilities are implemented. Each domain owns its data,
privacy rules, result card and specialist filters.

Entity hubs use the same engine for scoped current and historical search. Booking later exposes one
consistent contextual Reference Drawer for roster, teams, titles and storylines instead of scattering
unrelated lookup buttons around the booking screen. The complete Entity Hub, Reference Drawer and
navigation design follows PD-134 rules D8 and E1–E7 and the relevant profile/booking tickets; it must preserve this
search state contract.

## Rejected alternatives and implementation boundaries

Rejected: dropdown-only global search, client-side whole-world filtering, artificial uncertainty for
publicly demonstrated ability, leaking private CPU-company records, separate incompatible search
engines per domain, and runtime generative AI for matching or result explanations.

WM-021 owns the shared query primitives, persistent Search Workspace, Talent Search foundation,
saved views, shortlists and blacklist. WM-025 owns ring identity/alias integration; WM-024 and WM-032
consume its worker discovery and knowledge rules.
WM-011/012, WM-031, WM-033, WM-036 and WM-039 register company, contract, event, media and history
content later. Persisted search data must be versioned per save/player and remain backward compatible.
