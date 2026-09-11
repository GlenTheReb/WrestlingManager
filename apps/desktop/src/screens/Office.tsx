import type { CareerOffice } from '@wm/contracts';
import { useNavigation } from '../navigation';
import { Panel, currency, clockTime, gameDate } from '../game-ui';
import s from '../Game.module.css';
export function Office({ office }: { office: CareerOffice }) {
  const nav = useNavigation((v) => v.navigate);
  const openLive = useNavigation((v) => v.openLive);
  const total = office.show.segments.reduce(
    (sum, x) => sum + x.content.plan.durationSeconds,
    0,
  );
  return (
    <div className={s.officeGrid}>
      <Panel title="Decision centre" className={s.decisions}>
        <div className={s.decision}>
          <span className={s.stamp}>CREATIVE</span>
          <h3>{office.show.name}</h3>
          <p>
            {office.show.date === office.promotion.currentDate
              ? 'You are on the day of the show. The card needs your final decisions.'
              : `${gameDate(office.show.date)} · ${office.show.segments.length} segments booked.`}
          </p>
          <button
            className={s.primary}
            onClick={() =>
              office.show.status === 'live'
                ? openLive(office.show.id)
                : nav('booking')
            }
          >
            {office.show.status === 'live'
              ? 'Resume live show'
              : 'Open running order'}
          </button>
        </div>
        <div className={s.decision}>
          <span className={s.stamp}>TALENT RELATIONS</span>
          <h3>{office.rosterCount} wrestlers under your direction</h3>
          <p>
            Study their condition, strengths and actual repertoires before
            making promises in the ring.
          </p>
          <button onClick={() => nav('talent')}>Open talent search</button>
        </div>
        <div className={s.brief}>
          <h3>Your road agents</h3>
          {office.agents.map((agent) => (
            <div key={agent.id}>
              <b>{agent.name}</b>
              <span>
                Psychology {agent.psychology}/20 · Communication{' '}
                {agent.communication}/20
              </span>
              <p>{agent.philosophy}</p>
            </div>
          ))}
        </div>
      </Panel>
      <div className={s.officeRight}>
        <Panel title="Promotion position">
          <dl className={s.facts}>
            <div>
              <dt>Available cash</dt>
              <dd className={s.money}>
                {currency(office.promotion.cashPence)}
              </dd>
            </div>
            <div>
              <dt>Upcoming show</dt>
              <dd>{gameDate(office.show.date)}</dd>
            </div>
            <div>
              <dt>Booked time</dt>
              <dd>{clockTime(total)} / 120:00</dd>
            </div>
            <div>
              <dt>Home market</dt>
              <dd>{office.promotion.region}</dd>
            </div>
            <div>
              <dt>Career</dt>
              <dd>{office.promotion.saveId}</dd>
            </div>
            <div>
              <dt>World seed</dt>
              <dd>{office.promotion.seed}</dd>
            </div>
          </dl>
        </Panel>
        <Panel title="From the wrestling world" className={s.fill}>
          <div className={s.scroll}>
            {office.media.length ? (
              office.media.slice(0, 10).map((post) => (
                <article className={s.post} key={post.id}>
                  <b>{post.author}</b>
                  <small>{gameDate(post.date)}</small>
                  <p>{post.text}</p>
                </article>
              ))
            ) : (
              <div className={s.empty}>
                <h3>The world is waiting for opening night.</h3>
                <p>Coverage will follow what actually happens in your shows.</p>
              </div>
            )}
          </div>
        </Panel>
      </div>
    </div>
  );
}
