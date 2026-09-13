import { useEffect, useMemo, useRef } from 'react';
import { useQuery } from '@tanstack/react-query';
import type { ProfileAppearance, WorkerProfile } from '@wm/contracts';
import { errorMessage, gameApi } from '../api';
import {
  clockTime,
  currency,
  ErrorNotice,
  gameDate,
  Meter,
  Panel,
} from '../game-ui';
import { Icon, type IconName } from '../icons';
import { type ProfileTab, useNavigation } from '../navigation';
import s from '../Game.module.css';
import { CharacterPresentation } from './CharacterPresentation';
import { PersonIdentityPanel } from './PersonIdentity';
import { RelationshipsPanel } from './Relationships';

const humanize = (value: string) =>
  value
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/^./, (letter) => letter.toUpperCase());

const scoreBand = (value: number) =>
  value >= 90
    ? { className: s.scoreElite, label: 'Elite' }
    : value >= 75
      ? { className: s.scoreStrong, label: 'Strong' }
      : value >= 60
        ? { className: s.scoreCapable, label: 'Capable' }
        : value >= 40
          ? { className: s.scoreDeveloping, label: 'Developing' }
          : { className: s.scoreWeak, label: 'Weak' };

function Rating({ label, value }: { label: string; value: number }) {
  const band = scoreBand(value);
  return (
    <span
      className={`${s.profileRating} ${band.className}`}
      aria-label={`${label}: ${value} out of 100, ${band.label}`}
      title={`${value}/100 · ${band.label}`}
    >
      <b>{value}</b>
      <small>{band.label}</small>
    </span>
  );
}

const tabs: { id: ProfileTab; label: string; icon: IconName }[] = [
  { id: 'overview', label: 'Overview', icon: 'profile' },
  { id: 'attributes', label: 'Attributes', icon: 'attributes' },
  { id: 'character', label: 'Character', icon: 'mask' },
  { id: 'career', label: 'Career', icon: 'career' },
  { id: 'appearances', label: 'Matches & appearances', icon: 'history' },
  { id: 'relationships', label: 'Relationships', icon: 'people' },
  { id: 'contract', label: 'Contract', icon: 'contract' },
  { id: 'development', label: 'Development', icon: 'development' },
  { id: 'media', label: 'Media', icon: 'media' },
];

function ProfileHeader({ profile }: { profile: WorkerProfile }) {
  const { worker, wrestling, character, company } = profile;
  const availability = worker.condition.injuryDays
    ? `${worker.condition.injuryDays} days from clearance`
    : 'Available for selection';
  return (
    <section className={s.profileHero}>
      <div
        className={s.profilePortrait}
        role="img"
        aria-label="Portrait not supplied"
      >
        <span>
          {worker.name
            .split(' ')
            .map((part) => part[0])
            .slice(0, 2)
            .join('')}
        </span>
        <small>PORTRAIT</small>
      </div>
      <div className={s.profileIdentity}>
        <span className={s.eyebrow}>
          Active ring identity · {company.initials}
        </span>
        <h1>{worker.name}</h1>
        <p className={s.profileRoleLine}>
          {wrestling.archetype} · Active wrestler · {worker.age} years old
        </p>
        <div className={s.profileChips}>
          <span>{humanize(character.active.alignmentIntent)}</span>
          <span>{character.active.gimmick.name}</span>
          <span
            className={worker.condition.injuryDays ? s.dangerChip : s.goodChip}
          >
            {availability}
          </span>
        </div>
        <p className={s.profileMeta}>
          {worker.nationality} · {worker.weightKg} kg · {company.name} ·{' '}
          {company.region}
        </p>
      </div>
      <div className={s.profileOverall}>
        <span>Current-style OVR</span>
        <strong className={scoreBand(wrestling.overall).className}>
          {wrestling.overall}
        </strong>
        <small>{humanize(wrestling.primary)} context</small>
      </div>
    </section>
  );
}

function GroupRatings({ profile }: { profile: WorkerProfile }) {
  return (
    <div className={s.profileGroupRatings}>
      {Object.entries(profile.wrestling.groups).map(([group, value]) => (
        <article key={group}>
          <span>{humanize(group)}</span>
          <Rating label={humanize(group)} value={value} />
        </article>
      ))}
    </div>
  );
}

