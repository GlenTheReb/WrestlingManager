import { useState, type FormEvent } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type { PromotionOverview } from '@wm/contracts';
import { gameApi, errorMessage } from '../api';
import { useNavigation } from '../navigation';
import { Panel, ErrorNotice, gameDate } from '../game-ui';
import s from '../Game.module.css';

export function CareerLibrary() {
  const [creating, setCreating] = useState(false),
    [saveId, setSaveId] = useState('uwf-career'),
    [seed, setSeed] = useState('2026');
  const cache = useQueryClient();
  const openSave = useNavigation((v) => v.openSave);
  const saves = useQuery({ queryKey: ['saves'], queryFn: gameApi.listSaves });
  const accept = (overview: PromotionOverview) => {
    cache.setQueryData(['promotion', overview.saveId], overview);
    void cache.invalidateQueries({ queryKey: ['saves'] });
    openSave(overview.saveId);
  };
  const create = useMutation({
    mutationFn: gameApi.createGame,
    onSuccess: accept,
  });
  const load = useMutation({ mutationFn: gameApi.loadGame, onSuccess: accept });
  const busy = create.isPending || load.isPending;
  function submit(e: FormEvent) {
    e.preventDefault();
    load.reset();
    create.mutate({ saveId, seed });
  }
  return (
    <div className={s.library}>
      <div className={s.libraryIdentity}>
        <span className={s.eyebrow}>WRESTLING MANAGER</span>
        <div className={s.bigLogo}>
          WM<span className={s.logoDot}>.</span>
        </div>
        <h1>Take the book.</h1>
        <p>
          Plan the contest. Trust the people.
          <br />
          Live with the consequences.
        </p>
        <div className={s.ruleLine} />
        <span>YOUR WORLD. YOUR ROSTER. YOUR CALL.</span>
        <small>WRESTLING MANAGER · OFFLINE CAREERS</small>
      </div>
      <div className={s.libraryContent}>
        <div className={s.screenTitle}>
          <h1>Save library</h1>
          <button
            className={s.primary}
            disabled={busy}
            onClick={() => {
              setCreating(true);
              create.reset();
              load.reset();
            }}
          >
            + New career
          </button>
        </div>
        {creating && (
          <Panel title="Establish a promotion">
            <form className={s.form} onSubmit={submit}>
              <label>
                Save name
                <input
                  autoFocus
                  aria-label="Save name"
                  required
                  maxLength={48}
                  pattern="[a-z0-9][a-z0-9\-]{0,47}"
                  value={saveId}
                  onChange={(e) => setSaveId(e.target.value)}
                  disabled={busy}
                />
                <small>Lower-case letters, numbers and hyphens.</small>
              </label>
              <label>
                World seed
                <input
                  required
                  inputMode="numeric"
                  aria-label="World seed"
                  pattern="0|[1-9][0-9]{0,19}"
                  value={seed}
                  onChange={(e) => setSeed(e.target.value)}
                  disabled={busy}
                />
                <small>The same seed creates the same founding roster.</small>
              </label>
              <div className={s.actions}>
                <button type="submit" className={s.primary} disabled={busy}>
                  {create.isPending ? 'Creating career…' : 'Create career'}
                </button>
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => {
                    setCreating(false);
                    create.reset();
                  }}
                >
                  Cancel
                </button>
              </div>
            </form>
          </Panel>
        )}
        {(create.isError || load.isError) && (
          <ErrorNotice message={errorMessage(create.error ?? load.error)} />
        )}
        <Panel title="Saved careers">
          <div className={s.scroll}>
            {saves.isPending ? (
              <p className={s.empty} role="status">
                Finding saved careers…
              </p>
            ) : saves.isError ? (
              <div className={s.empty}>
                <ErrorNotice message={errorMessage(saves.error)} />
                <button onClick={() => void saves.refetch()}>
                  Retry loading saves
                </button>
              </div>
            ) : saves.data.length === 0 ? (
              <p className={s.empty}>
                No careers yet. Create UWF to meet your roster and book opening
                night.
              </p>
            ) : (
              <table>
                <thead>
                  <tr>
                    <th>Career</th>
                    <th>Promotion</th>
                    <th>Date</th>
                    <th />
                  </tr>
                </thead>
                <tbody>
                  {saves.data.map((save) => (
                    <tr key={save.saveId}>
                      <th>{save.saveId}</th>
                      <td>{save.promotionName}</td>
                      <td>{gameDate(save.currentDate)}</td>
                      <td>
                        <button
                          disabled={busy}
                          aria-label={`Open career ${save.saveId}`}
                          onClick={() => {
                            create.reset();
                            load.mutate(save.saveId);
                          }}
                        >
                          Open career →
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </div>
        </Panel>
        <p className={s.muted}>
          Careers save automatically. Existing foundation saves are backed up
          before upgrading.
        </p>
      </div>
    </div>
  );
}
