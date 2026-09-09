import { useEffect, useState, type FormEvent } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type {
  CareerOffice,
  ShowCard,
  Segment,
  SegmentPlan,
  MatchPlan,
  RosterRow,
  PlannedBeat,
  BeatKind,
  Finish,
} from '@wm/contracts';
import { gameApi, errorMessage } from '../api';
import { useNavigation } from '../navigation';
import { Panel, ErrorNotice, clockTime, gameDate } from '../game-ui';
import s from '../Game.module.css';
import { useHistory } from '../useHistory';

const finishOptions: { value: Finish; label: string }[] = [
  { value: 'pinfall', label: 'Pinfall' },
  { value: 'submission', label: 'Submission' },
  { value: 'countOut', label: 'Count-out' },
  { value: 'disqualification', label: 'Disqualification' },
  { value: 'draw', label: 'Time-limit draw' },
  { value: 'noContest', label: 'No contest' },
];
const beats: { value: BeatKind; label: string }[] = [
  { value: 'move', label: 'Specific move' },
  { value: 'control', label: 'Control period' },
  { value: 'falseFinish', label: 'False finish' },
  { value: 'interference', label: 'Interference' },
  { value: 'weapon', label: 'Weapon spot' },
  { value: 'refBump', label: 'Referee bump' },
  { value: 'callback', label: 'Callback / counter' },
  { value: 'injurySell', label: 'Sell worked injury' },
];
export const defaultPlan = (rows: RosterRow[]): MatchPlan => ({
  matchType: 'Singles',
  workerA: rows[0]?.id ?? '',
  workerB: rows[1]?.id ?? '',
  winnerId: rows[0]?.id ?? null,
  finish: 'pinfall',
  cleanFinish: true,
  durationSeconds: 600,
  style: rows[0]?.style ?? 'Technical',
  pace: 3,
  risk: 2,
  freedom: 65,
  purpose: 'Establish the winner; give the opponent a credible showing.',
  protectedWorkerId: null,
  agentId: 1,
  beats: [],
});

