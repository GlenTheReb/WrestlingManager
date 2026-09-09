import { useEffect, useRef, type ReactNode } from 'react';
import s from './Game.module.css';
export const currency = (pence: number) =>
  new Intl.NumberFormat('en-GB', {
    style: 'currency',
    currency: 'GBP',
    maximumFractionDigits: 0,
  }).format(pence / 100);
export const gameDate = (date: string) =>
  new Intl.DateTimeFormat('en-GB', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
    timeZone: 'UTC',
  }).format(new Date(`${date}T00:00:00Z`));
export const clockTime = (seconds: number) =>
  `${Math.floor(seconds / 60)
    .toString()
    .padStart(2, '0')}:${(seconds % 60).toString().padStart(2, '0')}`;
export function Meter({ label, value }: { label: string; value: number }) {
  return (
    <div className={s.meter}>
      <div>
        <span>{label}</span>
        <b>{value}</b>
      </div>
      <meter min={0} max={100} value={value} aria-label={label} />
    </div>
  );
}
export function ErrorNotice({ message }: { message: string }) {
  return (
    <p className={s.error} role="alert">
      {message}
    </p>
  );
}
export function Panel({
  title,
  action,
  children,
  className = '',
}: {
  title: string;
  action?: ReactNode;
  children: ReactNode;
  className?: string | undefined;
}) {
  return (
    <section className={`${s.panel} ${className}`}>
      <header className={s.panelTitle}>
        <h2>{title}</h2>
        {action}
      </header>
      {children}
    </section>
  );
}
export function Overlay({
  title,
  onClose,
  children,
}: {
  title: string;
  onClose: () => void;
  children: ReactNode;
}) {
  const ref = useRef<HTMLDialogElement>(null);
  useEffect(() => {
    const dialog = ref.current;
    dialog?.showModal();
    return () => dialog?.close();
  }, []);
  return (
    <dialog
      ref={ref}
      className={s.overlay}
      onCancel={(event) => {
        event.preventDefault();
        onClose();
      }}
    >
      <header>
        <h2>{title}</h2>
        <button onClick={onClose} aria-label="Close panel">
          ×
        </button>
      </header>
      {children}
    </dialog>
  );
}