function BookingCard({ profile }: { profile: WorkerProfile }) {
  const booking = profile.nextBooking;
  if (!booking)
    return <p className={s.empty}>No future appearance is currently booked.</p>;
  const others = booking.participants
    .filter((participant) => participant.workerId !== profile.worker.id)
    .map((participant) => participant.name)
    .join(', ');
  return (
    <article className={s.profileBookingCard}>
      <span>
        {gameDate(booking.date)} · {humanize(booking.kind)}
      </span>
      <strong>{booking.title}</strong>
      <p>{booking.showName}</p>
      {others && <small>With {others}</small>}
    </article>
  );
}

function NewsList({
  profile,
  compact = false,
}: {
  profile: WorkerProfile;
  compact?: boolean;
}) {
  const items = compact ? profile.recentNews.slice(0, 4) : profile.recentNews;
  if (!items.length)
    return (
      <p className={s.empty}>No worker-linked coverage is recorded yet.</p>
    );
  return (
    <div className={s.profileNewsList}>
      {items.map((item) => (
        <article key={item.id}>
          <header>
            <span>{item.category}</span>
            <time>{gameDate(item.date)}</time>
          </header>
          <strong>{item.title}</strong>
          {!compact && <p>{item.body}</p>}
        </article>
      ))}
    </div>
  );
}

function AppearanceSummary({
  appearance,
  workerId,
  openReport,
}: {
  appearance: ProfileAppearance;
  workerId: string;
  openReport?: (showId: number) => void;
}) {
  const others = appearance.participants
    .filter((participant) => participant.workerId !== workerId)
    .map((participant) => participant.name)
    .join(', ');
  return (
    <article className={s.appearanceCard}>
      <header>
        <div>
          <span className={s.eyebrow}>
            {humanize(appearance.kind)} · {gameDate(appearance.date)}
          </span>
          <h3>{appearance.title}</h3>
          <p>
            {appearance.showName}
            {others ? ` · with ${others}` : ''}
          </p>
        </div>
        <strong>{clockTime(appearance.durationSeconds)}</strong>
      </header>
      <p className={s.appearanceResult}>{appearance.result}</p>
      {openReport && (
        <button onClick={() => openReport(appearance.showId)}>
          Open show report
        </button>
      )}
      <dl className={s.performanceStrip}>
        {Object.entries(appearance.performance).map(([key, value]) => (
          <div key={key}>
            <dt>{humanize(key)}</dt>
            <dd className={scoreBand(value).className}>{value}</dd>
          </div>
        ))}
      </dl>
      {appearance.reasons.length > 0 && (
        <details>
          <summary>Performance explanation</summary>
          <ul>
            {appearance.reasons.map((reason) => (
              <li key={reason}>{reason}</li>
            ))}
          </ul>
        </details>
      )}
    </article>
  );
}

