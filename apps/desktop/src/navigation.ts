import { create } from 'zustand';

export type GameScreen =
  'saves' | 'overview' | 'roster' | 'booking' | 'live' | 'reports' | 'news';
type Navigation = {
  screen: GameScreen;
  activeSaveId: string | null;
  liveShowId: number | null;
  profileId: string | null;
  reportShowId: number | null;
  plannerDirty: boolean;
  setPlannerDirty: (dirty: boolean) => void;
  openReport: (showId: number) => void;
  navigate: (screen: GameScreen) => void;
  openLive: (showId: number) => void;
  inspectWorker: (workerId: string | null) => void;
  openSave: (saveId: string) => void;
  showLibrary: () => void;
};

// Selection belongs to the interface; promotion state belongs to Rust/SQLite.
export const useNavigation = create<Navigation>((set) => ({
  screen: 'saves',
  activeSaveId: null,
  liveShowId: null,
  profileId: null,
  reportShowId: null,
  plannerDirty: false,
  setPlannerDirty: (plannerDirty) => set({ plannerDirty }),
  openReport: (reportShowId) => set({ reportShowId, screen: 'reports' }),
  navigate: (screen) => set({ screen }),
  openLive: (liveShowId) => set({ liveShowId, screen: 'live' }),
  inspectWorker: (profileId) => set({ profileId }),
  openSave: (saveId) =>
    set({
      activeSaveId: saveId,
      screen: 'overview',
      liveShowId: null,
      profileId: null,
      reportShowId: null,
      plannerDirty: false,
    }),
  showLibrary: () => set({ screen: 'saves' }),
}));
