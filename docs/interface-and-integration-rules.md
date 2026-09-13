# Wrestling Manager interface and integration rules

Status: Owner-confirmed direction on 11 September 2026. These rules apply to every player-facing
execution ticket; accepted PD-134 rules D1–D9 and E1–E7 settle the destination visual, navigation,
search and contextual-reference direction.

## Research before design

- Inspect the relevant TEW IX counterpart when one exists and at least one current real-world
  wrestling article, results page or operational workflow that serves the same player need.
- Record the useful information hierarchy and workflow in the execution ticket. References are
  research inputs, not permission to copy layouts, wording, brands or assets.
- Improve the reference deliberately: reduce repeated navigation, preserve working context, expose
  causes beside results and make the player's likely next action obvious.

## Design from the player's task

Every screen must answer, in order: Where am I? What changed or needs attention? What can I decide or
do now? What evidence helps that decision? Where can I inspect the underlying person, company, show
or history?

- Lead with identity, status and the primary decision; place supporting detail behind clear tabs,
  filters or progressive disclosure.
- Keep dense information scannable through meaningful grouping, comparison, sorting and consistent
  labels. Do not solve density by hiding essential facts across many screens.
- Preserve query, filter, selection, tab, scroll and draft state when navigating to supporting
  records and back.
- Define loading, empty, unknown, restricted, stale, error and keyboard/focus behavior alongside the
  normal state.
- Pictures are contextual evidence or identity aids, not decoration. Prefer a worker portrait,
  company mark, show/venue image or category artwork in that order; fall back to a consistent
  silhouette/icon and omit the image when none is useful.
- A quick-search field handles known targets without forcing filter setup. A compact filter-icon
  button opens/closes a left drawer with grouped, searchable criteria; active rules remain visible as
  removable chips when the drawer closes. Native Ctrl-click multi-select walls are not acceptable.
- Company Roster, world Talent Search and Global Search are distinct screens that share interaction
  patterns. Do not label a worker-only command as global search or treat world discovery as the
  company's employed roster.
- A profile quick preview may use a drawer/overlay. The complete FM-style worker, company, show or
  other entity profile is a navigable page with stable tabs, contextual actions and Back restoration.

## Build a connected game, not isolated menus

- Each fact has one owning system. Other screens consume its projection and deep-link back instead of
  storing a second UI-only copy.
- Related systems meet at the player's point of work: booking can reference roster, teams, titles and
  storylines; profiles expose relevant matches, news and actions; reports connect to workers, shows,
  venues, consequences and media.
- Use one consistent contextual drawer or linked panel when supporting information is needed during a
  task. Do not scatter unrelated shortcut buttons across the screen.
- A player action calls the owning command, records a dated consequence where appropriate and makes
  the result inspectable from all relevant surfaces.
- Missing future systems stay absent or are clearly marked; interfaces must not fabricate placeholder
  data or controls merely to look complete.

## Review gate

Before accepting a player-facing slice, verify the primary workflow at its intended density and at a
compact desktop width. Confirm that a player can find the main information and complete the main task
without prior explanation, that Back restores context, and that linked systems agree on the same
facts. The handoff records the references consulted and the specific improvements made over them.
