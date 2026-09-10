import type {
  PersonIdentity,
  PersonalityDescription,
  TraitOverview,
} from '@wm/contracts';
import { Panel } from '../game-ui';
import s from '../Game.module.css';

const label = (value: string) =>
  value
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/^./, (c) => c.toUpperCase());
const band = (value: number | null) =>
  value === null
    ? 'Unknown'
    : value < 20
      ? 'Very low'
      : value < 40
        ? 'Low'
        : value <= 60
          ? 'Moderate'
          : value <= 80
            ? 'High'
            : 'Very high';

export function PersonIdentityPanel({
  identity,
  description,
  biography,
  traits,
}: {
  identity: PersonIdentity;
  description: PersonalityDescription;
  biography: string;
  traits: TraitOverview;
}) {
  return (
    <div className={s.profileGrid}>
      <Panel title="Personality">
        <p>
          <strong>{description.text}</strong>
          {description.partial && ' · Partial assessment'}
        </p>
        <p className={s.help}>
          These tendencies describe behaviour. Higher is not always better.
        </p>
        <dl className={s.attributes}>
          {Object.entries(identity.personality).map(([key, assessment]) => (
            <div key={key}>
              <dt>{label(key)}</dt>
              <dd
                title={
                  assessment.value === null
                    ? 'Not yet known'
                    : `${assessment.value}/100 · ${assessment.source}`
                }
              >
                {band(assessment.value)}
              </dd>
            </div>
          ))}
        </dl>
        {description.reasons.length > 0 && (
          <details>
            <summary>Why this description?</summary>
            <ul>
              {description.reasons.map((reason) => (
                <li key={reason.field}>
                  {reason.label}: {label(reason.field)} · {reason.source}
                </li>
              ))}
            </ul>
          </details>
        )}
      </Panel>
      <Panel title="Shared qualities">
        <p className={s.help}>
          Personal skills, reliability and effort. Separate from wrestling
          ratings.
        </p>
        <dl className={s.attributes}>
          {Object.entries(identity.qualities).map(([key, assessment]) => (
            <div key={key}>
              <dt>{label(key)}</dt>
              <dd
                title={
                  assessment.value === null
                    ? 'Not yet known'
                    : `${assessment.value}/100 · ${assessment.source}`
                }
              >
                {band(assessment.value)}
              </dd>
            </div>
          ))}
        </dl>
      </Panel>
      <Panel title="Career motivations">
        {identity.motivations ? (
          <dl className={s.attributes}>
            <div>
              <dt>Primary</dt>
              <dd>{label(identity.motivations.primary)}</dd>
            </div>
            <div>
              <dt>Secondary</dt>
              <dd>
                {identity.motivations.secondary.map(label).join(', ') ||
                  'None recorded'}
              </dd>
            </div>
          </dl>
        ) : (
          <p>Motivations not yet known.</p>
        )}
      </Panel>
      <Panel title="Languages and interests">
        <h4>Languages</h4>
        {identity.languages.length ? (
          <ul>
            {identity.languages.map((language) => (
              <li key={language.name}>
                {language.name} ·{' '}
                {language.proficiency
                  ? label(language.proficiency)
                  : 'Proficiency unknown'}
                {language.native && ' · Native'}
              </li>
            ))}
          </ul>
        ) : (
          <p>No languages recorded.</p>
        )}
        <h4>Interests</h4>
        {identity.hobbies.length ? (
          <ul>
            {identity.hobbies.map((interest) => (
              <li key={interest.hobby}>
                {label(interest.hobby)} · {label(interest.involvement)}
              </li>
            ))}
          </ul>
        ) : (
          <p>No interests recorded.</p>
        )}
      </Panel>
      <Panel title="Biography">
        <p>{biography || 'No biographical facts recorded.'}</p>
        <small>{label(identity.biography.mode)} biography</small>
      </Panel>
      <Panel title="Exceptional traits">
        {traits.states.length ? (
          <ul>
            {traits.states.map((trait) => (
              <li
                key={`${trait.traitId}-${trait.companyId}-${trait.visibility.kind}`}
              >
                <strong>{label(trait.traitId)}</strong> · {label(trait.status)}
                {trait.companyId && ` · ${trait.companyId}`}
                {trait.visibility.kind === 'company' &&
                  ' · Confidential company information'}
                <small> · {trait.evidenceIds.length} evidence item(s)</small>
              </li>
            ))}
          </ul>
        ) : (
          <p>
            {traits.hasEvidence
              ? 'No active exceptional traits from the recorded evidence.'
              : 'No exceptional-trait evidence recorded yet.'}
          </p>
        )}
        <p className={s.help}>
          Traits require recorded events. Contract, media and sponsor effects
          will arrive with those management systems.
        </p>
        {traits.history.length > 0 && (
          <details>
            <summary>Trait history (latest 30)</summary>
            {traits.history.map((entry, index) => (
              <article className={s.post} key={`${entry.date}-${index}`}>
                <strong>
                  {entry.date} · {label(entry.state.traitId)}
                </strong>
                <p>
                  {label(entry.from)} → {label(entry.state.status)}
                </p>
                <small>{entry.reason}</small>
              </article>
            ))}
          </details>
        )}
      </Panel>
    </div>
  );
}
