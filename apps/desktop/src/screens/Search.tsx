import { useDeferredValue, useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import type { CareerOffice } from '@wm/contracts';
import { gameApi, errorMessage } from '../api';
import { Overlay, ErrorNotice, gameDate } from '../game-ui';
import { useNavigation } from '../navigation';
import s from '../Game.module.css';

export function Search({
  saveId,
  office,
  onClose,
}: {
  saveId: string;
  office: CareerOffice;
  onClose: () => void;
}) {
  const [text, setText] = useState('');
  const search = useDeferredValue(text);
  const { navigate, inspectWorker } = useNavigation();
  const people = useQuery({
    queryKey: ['search', saveId, search],
    queryFn: () => gameApi.roster(saveId, search, 0, 20),
  });
  const matches = (name: string) =>
    name.toLowerCase().includes(search.toLowerCase());
  return (
    <Overlay title="Search the career" onClose={onClose}>
      <div className={s.searchPanel}>
        <input
          autoFocus
          aria-label="Search career"
          placeholder="Wrestler, promotion or show…"
          value={text}
          onChange={(e) => setText(e.target.value)}
        />
        <small>
          People, promotion and scheduled show · first 20 matching wrestlers
        </small>
        {matches(office.promotion.name) && (
          <button
            onClick={() => {
              navigate('overview');
              onClose();
            }}
          >
            <b>{office.promotion.name}</b>
            <span>Promotion office</span>
          </button>
        )}
        {matches(office.show.name) && (
          <button
            onClick={() => {
              navigate('booking');
              onClose();
            }}
          >
            <b>{office.show.name}</b>
            <span>{gameDate(office.show.date)} · Running order</span>
          </button>
        )}
        {people.isError && <ErrorNotice message={errorMessage(people.error)} />}
        {people.data?.rows.map((worker) => (
          <button
            key={worker.id}
            onClick={() => {
              onClose();
              inspectWorker(worker.id);
            }}
          >
            <b>{worker.name}</b>
            <span>
              {worker.age} · {worker.archetype} · Style OVR {worker.overall} ·{' '}
              {worker.condition.injuryDays ? 'Injured' : 'Available'}
            </span>
          </button>
        ))}
        {people.data?.total === 0 &&
          !matches(office.show.name) &&
          !matches(office.promotion.name) && (
            <p>No matching people, promotion or show.</p>
          )}
      </div>
    </Overlay>
  );
}
