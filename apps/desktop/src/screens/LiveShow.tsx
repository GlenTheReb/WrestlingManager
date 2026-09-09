import { useEffect, useState } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type {
  LiveView,
  LiveInstruction,
  InstructionKind,
  Finish,
} from '@wm/contracts';
import { gameApi, errorMessage } from '../api';
import { Panel, Meter, ErrorNotice, clockTime } from '../game-ui';
import { useNavigation } from '../navigation';
import s from '../Game.module.css';

const modes = {
  full: { label: 'Full match · 1×', seconds: 1, delay: 1000, importance: 1 },
  quick: { label: 'Quick simulation', seconds: 15, delay: 220, importance: 1 },
  extended: {
    label: 'Extended highlights',
    seconds: 8,
    delay: 240,
    importance: 2,
  },
  key: { label: 'Key highlights', seconds: 25, delay: 240, importance: 3 },
  instant: { label: 'Instant result', seconds: 3600, delay: 50, importance: 3 },
};
type Mode = keyof typeof modes;
const instructions: { kind: InstructionKind; label: string }[] = [
  { kind: 'slowDown', label: 'Slow the pace' },
  { kind: 'raisePace', label: 'Raise the pace' },
  { kind: 'protect', label: 'Protect a wrestler' },
  { kind: 'goHome', label: 'Go to the finish' },
  { kind: 'extend', label: 'Extend by two minutes' },
  { kind: 'changeFinish', label: 'Alter finish method' },
  { kind: 'abandonSpot', label: 'Abandon a planned spot' },
  { kind: 'takeRisks', label: 'Allow more risk' },
];

