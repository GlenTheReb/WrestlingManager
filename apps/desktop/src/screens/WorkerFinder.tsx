import { useDeferredValue, useEffect, useMemo, useRef, useState } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type {
  NumberRange,
  RosterRow,
  WorkerFilterOptions,
  WorkerSearchRequest,
  WorkerSortKey,
} from '@wm/contracts';
import { gameApi, errorMessage } from '../api';
import { defaultWorkerSearch, useNavigation } from '../navigation';
import { ErrorNotice, Overlay } from '../game-ui';
import s from '../Game.module.css';

type ColumnKey =
  | 'name'
  | 'age'
  | 'personality'
  | 'archetype'
  | 'overall'
  | 'movement'
  | 'physicality'
  | 'ringcraft'
  | 'psychology'
  | 'fundamentals'
  | 'entertainment'
  | 'fatigue'
  | 'morale'
  | 'momentum'
  | 'status';

type ChoiceFilterKey =
  | 'nationalities'
  | 'languages'
  | 'schools'
  | 'archetypes'
  | 'primaryDisciplines';

type ExcludedChoiceFilterKey =
  | 'excludedNationalities'
  | 'excludedLanguages'
  | 'excludedSchools'
  | 'excludedArchetypes'
  | 'excludedPrimaryDisciplines';

const columns: { key: ColumnKey; label: string; sort?: WorkerSortKey }[] = [
  { key: 'name', label: 'Worker', sort: 'name' },
  { key: 'age', label: 'Age', sort: 'age' },
  { key: 'personality', label: 'Personality' },
  { key: 'archetype', label: 'Archetype' },
  { key: 'overall', label: 'Style OVR', sort: 'overall' },
  { key: 'movement', label: 'MOV', sort: 'movement' },
  { key: 'physicality', label: 'PHY', sort: 'physicality' },
  { key: 'ringcraft', label: 'RIN', sort: 'ringcraft' },
  { key: 'psychology', label: 'PSY', sort: 'psychology' },
  { key: 'fundamentals', label: 'FUN', sort: 'fundamentals' },
  { key: 'entertainment', label: 'ENT', sort: 'entertainment' },
  { key: 'fatigue', label: 'Fatigue', sort: 'fatigue' },
  { key: 'morale', label: 'Morale', sort: 'morale' },
  { key: 'momentum', label: 'Momentum', sort: 'momentum' },
  { key: 'status', label: 'Availability' },
];

const score = (worker: RosterRow, key: ColumnKey): string | number => {
  if (key === 'name') return worker.name;
  if (key === 'age') return worker.age;
  if (key === 'personality') return worker.personalityDescription;
  if (key === 'archetype') return worker.archetype;
  if (key === 'overall') return worker.overall;
  if (key === 'status')
    return worker.condition.injuryDays
      ? `Injured · ${worker.condition.injuryDays} days`
      : 'Cleared';
  if (key === 'fatigue' || key === 'morale' || key === 'momentum')
    return worker.condition[key];
  return worker.groups[key];
};

const updateRange = (
  current: WorkerSearchRequest,
  key: keyof Pick<
    WorkerSearchRequest['filters'],
    | 'age'
    | 'overall'
    | 'movement'
    | 'physicality'
    | 'ringcraft'
    | 'psychology'
    | 'fundamentals'
    | 'entertainment'
  >,
  edge: keyof NumberRange,
  value: string,
): WorkerSearchRequest => ({
  ...current,
  offset: 0,
  filters: {
    ...current.filters,
    [key]: {
      ...current.filters[key],
      [edge]: value === '' ? null : Number(value),
    },
  },
});

function RangeFilter({
  label,
  value,
  onChange,
}: {
  label: string;
  value: NumberRange;
  onChange: (edge: keyof NumberRange, value: string) => void;
}) {
  return (
    <fieldset className={s.finderRange}>
      <legend>{label}</legend>
      <input
        type="number"
        aria-label={`${label} minimum`}
        placeholder="Min"
        value={value.minimum ?? ''}
        onChange={(event) => onChange('minimum', event.target.value)}
      />
      <span>–</span>
      <input
        type="number"
        aria-label={`${label} maximum`}
        placeholder="Max"
        value={value.maximum ?? ''}
        onChange={(event) => onChange('maximum', event.target.value)}
      />
    </fieldset>
  );
}

