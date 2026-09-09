import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import type { CareerOffice, SegmentReport } from '@wm/contracts';
import { gameApi, errorMessage } from '../api';
import { useNavigation } from '../navigation';
import { Panel, ErrorNotice, currency, gameDate } from '../game-ui';
import s from '../Game.module.css';
export function Reports({
  saveId,
  office,
  initialShowId,
}: {
  saveId: string;
  office: CareerOffice;
  initialShowId?: number | null;
}) {
  const [selected, setSelected] = useState<number | null>(
    initialShowId ?? null,
  );
  const showId = selected ?? office.recentShows[0]?.id;
  const query = useQuery({
    queryKey: ['report', saveId, showId],
    queryFn: () => gameApi.report(saveId, showId!),
    enabled: showId !== undefined,
  });
  const openLive = useNavigation((v) => v.openLive);
  const [segment, setSegment] = useState<number | null>(null);
  const report = query.data;
  const selectedSegment =
    report?.segments.find((s) => s.segmentId === segment) ??
    report?.segments[0];
  return (
    <div className={s.screenColumn}>
      <div className={s.toolbar}>
        <label>
          Completed show
          <select
            value={showId ?? ''}
            onChange={(e) => {
              setSelected(Number(e.target.value));
              setSegment(null);
            }}
          >
            {showId !== undefined &&
              !office.recentShows.some((show) => show.id === showId) && (
                <option value={showId}>
                  {report?.name ?? `Show ${showId}`}
                </option>
              )}
            {office.recentShows.map((show) => (
              <option key={show.id} value={show.id}>
                {gameDate(show.date)} · {show.name}
              </option>
            ))}
          </select>
        </label>
        <div className={s.spacer} />
        {showId !== undefined && (
          <button onClick={() => openLive(showId)}>Review event feed</button>
        )}
      </div>
      {query.isError ? (
        <ErrorNotice message={errorMessage(query.error)} />
      ) : !report ? (
        <div className={s.empty}>
          <h2>
            {query.isPending && showId !== undefined
              ? 'Preparing the report…'
              : 'The record starts with your first show.'}
          </h2>
          <p>
            Results, worker development and media reaction will appear here
            after the final segment.
          </p>
        </div>
      ) : (
        <div className={s.reportGrid}>
          <div className={s.screenColumn}>
            <div className={s.businessStrip}>
              <div>
                <span>Attendance</span>
                <b>{report.attendance}</b>
              </div>
              <div>
                <span>Gate</span>
                <b>{currency(report.revenuePence)}</b>
              </div>
              <div>
                <span>Costs</span>
                <b>{currency(report.costsPence)}</b>
              </div>
              <div>
                <span>Net</span>
                <b>{currency(report.revenuePence - report.costsPence)}</b>
              </div>
            </div>
            <Panel title="Performance record" className={s.reportTable}>
              <div className={s.scroll}>
                <table>
                  <thead>
                    <tr>
                      <th>Segment</th>
                      <th title="Execution of moves and transitions">
                        Execution
                      </th>
                      <th>Psychology</th>
                      <th>Engagement</th>
                      <th>Safety</th>
                    </tr>
                  </thead>
                  <tbody>
                    {report.segments.map((item) => (
                      <tr
                        key={item.segmentId}
                        className={
                          selectedSegment?.segmentId === item.segmentId
                            ? s.selected
                            : ''
                        }
                      >
                        <th>
                          <button
                            className={s.nameButton}
                            onClick={() => setSegment(item.segmentId)}
                          >
                            {item.title}
                          </button>
                          <small>{item.result}</small>
                        </th>
                        <td>{item.performance.execution}</td>
                        <td>{item.performance.psychology}</td>
                        <td>{item.performance.engagement}</td>
                        <td>{item.performance.safety}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </Panel>
            {selectedSegment && <SegmentDetail report={selectedSegment} />}
          </div>
          <Panel title="Media reaction" className={s.mediaPanel}>
            <div className={s.scroll}>
              {office.media
                .filter((p) => p.date === report.date)
                .map((post) => (
                  <article className={s.post} key={post.id}>
                    <b>{post.author}</b>
                    <p>{post.text}</p>
                  </article>
                ))}
            </div>
          </Panel>
        </div>
      )}
    </div>
  );
}
function SegmentDetail({ report }: { report: SegmentReport }) {
  return (
    <Panel title="Why it happened · what it changed" className={s.fill}>
      <div className={s.scroll}>
        <div className={s.reportReasons}>
          {report.reasons.map((reason, i) => (
            <p key={i}>{reason}</p>
          ))}
          <small>
            Story advancement {report.performance.story} · Protection{' '}
            {report.performance.protection} · Chemistry{' '}
            {report.chemistryChange > 0 ? '+' : ''}
            {report.chemistryChange}
          </small>
        </div>
        <table>
          <thead>
            <tr>
              <th>Performer</th>
              <th>Confidence</th>
              <th>Momentum</th>
              <th>Fatigue</th>
              <th>Development</th>
              <th>Wear</th>
            </tr>
          </thead>
          <tbody>
            {report.changes.map((change) => (
              <tr key={change.workerId}>
                <th>
                  {change.name}
                  <small>{change.note}</small>
                </th>
                <td>
                  {change.confidence > 0 ? '+' : ''}
                  {change.confidence}
                </td>
                <td>
                  {change.momentum > 0 ? '+' : ''}
                  {change.momentum}
                </td>
                <td>+{change.fatigue}</td>
                <td>+{change.development}</td>
                <td>+{change.wear}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </Panel>
  );
}
