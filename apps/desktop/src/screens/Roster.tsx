import { useMemo, useRef, useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import {
  useReactTable,
  getCoreRowModel,
  flexRender,
  type ColumnDef,
} from '@tanstack/react-table';
import { useVirtualizer } from '@tanstack/react-virtual';
import type { RosterRow } from '@wm/contracts';
import { gameApi, errorMessage } from '../api';
import { useNavigation } from '../navigation';
import { ErrorNotice, Panel, Meter, Overlay, currency } from '../game-ui';
import s from '../Game.module.css';

export function Roster({ saveId }: { saveId: string }) {
  const [search, setSearch] = useState(''),
    [offset, setOffset] = useState(0);
  const inspect = useNavigation((v) => v.inspectWorker);
  const roster = useQuery({
    queryKey: ['roster', saveId, search, offset],
    queryFn: () => gameApi.roster(saveId, search, offset, 64),
  });
  const columns = useMemo<ColumnDef<RosterRow>[]>(
    () => [
      {
        accessorKey: 'name',
        header: 'Wrestler',
        cell: (info) => (
          <button
            className={s.nameButton}
            onClick={() => inspect(info.row.original.id)}
          >
            {info.getValue<string>()}
          </button>
        ),
      },
      { accessorKey: 'age', header: 'Age' },
      { accessorKey: 'style', header: 'Style' },
      { accessorKey: 'psychology', header: 'Psych' },
      { accessorKey: 'stamina', header: 'Stam' },
      {
        id: 'fatigue',
        header: 'Fatigue',
        accessorFn: (r) => r.condition.fatigue,
      },
      { id: 'morale', header: 'Morale', accessorFn: (r) => r.condition.morale },
      {
        id: 'momentum',
        header: 'Momentum',
        accessorFn: (r) => r.condition.momentum,
      },
      {
        id: 'status',
        header: 'Availability',
        cell: ({ row }) => (
          <span
            className={row.original.condition.injuryDays ? s.danger : s.good}
          >
            {row.original.condition.injuryDays
              ? `Injured · ${row.original.condition.injuryDays} days`
              : 'Cleared'}
          </span>
        ),
      },
    ],
    [inspect],
  );
  const table = useReactTable({
    data: roster.data?.rows ?? [],
    columns,
    getCoreRowModel: getCoreRowModel(),
  });
  const parent = useRef<HTMLDivElement>(null);
  const rows = table.getRowModel().rows;
  const virtual = useVirtualizer({
    count: rows.length,
    getScrollElement: () => parent.current,
    estimateSize: () => 43,
    overscan: 8,
  });
  const visible = virtual.getVirtualItems();
  return (
    <div className={s.screenColumn}>
      <div className={s.toolbar}>
        <label>
          Find wrestler
          <input
            placeholder="Name…"
            value={search}
            onChange={(e) => {
              setSearch(e.target.value);
              setOffset(0);
            }}
          />
        </label>
        <span>{roster.data?.total ?? 0} wrestlers · attributes out of 20</span>
        <div className={s.spacer} />
        <button
          disabled={!offset}
          onClick={() => setOffset((v) => Math.max(0, v - 64))}
        >
          Previous
        </button>
        <button
          disabled={offset + 64 >= (roster.data?.total ?? 0)}
          onClick={() => setOffset((v) => v + 64)}
        >
          Next
        </button>
      </div>
      {roster.isError && <ErrorNotice message={errorMessage(roster.error)} />}
      <div className={s.rosterScroll} ref={parent}>
        <table className={s.roster}>
          <thead>
            {table.getHeaderGroups().map((group) => (
              <tr key={group.id}>
                {group.headers.map((header) => (
                  <th key={header.id}>
                    {flexRender(
                      header.column.columnDef.header,
                      header.getContext(),
                    )}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody>
            {visible.length > 0 && (
              <tr aria-hidden="true">
                <td
                  colSpan={columns.length}
                  style={{ height: visible[0]?.start ?? 0, padding: 0 }}
                />
              </tr>
            )}
            {visible.map((v) => {
              const row = rows[v.index];
              return row ? (
                <tr key={row.id} style={{ height: 43 }}>
                  {row.getVisibleCells().map((cell) => (
                    <td key={cell.id}>
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext(),
                      )}
                    </td>
                  ))}
                </tr>
              ) : null;
            })}
            {visible.length > 0 && (
              <tr aria-hidden="true">
                <td
                  colSpan={columns.length}
                  style={{
                    height: virtual.getTotalSize() - (visible.at(-1)?.end ?? 0),
                    padding: 0,
                  }}
                />
              </tr>
            )}
          </tbody>
        </table>
        {roster.isPending && (
          <p className={s.empty} role="status">
            Loading the roster…
          </p>
        )}
      </div>
      <p className={s.help}>
        Select a wrestler to inspect their attributes, moveset and match
        history. Condition is updated by performed matches and daily recovery.
      </p>
    </div>
  );
}

export function ProfilePanel({
  saveId,
  workerId,
  onClose,
}: {
  saveId: string;
  workerId: string;
  onClose: () => void;
}) {
  const profile = useQuery({
    queryKey: ['profile', saveId, workerId],
    queryFn: () => gameApi.profile(saveId, workerId),
  });
  const [tab, setTab] = useState<'overview' | 'moves' | 'history'>('overview');
  const w = profile.data?.worker;
  return (
    <Overlay title={w?.name ?? 'Wrestler profile'} onClose={onClose}>
      {profile.isError ? (
        <ErrorNotice message={errorMessage(profile.error)} />
      ) : !w ? (
        <p role="status">Loading profile…</p>
      ) : (
        <>
          <div className={s.profileHead}>
            <div className={s.monogram}>
              {w.name
                .split(' ')
                .map((p) => p[0])
                .slice(0, 2)
                .join('')}
            </div>
            <div>
              <h3>
                {w.style} · {w.age} years old
              </h3>
              <p>
                {w.nationality} · {w.weightKg} kg · {w.language}
              </p>
              <span>
                {w.personality} · {w.school}
              </span>
            </div>
          </div>
          <nav className={s.tabs}>
            {(['overview', 'moves', 'history'] as const).map((value) => (
              <button
                key={value}
                aria-pressed={tab === value}
                onClick={() => setTab(value)}
              >
                {value === 'moves'
                  ? 'Moveset'
                  : value === 'history'
                    ? 'Match history'
                    : 'Profile'}
              </button>
            ))}
          </nav>
          <div className={s.overlayBody}>
            {tab === 'overview' ? (
              <div className={s.profileGrid}>
                <Panel title="Abilities">
                  <dl className={s.attributes}>
                    {Object.entries(w.attributes).map(([key, value]) => (
                      <div key={key}>
                        <dt>{key}</dt>
                        <dd className={value >= 15 ? s.good : ''}>{value}</dd>
                      </div>
                    ))}
                  </dl>
                </Panel>
                <Panel title="Condition">
                  <div className={s.meters}>
                    <Meter label="Fatigue" value={w.condition.fatigue} />
                    <Meter label="Confidence" value={w.condition.confidence} />
                    <Meter label="Morale" value={w.condition.morale} />
                    <Meter label="Momentum" value={w.condition.momentum} />
                    <Meter label="Wear and tear" value={w.condition.wear} />
                  </div>
                </Panel>
                <div className={s.biography}>
                  <p>{w.background}</p>
                  <p>
                    <b>Ambition</b> · {w.ambition}
                  </p>
                  <p>
                    Appearance agreement: {currency(w.appearanceFee)} per show.{' '}
                    {w.condition.injuryDays
                      ? `Not cleared for ${w.condition.injuryDays} days.`
                      : 'Medically cleared.'}
                  </p>
                </div>
              </div>
            ) : tab === 'moves' ? (
              <table>
                <thead>
                  <tr>
                    <th>Move</th>
                    <th>Style</th>
                    <th>Proficiency</th>
                    <th>Difficulty</th>
                    <th>Risk</th>
                  </tr>
                </thead>
                <tbody>
                  {w.moves.map((m) => (
                    <tr key={m.id}>
                      <th>
                        {m.signature ? '★ ' : ''}
                        {m.name}
                      </th>
                      <td>{m.style}</td>
                      <td>{m.proficiency}%</td>
                      <td>{m.difficulty}/20</td>
                      <td>{m.risk}/5</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ) : profile.data?.history.length ? (
              <div>
                {profile.data.history.map((h, i) => (
                  <article className={s.post} key={`${h.segmentId}-${i}`}>
                    <b>{h.title}</b>
                    <p>{h.result}</p>
                    <small>
                      Execution {h.performance.execution} · Engagement{' '}
                      {h.performance.engagement} · Safety {h.performance.safety}
                    </small>
                  </article>
                ))}
              </div>
            ) : (
              <p className={s.empty}>
                No performances recorded in this career yet.
              </p>
            )}
          </div>
        </>
      )}
    </Overlay>
  );
}