function Overview({
  profile,
  openTab,
  openReport,
}: {
  profile: WorkerProfile;
  openTab: (tab: ProfileTab) => void;
  openReport: (showId: number) => void;
}) {
  return (
    <div className={s.profileOverviewLayout}>
      <main className={s.profileOverviewMain}>
        <Panel
          title="Wrestling profile"
          action={
            <button onClick={() => openTab('attributes')}>
              Full attributes
            </button>
          }
        >
          <GroupRatings profile={profile} />
          <div className={s.profileSummaryLine}>
            <span>
              <b>{profile.wrestling.archetype}</b> archetype
            </span>
            <span>
              <b>{humanize(profile.wrestling.primary)}</b> primary Discipline
            </span>
            <span>
              <b>{profile.worker.condition.popularity}</b> current popularity
            </span>
          </div>
        </Panel>
        <div className={s.profileTwoColumn}>
          <Panel title="Current state">
            <div className={s.meters}>
              <Meter
                label="Popularity"
                value={profile.worker.condition.popularity}
              />
              <Meter
                label="Momentum"
                value={profile.worker.condition.momentum}
              />
              <Meter label="Morale" value={profile.worker.condition.morale} />
              <Meter
                label="Confidence"
                value={profile.worker.condition.confidence}
              />
              <Meter label="Fatigue" value={profile.worker.condition.fatigue} />
              <Meter
                label="Wear and tear"
                value={profile.worker.condition.wear}
              />
            </div>
          </Panel>
          <Panel title="Next booking">
            <BookingCard profile={profile} />
          </Panel>
        </div>
        <Panel
          title="Recent form"
          action={
            <button onClick={() => openTab('appearances')}>
              Complete history
            </button>
          }
        >
          {profile.history.length ? (
            <div className={s.recentFormList}>
              {profile.history.slice(0, 3).map((appearance) => (
                <AppearanceSummary
                  key={`${appearance.showId}-${appearance.segmentId}`}
                  appearance={appearance}
                  workerId={profile.worker.id}
                  openReport={openReport}
                />
              ))}
            </div>
          ) : (
            <p className={s.empty}>
              No performances recorded in this career yet.
            </p>
          )}
        </Panel>
      </main>
      <aside className={s.profileMediaRail}>
        <Panel title="Information">
          <p>{profile.biography}</p>
          <button onClick={() => openTab('career')}>
            Open personal record
          </button>
        </Panel>
        <Panel
          title="Recent news"
          action={<button onClick={() => openTab('media')}>All media</button>}
        >
          <NewsList profile={profile} compact />
        </Panel>
        <Panel title="Relationship pulse">
          <strong>{profile.relationships.management.summary}</strong>
          <p className={s.help}>
            Their current relationship with your management team.
          </p>
          <button onClick={() => openTab('relationships')}>
            Talk and review history
          </button>
        </Panel>
      </aside>
    </div>
  );
}