function FilterIcon() {
  return (
    <svg aria-hidden="true" viewBox="0 0 24 24" width="18" height="18">
      <path
        d="M4 6h16M7 12h10M10 18h4"
        fill="none"
        stroke="currentColor"
        strokeLinecap="round"
        strokeWidth="2"
      />
    </svg>
  );
}

function ChoiceFilter({
  label,
  values,
  included,
  excluded,
  onSet,
}: {
  label: string;
  values: string[];
  included: string[];
  excluded: string[];
  onSet: (value: string, state: 'any' | 'include' | 'exclude') => void;
}) {
  const [query, setQuery] = useState('');
  const visible = values.filter((value) =>
    value.toLocaleLowerCase().includes(query.trim().toLocaleLowerCase()),
  );
  return (
    <section className={s.finderChoice} aria-label={`${label} filter`}>
      <div className={s.finderChoiceHead}>
        <b>{label}</b>
        <small>
          {included.length + excluded.length
            ? `${included.length} include · ${excluded.length} exclude`
            : 'Any'}
        </small>
      </div>
      {values.length > 6 && (
        <input
          type="search"
          value={query}
          placeholder={`Find ${label.toLocaleLowerCase()}…`}
          aria-label={`Find ${label.toLocaleLowerCase()}`}
          onChange={(event) => setQuery(event.target.value)}
        />
      )}
      <div className={s.finderChoiceList}>
        {visible.map((value) => {
          const state = included.includes(value)
            ? 'include'
            : excluded.includes(value)
              ? 'exclude'
              : 'any';
          return (
            <div className={s.finderChoiceRow} key={value}>
              <span>{value}</span>
              <button
                aria-label={`Include ${value}`}
                aria-pressed={state === 'include'}
                title={`Include ${value}`}
                onClick={() =>
                  onSet(value, state === 'include' ? 'any' : 'include')
                }
              >
                +
              </button>
              <button
                aria-label={`Exclude ${value}`}
                aria-pressed={state === 'exclude'}
                title={`Exclude ${value}`}
                onClick={() =>
                  onSet(value, state === 'exclude' ? 'any' : 'exclude')
                }
              >
                −
              </button>
            </div>
          );
        })}
        {!visible.length && <small>No matching options.</small>}
      </div>
    </section>
  );
}

