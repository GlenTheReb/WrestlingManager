import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { gameApi, errorMessage } from '../api';
import { ErrorNotice, Panel, Meter, Overlay, currency } from '../game-ui';
import s from '../Game.module.css';
import { PersonIdentityPanel } from './PersonIdentity';
import { RelationshipsPanel } from './Relationships';

const humanize = (value: string) =>
  value
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/^./, (letter) => letter.toUpperCase());

const scoreClass = (value: number) => (value >= 85 ? s.good : '');
export { WorkerFinder as Roster } from './WorkerFinder';

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
  const [tab, setTab] = useState<
    'overview' | 'identity' | 'relationships' | 'moves' | 'history'
  >('overview');
  const w = profile.data?.worker;
  const wrestling = profile.data?.wrestling;
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
                {wrestling?.archetype ?? w.style} · {w.age} years old
              </h3>
              <p>
                {w.nationality} · {w.weightKg} kg · {w.language}
              </p>
              <span>
                Current-style overall {wrestling?.overall ?? '—'} ·{' '}
                {w.personality} · {w.school}
              </span>
            </div>
          </div>
          <nav className={s.tabs}>
            {(
              [
                'overview',
                'identity',
                'relationships',
                'moves',
                'history',
              ] as const
            ).map((value) => (
              <button
                key={value}
                aria-pressed={tab === value}
                onClick={() => setTab(value)}
              >
                {value === 'identity'
                  ? 'Person & traits'
                  : value === 'relationships'
                    ? 'Relationships'
                    : value === 'moves'
                      ? 'Moveset'
                      : value === 'history'
                        ? 'Match history'
                        : 'Profile'}
              </button>
            ))}
          </nav>
          <div className={s.overlayBody}>
            {tab === 'identity' && profile.data ? (
              <PersonIdentityPanel
                identity={w.identity}
                description={profile.data.personalityDescription}
                biography={profile.data.biography}
                traits={profile.data.exceptionalTraits}
              />
            ) : tab === 'relationships' && profile.data ? (
              <RelationshipsPanel
                saveId={saveId}
                workerId={workerId}
                relationships={profile.data.relationships}
              />
            ) : tab === 'overview' ? (
              <div className={s.profileGrid}>
                <Panel title="Six base ratings">
                  <dl className={s.attributes}>
                    {Object.entries(wrestling?.groups ?? {}).map(
                      ([key, value]) => (
                        <div key={key}>
                          <dt>{humanize(key)}</dt>
                          <dd className={scoreClass(value)}>{value}</dd>
                        </div>
                      ),
                    )}
                  </dl>
                </Panel>
                <Panel title="Style identity">
                  <dl className={s.attributes}>
                    <div>
                      <dt>Primary Discipline</dt>
                      <dd>{humanize(wrestling?.primary ?? '')}</dd>
                    </div>
                    <div>
                      <dt>Secondary Disciplines</dt>
                      <dd>
                        {wrestling?.secondaries.map(humanize).join(', ') ||
                          'None'}
                      </dd>
                    </div>
                    {Object.entries(w.wrestlingStyle.approach).map(
                      ([key, value]) => (
                        <div key={key}>
                          <dt>{humanize(key)}</dt>
                          <dd>{humanize(value)}</dd>
                        </div>
                      ),
                    )}
                    <div>
                      <dt>Specialisations</dt>
                      <dd>
                        {w.wrestlingStyle.specialisations
                          .map(humanize)
                          .join(', ') || 'None'}
                      </dd>
                    </div>
                  </dl>
                </Panel>
                {Object.entries(w.attributes).map(([group, values]) => (
                  <Panel key={group} title={`${humanize(group)} sub-stats`}>
                    <dl className={s.attributes}>
                      {Object.entries(values).map(([key, value]) => (
                        <div key={key}>
                          <dt>{humanize(key)}</dt>
                          <dd className={scoreClass(value)}>{value}</dd>
                        </div>
                      ))}
                    </dl>
                  </Panel>
                ))}
                <Panel title="Discipline Fits">
                  <dl className={s.attributes}>
                    {wrestling?.disciplineFits.map((fit) => (
                      <div key={fit.discipline}>
                        <dt>{humanize(fit.discipline)}</dt>
                        <dd className={scoreClass(fit.score)}>{fit.score}</dd>
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
                  <p>{profile.data?.biography || w.background}</p>
                  <p>
                    <b>Career motivations</b> · {w.ambition}
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