export function Booking({
  saveId,
  office,
}: {
  saveId: string;
  office: CareerOffice;
}) {
  const cache = useQueryClient();
  const openLive = useNavigation((v) => v.openLive);
  const query = useQuery({
    queryKey: ['card', saveId, office.show.id],
    queryFn: () => gameApi.card(saveId, office.show.id),
    initialData: office.show,
  });
  const roster = useQuery({
    queryKey: ['booking-roster', saveId],
    queryFn: () => gameApi.roster(saveId),
  });
  const [selected, setSelected] = useState<number | null>(null),
    [kind, setKind] = useState<'match' | 'angle'>('match');
  const show = query.data;
  const total = show.segments.reduce(
    (sum, segment) => sum + segment.content.plan.durationSeconds,
    0,
  );
  const accept = (card: ShowCard) => {
    cache.setQueryData(['card', saveId, card.id], card);
    void cache.invalidateQueries({ queryKey: ['office', saveId] });
  };
  const start = useMutation({
    mutationFn: () => gameApi.startShow(saveId, show.id),
    onSuccess: (view) => {
      cache.setQueryData(['live', saveId, show.id], view);
      void cache.invalidateQueries({ queryKey: ['office', saveId] });
      openLive(show.id);
    },
  });
  const remove = useMutation({
    mutationFn: (id: number) =>
      gameApi.deleteSegment(saveId, show.id, show.revision, id),
    onSuccess: (card) => {
      accept(card);
      setSelected(null);
    },
  });
  const reorder = useMutation({
    mutationFn: (ids: number[]) =>
      gameApi.reorder(saveId, show.id, show.revision, ids),
    onSuccess: accept,
  });
  const move = (index: number, direction: number) => {
    const ids = show.segments.map((x) => x.id);
    const from = ids[index],
      to = ids[index + direction];
    if (from !== undefined && to !== undefined) {
      ids[index] = to;
      ids[index + direction] = from;
      reorder.mutate(ids);
    }
  };
  const editable = show.status === 'draft';
  const picked = show.segments.find((x) => x.id === selected);
  return (
    <div className={s.screenColumn}>
      <div className={s.toolbar}>
        <div>
          <b>{show.name}</b>
          <span>
            {gameDate(show.date)} · {clockTime(total)} booked / 120:00 available
          </span>
        </div>
        <div className={s.spacer} />
        <button
          onClick={() => {
            setSelected(null);
            setKind('angle');
          }}
          disabled={!editable}
        >
          + Angle
        </button>
        <button
          onClick={() => {
            setSelected(null);
            setKind('match');
          }}
          disabled={!editable}
        >
          + Match
        </button>
        <button
          className={s.primary}
          disabled={
            start.isPending ||
            !show.segments.length ||
            show.date !== office.promotion.currentDate
          }
          onClick={() => start.mutate()}
        >
          {show.status === 'live' ? 'Resume show' : 'Go on air'}
        </button>
      </div>
      {(start.isError ||
        remove.isError ||
        reorder.isError ||
        query.isError) && (
        <ErrorNotice
          message={errorMessage(
            start.error ?? remove.error ?? reorder.error ?? query.error,
          )}
        />
      )}
      <div className={s.bookingGrid}>
        <Panel title="Running order" className={s.cardPanel}>
          <div className={s.scroll}>
            {show.segments.length ? (
              show.segments.map((segment, index) => (
                <div
                  className={`${s.cardRow} ${selected === segment.id ? s.selected : ''}`}
                  key={segment.id}
                  draggable={editable && !reorder.isPending}
                  onDragStart={(event) =>
                    event.dataTransfer.setData(
                      'application/x-wm-segment',
                      String(segment.id),
                    )
                  }
                  onDragOver={(event) => {
                    if (editable) event.preventDefault();
                  }}
                  onDrop={(event) => {
                    event.preventDefault();
                    const source = Number(
                      event.dataTransfer.getData('application/x-wm-segment'),
                    );
                    if (
                      !editable ||
                      reorder.isPending ||
                      source === segment.id ||
                      !show.segments.some((s) => s.id === source)
                    )
                      return;
                    const ids = show.segments
                      .map((s) => s.id)
                      .filter((id) => id !== source);
                    ids.splice(ids.indexOf(segment.id), 0, source);
                    reorder.mutate(ids);
                  }}
                >
                  <button
                    className={s.cardSelect}
                    onClick={() => {
                      setSelected(segment.id);
                      setKind(segment.content.kind);
                    }}
                  >
                    <span className={s.orderNumber}>
                      {String(index + 1).padStart(2, '0')}
                    </span>
                    <span>
                      <strong>{segment.title}</strong>
                      <small>
                        {segment.content.kind === 'match'
                          ? `${segment.content.plan.matchType} · ${segment.content.plan.style}`
                          : 'Character segment'}
                      </small>
                    </span>
                    <b>{clockTime(segment.content.plan.durationSeconds)}</b>
                  </button>
                  <div className={s.rowActions}>
                    <button
                      title="Move earlier"
                      aria-label={`Move segment ${index + 1} earlier`}
                      disabled={!editable || index === 0 || reorder.isPending}
                      onClick={() => move(index, -1)}
                    >
                      ↑
                    </button>
                    <button
                      title="Move later"
                      aria-label={`Move segment ${index + 1} later`}
                      disabled={
                        !editable ||
                        index === show.segments.length - 1 ||
                        reorder.isPending
                      }
                      onClick={() => move(index, 1)}
                    >
                      ↓
                    </button>
                    <button
                      title="Remove segment"
                      aria-label={`Remove segment ${index + 1}`}
                      disabled={!editable || remove.isPending}
                      onClick={() => remove.mutate(segment.id)}
                    >
                      Remove
                    </button>
                  </div>
                </div>
              ))
            ) : (
              <div className={s.empty}>
                <h3>The running order is yours.</h3>
                <p>
                  Choose two wrestlers, lock the result, then let your agent
                  help with the performance.
                </p>
              </div>
            )}
          </div>
          <div className={s.cardFooter}>
            <b>{show.segments.length} segments</b>
            <span>Unused time {clockTime(show.capacitySeconds - total)}</span>
          </div>
        </Panel>
        {roster.data?.rows.length ? (
          <PlanEditor
            key={`${selected ?? 'new'}-${kind}`}
            saveId={saveId}
            office={office}
            show={show}
            initial={picked}
            kind={kind}
            rows={roster.data.rows}
            onSaved={(card) => {
              accept(card);
              if (selected === null)
                setSelected(card.segments.at(-1)?.id ?? null);
            }}
          />
        ) : (
          <Panel title="Talent">
            <p className={s.empty} role="status">
              Loading your booking roster…
            </p>
          </Panel>
        )}
      </div>
    </div>
  );
}

