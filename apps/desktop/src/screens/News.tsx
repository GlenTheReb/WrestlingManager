import { useState } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type { NewsItem } from '@wm/contracts';
import { gameApi, errorMessage } from '../api';
import { ErrorNotice, gameDate } from '../game-ui';
import { useNavigation } from '../navigation';
import s from '../Game.module.css';

export function News({ saveId }: { saveId: string }) {
  const [category, setCategory] = useState('');
  const [unreadOnly, setUnreadOnly] = useState(false);
  const [offset, setOffset] = useState(0);
  const [selected, setSelected] = useState<NewsItem | null>(null);
  const cache = useQueryClient();
  const { navigate, inspectWorker, openReport } = useNavigation();
  const page = useQuery({
    queryKey: ['news', saveId, category, unreadOnly, offset],
    queryFn: () => gameApi.news(saveId, category, unreadOnly, offset),
  });
  const mark = useMutation({
    mutationFn: ({ id, read }: { id: number; read: boolean }) =>
      gameApi.setNewsRead(saveId, id, read),
    onSuccess: (_, changed) => {
      setSelected((item) =>
        item?.id === changed.id ? { ...item, read: changed.read } : item,
      );
      void cache.invalidateQueries({ queryKey: ['news', saveId] });
    },
  });
  const open = (item: NewsItem) => {
    setSelected(item);
    if (!item.read) mark.mutate({ id: item.id, read: true });
  };
  return (
    <div className={s.screenColumn}>
      <div className={s.newsMasthead}>
        <div>
          <span className={s.eyebrow}>THE WRESTLING OFFICE</span>
          <h1>
            News & inbox<span className={s.logoDot}>.</span>
          </h1>
        </div>
        <div>
          <b>{page.data?.unread ?? 0}</b>
          <span>unread dispatches</span>
        </div>
      </div>
      <div className={s.newsLayout}>
        <section className={s.newsList} aria-label="News items">
          <div className={s.newsFilters}>
            <select
              aria-label="News category"
              value={category}
              onChange={(event) => {
                setCategory(event.target.value);
                setOffset(0);
              }}
            >
              <option value="">All departments</option>
              {['Office', 'People', 'Results', 'Medical', 'Business'].map(
                (value) => (
                  <option key={value}>{value}</option>
                ),
              )}
            </select>
            <label>
              <input
                type="checkbox"
                checked={unreadOnly}
                onChange={(event) => {
                  setUnreadOnly(event.target.checked);
                  setOffset(0);
                }}
              />
              Unread only
            </label>
          </div>
          <div className={s.scroll}>
            {page.isError && <ErrorNotice message={errorMessage(page.error)} />}
            {page.isPending && (
              <p className={s.empty} role="status">
                Opening the news desk…
              </p>
            )}
            {page.data?.items.length === 0 && (
              <p className={s.empty}>
                Nothing in this view. New dispatches follow events in your
                career.
              </p>
            )}
            {page.data?.items.map((item) => (
              <button
                className={`${s.newsItem} ${!item.read ? s.newsUnread : ''}`}
                key={item.id}
                aria-pressed={selected?.id === item.id}
                onClick={() => open(item)}
              >
                <span>
                  <i />
                  {item.category}
                  <time>{gameDate(item.date)}</time>
                </span>
                <strong>{item.title}</strong>
                <small>{item.body.slice(0, 112)}…</small>
              </button>
            ))}
          </div>
          <div className={s.newsPagination}>
            <button
              disabled={offset === 0}
              onClick={() => setOffset((value) => Math.max(0, value - 30))}
            >
              Previous
            </button>
            <span>{page.data?.total ?? 0} articles</span>
            <button
              disabled={offset + 30 >= (page.data?.total ?? 0)}
              onClick={() => setOffset((value) => value + 30)}
            >
              Next
            </button>
          </div>
        </section>
        <article className={s.newsArticle}>
          {selected ? (
            <>
              <header>
                <span className={s.eyebrow}>{selected.category} DISPATCH</span>
                <time>{gameDate(selected.date)}</time>
                <h2>{selected.title}</h2>
              </header>
              <div className={s.articleBody}>
                {selected.body.split('\n\n').map((paragraph, index) => (
                  <p key={index}>{paragraph}</p>
                ))}
              </div>
              <footer>
                {selected.workerId && (
                  <button onClick={() => inspectWorker(selected.workerId)}>
                    Open wrestler profile
                  </button>
                )}
                {selected.showId !== null && (
                  <button
                    className={s.primary}
                    onClick={() =>
                      selected.category === 'Office'
                        ? navigate('booking')
                        : openReport(selected.showId!)
                    }
                  >
                    {selected.category === 'Office'
                      ? 'Open running order'
                      : 'Open show report'}
                  </button>
                )}
                <button
                  disabled={mark.isPending}
                  onClick={() =>
                    mark.mutate({ id: selected.id, read: !selected.read })
                  }
                >
                  {selected.read ? 'Mark unread' : 'Mark read'}
                </button>
              </footer>
              {mark.isError && (
                <ErrorNotice message={errorMessage(mark.error)} />
              )}
            </>
          ) : (
            <div className={s.newsWelcome}>
              <span className={s.bigLogo}>WM.</span>
              <h2>The story of your career.</h2>
              <p>Select a dispatch to read the details and act on it.</p>
              <small>
                Confirmed results, company business and medical updates.
              </small>
            </div>
          )}
        </article>
      </div>
    </div>
  );
}