export function WorkerFinder({ saveId }: { saveId: string }) {
  const input = useRef<HTMLInputElement>(null);
  const cache = useQueryClient();
  const inspect = useNavigation((state) => state.inspectWorker);
  const request = useNavigation((state) => state.workerSearch);
  const setRequest = useNavigation((state) => state.setWorkerSearch);
  const visibleColumns = useNavigation((state) => state.workerColumns);
  const setVisibleColumns = useNavigation((state) => state.setWorkerColumns);
  const compareIds = useNavigation((state) => state.workerCompareIds);
  const setCompareIds = useNavigation((state) => state.setWorkerCompareIds);
  const scrollTop = useNavigation((state) => state.workerScrollTop);
  const setScrollTop = useNavigation((state) => state.setWorkerScrollTop);
  const focusSignal = useNavigation((state) => state.workerSearchFocus);
  const deferred = useDeferredValue(request);
  const [filtersOpen, setFiltersOpen] = useState(false);
  const [filterSearch, setFilterSearch] = useState('');
  const [viewName, setViewName] = useState('');
  const [shortlistName, setShortlistName] = useState('');
  const [activeShortlist, setActiveShortlist] = useState<number | null>(null);
  const [compareOpen, setCompareOpen] = useState(false);
  const resultsScroll = useRef<HTMLDivElement>(null);

  useEffect(() => input.current?.focus(), [focusSignal]);
  useEffect(() => {
    if (resultsScroll.current) resultsScroll.current.scrollTop = scrollTop;
  }, [scrollTop]);

  const result = useQuery({
    queryKey: ['worker-search', saveId, deferred],
    queryFn: () => gameApi.workerSearch(saveId, deferred),
  });
  const options = useQuery({
    queryKey: ['worker-filter-options', saveId],
    queryFn: () => gameApi.workerFilterOptions(saveId),
  });
  const lists = useQuery({
    queryKey: ['worker-discovery-lists', saveId],
    queryFn: () => gameApi.workerDiscoveryLists(saveId),
  });
  const comparison = useQuery({
    queryKey: ['worker-comparison', saveId, compareIds],
    queryFn: () => gameApi.compareWorkers(saveId, compareIds),
    enabled: compareOpen && compareIds.length > 0,
  });
  const refreshLists = (
    data: Awaited<ReturnType<typeof gameApi.workerDiscoveryLists>>,
  ) => {
    cache.setQueryData(['worker-discovery-lists', saveId], data);
    void cache.invalidateQueries({ queryKey: ['worker-search', saveId] });
  };
  const mutation = useMutation({
    mutationFn: (
      action: () => ReturnType<typeof gameApi.workerDiscoveryLists>,
    ) => action(),
    onSuccess: refreshLists,
  });
  const saveView = useMutation({
    mutationFn: () =>
      gameApi.saveWorkerView(saveId, null, viewName, request, visibleColumns),
    onSuccess: (data) => {
      refreshLists(data);
      setViewName('');
    },
  });
  const createList = useMutation({
    mutationFn: (name: string) => gameApi.createWorkerShortlist(saveId, name),
    onSuccess: (data, createdName) => {
      refreshLists(data);
      setShortlistName('');
      setActiveShortlist(
        data.shortlists.find(
          (list) =>
            list.name.localeCompare(createdName.trim(), undefined, {
              sensitivity: 'accent',
            }) === 0,
        )?.id ?? null,
      );
    },
  });
  const shownColumns = useMemo(
    () => columns.filter((column) => visibleColumns.includes(column.key)),
    [visibleColumns],
  );
  const setFilter = <K extends keyof WorkerSearchRequest['filters']>(
    key: K,
    value: WorkerSearchRequest['filters'][K],
  ) =>
    setRequest({
      ...request,
      offset: 0,
      filters: { ...request.filters, [key]: value },
    });
  const setChoice = (
    includedKey: ChoiceFilterKey,
    excludedKey: ExcludedChoiceFilterKey,
    value: string,
    state: 'any' | 'include' | 'exclude',
  ) => {
    const included = request.filters[includedKey].filter(
      (candidate) => candidate !== value,
    );
    const excluded = request.filters[excludedKey].filter(
      (candidate) => candidate !== value,
    );
    if (state === 'include') included.push(value);
    if (state === 'exclude') excluded.push(value);
    setRequest({
      ...request,
      offset: 0,
      filters: {
        ...request.filters,
        [includedKey]: included,
        [excludedKey]: excluded,
      },
    });
  };
  const changeSort = (key: WorkerSortKey, secondary: boolean) => {
    const existing = request.sort.find((sort) => sort.key === key);
    const next = {
      key,
      direction: existing
        ? existing.direction === 'ascending'
          ? ('descending' as const)
          : ('ascending' as const)
        : key === 'name' || key === 'age' || key === 'fatigue'
          ? ('ascending' as const)
          : ('descending' as const),
    };
    setRequest({
      ...request,
      offset: 0,
      sort: secondary
        ? [...request.sort.filter((sort) => sort.key !== key), next].slice(-2)
        : [next],
    });
  };
  const toggleCompare = (id: string) => {
    if (compareIds.includes(id)) {
      setCompareIds(compareIds.filter((candidate) => candidate !== id));
    } else if (compareIds.length < 4) {
      setCompareIds([...compareIds, id]);
    }
  };
  const error =
    result.error ??
    options.error ??
    lists.error ??
    mutation.error ??
    saveView.error ??
    createList.error;
  const filterOptions: WorkerFilterOptions = options.data ?? {
    nationalities: [],
    languages: [],
    schools: [],
    archetypes: [],
    primaryDisciplines: [],
  };
  const activeFilters = (() => {
    const chips: { key: string; label: string; clear: () => void }[] = [];
    const addRangeChip = (
      key: keyof Pick<
        WorkerSearchRequest['filters'],
        | 'age'
        | 'overall'
        | 'movement'
        | 'physicality'
        | 'ringcraft'
        | 'psychology'
        | 'fundamentals'
        | 'entertainment'
      >,
      label: string,
    ) => {
      const range = request.filters[key];
      if (range.minimum === null && range.maximum === null) return;
      chips.push({
        key,
        label: `${label} ${range.minimum ?? 'min'}–${range.maximum ?? 'max'}`,
        clear: () => setFilter(key, { minimum: null, maximum: null }),
      });
    };
    addRangeChip('age', 'Age');
    addRangeChip('overall', 'Style OVR');
    addRangeChip('movement', 'Movement');
    addRangeChip('physicality', 'Physicality');
    addRangeChip('ringcraft', 'Ringcraft');
    addRangeChip('psychology', 'Psychology');
    addRangeChip('fundamentals', 'Fundamentals');
    addRangeChip('entertainment', 'Entertainment');
    if (request.filters.availability !== 'any') {
      chips.push({
        key: 'availability',
        label:
          request.filters.availability === 'cleared'
            ? 'Medically cleared'
            : 'Currently injured',
        clear: () => setFilter('availability', 'any'),
      });
    }
    const addChoices = (
      includedKey: ChoiceFilterKey,
      excludedKey: ExcludedChoiceFilterKey,
    ) => {
      request.filters[includedKey].forEach((value) =>
        chips.push({
          key: `${includedKey}-include-${value}`,
          label: value,
          clear: () => setChoice(includedKey, excludedKey, value, 'any'),
        }),
      );
      request.filters[excludedKey].forEach((value) =>
        chips.push({
          key: `${includedKey}-exclude-${value}`,
          label: `Not ${value}`,
          clear: () => setChoice(includedKey, excludedKey, value, 'any'),
        }),
      );
    };
    addChoices('nationalities', 'excludedNationalities');
    addChoices('languages', 'excludedLanguages');
    addChoices('schools', 'excludedSchools');
    addChoices('archetypes', 'excludedArchetypes');
    addChoices('primaryDisciplines', 'excludedPrimaryDisciplines');
    if (request.filters.shortlistId !== null) {
      const list = lists.data?.shortlists.find(
        (candidate) => candidate.id === request.filters.shortlistId,
      );
      chips.push({
        key: 'shortlist',
        label: list ? `List: ${list.name}` : 'Selected shortlist',
        clear: () => setFilter('shortlistId', null),
      });
    }
    if (request.filters.blacklist !== 'include') {
      chips.push({
        key: 'blacklist',
        label:
          request.filters.blacklist === 'exclude'
            ? 'Hide blacklisted'
            : 'Blacklisted only',
        clear: () => setFilter('blacklist', 'include'),
      });
    }
    return chips;
  })();
  const catalogueMatches = (...terms: string[]) =>
    !filterSearch.trim() ||
    terms.some((term) =>
      term
        .toLocaleLowerCase()
        .includes(filterSearch.trim().toLocaleLowerCase()),
    );

  return (
    <div className={s.finderScreen}>
      <header className={s.finderHeader}>
        <div>
          <p className={s.eyebrow}>Talent intelligence</p>
          <h1>Talent Search</h1>
          <span>
            Find workers quickly or open filters for a detailed search.
          </span>
        </div>
        <div className={s.finderSearch}>
          <input
            ref={input}
            value={request.text}
            placeholder="Search workers by name…"
            onChange={(event) => {
              const text = event.target.value;
              const wasDefault =
                request.sort.length === 1 && request.sort[0]?.key === 'name';
              const wasRelevance =
                request.sort.length === 1 &&
                request.sort[0]?.key === 'relevance';
              setRequest({
                ...request,
                text,
                offset: 0,
                sort:
                  text && wasDefault
                    ? [{ key: 'relevance', direction: 'descending' }]
                    : !text && wasRelevance
                      ? [{ key: 'name', direction: 'ascending' }]
                      : request.sort,
              });
            }}
          />
          <kbd>Ctrl K</kbd>
        </div>
        <button
          className={s.finderFilterToggle}
          aria-label={
            filtersOpen ? 'Close search filters' : 'Open search filters'
          }
          aria-expanded={filtersOpen}
          title={filtersOpen ? 'Close filters' : 'Open filters'}
          onClick={() => setFiltersOpen((open) => !open)}
        >
          <FilterIcon />
          <span>Filters</span>
          {activeFilters.length > 0 && <b>{activeFilters.length}</b>}
        </button>
      </header>

      {error && <ErrorNotice message={errorMessage(error)} />}

      <div className={s.finderLayout}>
        {filtersOpen && (
          <aside className={s.finderFilters} aria-label="Talent search filters">
            <div className={s.finderAsideHead}>
              <b>Filters</b>
              <button
                disabled={!activeFilters.length}
                onClick={() =>
                  setRequest({
                    ...request,
                    offset: 0,
                    filters: defaultWorkerSearch().filters,
                  })
                }
              >
                Reset
              </button>
            </div>
            <input
              className={s.finderFilterSearch}
              type="search"
              value={filterSearch}
              placeholder="Find a filter…"
              aria-label="Find a filter"
              onChange={(event) => setFilterSearch(event.target.value)}
            />
            {catalogueMatches('availability', 'age', 'medical', 'injury') && (
              <details className={s.finderFilterGroup} open>
                <summary>Availability & age</summary>
                <label>
                  Availability
                  <select
                    value={request.filters.availability}
                    onChange={(event) =>
                      setFilter(
                        'availability',
                        event.target
                          .value as WorkerSearchRequest['filters']['availability'],
                      )
                    }
                  >
                    <option value="any">Any</option>
                    <option value="cleared">Medically cleared</option>
                    <option value="injured">Currently injured</option>
                  </select>
                </label>
                <RangeFilter
                  label="Age"
                  value={request.filters.age}
                  onChange={(edge, value) =>
                    setRequest(updateRange(request, 'age', edge, value))
                  }
                />
              </details>
            )}
            {catalogueMatches(
              'ability',
              'rating',
              'overall',
              'movement',
              'physicality',
              'ringcraft',
              'psychology',
              'fundamentals',
              'entertainment',
            ) && (
              <details className={s.finderFilterGroup}>
                <summary>Ability</summary>
                <RangeFilter
                  label="Style OVR"
                  value={request.filters.overall}
                  onChange={(edge, value) =>
                    setRequest(updateRange(request, 'overall', edge, value))
                  }
                />
                {(
                  [
                    'movement',
                    'physicality',
                    'ringcraft',
                    'psychology',
                    'fundamentals',
                    'entertainment',
                  ] as const
                ).map((key) => (
                  <RangeFilter
                    key={key}
                    label={key.charAt(0).toUpperCase() + key.slice(1)}
                    value={request.filters[key]}
                    onChange={(edge, value) =>
                      setRequest(updateRange(request, key, edge, value))
                    }
                  />
                ))}
              </details>
            )}
            {catalogueMatches(
              'identity',
              'nationality',
              'style',
              'archetype',
              'discipline',
              'language',
              'school',
              'training',
              'background',
            ) && (
              <details className={s.finderFilterGroup}>
                <summary>Identity & style</summary>
                <ChoiceFilter
                  label="Nationality"
                  values={filterOptions.nationalities}
                  included={request.filters.nationalities}
                  excluded={request.filters.excludedNationalities}
                  onSet={(value, state) =>
                    setChoice(
                      'nationalities',
                      'excludedNationalities',
                      value,
                      state,
                    )
                  }
                />
                <ChoiceFilter
                  label="Archetype"
                  values={filterOptions.archetypes}
                  included={request.filters.archetypes}
                  excluded={request.filters.excludedArchetypes}
                  onSet={(value, state) =>
                    setChoice('archetypes', 'excludedArchetypes', value, state)
                  }
                />
                <ChoiceFilter
                  label="Primary discipline"
                  values={filterOptions.primaryDisciplines}
                  included={request.filters.primaryDisciplines}
                  excluded={request.filters.excludedPrimaryDisciplines}
                  onSet={(value, state) =>
                    setChoice(
                      'primaryDisciplines',
                      'excludedPrimaryDisciplines',
                      value,
                      state,
                    )
                  }
                />
                <details className={s.finderAdvanced}>
                  <summary>Advanced background</summary>
                  <ChoiceFilter
                    label="Languages"
                    values={filterOptions.languages}
                    included={request.filters.languages}
                    excluded={request.filters.excludedLanguages}
                    onSet={(value, state) =>
                      setChoice('languages', 'excludedLanguages', value, state)
                    }
                  />
                  <ChoiceFilter
                    label="Training lineage"
                    values={filterOptions.schools}
                    included={request.filters.schools}
                    excluded={request.filters.excludedSchools}
                    onSet={(value, state) =>
                      setChoice('schools', 'excludedSchools', value, state)
                    }
                  />
                </details>
              </details>
            )}
            {catalogueMatches('list', 'shortlist', 'blacklist') && (
              <details className={s.finderFilterGroup}>
                <summary>Personal lists</summary>
                <label>
                  Shortlist
                  <select
                    value={request.filters.shortlistId ?? ''}
                    onChange={(event) =>
                      setFilter(
                        'shortlistId',
                        event.target.value ? Number(event.target.value) : null,
                      )
                    }
                  >
                    <option value="">Any list</option>
                    {lists.data?.shortlists.map((list) => (
                      <option key={list.id} value={list.id}>
                        {list.name} ({list.memberCount})
                      </option>
                    ))}
                  </select>
                </label>
                <label>
                  Blacklist
                  <select
                    value={request.filters.blacklist}
                    onChange={(event) =>
                      setFilter(
                        'blacklist',
                        event.target
                          .value as WorkerSearchRequest['filters']['blacklist'],
                      )
                    }
                  >
                    <option value="include">Include blacklisted</option>
                    <option value="exclude">Hide blacklisted</option>
                    <option value="only">Blacklisted only</option>
                  </select>
                </label>
              </details>
            )}
          </aside>
        )}

        <section className={s.finderResults}>
          <div className={s.finderTools}>
            <b>{result.data?.total ?? 0} workers</b>
            <div className={s.spacer} />
            <details>
              <summary>Columns</summary>
              <div className={s.finderPopover}>
                {columns.map((column) => (
                  <label key={column.key}>
                    <input
                      type="checkbox"
                      checked={visibleColumns.includes(column.key)}
                      disabled={column.key === 'name'}
                      onChange={() =>
                        setVisibleColumns(
                          visibleColumns.includes(column.key)
                            ? visibleColumns.filter((key) => key !== column.key)
                            : [...visibleColumns, column.key],
                        )
                      }
                    />
                    {column.label}
                  </label>
                ))}
              </div>
            </details>
            <select
              aria-label="Page size"
              value={request.limit}
              onChange={(event) =>
                setRequest({
                  ...request,
                  limit: Number(event.target.value),
                  offset: 0,
                })
              }
            >
              <option value={25}>25 rows</option>
              <option value={50}>50 rows</option>
              <option value={100}>100 rows</option>
            </select>
          </div>

          {activeFilters.length > 0 && (
            <div className={s.finderActiveFilters} aria-label="Active filters">
              {activeFilters.map((filter) => (
                <button key={filter.key} onClick={filter.clear}>
                  {filter.label} <span aria-hidden="true">×</span>
                  <span className={s.srOnly}>Remove filter</span>
                </button>
              ))}
            </div>
          )}

          <div className={s.finderSavedBar}>
            <label>
              View
              <select
                aria-label="Load saved view"
                defaultValue=""
                onChange={(event) => {
                  const view = lists.data?.savedViews.find(
                    (candidate) => candidate.id === Number(event.target.value),
                  );
                  if (view) {
                    setRequest({ ...view.request, offset: 0 });
                    setVisibleColumns(view.columns);
                  }
                }}
              >
                <option value="">Default view</option>
                {lists.data?.savedViews.map((view) => (
                  <option key={view.id} value={view.id}>
                    {view.name}
                  </option>
                ))}
              </select>
            </label>
            <label>
              Active shortlist
              <select
                aria-label="Active shortlist"
                value={activeShortlist ?? ''}
                onChange={(event) =>
                  setActiveShortlist(
                    event.target.value ? Number(event.target.value) : null,
                  )
                }
              >
                <option value="">None selected</option>
                {lists.data?.shortlists.map((list) => (
                  <option key={list.id} value={list.id}>
                    {list.name} ({list.memberCount})
                  </option>
                ))}
              </select>
            </label>
            <div className={s.spacer} />
            <details className={s.finderSavedMenu}>
              <summary>Saved views & lists</summary>
              <div className={s.finderSavedPanel}>
                <section>
                  <b>Save this view</b>
                  <div>
                    <input
                      value={viewName}
                      placeholder="View name"
                      aria-label="Saved view name"
                      onChange={(event) => setViewName(event.target.value)}
                    />
                    <button
                      disabled={!viewName.trim() || saveView.isPending}
                      onClick={() => saveView.mutate()}
                    >
                      Save
                    </button>
                  </div>
                </section>
                <section>
                  <b>Create shortlist</b>
                  <div>
                    <input
                      value={shortlistName}
                      placeholder="Shortlist name"
                      aria-label="New shortlist name"
                      onChange={(event) => setShortlistName(event.target.value)}
                    />
                    <button
                      disabled={!shortlistName.trim() || createList.isPending}
                      onClick={() => createList.mutate(shortlistName)}
                    >
                      Create
                    </button>
                  </div>
                </section>
                <section>
                  <b>Manage</b>
                  {lists.data?.savedViews.map((view) => (
                    <div key={`view-${view.id}`}>
                      <span>{view.name}</span>
                      <button
                        onClick={() =>
                          window.confirm(`Delete saved view “${view.name}”?`) &&
                          mutation.mutate(() =>
                            gameApi.deleteWorkerView(saveId, view.id),
                          )
                        }
                      >
                        Delete view
                      </button>
                    </div>
                  ))}
                  {lists.data?.shortlists.map((list) => (
                    <div key={`list-${list.id}`}>
                      <span>
                        {list.name} ({list.memberCount})
                      </span>
                      <button
                        onClick={() =>
                          window.confirm(
                            `Delete shortlist “${list.name}” and its membership?`,
                          ) &&
                          mutation.mutate(() =>
                            gameApi.deleteWorkerShortlist(saveId, list.id),
                          )
                        }
                      >
                        Delete list
                      </button>
                    </div>
                  ))}
                </section>
              </div>
            </details>
          </div>

          <div
            className={s.finderTableScroll}
            ref={resultsScroll}
            onScroll={(event) => setScrollTop(event.currentTarget.scrollTop)}
          >
            <table className={s.roster}>
              <thead>
                <tr>
                  <th aria-label="Compare">Compare</th>
                  {shownColumns.map((column) => {
                    const position = column.sort
                      ? request.sort.findIndex(
                          (sort) => sort.key === column.sort,
                        )
                      : -1;
                    const direction =
                      position >= 0 ? request.sort[position]?.direction : null;
                    return (
                      <th
                        key={column.key}
                        aria-sort={
                          direction === 'ascending'
                            ? 'ascending'
                            : direction === 'descending'
                              ? 'descending'
                              : undefined
                        }
                      >
                        {column.sort ? (
                          <button
                            className={s.sortHeader}
                            title="Click to sort; Shift-click adds a second sort"
                            onClick={(event) =>
                              changeSort(column.sort!, event.shiftKey)
                            }
                          >
                            {column.label}{' '}
                            {direction
                              ? direction === 'ascending'
                                ? '▲'
                                : '▼'
                              : ''}
                            {position > 0 ? ` ${position + 1}` : ''}
                          </button>
                        ) : (
                          column.label
                        )}
                      </th>
                    );
                  })}
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {result.data?.rows.map((hit) => (
                  <tr key={hit.worker.id}>
                    <td>
                      <input
                        type="checkbox"
                        aria-label={`Compare ${hit.worker.name}`}
                        checked={compareIds.includes(hit.worker.id)}
                        disabled={
                          !compareIds.includes(hit.worker.id) &&
                          compareIds.length >= 4
                        }
                        onChange={() => toggleCompare(hit.worker.id)}
                      />
                    </td>
                    {shownColumns.map((column) => (
                      <td
                        key={column.key}
                        className={
                          column.key === 'status' &&
                          hit.worker.condition.injuryDays
                            ? s.danger
                            : undefined
                        }
                      >
                        {column.key === 'name' ? (
                          <>
                            <button
                              className={s.nameButton}
                              onClick={() => inspect(hit.worker.id)}
                            >
                              {hit.worker.name}
                            </button>
                            {hit.matchReason && (
                              <small className={s.matchReason}>
                                {hit.matchReason}
                              </small>
                            )}
                          </>
                        ) : (
                          score(hit.worker, column.key)
                        )}
                      </td>
                    ))}
                    <td className={s.finderRowActions}>
                      <button
                        disabled={!activeShortlist || mutation.isPending}
                        title={
                          activeShortlist &&
                          hit.shortlistIds.includes(activeShortlist)
                            ? 'Remove from active shortlist'
                            : 'Add to active shortlist'
                        }
                        onClick={() =>
                          activeShortlist &&
                          mutation.mutate(() =>
                            gameApi.setWorkerShortlistMember(
                              saveId,
                              activeShortlist,
                              hit.worker.id,
                              !hit.shortlistIds.includes(activeShortlist),
                            ),
                          )
                        }
                      >
                        {activeShortlist &&
                        hit.shortlistIds.includes(activeShortlist)
                          ? 'Remove from list'
                          : 'Add to list'}
                      </button>
                      <button
                        className={hit.blacklisted ? s.danger : undefined}
                        title={
                          hit.blacklisted
                            ? 'Remove from blacklist'
                            : 'Add to personal blacklist'
                        }
                        onClick={() =>
                          mutation.mutate(() =>
                            gameApi.setWorkerBlacklisted(
                              saveId,
                              hit.worker.id,
                              !hit.blacklisted,
                            ),
                          )
                        }
                      >
                        {hit.blacklisted ? 'Unhide' : 'Hide'}
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
            {result.isPending && (
              <p className={s.empty} role="status">
                Searching workers…
              </p>
            )}
            {!result.isPending && result.data?.rows.length === 0 && (
              <p className={s.empty}>
                No workers match this view. Relax one filter or reset the
                search.
              </p>
            )}
          </div>

          <footer className={s.finderFooter}>
            <span>
              {result.data?.total ? request.offset + 1 : 0}–
              {Math.min(
                request.offset + request.limit,
                result.data?.total ?? 0,
              )}{' '}
              of {result.data?.total ?? 0}
            </span>
            <button
              disabled={request.offset === 0}
              onClick={() => {
                setScrollTop(0);
                setRequest({
                  ...request,
                  offset: Math.max(0, request.offset - request.limit),
                });
              }}
            >
              Previous
            </button>
            <button
              disabled={
                request.offset + request.limit >= (result.data?.total ?? 0)
              }
              onClick={() => {
                setScrollTop(0);
                setRequest({
                  ...request,
                  offset: request.offset + request.limit,
                });
              }}
            >
              Next
            </button>
          </footer>
        </section>
      </div>

      {compareIds.length > 0 && (
        <div className={s.compareTray}>
          <b>{compareIds.length}/4 selected</b>
          <span>
            Select workers across pages; the tray stays with your search.
          </span>
          <button
            disabled={compareIds.length < 2}
            onClick={() => setCompareOpen(true)}
          >
            Compare
          </button>
          <button onClick={() => setCompareIds([])}>Clear</button>
        </div>
      )}

      {compareOpen && (
        <Overlay
          title="Worker comparison"
          onClose={() => setCompareOpen(false)}
        >
          {comparison.isError ? (
            <ErrorNotice message={errorMessage(comparison.error)} />
          ) : comparison.isPending ? (
            <p role="status">Building comparison…</p>
          ) : (
            <div className={s.comparisonScroll}>
              <table className={s.comparisonTable}>
                <thead>
                  <tr>
                    <th>Rating</th>
                    {comparison.data?.map((worker) => (
                      <th key={worker.id}>
                        <button
                          className={s.nameButton}
                          onClick={() => inspect(worker.id)}
                        >
                          {worker.name}
                        </button>
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {(
                    [
                      'overall',
                      'movement',
                      'physicality',
                      'ringcraft',
                      'psychology',
                      'fundamentals',
                      'entertainment',
                      'age',
                      'morale',
                      'momentum',
                      'fatigue',
                      'status',
                    ] as ColumnKey[]
                  ).map((key) => (
                    <tr key={key}>
                      <th>
                        {columns.find((column) => column.key === key)?.label}
                      </th>
                      {comparison.data?.map((worker) => (
                        <td key={worker.id}>{score(worker, key)}</td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </Overlay>
      )}
    </div>
  );
}