function Attributes({ profile }: { profile: WorkerProfile }) {
  return (
    <div className={s.profileSectionStack}>
      <Panel title="Six base ratings">
        <GroupRatings profile={profile} />
      </Panel>
      <div className={s.profileAttributeGrid}>
        {Object.entries(profile.worker.attributes).map(([group, values]) => (
          <Panel title={humanize(group)} key={group}>
            <dl className={s.profileAttributeList}>
              {Object.entries(values).map(([name, value]) => (
                <div key={name}>
                  <dt>{humanize(name)}</dt>
                  <dd>
                    <Rating label={humanize(name)} value={value} />
                  </dd>
                </div>
              ))}
            </dl>
          </Panel>
        ))}
      </div>
      <div className={s.profileTwoColumn}>
        <Panel title="Discipline fit">
          <dl className={s.profileAttributeList}>
            {profile.wrestling.disciplineFits.map((fit) => (
              <div key={fit.discipline}>
                <dt>{humanize(fit.discipline)}</dt>
                <dd>
                  <Rating label={humanize(fit.discipline)} value={fit.score} />
                </dd>
              </div>
            ))}
          </dl>
        </Panel>
        <Panel title="Style identity">
          <dl className={s.attributes}>
            <div>
              <dt>Primary</dt>
              <dd>{humanize(profile.wrestling.primary)}</dd>
            </div>
            <div>
              <dt>Secondary</dt>
              <dd>
                {profile.wrestling.secondaries.map(humanize).join(', ') ||
                  'None established'}
              </dd>
            </div>
            {Object.entries(profile.worker.wrestlingStyle.approach).map(
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
                {profile.worker.wrestlingStyle.specialisations
                  .map(humanize)
                  .join(', ') || 'None established'}
              </dd>
            </div>
          </dl>
        </Panel>
      </div>
      <Panel title="Working repertoire">
        {profile.worker.moves.length ? (
          <div className={s.profileTableScroll}>
            <table>
              <thead>
                <tr>
                  <th>Move</th>
                  <th>Use</th>
                  <th>Style</th>
                  <th>Proficiency</th>
                  <th>Difficulty</th>
                  <th>Risk</th>
                </tr>
              </thead>
              <tbody>
                {profile.worker.moves.map((move) => (
                  <tr key={move.id}>
                    <th>{move.name}</th>
                    <td>{move.signature ? 'Signature' : 'Regular'}</td>
                    <td>{move.style}</td>
                    <td>
                      <Rating
                        label={`${move.name} proficiency`}
                        value={move.proficiency}
                      />
                    </td>
                    <td>{move.difficulty}/20</td>
                    <td>{move.risk}/5</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <p className={s.empty}>No working repertoire is recorded.</p>
        )}
      </Panel>
    </div>
  );
}

function Career({ profile }: { profile: WorkerProfile }) {
  const timeline = useMemo(() => {
    const characters = profile.character.history.map((character) => ({
      date: character.startedOn,
      title: `${character.ringName} character began`,
      detail: character.gimmick.name,
    }));
    const appearances = profile.history.map((appearance) => ({
      date: appearance.date,
      title: appearance.title,
      detail: `${humanize(appearance.kind)} · ${appearance.showName}`,
    }));
    return [...characters, ...appearances]
      .sort((a, b) => b.date.localeCompare(a.date))
      .slice(0, 30);
  }, [profile]);
  return (
    <div className={s.profileSectionStack}>
      <Panel title="Personal record">
        <dl className={s.attributes}>
          <div>
            <dt>Legal name</dt>
            <dd>{profile.character.legalName ?? 'Not recorded'}</dd>
          </div>
          <div>
            <dt>Public ring name</dt>
            <dd>{profile.character.active.ringName}</dd>
          </div>
          <div>
            <dt>Nationality</dt>
            <dd>{profile.worker.nationality}</dd>
          </div>
          <div>
            <dt>Language</dt>
            <dd>{profile.worker.language}</dd>
          </div>
          <div>
            <dt>Training background</dt>
            <dd>{profile.worker.school}</dd>
          </div>
          <div>
            <dt>Career matches</dt>
            <dd>{profile.worker.condition.matches}</dd>
          </div>
        </dl>
        <p className={s.help}>
          Legal identity is private profile information and is not used as a
          public booking label.
        </p>
      </Panel>
      <PersonIdentityPanel
        identity={profile.worker.identity}
        description={profile.personalityDescription}
        biography={profile.biography}
        traits={profile.exceptionalTraits}
      />
      <Panel title="Career timeline">
        {timeline.length ? (
          <ol className={s.careerTimeline}>
            {timeline.map((entry, index) => (
              <li key={`${entry.date}-${entry.title}-${index}`}>
                <time>{gameDate(entry.date)}</time>
                <strong>{entry.title}</strong>
                <span>{entry.detail}</span>
              </li>
            ))}
          </ol>
        ) : (
          <p className={s.empty}>No dated career events are recorded yet.</p>
        )}
      </Panel>
    </div>
  );
}

function Appearances({
  profile,
  openReport,
}: {
  profile: WorkerProfile;
  openReport: (showId: number) => void;
}) {
  return profile.history.length ? (
    <div className={s.profileSectionStack}>
      <div className={s.sectionIntro}>
        <div>
          <span className={s.eyebrow}>Newest first · latest 30</span>
          <h2>Matches & appearances</h2>
        </div>
        <span>{profile.history.length} recorded</span>
      </div>
      {profile.history.map((appearance) => (
        <AppearanceSummary
          key={`${appearance.showId}-${appearance.segmentId}`}
          appearance={appearance}
          workerId={profile.worker.id}
          openReport={openReport}
        />
      ))}
    </div>
  ) : (
    <p className={s.empty}>No performances recorded in this career yet.</p>
  );
}

function Contract({ profile }: { profile: WorkerProfile }) {
  return (
    <div className={s.profileTwoColumn}>
      <Panel title="Current terms">
        <span className={s.eyebrow}>Per-appearance arrangement</span>
        <p className={s.contractValue}>
          {currency(profile.worker.appearanceFee)} <small>per show</small>
        </p>
        <p>
          No long-form agreement is recorded. Booking this person uses the
          stated appearance terms.
        </p>
      </Panel>
      <Panel title="Availability">
        <div className={s.profileStatusCallout}>
          <Icon name="health" />
          <div>
            <strong>
              {profile.worker.condition.injuryDays
                ? 'Not medically cleared'
                : 'Medically cleared'}
            </strong>
            <p>
              {profile.worker.condition.injuryDays
                ? `${profile.worker.condition.injuryDays} recovery days remain.`
                : 'Available for match booking, subject to fatigue and existing plans.'}
            </p>
          </div>
        </div>
        <BookingCard profile={profile} />
      </Panel>
    </div>
  );
}

function Development({ profile }: { profile: WorkerProfile }) {
  return (
    <div className={s.profileSectionStack}>
      <Panel title="Development record">
        <Meter
          label="Recorded development"
          value={profile.worker.condition.development}
        />
        <dl className={s.attributes}>
          <div>
            <dt>Age</dt>
            <dd>{profile.worker.age}</dd>
          </div>
          <div>
            <dt>Match experience</dt>
            <dd>{profile.worker.condition.matches}</dd>
          </div>
          <div>
            <dt>Current Discipline</dt>
            <dd>{humanize(profile.wrestling.primary)}</dd>
          </div>
          <div>
            <dt>Current archetype</dt>
            <dd>{profile.wrestling.archetype}</dd>
          </div>
        </dl>
        <p className={s.help}>
          This record reflects development already earned through completed
          appearances. Training plans will appear only when the career contains
          canonical training activity.
        </p>
      </Panel>
    </div>
  );
}

function Media({ profile }: { profile: WorkerProfile }) {
  return (
    <div className={s.profileTwoColumn}>
      <Panel title="Worker-linked news">
        <NewsList profile={profile} />
      </Panel>
      <Panel title="Media context">
        <p>{profile.biography}</p>
        <dl className={s.attributes}>
          <div>
            <dt>Current character</dt>
            <dd>{profile.character.active.gimmick.name}</dd>
          </div>
          <div>
            <dt>Audience evidence</dt>
            <dd>{profile.character.audienceResponses.length} recorded</dd>
          </div>
          <div>
            <dt>Linked coverage</dt>
            <dd>{profile.recentNews.length} recorded</dd>
          </div>
        </dl>
        <p className={s.help}>
          Private conversations remain in Relationships and the company inbox;
          they are not public posts.
        </p>
      </Panel>
    </div>
  );
}

function FullProfile({
  profile,
  saveId,
  openReport,
}: {
  profile: WorkerProfile;
  saveId: string;
  openReport: (showId: number) => void;
}) {
  const body = useRef<HTMLDivElement>(null);
  const tab = useNavigation((state) => state.profileTab);
  const setTab = useNavigation((state) => state.setProfileTab);
  useEffect(() => {
    if (body.current) body.current.scrollTop = 0;
  }, [tab]);
  return (
    <>
      <nav className={s.profileTabs} aria-label="Person profile sections">
        {tabs.map((item) => (
          <button
            key={item.id}
            aria-current={tab === item.id ? 'page' : undefined}
            onClick={() => setTab(item.id)}
          >
            <Icon name={item.icon} />
            {item.label}
          </button>
        ))}
      </nav>
      <div className={s.profileBody} ref={body}>
        {tab === 'overview' ? (
          <Overview
            profile={profile}
            openTab={setTab}
            openReport={openReport}
          />
        ) : tab === 'attributes' ? (
          <Attributes profile={profile} />
        ) : tab === 'character' ? (
          <CharacterPresentation
            saveId={saveId}
            workerId={profile.worker.id}
            character={profile.character}
          />
        ) : tab === 'career' ? (
          <Career profile={profile} />
        ) : tab === 'appearances' ? (
          <Appearances profile={profile} openReport={openReport} />
        ) : tab === 'relationships' ? (
          <RelationshipsPanel
            saveId={saveId}
            workerId={profile.worker.id}
            relationships={profile.relationships}
          />
        ) : tab === 'contract' ? (
          <Contract profile={profile} />
        ) : tab === 'development' ? (
          <Development profile={profile} />
        ) : (
          <Media profile={profile} />
        )}
      </div>
    </>
  );
}

function QuickProfile({ profile }: { profile: WorkerProfile }) {
  const setMode = useNavigation((state) => state.setProfileMode);
  const setTab = useNavigation((state) => state.setProfileTab);
  const openFull = (tab: ProfileTab) => {
    setTab(tab);
    setMode('full');
  };
  return (
    <div className={s.quickProfileBody}>
      <GroupRatings profile={profile} />
      <div className={s.quickCondition}>
        <Meter label="Popularity" value={profile.worker.condition.popularity} />
        <Meter label="Momentum" value={profile.worker.condition.momentum} />
        <Meter label="Morale" value={profile.worker.condition.morale} />
        <Meter label="Fatigue" value={profile.worker.condition.fatigue} />
      </div>
      <Panel title="Next booking">
        <BookingCard profile={profile} />
      </Panel>
      <Panel title="Latest information">
        <NewsList profile={profile} compact />
      </Panel>
      <div className={s.quickProfileActions}>
        <button onClick={() => openFull('relationships')}>
          Speak with {profile.worker.name}
        </button>
        <button className={s.primary} onClick={() => openFull('overview')}>
          Full profile
        </button>
      </div>
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
  const ref = useRef<HTMLDialogElement>(null);
  const mode = useNavigation((state) => state.profileMode);
  const setMode = useNavigation((state) => state.setProfileMode);
  const setTab = useNavigation((state) => state.setProfileTab);
  const openReport = useNavigation((state) => state.openReport);
  const compareIds = useNavigation((state) => state.workerCompareIds);
  const setCompareIds = useNavigation((state) => state.setWorkerCompareIds);
  const profile = useQuery({
    queryKey: ['profile', saveId, workerId],
    queryFn: () => gameApi.profile(saveId, workerId),
  });
  useEffect(() => {
    const dialog = ref.current;
    dialog?.showModal();
    return () => dialog?.close();
  }, []);
  const alreadyCompared = compareIds.includes(workerId);
  const comparisonFull = compareIds.length >= 4 && !alreadyCompared;
  const addToComparison = () => {
    if (!alreadyCompared && !comparisonFull)
      setCompareIds([...compareIds, workerId]);
  };
  const openShowReport = (showId: number) => {
    onClose();
    openReport(showId);
  };
  return (
    <dialog
      ref={ref}
      aria-label={
        profile.data ? `${profile.data.worker.name} profile` : 'Person profile'
      }
      className={`${s.profileDialog} ${mode === 'full' ? s.profileDialogFull : s.profileDialogQuick}`}
      onCancel={(event) => {
        event.preventDefault();
        onClose();
      }}
    >
      <header className={s.profileDialogBar}>
        <span>{mode === 'full' ? 'PERSON HUB' : 'QUICK PROFILE'}</span>
        <div>
          {profile.data && (
            <details className={s.profileActions}>
              <summary>
                <Icon name="actions" />
                Actions
              </summary>
              <div>
                <button
                  onClick={() => {
                    setTab('relationships');
                    setMode('full');
                  }}
                >
                  Speak with person
                </button>
                <button
                  onClick={() => {
                    setTab('character');
                    setMode('full');
                  }}
                >
                  Manage character
                </button>
                <button
                  disabled={alreadyCompared || comparisonFull}
                  onClick={addToComparison}
                >
                  {alreadyCompared
                    ? 'Added to comparison'
                    : comparisonFull
                      ? 'Comparison full'
                      : 'Add to comparison'}
                </button>
              </div>
            </details>
          )}
          {mode === 'quick' && (
            <button onClick={() => setMode('full')}>Full profile</button>
          )}
          <button onClick={onClose} aria-label="Close person profile">
            ×
          </button>
        </div>
      </header>
      {profile.isError ? (
        <ErrorNotice message={errorMessage(profile.error)} />
      ) : !profile.data ? (
        <p className={s.profileLoading} role="status">
          Loading person profile…
        </p>
      ) : (
        <>
          <ProfileHeader profile={profile.data} />
          {mode === 'quick' ? (
            <QuickProfile profile={profile.data} />
          ) : (
            <FullProfile
              profile={profile.data}
              saveId={saveId}
              openReport={openShowReport}
            />
          )}
        </>
      )}
    </dialog>
  );
}
