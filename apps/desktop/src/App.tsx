import { useEffect, useState } from 'react';
import {
  useIsMutating,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query';
import { gameApi, desktopAvailable, errorMessage } from './api';
import { useNavigation, type GameScreen } from './navigation';
import { CareerLibrary } from './screens/CareerLibrary';
import { Office } from './screens/Office';
import { Roster, ProfilePanel } from './screens/Roster';
import { Booking } from './screens/Booking';
import { LiveShow } from './screens/LiveShow';
import { Reports } from './screens/Reports';
import { News } from './screens/News';
import { ErrorNotice, Overlay, gameDate, currency } from './game-ui';
import s from './Game.module.css';

const navigation: {
  screen: GameScreen;
  label: string;
  key: string;
  group: string;
}[] = [
  { screen: 'overview', label: 'Overview', key: 'F1', group: 'HOME' },
  { screen: 'news', label: 'News & inbox', key: 'F6', group: 'HOME' },
  { screen: 'booking', label: 'Shows', key: 'F3', group: 'BOOKING' },
  { screen: 'live', label: 'Live show', key: 'F4', group: 'BOOKING' },
  { screen: 'reports', label: 'Reports & media', key: 'F5', group: 'BOOKING' },
  { screen: 'talent', label: 'Talent search', key: 'F2', group: 'TALENT' },
];

export function App() {
  const {
    screen,
    activeSaveId,
    navigate,
    showLibrary,
    liveShowId,
    profileId,
    inspectWorker,
    reportShowId,
    plannerDirty,
    openWorkerFinder,
  } = useNavigation();
  const [menu, setMenu] = useState(false);
  const [confirmExit, setConfirmExit] = useState(false);
  const pendingWrites = useIsMutating();
  const [displayError, setDisplayError] = useState('');
  const available = desktopAvailable();
  const cache = useQueryClient();
  const office = useQuery({
    queryKey: ['office', activeSaveId],
    queryFn: () => gameApi.office(activeSaveId!),
    enabled: available && activeSaveId !== null && screen !== 'saves',
  });
  const news = useQuery({
    queryKey: ['news', activeSaveId, 'badge'],
    queryFn: () => gameApi.news(activeSaveId!, '', false, 0, 1),
    enabled: available && activeSaveId !== null && screen !== 'saves',
  });
  const quit = useMutation({
    mutationFn: gameApi.exit,
    onError: (error) => setDisplayError(errorMessage(error)),
  });
  const day = useMutation({
    mutationFn: () => gameApi.continueDay(activeSaveId!),
    onSuccess: (result) => {
      cache.setQueryData(['office', activeSaveId], result);
      void cache.invalidateQueries({ queryKey: ['news', activeSaveId] });
      void cache.invalidateQueries({ queryKey: ['roster', activeSaveId] });
      void cache.invalidateQueries({
        queryKey: ['worker-search', activeSaveId],
      });
      void cache.invalidateQueries({
        queryKey: ['worker-filter-options', activeSaveId],
      });
      void cache.invalidateQueries({ queryKey: ['profile', activeSaveId] });
      void cache.invalidateQueries({
        queryKey: ['booking-roster', activeSaveId],
      });
    },
  });
  const fullscreen = () => {
    void gameApi.fullscreen().catch((e) => setDisplayError(errorMessage(e)));
  };
  useEffect(() => {
    const shortcut = (event: KeyboardEvent) => {
      if (
        (event.ctrlKey || event.metaKey) &&
        event.key.toLowerCase() === 'k' &&
        activeSaveId
      ) {
        event.preventDefault();
        openWorkerFinder();
        return;
      }
      if (event.key === 'F11') {
        event.preventDefault();
        void gameApi
          .fullscreen()
          .catch((e) => setDisplayError(errorMessage(e)));
        return;
      }
      if (
        event.key === 'Escape' &&
        !profileId &&
        !document.querySelector('dialog[open]')
      ) {
        event.preventDefault();
        setMenu((value) => !value);
        return;
      }
      if (
        event.target instanceof HTMLInputElement ||
        event.target instanceof HTMLSelectElement ||
        event.target instanceof HTMLTextAreaElement
      )
        return;
      const item = navigation.find((item) => item.key === event.key);
      if (item && activeSaveId) {
        event.preventDefault();
        navigate(item.screen);
      }
    };
    window.addEventListener('keydown', shortcut);
    return () => window.removeEventListener('keydown', shortcut);
  }, [activeSaveId, navigate, openWorkerFinder, profileId]);
  const career = office.data;
  return (
    <div
      className={`${s.gameShell} ${activeSaveId && screen !== 'saves' ? s.careerShell : ''}`}
    >
      <header className={s.topbar}>
        <button
          className={s.brand}
          onClick={() => setMenu(true)}
          title="Game menu · Escape"
        >
          <span>
            WM<span className={s.logoDot}>.</span>
          </span>
          <small>
            WRESTLING
            <br />
            MANAGER
          </small>
        </button>
        {activeSaveId && screen !== 'saves' ? (
          <>
            <div className={s.promotionIdentity}>
              <b>{career?.promotion.name ?? 'Opening career…'}</b>
              <span>{career?.promotion.region}</span>
            </div>
            <div className={s.spacer} />
            <button
              className={s.inboxButton}
              onClick={() => navigate('news')}
              aria-label={`News and inbox, ${news.data?.unread ?? 0} unread`}
            >
              Inbox <b>{news.data?.unread ?? 0}</b>
            </button>
            <button
              onClick={openWorkerFinder}
              title="Open talent search · Ctrl+K"
            >
              Talent search <kbd>Ctrl K</kbd>
            </button>
            {career && (
              <>
                <div className={s.headerBalance}>
                  <small>FUNDS</small>
                  <b>{currency(career.promotion.cashPence)}</b>
                </div>
                <time className={s.gameDate}>
                  {gameDate(career.promotion.currentDate)}
                </time>
                <button
                  className={s.continueButton}
                  disabled={day.isPending || career.show.status === 'live'}
                  onClick={() => day.mutate()}
                >
                  {day.isPending ? 'Advancing…' : 'Continue'} <span>›</span>
                </button>
              </>
            )}
          </>
        ) : (
          <>
            <div className={s.spacer} />
            <span className={s.muted}>THE BOOK IS YOURS</span>
          </>
        )}
        <button
          className={s.menuButton}
          onClick={() => setMenu(true)}
          aria-label="Game menu"
        >
          ☰
        </button>
      </header>
      {activeSaveId && screen !== 'saves' && (
        <nav className={s.mainNav} aria-label="Management screens">
          {['HOME', 'BOOKING', 'TALENT'].map((group) => (
            <div className={s.navGroup} key={group}>
              <span>{group}</span>
              {navigation
                .filter((item) => item.group === group)
                .map((item) => (
                  <button
                    key={item.screen}
                    aria-current={screen === item.screen ? 'page' : undefined}
                    onClick={() => navigate(item.screen)}
                  >
                    {item.label}
                    <kbd>{item.key}</kbd>
                  </button>
                ))}
            </div>
          ))}
        </nav>
      )}
      <main className={s.gameMain}>
        {!available ? (
          <div className={s.desktopNotice}>
            <h1>Open Wrestling Manager on your desktop</h1>
            <p>Local careers and live shows run in the desktop application.</p>
          </div>
        ) : screen === 'saves' ? (
          <CareerLibrary />
        ) : office.isError ? (
          <div className={s.empty}>
            <ErrorNotice message={errorMessage(office.error)} />
            <button onClick={() => void office.refetch()}>Reload career</button>
          </div>
        ) : !career || !activeSaveId ? (
          <p className={s.empty} role="status">
            Opening the promotion office…
          </p>
        ) : (
          <>
            {day.isError && (
              <div className={s.bannerError}>
                <ErrorNotice message={errorMessage(day.error)} />
                <button
                  onClick={() => day.reset()}
                  aria-label="Dismiss message"
                >
                  ×
                </button>
              </div>
            )}
            {screen === 'overview' ? (
              <Office office={career} />
            ) : screen === 'news' ? (
              <News key={activeSaveId} saveId={activeSaveId} />
            ) : screen === 'talent' ? (
              <Roster saveId={activeSaveId} />
            ) : screen === 'booking' ? (
              <Booking saveId={activeSaveId} office={career} />
            ) : screen === 'live' ? (
              <LiveShow
                key={liveShowId ?? career.show.id}
                saveId={activeSaveId}
                showId={
                  liveShowId ??
                  (career.show.status === 'live' ? career.show.id : null)
                }
              />
            ) : (
              <Reports
                key={reportShowId ?? 'latest'}
                saveId={activeSaveId}
                office={career}
                initialShowId={reportShowId}
              />
            )}
          </>
        )}
      </main>
      <footer className={s.statusBar}>
        <span>
          <i />
          LOCAL CAREER · AUTOMATIC SAVING
        </span>
        <span>
          Ctrl+K talent search <b>·</b> F1–F6 screens <b>·</b> F11 display{' '}
          <b>·</b> Esc menu
        </span>
      </footer>
      {profileId && activeSaveId && (
        <ProfilePanel
          key={profileId}
          saveId={activeSaveId}
          workerId={profileId}
          onClose={() => inspectWorker(null)}
        />
      )}
      {menu && (
        <Overlay
          title="Wrestling Manager"
          onClose={() => {
            setMenu(false);
            setConfirmExit(false);
          }}
        >
          <div className={s.gameMenu}>
            <button className={s.primary} onClick={() => setMenu(false)}>
              Return to game
            </button>
            <button
              onClick={() => {
                showLibrary();
                setMenu(false);
              }}
            >
              Save library
            </button>
            <button onClick={fullscreen}>Toggle fullscreen / windowed</button>
            {confirmExit && plannerDirty ? (
              <div className={s.exitPrompt}>
                <p>Your match instructions have unsaved changes.</p>
                <button
                  className={s.primary}
                  onClick={() => {
                    setMenu(false);
                    setConfirmExit(false);
                  }}
                >
                  Return to my booking
                </button>
                <button
                  disabled={pendingWrites > 0}
                  onClick={() => quit.mutate()}
                >
                  Discard edits and exit
                </button>
              </div>
            ) : (
              <button
                className={s.exitButton}
                disabled={pendingWrites > 0 || !available}
                onClick={() =>
                  plannerDirty ? setConfirmExit(true) : quit.mutate()
                }
              >
                {quit.isPending ? 'Closing…' : 'Exit game'}
              </button>
            )}
            {pendingWrites > 0 && !quit.isPending && (
              <small>Finishing the current operation before exit…</small>
            )}
            <p>
              Saved plans and simulation progress are stored automatically.
              Match-planner edits are stored when you choose Save instructions.
            </p>
            <small>
              Full-match viewing advances the simulation one second at a time.
              Space pauses playback; use the agent channel for live changes.
            </small>
            {displayError && <ErrorNotice message={displayError} />}
          </div>
        </Overlay>
      )}
    </div>
  );
}