function PlanEditor({
  saveId,
  office,
  show,
  initial,
  kind,
  rows,
  onSaved,
}: {
  saveId: string;
  office: CareerOffice;
  show: ShowCard;
  initial: Segment | undefined;
  kind: 'match' | 'angle';
  rows: RosterRow[];
  onSaved: (show: ShowCard) => void;
}) {
  const matchHistory = useHistory<MatchPlan>(
    initial?.content.kind === 'match'
      ? initial.content.plan
      : defaultPlan(rows),
  );
  const { value: plan, set: setPlan } = matchHistory;
  const angleHistory = useHistory(
    initial?.content.kind === 'angle'
      ? initial.content.plan
      : {
          participants: [rows[0]?.id ?? ''],
          purpose: 'Challenge',
          durationSeconds: 180,
        },
  );
  const { value: angle, set: setAngle } = angleHistory;
  const history = kind === 'match' ? matchHistory : angleHistory;
  const currentDraft = JSON.stringify(kind === 'match' ? plan : angle);
  const [savedDraft, setSavedDraft] = useState(currentDraft);
  const setPlannerDirty = useNavigation((state) => state.setPlannerDirty);
  useEffect(() => {
    setPlannerDirty(currentDraft !== savedDraft);
    return () => setPlannerDirty(false);
  }, [currentDraft, savedDraft, setPlannerDirty]);
  const [notes, setNotes] = useState<string[]>([]);
  const [tab, setTab] = useState<'brief' | 'sequence'>('brief');
  const [focusedBeat, setFocusedBeat] = useState<number | null>(null);
  useEffect(() => {
    if (focusedBeat === null || tab !== 'sequence') return;
    const input = document.getElementById(`plan-beat-${focusedBeat}`);
    input?.scrollIntoView({ block: 'nearest' });
    input?.focus({ preventScroll: true });
  }, [focusedBeat, tab]);
  const a = useQuery({
    queryKey: ['profile', saveId, plan.workerA],
    queryFn: () => gameApi.profile(saveId, plan.workerA),
    enabled: !!plan.workerA,
  });
  const b = useQuery({
    queryKey: ['profile', saveId, plan.workerB],
    queryFn: () => gameApi.profile(saveId, plan.workerB),
    enabled: !!plan.workerB,
  });
  const save = useMutation({
    mutationFn: (content: SegmentPlan) =>
      gameApi.saveSegment({
        saveId,
        showId: show.id,
        revision: show.revision,
        segmentId: initial?.id ?? null,
        content,
      }),
    onSuccess: (card, content) => {
      setSavedDraft(JSON.stringify(content.plan));
      onSaved(card);
    },
  });
  const advice = useMutation({
    mutationFn: (input: MatchPlan) => gameApi.agent(saveId, input),
    onSuccess: (result) => {
      setPlan(result.plan);
      setNotes(result.notes);
      setTab('sequence');
    },
  });
  const busy = save.isPending || advice.isPending || show.status !== 'draft';
  const agent = office.agents.find((a) => a.id === plan.agentId);
  const warnings: string[] = [];
  if (kind === 'match') {
    const workers = [a.data?.worker, b.data?.worker].filter(
      (w) => w !== undefined,
    );
    if (
      plan.durationSeconds > 1200 &&
      workers.some((w) => w.attributes.stamina < 13)
    )
      warnings.push(
        'The planned length exceeds one or both wrestlers’ conditioning. Expect fatigue and less reliable execution late on.',
      );
    if (workers.some((w) => w.condition.fatigue > 35))
      warnings.push(
        'A performer is carrying fatigue into this show. Consider recovery holds or a shorter match.',
      );
    if (plan.risk >= 4)
      warnings.push(
        'High risk creates more dangerous failed landings. Your agent can reduce the exposure, but cannot remove it.',
      );
    if (plan.pace >= 4 && plan.durationSeconds > 900)
      warnings.push(
        'A long match at high pace leaves less stamina for the finish.',
      );
  }
  function submit(event: FormEvent) {
    event.preventDefault();
    save.mutate(
      kind === 'match'
        ? { kind: 'match', plan }
        : { kind: 'angle', plan: angle },
    );
  }
  function choose(side: 'workerA' | 'workerB', id: string) {
    setPlan((p) => ({
      ...p,
      [side]: id,
      winnerId: p.winnerId === p[side] ? id : p.winnerId,
      protectedWorkerId:
        p.protectedWorkerId === p[side] ? id : p.protectedWorkerId,
      beats: p.beats.filter((b) => b.actorId !== p[side]),
    }));
  }
  function setBeat(index: number, patch: Partial<PlannedBeat>) {
    setPlan((p) => ({
      ...p,
      beats: p.beats.map((beat, i) =>
        i === index ? { ...beat, ...patch } : beat,
      ),
    }));
  }
  function template(type: string) {
    const next = { ...plan, beats: [] };
    if (type === 'sprint') {
      next.durationSeconds = 360;
      next.pace = 5;
      next.risk = 3;
      next.purpose = 'A fast opening contest to energise the building.';
    } else if (type === 'story') {
      next.durationSeconds = 900;
      next.pace = 2;
      next.risk = 2;
      next.protectedWorkerId = next.workerB;
      next.purpose =
        'Give the underdog a sustained hope spot before the decisive finish.';
    } else {
      next.durationSeconds = 600;
      next.pace = 3;
      next.risk = 2;
      next.purpose = 'Build a clear contest and a decisive closing stretch.';
    }
    advice.mutate(next);
  }
  return (
    <>
      <Panel
        title={initial ? 'Segment instructions' : `New ${kind}`}
        className={s.editor}
      >
        <form onSubmit={submit} className={s.editorForm}>
          <fieldset disabled={busy}>
            <div className={s.plannerHistory}>
              <button
                type="button"
                disabled={!history.canUndo}
                onClick={history.undo}
              >
                Undo edit
              </button>
              <button
                type="button"
                disabled={!history.canRedo}
                onClick={history.redo}
              >
                Redo edit
              </button>
              <small>Save to apply your instructions</small>
            </div>
            {kind === 'match' ? (
              <>
                <nav className={s.tabs}>
                  <button
                    type="button"
                    aria-pressed={tab === 'brief'}
                    onClick={() => setTab('brief')}
                  >
                    Match brief
                  </button>
                  <button
                    type="button"
                    aria-pressed={tab === 'sequence'}
                    onClick={() => setTab('sequence')}
                  >
                    Sequence planner <span>{plan.beats.length}</span>
                  </button>
                </nav>
                <div
                  className={s.planTimeline}
                  aria-label="Match plan timeline"
                >
                  <div>
                    <span>OPENING BELL · 00:00</span>
                    <span>
                      BOOKED FINISH · {clockTime(plan.durationSeconds)}
                    </span>
                  </div>
                  <div className={s.timelineTrack}>
                    {plan.beats.map((beat, index) => (
                      <button
                        type="button"
                        className={s.timelineSpot}
                        style={{
                          left: `${Math.max(0, Math.min(100, (beat.atSecond / plan.durationSeconds) * 100))}%`,
                        }}
                        key={index}
                        aria-label={`Edit beat ${index + 1} at ${clockTime(beat.atSecond)}`}
                        title={`${clockTime(beat.atSecond)} · ${beat.kind}`}
                        onClick={() => {
                          setTab('sequence');
                          setFocusedBeat(index);
                        }}
                      />
                    ))}
                  </div>
                  <small>
                    {plan.beats.length
                      ? 'Select a marker to edit its move, actor or timing.'
                      : 'The agent calls the exchanges. Add key beats when you want more control.'}
                  </small>
                </div>
                <div className={s.editorScroll}>
                  {tab === 'brief' ? (
                    <>
                      <div className={s.formGrid}>
                        <label>
                          Wrestler A
                          <select
                            aria-label="Wrestler A"
                            value={plan.workerA}
                            onChange={(e) => choose('workerA', e.target.value)}
                          >
                            {rows.map((w) => (
                              <option
                                key={w.id}
                                value={w.id}
                                disabled={w.condition.injuryDays > 0}
                              >
                                {w.name}
                                {w.condition.injuryDays ? ' · injured' : ''}
                              </option>
                            ))}
                          </select>
                        </label>
                        <label>
                          Wrestler B
                          <select
                            aria-label="Wrestler B"
                            value={plan.workerB}
                            onChange={(e) => choose('workerB', e.target.value)}
                          >
                            {rows.map((w) => (
                              <option
                                key={w.id}
                                value={w.id}
                                disabled={w.condition.injuryDays > 0}
                              >
                                {w.name}
                                {w.condition.injuryDays ? ' · injured' : ''}
                              </option>
                            ))}
                          </select>
                        </label>
                        <label>
                          Booked winner
                          <select
                            value={plan.winnerId ?? ''}
                            disabled={
                              plan.finish === 'draw' ||
                              plan.finish === 'noContest'
                            }
                            onChange={(e) =>
                              setPlan({
                                ...plan,
                                winnerId: e.target.value || null,
                              })
                            }
                          >
                            <option value="">No winner</option>
                            <option value={plan.workerA}>
                              {a.data?.worker.name ?? 'Wrestler A'}
                            </option>
                            <option value={plan.workerB}>
                              {b.data?.worker.name ?? 'Wrestler B'}
                            </option>
                          </select>
                        </label>
                        <label>
                          Finish
                          <select
                            value={plan.finish}
                            onChange={(e) => {
                              const finish = e.target.value as Finish;
                              setPlan({
                                ...plan,
                                finish,
                                winnerId:
                                  finish === 'draw' || finish === 'noContest'
                                    ? null
                                    : (plan.winnerId ?? plan.workerA),
                              });
                            }}
                          >
                            {finishOptions.map((o) => (
                              <option value={o.value} key={o.value}>
                                {o.label}
                              </option>
                            ))}
                          </select>
                        </label>
                        <label>
                          Match type
                          <select
                            value={plan.matchType}
                            onChange={(e) =>
                              setPlan({ ...plan, matchType: e.target.value })
                            }
                          >
                            <option>Singles</option>
                            <option>No disqualification</option>
                          </select>
                        </label>
                        <label>
                          Length (minutes)
                          <input
                            type="number"
                            min={2}
                            max={60}
                            step={0.5}
                            value={plan.durationSeconds / 60}
                            onChange={(e) =>
                              setPlan({
                                ...plan,
                                durationSeconds: Math.round(
                                  Number(e.target.value) * 60,
                                ),
                              })
                            }
                          />
                        </label>
                        <label>
                          Style
                          <select
                            value={plan.style}
                            onChange={(e) =>
                              setPlan({ ...plan, style: e.target.value })
                            }
                          >
                            {[
                              'Technical',
                              'Sports',
                              'Power',
                              'Lucha',
                              'Strong style',
                              'Hardcore',
                              'Entertainment',
                              'Comedy',
                            ].map((style) => (
                              <option key={style}>{style}</option>
                            ))}
                          </select>
                        </label>
                        <label>
                          Protect
                          <select
                            value={plan.protectedWorkerId ?? ''}
                            onChange={(e) =>
                              setPlan({
                                ...plan,
                                protectedWorkerId: e.target.value || null,
                              })
                            }
                          >
                            <option value="">No special protection</option>
                            <option value={plan.workerA}>
                              {a.data?.worker.name}
                            </option>
                            <option value={plan.workerB}>
                              {b.data?.worker.name}
                            </option>
                          </select>
                        </label>
                        <label title="Higher pace spends stamina faster. Long matches need space to recover.">
                          Pace · {plan.pace}/5
                          <input
                            type="range"
                            min={1}
                            max={5}
                            value={plan.pace}
                            onChange={(e) =>
                              setPlan({ ...plan, pace: Number(e.target.value) })
                            }
                          />
                        </label>
                        <label title="Controls the risk of automatically selected moves. Manual spots can push beyond it.">
                          Risk · {plan.risk}/5
                          <input
                            type="range"
                            min={1}
                            max={5}
                            value={plan.risk}
                            onChange={(e) =>
                              setPlan({ ...plan, risk: Number(e.target.value) })
                            }
                          />
                        </label>
                        <label className={s.wide}>
                          Performer freedom · {plan.freedom}%
                          <input
                            type="range"
                            min={0}
                            max={100}
                            value={plan.freedom}
                            onChange={(e) =>
                              setPlan({
                                ...plan,
                                freedom: Number(e.target.value),
                              })
                            }
                          />
                        </label>
                        <label className={s.wide}>
                          Match purpose
                          <input
                            maxLength={160}
                            required
                            value={plan.purpose}
                            onChange={(e) =>
                              setPlan({ ...plan, purpose: e.target.value })
                            }
                          />
                        </label>
                        <label className={s.check}>
                          <input
                            type="checkbox"
                            checked={plan.cleanFinish}
                            onChange={(e) =>
                              setPlan({
                                ...plan,
                                cleanFinish: e.target.checked,
                              })
                            }
                          />{' '}
                          Present the result as clean
                        </label>
                      </div>
                      <div className={s.agentBox}>
                        <label>
                          Road agent
                          <select
                            value={plan.agentId}
                            onChange={(e) =>
                              setPlan({
                                ...plan,
                                agentId: Number(e.target.value),
                              })
                            }
                          >
                            {office.agents.map((agent) => (
                              <option key={agent.id} value={agent.id}>
                                {agent.name}
                              </option>
                            ))}
                          </select>
                        </label>
                        <p>{agent?.philosophy}</p>
                        <small>
                          Psychology {agent?.psychology}/20 · Communication{' '}
                          {agent?.communication}/20 · Roster knowledge{' '}
                          {agent?.knowledge}%
                        </small>
                        <div className={s.actions}>
                          <button
                            type="button"
                            onClick={() => advice.mutate(plan)}
                          >
                            Ask agent to plan
                          </button>
                          <select
                            aria-label="Apply match template"
                            value=""
                            onChange={(e) => template(e.target.value)}
                          >
                            <option value="" disabled>
                              Use a template…
                            </option>
                            <option value="balanced">Balanced contest</option>
                            <option value="sprint">Opening sprint</option>
                            <option value="story">Underdog story</option>
                          </select>
                        </div>
                      </div>
                    </>
                  ) : (
                    <>
                      <div className={s.sequenceIntro}>
                        <p>
                          Place key beats at exact match times. Your agent
                          handles the exchanges between them. Moves come from
                          the selected actor’s repertoire.
                        </p>
                        <button
                          type="button"
                          onClick={() =>
                            setPlan((p) => ({
                              ...p,
                              beats: [
                                ...p.beats,
                                {
                                  atSecond: Math.min(
                                    (p.beats.at(-1)?.atSecond ?? 15) + 30,
                                    p.durationSeconds - 10,
                                  ),
                                  actorId: p.workerA,
                                  kind: 'move',
                                  moveId: a.data?.worker.moves[0]?.id ?? null,
                                  durationSeconds: 30,
                                },
                              ],
                            }))
                          }
                        >
                          + Add sequence beat
                        </button>
                        <button
                          type="button"
                          onClick={() =>
                            setPlan((p) => ({
                              ...p,
                              beats: [...p.beats].sort(
                                (a, b) => a.atSecond - b.atSecond,
                              ),
                            }))
                          }
                        >
                          Sort by time
                        </button>
                      </div>
                      {plan.beats.length === 0 && (
                        <p className={s.empty}>
                          No fixed spots. The road agent and performers will
                          construct the match around your brief.
                        </p>
                      )}
                      {plan.beats.map((beat, index) => {
                        const actor =
                          beat.actorId === plan.workerA
                            ? a.data?.worker
                            : b.data?.worker;
                        return (
                          <div className={s.beat} key={index}>
                            <label>
                              Time (sec)
                              <input
                                type="number"
                                aria-label={`Beat ${index + 1} time`}
                                id={`plan-beat-${index}`}
                                min={10}
                                max={plan.durationSeconds - 6}
                                value={beat.atSecond}
                                onChange={(e) =>
                                  setBeat(index, {
                                    atSecond: Number(e.target.value),
                                  })
                                }
                              />
                            </label>
                            <label>
                              Actor
                              <select
                                value={beat.actorId}
                                onChange={(e) =>
                                  setBeat(index, {
                                    actorId: e.target.value,
                                    moveId:
                                      beat.kind === 'move'
                                        ? ((e.target.value === plan.workerA
                                            ? a.data?.worker.moves[0]?.id
                                            : b.data?.worker.moves[0]?.id) ??
                                          null)
                                        : null,
                                  })
                                }
                              >
                                <option value={plan.workerA}>
                                  {a.data?.worker.name}
                                </option>
                                <option value={plan.workerB}>
                                  {b.data?.worker.name}
                                </option>
                              </select>
                            </label>
                            <label>
                              Beat
                              <select
                                value={beat.kind}
                                onChange={(e) =>
                                  setBeat(index, {
                                    kind: e.target.value as BeatKind,
                                    moveId:
                                      e.target.value === 'move'
                                        ? (actor?.moves[0]?.id ?? null)
                                        : null,
                                  })
                                }
                              >
                                {beats.map((type) => (
                                  <option key={type.value} value={type.value}>
                                    {type.label}
                                  </option>
                                ))}
                              </select>
                            </label>
                            <label className={s.beatDetail}>
                              {beat.kind === 'move'
                                ? 'Move from repertoire'
                                : beat.kind === 'control'
                                  ? 'Control duration (sec)'
                                  : 'Direction'}
                              {beat.kind === 'move' ? (
                                <select
                                  aria-label={`Beat ${index + 1} move`}
                                  value={beat.moveId ?? ''}
                                  onChange={(e) =>
                                    setBeat(index, { moveId: e.target.value })
                                  }
                                >
                                  {actor?.moves.map((m) => (
                                    <option key={m.id} value={m.id}>
                                      {m.signature ? '★ ' : ''}
                                      {m.name} · {m.proficiency}%
                                    </option>
                                  ))}
                                </select>
                              ) : beat.kind === 'control' ? (
                                <input
                                  type="number"
                                  min={5}
                                  max={300}
                                  value={beat.durationSeconds}
                                  onChange={(e) =>
                                    setBeat(index, {
                                      durationSeconds: Number(e.target.value),
                                    })
                                  }
                                />
                              ) : (
                                <span>
                                  {beat.kind === 'injurySell'
                                    ? 'Worked injury, not a real injury'
                                    : 'Agent coordinates the execution'}
                                </span>
                              )}
                            </label>
                            <button
                              type="button"
                              aria-label={`Remove beat ${index + 1}`}
                              onClick={() =>
                                setPlan((p) => ({
                                  ...p,
                                  beats: p.beats.filter((_, i) => i !== index),
                                }))
                              }
                            >
                              ×
                            </button>
                          </div>
                        );
                      })}
                    </>
                  )}
                  {notes.length > 0 && (
                    <aside className={s.agentNotes}>
                      <b>Agent’s working notes</b>
                      {notes.map((note, i) => (
                        <p key={i}>{note}</p>
                      ))}
                    </aside>
                  )}
                </div>
              </>
            ) : (
              <div className={s.editorScroll}>
                <div className={s.formGrid}>
                  <label>
                    Primary performer
                    <select
                      value={angle.participants[0]}
                      onChange={(e) =>
                        setAngle({
                          ...angle,
                          participants: [
                            e.target.value,
                            ...angle.participants.slice(1),
                          ],
                        })
                      }
                    >
                      {rows.map((w) => (
                        <option key={w.id} value={w.id}>
                          {w.name}
                        </option>
                      ))}
                    </select>
                  </label>
                  <label>
                    Second performer
                    <select
                      value={angle.participants[1] ?? ''}
                      onChange={(e) =>
                        setAngle({
                          ...angle,
                          participants: e.target.value
                            ? [angle.participants[0] ?? '', e.target.value]
                            : angle.participants.slice(0, 1),
                        })
                      }
                    >
                      <option value="">None</option>
                      {rows.map((w) => (
                        <option key={w.id} value={w.id}>
                          {w.name}
                        </option>
                      ))}
                    </select>
                  </label>
                  <label>
                    Purpose
                    <select
                      value={angle.purpose}
                      onChange={(e) =>
                        setAngle({ ...angle, purpose: e.target.value })
                      }
                    >
                      {[
                        'Challenge',
                        'Confrontation',
                        'Interview',
                        'Celebration',
                        'Betrayal',
                        'Comedy',
                      ].map((value) => (
                        <option key={value}>{value}</option>
                      ))}
                    </select>
                  </label>
                  <label>
                    Length (minutes)
                    <input
                      type="number"
                      min={0.5}
                      max={10}
                      step={0.5}
                      value={angle.durationSeconds / 60}
                      onChange={(e) =>
                        setAngle({
                          ...angle,
                          durationSeconds: Math.round(
                            Number(e.target.value) * 60,
                          ),
                        })
                      }
                    />
                  </label>
                </div>
                <p className={s.help}>
                  This segment uses the performers’ entertainment ability and
                  the crowd state left by earlier segments.
                </p>
              </div>
            )}
            {(save.isError || advice.isError) && (
              <ErrorNotice message={errorMessage(save.error ?? advice.error)} />
            )}
            <div className={s.editorFooter}>
              <span>
                {kind === 'match'
                  ? 'Booked outcomes are locked.'
                  : 'The crowd carries into the next segment.'}
              </span>
              <button type="submit" className={s.primary}>
                {save.isPending
                  ? 'Saving…'
                  : initial
                    ? 'Save instructions'
                    : 'Add to running order'}
              </button>
            </div>
          </fieldset>
        </form>
      </Panel>
      <Panel title="Production meeting" className={s.bookingContext}>
        <div className={s.contextBody}>
          <span className={s.eyebrow}>STAFF ASSESSMENT</span>
          <h3>{kind === 'match' ? agent?.name : 'Segment producer'}</h3>
          <p>
            {kind === 'match'
              ? agent?.philosophy
              : 'Give the audience a reason to care about what follows.'}
          </p>
          {warnings.length ? (
            <>
              <h3>Booking warnings</h3>
              {warnings.map((warning) => (
                <p className={s.bookingWarning} key={warning}>
                  {warning}
                </p>
              ))}
              <small>
                You can keep this booking. These are risks, not guaranteed
                outcomes.
              </small>
            </>
          ) : (
            <p>
              No major conditioning or pace warnings for this brief. Execution
              and crowd response remain uncertain.
            </p>
          )}
          <h3>Show flow</h3>
          <p>
            {show.segments.length} saved segments. Crowd energy, fatigue and
            trust carry between them.
          </p>
          <p>
            {show.segments.filter(
              (segment) =>
                segment.content.kind === 'match' &&
                segment.content.plan.beats.some(
                  (beat) =>
                    beat.kind === 'interference' || beat.kind === 'refBump',
                ),
            ).length >= 2
              ? 'Multiple matches already use outside disruption. Repetition may frustrate the audience.'
              : 'Leave contrast between segments; an energetic opener can raise the standard for what follows.'}
          </p>
          <h3>Working brief</h3>
          <p>{kind === 'match' ? plan.purpose : angle.purpose}</p>
          <small>
            Advice is directional. The live performance determines the outcome
            of these choices.
          </small>
        </div>
      </Panel>
    </>
  );
}
