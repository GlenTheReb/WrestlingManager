import { useEffect, useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import type {
  AlignmentIntent,
  CharacterProfile,
  GimmickBrief,
} from '@wm/contracts';
import { gameApi, errorMessage } from '../api';
import { ErrorNotice, Panel } from '../game-ui';
import { Icon } from '../icons';
import s from '../Game.module.css';

const label = (value: string) =>
  value
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/^./, (c) => c.toUpperCase());
const requestId = () =>
  globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;

export function CharacterPresentation({
  saveId,
  workerId,
  character,
}: {
  saveId: string;
  workerId: string;
  character: CharacterProfile;
}) {
  const cache = useQueryClient();
  const active = character.active;
  const [editing, setEditing] = useState(false);
  const [ringName, setRingName] = useState(active.ringName);
  const [alignment, setAlignment] = useState<AlignmentIntent>(
    active.alignmentIntent,
  );
  const [launchOn, setLaunchOn] = useState('');
  const [masked, setMasked] = useState(active.masked);
  const [concealed, setConcealed] = useState(active.concealed);
  const [gimmick, setGimmick] = useState<GimmickBrief>(active.gimmick);
  const [tagText, setTagText] = useState(active.gimmick.tags.join(', '));
  const [traitText, setTraitText] = useState(
    active.gimmick.traitsToEmphasize.join(', '),
  );
  const resetDraft = () => {
    setRingName(active.ringName);
    setAlignment(active.alignmentIntent);
    setLaunchOn('');
    setMasked(active.masked);
    setConcealed(active.concealed);
    setGimmick(active.gimmick);
    setTagText(active.gimmick.tags.join(', '));
    setTraitText(active.gimmick.traitsToEmphasize.join(', '));
  };
  useEffect(() => {
    if (!editing) resetDraft();
    // Reset only when the canonical identity changes outside an open draft.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [active.id, active.revision, editing]);
  const listFrom = (value: string) =>
    value
      .split(',')
      .map((item) => item.trim())
      .filter(Boolean);
  const refresh = () =>
    cache.invalidateQueries({ queryKey: ['profile', saveId, workerId] });
  const propose = useMutation({
    mutationFn: () =>
      gameApi.proposeCharacterChange({
        saveId,
        workerId,
        requestId: requestId(),
        expectedRevision: character.pendingChange?.revision ?? 0,
        ringName,
        alignmentIntent: alignment,
        masked,
        concealed,
        intendedLaunchOn: launchOn || null,
        gimmick: {
          ...gimmick,
          tags: listFrom(tagText),
          traitsToEmphasize: listFrom(traitText),
        },
      }),
    onSuccess: () => {
      setEditing(false);
      void refresh();
    },
  });
  const action = useMutation({
    mutationFn: (kind: 'launch' | 'retire' | 'revive') => {
      const request = {
        saveId,
        workerId,
        requestId: requestId(),
        changeId: character.pendingChange?.id ?? null,
        expectedRevision:
          kind === 'launch'
            ? (character.pendingChange?.revision ?? 0)
            : active.revision,
      };
      return kind === 'launch'
        ? gameApi.launchCharacterChange(request)
        : gameApi.setCharacterRetired(request, kind === 'retire');
    },
    onSuccess: () => void refresh(),
  });
  const error = propose.error ?? action.error;
  return (
    <div className={s.characterLayout}>
      {error && <ErrorNotice message={errorMessage(error)} />}
      <section className={s.characterHero}>
        <div className={s.characterIcon}>
          <Icon name="mask" />
        </div>
        <div>
          <span className={s.eyebrow}>Current wrestling identity</span>
          <h3>{active.ringName}</h3>
          <div className={s.chipRow}>
            <span
              className={`${s.statusChip} ${s[`alignment${label(active.alignmentIntent)}`]}`}
            >
              {label(active.alignmentIntent)}
            </span>
            <span className={s.statusChip}>{label(active.status)}</span>
            {active.masked && <span className={s.statusChip}>Masked</span>}
            {active.concealed && (
              <span className={s.statusChip}>Secret identity</span>
            )}
          </div>
        </div>
        <button
          className={s.primaryAction}
          onClick={() => {
            if (!editing) resetDraft();
            setEditing(!editing);
          }}
        >
          <Icon name="edit" /> Plan a change
        </button>
      </section>

      {editing && (
        <Panel title="Build the next presentation">
          <p className={s.help}>
            Describe the wrestling idea. Advisors calculate comfort and risk;
            this cannot edit wrestling ratings.
          </p>
          <div className={s.creativeForm}>
            <label>
              Ring name
              <input
                value={ringName}
                maxLength={60}
                onChange={(e) => setRingName(e.target.value)}
              />
            </label>
            <label>
              Alignment
              <select
                value={alignment}
                onChange={(e) =>
                  setAlignment(e.target.value as AlignmentIntent)
                }
              >
                <option value="face">Face</option>
                <option value="heel">Heel</option>
                <option value="tweener">Tweener</option>
                <option value="unaligned">Unaligned</option>
              </select>
            </label>
            <label>
              Gimmick name
              <input
                value={gimmick.name}
                maxLength={80}
                onChange={(e) =>
                  setGimmick({ ...gimmick, name: e.target.value })
                }
              />
            </label>
            <label>
              Core fantasy
              <input
                value={gimmick.coreFantasy}
                maxLength={160}
                onChange={(e) =>
                  setGimmick({ ...gimmick, coreFantasy: e.target.value })
                }
              />
            </label>
            <label className={s.fullField}>
              What should the audience understand?
              <textarea
                value={gimmick.description}
                maxLength={600}
                onChange={(e) =>
                  setGimmick({ ...gimmick, description: e.target.value })
                }
              />
            </label>
            <label>
              Tone
              <input
                value={gimmick.tone}
                onChange={(e) =>
                  setGimmick({ ...gimmick, tone: e.target.value })
                }
              />
            </label>
            <label>
              Promo voice
              <input
                value={gimmick.promoVoice}
                maxLength={120}
                onChange={(e) =>
                  setGimmick({ ...gimmick, promoVoice: e.target.value })
                }
              />
            </label>
            <label>
              Presentation level
              <select
                value={gimmick.presentationIntensity}
                onChange={(e) =>
                  setGimmick({
                    ...gimmick,
                    presentationIntensity: e.target.value,
                  })
                }
              >
                <option>Subtle</option>
                <option>Balanced</option>
                <option>Heightened</option>
                <option>Theatrical</option>
              </select>
            </label>
            <label>
              Public launch date
              <input
                type="date"
                value={launchOn}
                onChange={(e) => setLaunchOn(e.target.value)}
              />
            </label>
            <label className={s.fullField}>
              Entrance and match behaviour
              <input
                value={gimmick.entranceAndMatchBehavior}
                maxLength={240}
                onChange={(e) =>
                  setGimmick({
                    ...gimmick,
                    entranceAndMatchBehavior: e.target.value,
                  })
                }
              />
            </label>
            <label className={s.fullField}>
              Attire and mask
              <input
                value={gimmick.attireAndMask}
                maxLength={180}
                onChange={(e) =>
                  setGimmick({ ...gimmick, attireAndMask: e.target.value })
                }
              />
            </label>
            <label className={s.fullField}>
              Wrestling presentation tags
              <input
                value={tagText}
                placeholder="underdog, powerhouse, supernatural"
                onChange={(e) => setTagText(e.target.value)}
              />
              <span>Up to 12 short, comma-separated ideas.</span>
            </label>
            <label className={s.fullField}>
              Catchphrases and signature gestures
              <input
                value={gimmick.catchphrasesAndGestures}
                maxLength={240}
                onChange={(e) =>
                  setGimmick({
                    ...gimmick,
                    catchphrasesAndGestures: e.target.value,
                  })
                }
              />
            </label>
            <label className={s.fullField}>
              Existing traits to emphasize
              <input
                value={traitText}
                placeholder="intensity, dry humour, resilience"
                onChange={(e) => setTraitText(e.target.value)}
              />
              <span>
                Up to eight comma-separated traits; this cannot create new
                personality traits.
              </span>
            </label>
            <label className={s.checkField}>
              <input
                type="checkbox"
                checked={masked}
                onChange={(e) => {
                  setMasked(e.target.checked);
                  if (!e.target.checked) setConcealed(false);
                }}
              />{' '}
              Masked presentation
            </label>
            <label className={s.checkField}>
              <input
                type="checkbox"
                checked={concealed}
                disabled={!masked}
                onChange={(e) => setConcealed(e.target.checked)}
              />{' '}
              Conceal the person's identity
            </label>
          </div>
          <div className={s.formActions}>
            <button onClick={() => setEditing(false)}>Cancel</button>
            <button
              className={s.primaryAction}
              disabled={propose.isPending}
              onClick={() => propose.mutate()}
            >
              <Icon name="spark" /> Ask the worker
            </button>
          </div>
        </Panel>
      )}

      {character.pendingChange && (
        <Panel title="Character plan">
          <div className={s.characterPlanSummary}>
            <div>
              <span className={s.eyebrow}>Proposed direction</span>
              <h4>{character.pendingChange.proposedRingName}</h4>
            </div>
            <div className={s.chipRow}>
              <span className={s.statusChip}>
                {label(character.pendingChange.proposedAlignment)}
              </span>
              <span className={s.statusChip}>
                {character.pendingChange.proposedGimmick.name}
              </span>
              {character.pendingChange.proposedMasked && (
                <span className={s.statusChip}>Masked</span>
              )}
              {character.pendingChange.proposedConcealed && (
                <span className={s.statusChip}>Secret identity</span>
              )}
            </div>
          </div>
          <div className={s.planReview}>
            <div>
              <span className={s.eyebrow}>Worker response</span>
              <strong>{character.pendingChange.workerResponse}</strong>
            </div>
            <div>
              <span className={s.eyebrow}>Readiness</span>
              <strong>{character.pendingChange.readiness}</strong>
            </div>
            <div>
              <span className={s.eyebrow}>Change risk</span>
              <strong>{character.pendingChange.risk}</strong>
            </div>
          </div>
          <ul>
            {character.pendingChange.advice.map((item) => (
              <li key={item}>{item}</li>
            ))}
          </ul>
          <p className={s.help}>
            Nothing becomes public until you launch it. Launching preserves the
            old name and gimmick in dated history.
          </p>
          <button
            className={s.primaryAction}
            disabled={
              action.isPending ||
              !['accepted', 'ready'].includes(character.pendingChange.status)
            }
            onClick={() => action.mutate('launch')}
          >
            <Icon name="launch" /> Launch publicly
          </button>
        </Panel>
      )}

      <div className={s.characterCards}>
        <Panel title="Gimmick brief">
          <h4>{active.gimmick.name}</h4>
          <p>{active.gimmick.description}</p>
          <dl className={s.attributes}>
            <div>
              <dt>Core fantasy</dt>
              <dd>{active.gimmick.coreFantasy}</dd>
            </div>
            <div>
              <dt>Tone</dt>
              <dd>{active.gimmick.tone}</dd>
            </div>
            <div>
              <dt>Promo voice</dt>
              <dd>{active.gimmick.promoVoice}</dd>
            </div>
            <div>
              <dt>Presentation</dt>
              <dd>{active.gimmick.presentationIntensity}</dd>
            </div>
          </dl>
          <div className={s.chipRow}>
            {active.gimmick.tags.map((tag) => (
              <span className={s.tagChip} key={tag}>
                {tag}
              </span>
            ))}
          </div>
        </Panel>
        <Panel title="Identity record">
          <dl className={s.attributes}>
            <div>
              <dt>Company</dt>
              <dd>{active.companyId.toUpperCase()}</dd>
            </div>
            <div>
              <dt>Brand</dt>
              <dd>{active.brand ?? 'Company-wide'}</dd>
            </div>
            <div>
              <dt>Known as</dt>
              <dd>{active.aliases.join(', ') || active.ringName}</dd>
            </div>
            <div>
              <dt>Public knowledge</dt>
              <dd>{label(active.identityKnowledge)}</dd>
            </div>
            <div>
              <dt>Active since</dt>
              <dd>{active.startedOn}</dd>
            </div>
          </dl>
          <button
            onClick={() =>
              action.mutate(active.status === 'retired' ? 'revive' : 'retire')
            }
          >
            {active.status === 'retired'
              ? 'Revive this character'
              : 'Retire this character'}
          </button>
        </Panel>
      </div>
      <Panel title="Character history">
        {character.history.length ? (
          character.history.map((item) => (
            <article
              className={s.identityHistory}
              key={`${item.id}-${item.startedOn}`}
            >
              <Icon name="history" />
              <div>
                <strong>{item.ringName}</strong>
                <span>
                  {item.startedOn} – {item.endedOn ?? 'Present'} ·{' '}
                  {label(item.alignmentIntent)} · {item.gimmick.name}
                </span>
              </div>
            </article>
          ))
        ) : (
          <p className={s.empty}>
            This is their first recorded wrestling identity.
          </p>
        )}
      </Panel>
      <Panel title="Audience response evidence">
        {character.audienceResponses.length ? (
          character.audienceResponses.map((item) => (
            <article
              className={s.identityHistory}
              key={`${item.date}-${item.context}`}
            >
              <Icon name="people" />
              <div>
                <strong>
                  {label(item.response)} · {label(item.intensity)} ·{' '}
                  {label(item.perceivedRole)}
                </strong>
                <span>
                  {item.date} · {label(item.acceptance)} ·{' '}
                  {label(item.intentMatch)} · {item.context}
                </span>
              </div>
            </article>
          ))
        ) : (
          <p className={s.empty}>
            No recorded audience evidence yet. Future show reports will add
            contextual reactions here.
          </p>
        )}
      </Panel>
    </div>
  );
}
