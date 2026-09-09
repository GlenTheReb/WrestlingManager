import { useCallback, useReducer, type SetStateAction } from 'react';

type History<T> = { past: T[]; present: T; future: T[] };
type Action<T> =
  { kind: 'edit'; value: SetStateAction<T> } | { kind: 'undo' | 'redo' };
function reduce<T>(state: History<T>, action: Action<T>): History<T> {
  if (action.kind === 'edit') {
    const next =
      typeof action.value === 'function'
        ? (action.value as (old: T) => T)(state.present)
        : action.value;
    if (next === state.present) return state;
    return {
      past: [...state.past, state.present].slice(-50),
      present: next,
      future: [],
    };
  }
  if (action.kind === 'undo' && state.past.length)
    return {
      past: state.past.slice(0, -1),
      present: state.past.at(-1)!,
      future: [state.present, ...state.future],
    };
  if (action.kind === 'redo' && state.future.length)
    return {
      past: [...state.past, state.present],
      present: state.future[0]!,
      future: state.future.slice(1),
    };
  return state;
}

// Keep planner history local and bounded; only explicit Save writes the career.
export function useHistory<T>(initial: T) {
  const [state, dispatch] = useReducer(reduce<T>, {
    past: [],
    present: initial,
    future: [],
  });
  const set = useCallback(
    (value: SetStateAction<T>) => dispatch({ kind: 'edit', value }),
    [],
  );
  return {
    value: state.present,
    set,
    undo: () => dispatch({ kind: 'undo' }),
    redo: () => dispatch({ kind: 'redo' }),
    canUndo: !!state.past.length,
    canRedo: !!state.future.length,
  };
}