export function LiveShow({
  saveId,
  showId,
}: {
  saveId: string;
  showId: number | null;
}) {
  const cache = useQueryClient();
  const nav = useNavigation((v) => v.navigate);
  const openReport = useNavigation((v) => v.openReport);
  const [playing, setPlaying] = useState(false),
    [mode, setMode] = useState<Mode>('full'),
    [instruction, setInstruction] = useState<InstructionKind>('slowDown');
  const [target, setTarget] = useState(''),
    [finish, setFinish] = useState<Finish>('submission'),
    [beat, setBeat] = useState(0);
  const [speed, setSpeed] = useState(1);
  const query = useQuery({
    queryKey: ['live', saveId, showId],
    queryFn: () => gameApi.live(saveId, showId!),
    enabled: showId !== null,
  });
  const card = useQuery({
    queryKey: ['card', saveId, showId],
    queryFn: () => gameApi.card(saveId, showId!),
    enabled: showId !== null,
  });
  const accept = (value: LiveView) => {
    const previous = cache.getQueryData<LiveView>(['live', saveId, showId]);
    if (!previous || value.tick >= previous.tick)
      cache.setQueryData(['live', saveId, showId], value);
    if (value.complete) {
      setPlaying(false);
      void cache.invalidateQueries({ queryKey: ['office', saveId] });
      void cache.invalidateQueries({ queryKey: ['news', saveId] });
      void cache.invalidateQueries({ queryKey: ['roster', saveId] });
      void cache.invalidateQueries({ queryKey: ['profile', saveId] });
      void cache.invalidateQueries({ queryKey: ['booking-roster', saveId] });
    }
  };
  const advance = useMutation({
    mutationFn: (view: LiveView) =>
      gameApi.advance({
        saveId,
        showId: view.showId,
        expectedTick: view.tick,
        seconds: modes[mode].seconds,
      }),
    onSuccess: accept,
    onError: () => setPlaying(false),
  });
  const send = useMutation({
    mutationFn: (value: LiveInstruction) =>
      gameApi.instruct(saveId, showId!, value),
    onSuccess: accept,
  });
  const view = query.data;
  const complete = view?.complete ?? false;
  const { mutate: advanceTick, isPending: advancing } = advance;
  useEffect(() => {
    if (!playing || !view || complete || advancing || send.isPending) return;
    const timer = setTimeout(
      () => advanceTick(view),
      modes[mode].delay / speed,
    );
    return () => clearTimeout(timer);
  }, [
    playing,
    view,
    complete,
    advancing,
    advanceTick,
    send.isPending,
    mode,
    speed,
  ]);
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (
        e.code === 'Space' &&
        !(e.target instanceof HTMLInputElement) &&
        !(e.target instanceof HTMLSelectElement) &&
        !(e.target instanceof HTMLButtonElement)
      ) {
        e.preventDefault();
        if (!complete) setPlaying((v) => !v);
      }
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [complete]);
  if (showId === null)
    return (
      <div className={s.empty}>
        <h2>No show is on air.</h2>
        <p>Build a running order and go on air from the booking screen.</p>
        <button onClick={() => nav('booking')}>Open booking</button>
      </div>
    );
  if (query.isError) return <ErrorNotice message={errorMessage(query.error)} />;
  if (!view)
    return (
      <p className={s.empty} role="status">
        Connecting to the production desk…
      </p>
    );
  const match = view.currentMatch;
  const currentSegment = card.data?.segments[view.segmentIndex];
  const upcomingBeats =
    currentSegment?.content.kind === 'match'
      ? currentSegment.content.plan.beats
          .map((b, i) => ({ ...b, index: i }))
          .filter((b) => b.atSecond > (match?.second ?? 0))
      : [];
  const events = view.events.filter(
    (e) => e.importance >= modes[mode].importance,
  );
  return (
    <div className={s.screenColumn}>
      <div className={s.productionBar}>
        <span className={`${s.onAir} ${playing ? s.livePulse : ''}`}>
          {complete ? 'OFF AIR' : playing ? 'ON AIR' : 'PAUSED'}
        </span>
        <div>
          <b>{view.segmentTitle}</b>
          <small>
            Show clock {clockTime(view.tick)} ·{' '}
            {match?.phase ?? (complete ? 'Show complete' : 'Production ready')}
          </small>
        </div>
        <div className={s.spacer} />
        <select
          aria-label="Viewing mode"
          value={mode}
          onChange={(e) => setMode(e.target.value as Mode)}
        >
          {Object.entries(modes).map(([value, m]) => (
            <option key={value} value={value}>
              {m.label}
            </option>
          ))}
        </select>
        <select
          aria-label="Playback speed"
          value={speed}
          onChange={(e) => setSpeed(Number(e.target.value))}
          disabled={mode === 'instant'}
        >
          {[1, 2, 4, 8].map((value) => (
            <option key={value} value={value}>
              {value}×
            </option>
          ))}
        </select>
        <button
          className={s.primary}
          disabled={complete || (advance.isPending && mode === 'instant')}
          onClick={() => setPlaying((v) => !v)}
        >
          {playing ? 'Pause' : 'Play'}
        </button>
        <button
          disabled={complete || advance.isPending || playing}
          onClick={() => advance.mutate(view)}
        >
          Advance {modes[mode].seconds}s
        </button>
        {complete && (
          <button onClick={() => openReport(view.showId)}>
            Open post-show report
          </button>
        )}
      </div>
      {(advance.isError || send.isError) && (
        <ErrorNotice message={errorMessage(advance.error ?? send.error)} />
      )}
      <div className={s.liveGrid}>
        <div className={s.matchCentre}>
          <div className={s.matchHeader}>
            <span>WM MATCH CENTRE · LIVE PRODUCTION</span>
            <b>
              {match
                ? `${clockTime(match.second)} / ${clockTime(match.durationSeconds)}`
                : clockTime(view.tick)}
            </b>
          </div>
          {match ? (
            <div className={s.matchup}>
              <div>
                <h2>{match.nameA}</h2>
                <Meter label="Stamina A" value={match.staminaA} />
              </div>
              <strong>VS</strong>
              <div>
                <h2>{match.nameB}</h2>
                <Meter label="Stamina B" value={match.staminaB} />
              </div>
            </div>
          ) : (
            <div className={s.segmentBanner}>
              <h2>{view.segmentTitle}</h2>
              <p>
                {complete
                  ? 'The performance is history. Review what it changed.'
                  : 'Press Play to begin the next segment.'}
              </p>
            </div>
          )}
          <div className={s.matchStrip}>
            <span>
              {match ? `Control: ${match.control}` : view.crowd.chant}
            </span>
            <span>
              {match
                ? `Pace ${match.pace}/5 · Risk ${match.risk}/5`
                : 'The crowd stays with the show.'}
            </span>
          </div>
          <Panel
            title="Live commentary"
            className={s.feedPanel}
            action={
              <span>
                {mode === 'full' ? 'SECOND-BY-SECOND' : 'FILTERED HIGHLIGHTS'}
              </span>
            }
          >
            <div
              className={s.eventFeed}
              aria-live={playing && mode !== 'full' ? 'off' : 'polite'}
            >
              {events.length ? (
                events
                  .slice()
                  .reverse()
                  .map((e) => (
                    <article
                      key={e.sequence}
                      className={`${s.event} ${e.importance === 3 ? s.keyEvent : ''}`}
                    >
                      <time>{clockTime(e.showSecond)}</time>
                      <div>
                        <small>{e.kind.replace(/([A-Z])/g, ' $1')}</small>
                        <p>{e.text}</p>
                      </div>
                    </article>
                  ))
              ) : (
                <p className={s.empty}>
                  The ring is ready. The first event will appear when the show
                  advances.
                </p>
              )}
            </div>
          </Panel>
        </div>
        <aside className={s.productionSide}>
          <Panel title="The room">
            <div className={s.meters}>
              <Meter label="Energy" value={view.crowd.energy} />
              <Meter label="Fatigue" value={view.crowd.fatigue} />
              <Meter label="Trust" value={view.crowd.trust} />
            </div>
            <div className={s.cohorts}>
              {view.crowd.cohorts.map((c) => (
                <div key={c.name}>
                  <b>{c.name}</b>
                  <span>
                    Energy {c.energy} · Trust {c.trust}
                  </span>
                </div>
              ))}
            </div>
          </Panel>
          <Panel title="Agent / referee channel">
            <div className={s.instructionForm}>
              <label>
                Instruction
                <select
                  value={instruction}
                  onChange={(e) =>
                    setInstruction(e.target.value as InstructionKind)
                  }
                >
                  {instructions.map((i) => (
                    <option key={i.kind} value={i.kind}>
                      {i.label}
                    </option>
                  ))}
                </select>
              </label>
              {instruction === 'protect' && (
                <label>
                  Protect
                  <select
                    value={target}
                    onChange={(e) => setTarget(e.target.value)}
                  >
                    <option value="">Choose wrestler…</option>
                    <option value={match?.workerA}>{match?.nameA}</option>
                    <option value={match?.workerB}>{match?.nameB}</option>
                  </select>
                </label>
              )}
              {instruction === 'changeFinish' && (
                <label>
                  Revised finish
                  <select
                    value={finish}
                    onChange={(e) => setFinish(e.target.value as Finish)}
                  >
                    <option value="pinfall">Pinfall</option>
                    <option value="submission">Submission</option>
                    <option value="countOut">Count-out</option>
                    <option value="disqualification">Disqualification</option>
                  </select>
                </label>
              )}
              {instruction === 'abandonSpot' && (
                <label>
                  Future beat
                  <select
                    value={beat}
                    onChange={(e) => setBeat(Number(e.target.value))}
                  >
                    <option value={-1}>Choose beat…</option>
                    {upcomingBeats.map((b) => (
                      <option key={b.index} value={b.index}>
                        {clockTime(b.atSecond)} · {b.kind}
                      </option>
                    ))}
                  </select>
                </label>
              )}
              <button
                disabled={
                  !match || complete || send.isPending || advance.isPending
                }
                onClick={() => {
                  setPlaying(false);
                  send.mutate({
                    kind: instruction,
                    workerId: instruction === 'protect' ? target || null : null,
                    finish: instruction === 'changeFinish' ? finish : null,
                    beatIndex: instruction === 'abandonSpot' ? beat : null,
                  });
                }}
              >
                Send via agent
              </button>
              <small>
                Delivery takes time. The booked winner remains locked. Sending
                pauses playback so you can read the acknowledgement.
              </small>
              {view.pendingInstructions.map((text, i) => (
                <p className={s.queued} key={i}>
                  {text}
                </p>
              ))}
            </div>
          </Panel>
        </aside>
      </div>
    </div>
  );
}
