import type { SVGProps } from 'react';

export type IconName =
  | 'profile'
  | 'mask'
  | 'history'
  | 'moves'
  | 'people'
  | 'spark'
  | 'edit'
  | 'launch';

const paths: Record<IconName, string> = {
  profile: 'M12 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8Zm-7 8a7 7 0 0 1 14 0',
  mask: 'M4 7c3-2 5-2 8-2s5 0 8 2l-1 9-7 4-7-4-1-9Zm3 4 3 1m7-1-3 1',
  history: 'M4 5v5h5M5 10a8 8 0 1 1 2 7m5-9v5l3 2',
  moves: 'm5 15 4-4 3 2 7-7M14 6h5v5',
  people:
    'M8 12a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm8-1a2.5 2.5 0 1 0 0-5M3 20a5 5 0 0 1 10 0m1-5a5 5 0 0 1 7 5',
  spark: 'm12 2 1.5 6.5L20 10l-6.5 1.5L12 18l-1.5-6.5L4 10l6.5-1.5L12 2Z',
  edit: 'm4 20 4-1 11-11-3-3L5 16l-1 4Zm10-13 3 3',
  launch: 'M5 19 19 5m-8 0h8v8',
};

export function Icon({
  name,
  ...props
}: SVGProps<SVGSVGElement> & { name: IconName }) {
  return (
    <svg aria-hidden="true" viewBox="0 0 24 24" {...props}>
      <path
        d={paths[name]}
        fill="none"
        stroke="currentColor"
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth="1.8"
      />
    </svg>
  );
}
