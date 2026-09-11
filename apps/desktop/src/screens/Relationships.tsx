import { useState } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type {
  InteractionKind,
  InteractionOutcome,
  RelationshipProfile,
} from '@wm/contracts';
import { errorMessage, gameApi } from '../api';
import { ErrorNotice, Panel } from '../game-ui';
import s from '../Game.module.css';

const humanize = (value: string) =>
  value
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/^./, (letter) => letter.toUpperCase());

const TARGET_PAGE_SIZE = 12;

function requestId() {
  return globalThis.crypto?.randomUUID?.() ?? `interaction-${Date.now()}`;
}

export function RelationshipsPanel({
  saveId,
  workerId,
  relationships,
}: {
  saveId: string;
  workerId: string;
  relationships: RelationshipProfile;
}) {
  const client = useQueryClient();
  const [target, setTarget] = useState('');
  const [targetSearch, setTargetSearch] = useState('');
  const [targetOffset, setTargetOffset] = useState(0);
  const [latest, setLatest] = useState<InteractionOutcome | null>(null);
  const targetOption = relationships.interactionOptions.find(
    (option) => option.requiresTarget,
  );
  const targetPage = useQuery({
    queryKey: [
      'relationship-targets',
      saveId,
      workerId,
      targetSearch,
      targetOffset,
    ],
    queryFn: () =>
      gameApi.relationshipTargets(
        saveId,
        workerId,
        targetSearch,
        targetOffset,
        TARGET_PAGE_SIZE,
      ),
    enabled: Boolean(targetOption?.enabled),
  });
  const interaction = useMutation({
    mutationFn: ({
      kind,
      contextWorkerId,
    }: {
      kind: InteractionKind;
      contextWorkerId: string | null;
    }) =>
      gameApi.interact({
        saveId,
        requestId: requestId(),
        workerId,
        kind,
        contextWorkerId,
        expectedRevision: relationships.management.revision,
      }),
    onSuccess: async (outcome) => {
      setLatest(outcome);
      await Promise.all([
        client.invalidateQueries({ queryKey: ['profile', saveId, workerId] }),
        client.invalidateQueries({ queryKey: ['roster', saveId] }),
        client.invalidateQueries({ queryKey: ['news', saveId] }),
        client.invalidateQueries({ queryKey: ['office', saveId] }),
      ]);
    },
  });

  return (
    <div className={s.relationshipLayout}>
      <Panel title="Relationship with management">
        <div className={s.relationshipSummary}>
          <strong>{relationships.management.summary}</strong>
          <span>
            Attention today: {relationships.attentionRemaining}/
            {relationships.attentionLimit}
          </span>
        </div>
        <dl className={s.relationshipSignals}>
          {Object.entries(relationships.management.signals).map(
            ([name, value]) => (
              <div key={name}>
                <dt>{name === 'affinity' ? 'Rapport' : humanize(name)}</dt>
                <dd>{value}</dd>
              </div>
            ),
          )}
        </dl>
        <p className={s.help}>
          These are their feelings about the current management relationship,
          not their opinion of the company or their on-screen character.
        </p>
      </Panel>

      <Panel title="Speak with wrestler">
        {latest && (
          <article className={s.dialogue} aria-live="polite">
            <span>{humanize(latest.tone)} response</span>
            <blockquote>{latest.response}</blockquote>
            <p>{latest.effects.join(' ')}</p>
            <details>
              <summary>Why did they respond this way?</summary>
              <ul>
                {latest.factors.map((factor) => (
                  <li key={factor}>{factor}</li>
                ))}
              </ul>
            </details>
          </article>
        )}
        <div className={s.interactionList}>
          {relationships.interactionOptions.map((option) => {
            const chosenTarget = targetPage.data?.rows.find(
              (candidate) => candidate.workerId === target,
            );
            return (
              <article className={s.interactionCard} key={option.kind}>
                <div>
                  <strong>{option.label}</strong>
                  <p>{option.description}</p>
                  <small>
                    {option.attentionCost} attention
                    {option.cooldownUntil
                      ? ` · next natural date ${option.cooldownUntil}`
                      : option.kind === 'introduceYourself'
                        ? ' · once per working relationship'
                        : ''}
                  </small>
                </div>
                {option.requiresTarget && (
                  <div className={s.targetPicker}>
                    <label>
                      Find colleague
                      <input
                        placeholder="Search roster"
                        value={targetSearch}
                        onChange={(event) => {
                          setTargetSearch(event.target.value);
                          setTargetOffset(0);
                          setTarget('');
                        }}
                      />
                    </label>
                    <label>
                      Colleague
                      <select
                        value={target}
                        onChange={(event) => setTarget(event.target.value)}
                      >
                        <option value="">Choose…</option>
                        {targetPage.data?.rows.map((candidate) => (
                          <option
                            key={candidate.workerId}
                            value={candidate.workerId}
                          >
                            {candidate.name} · {candidate.relationship}
                          </option>
                        ))}
                      </select>
                    </label>
                    <span className={s.targetPager}>
                      <button
                        type="button"
                        disabled={targetOffset === 0 || targetPage.isFetching}
                        onClick={() => {
                          setTarget('');
                          setTargetOffset((value) =>
                            Math.max(0, value - TARGET_PAGE_SIZE),
                          );
                        }}
                      >
                        Previous
                      </button>
                      <small>
                        {targetPage.data?.total === 0
                          ? 'No colleagues found'
                          : targetPage.data
                            ? `${targetOffset + 1}–${targetOffset + targetPage.data.rows.length} of ${targetPage.data.total}`
                            : 'Loading colleagues…'}
                      </small>
                      <button
                        type="button"
                        disabled={
                          targetPage.isFetching ||
                          !targetPage.data ||
                          targetOffset + targetPage.data.rows.length >=
                            targetPage.data.total
                        }
                        onClick={() => {
                          setTarget('');
                          setTargetOffset((value) => value + TARGET_PAGE_SIZE);
                        }}
                      >
                        Next
                      </button>
                    </span>
                  </div>
                )}
                <button
                  disabled={
                    interaction.isPending ||
                    !option.enabled ||
                    (option.requiresTarget && !chosenTarget)
                  }
                  title={option.unavailableReason ?? undefined}
                  onClick={() =>
                    interaction.mutate({
                      kind: option.kind,
                      contextWorkerId: option.requiresTarget ? target : null,
                    })
                  }
                >
                  {interaction.isPending &&
                  interaction.variables?.kind === option.kind
                    ? 'Speaking…'
                    : option.label}
                </button>
                {!option.enabled && option.unavailableReason && (
                  <small className={s.unavailable}>
                    {option.unavailableReason}
                  </small>
                )}
              </article>
            );
          })}
        </div>
        {interaction.isError && (
          <ErrorNotice message={errorMessage(interaction.error)} />
        )}
        {targetPage.isError && (
          <ErrorNotice message={errorMessage(targetPage.error)} />
        )}
      </Panel>

      <section className={s.relationshipSection}>
        <h3>How they see the locker room</h3>
        <p className={s.help}>
          Relationships are directional: the other person may feel differently.
        </p>
        <div className={s.relationshipCards}>
          {relationships.personal.map((relationship) => (
            <article className={s.relationshipCard} key={relationship.otherId}>
              <header>
                <strong>{relationship.otherName}</strong>
                <span>{relationship.summary}</span>
              </header>
              <dl className={s.relationshipSignals}>
                {Object.entries(relationship.signals).map(([name, value]) => (
                  <div key={name}>
                    <dt>{humanize(name)}</dt>
                    <dd>{value}</dd>
                  </div>
                ))}
              </dl>
              {relationship.memories.map((memory) => (
                <p className={s.relationshipMemory} key={memory.id}>
                  <time>{memory.occurredOn}</time> · {memory.summary}
                  {!memory.active && ' · no longer an active influence'}
                </p>
              ))}
            </article>
          ))}
        </div>
      </section>

      <section className={s.relationshipSection}>
        <h3>Conversation history</h3>
        {relationships.interactionHistory.length ? (
          relationships.interactionHistory.map((entry) => (
            <article className={s.post} key={entry.requestId}>
              <b>{entry.label}</b>
              <small>
                {entry.occurredOn} · {humanize(entry.tone)}
              </small>
              <p>{entry.response}</p>
              <p>{entry.effects.join(' ')}</p>
            </article>
          ))
        ) : (
          <p className={s.empty}>No conversations recorded yet.</p>
        )}
      </section>
    </div>
  );
}
