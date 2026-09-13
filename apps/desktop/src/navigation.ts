import { create } from 'zustand';
import type { WorkerSearchRequest } from '@wm/contracts';

export const defaultWorkerSearch = (): WorkerSearchRequest => ({
  text: '',
  filters: {
    age: { minimum: null, maximum: null },
    overall: { minimum: null, maximum: null },
    movement: { minimum: null, maximum: null },
    physicality: { minimum: null, maximum: null },
    ringcraft: { minimum: null, maximum: null },
    psychology: { minimum: null, maximum: null },
    fundamentals: { minimum: null, maximum: null },
    entertainment: { minimum: null, maximum: null },
    nationalities: [],
    excludedNationalities: [],
    languages: [],
    excludedLanguages: [],
    schools: [],
    excludedSchools: [],
    archetypes: [],
    excludedArchetypes: [],
    primaryDisciplines: [],
    excludedPrimaryDisciplines: [],
    availability: 'any',
    shortlistId: null,
    blacklist: 'include',
  },
  sort: [{ key: 'name', direction: 'ascending' }],
  offset: 0,
  limit: 50,
});

export type GameScreen =
  'saves' | 'overview' | 'talent' | 'booking' | 'live' | 'reports' | 'news';
export type ProfileMode = 'quick' | 'full';
export type ProfileTab =
  | 'overview'
  | 'attributes'
  | 'character'
  | 'career'
  | 'appearances'
  | 'relationships'
  | 'contract'
  | 'development'
  | 'media';
type Navigation = {
  screen: GameScreen;
  activeSaveId: string | null;
  liveShowId: number | null;
  profileId: string | null;
  profileMode: ProfileMode;
  profileTab: ProfileTab;
  reportShowId: number | null;
  plannerDirty: boolean;
  workerSearch: WorkerSearchRequest;
  workerColumns: string[];
  workerCompareIds: string[];
  workerSearchFocus: number;
  workerScrollTop: number;
  setWorkerSearch: (request: WorkerSearchRequest) => void;
  setWorkerColumns: (columns: string[]) => void;
  setWorkerCompareIds: (ids: string[]) => void;
  setWorkerScrollTop: (top: number) => void;
  openWorkerFinder: () => void;
  setPlannerDirty: (dirty: boolean) => void;
  openReport: (showId: number) => void;
  navigate: (screen: GameScreen) => void;
  openLive: (showId: number) => void;
  inspectWorker: (workerId: string | null, mode?: ProfileMode) => void;
  setProfileMode: (mode: ProfileMode) => void;
  setProfileTab: (tab: ProfileTab) => void;
  openSave: (saveId: string) => void;
  showLibrary: () => void;
};

// Selection belongs to the interface; promotion state belongs to Rust/SQLite.
export const useNavigation = create<Navigation>((set) => ({
  screen: 'saves',
  activeSaveId: null,
  liveShowId: null,
  profileId: null,
  profileMode: 'quick',
  profileTab: 'overview',
  reportShowId: null,
  plannerDirty: false,
  workerSearch: defaultWorkerSearch(),
  workerColumns: [
    'name',
    'age',
    'archetype',
    'overall',
    'movement',
    'physicality',
    'ringcraft',
    'psychology',
    'fundamentals',
    'entertainment',
    'status',
  ],
  workerCompareIds: [],
  workerSearchFocus: 0,
  workerScrollTop: 0,
  setWorkerSearch: (workerSearch) => set({ workerSearch }),
  setWorkerColumns: (workerColumns) => set({ workerColumns }),
  setWorkerCompareIds: (workerCompareIds) => set({ workerCompareIds }),
  setWorkerScrollTop: (workerScrollTop) => set({ workerScrollTop }),
  openWorkerFinder: () =>
    set((state) => ({
      screen: 'talent',
      workerSearchFocus: state.workerSearchFocus + 1,
    })),
  setPlannerDirty: (plannerDirty) => set({ plannerDirty }),
  openReport: (reportShowId) => set({ reportShowId, screen: 'reports' }),
  navigate: (screen) => set({ screen }),
  openLive: (liveShowId) => set({ liveShowId, screen: 'live' }),
  inspectWorker: (profileId, profileMode = 'quick') =>
    set((state) => ({
      profileId,
      profileMode: profileId ? profileMode : state.profileMode,
      profileTab:
        profileId && profileId !== state.profileId
          ? 'overview'
          : state.profileTab,
    })),
  setProfileMode: (profileMode) => set({ profileMode }),
  setProfileTab: (profileTab) => set({ profileTab }),
  openSave: (saveId) =>
    set({
      activeSaveId: saveId,
      screen: 'overview',
      liveShowId: null,
      profileId: null,
      profileMode: 'quick',
      profileTab: 'overview',
      reportShowId: null,
      plannerDirty: false,
      workerSearch: defaultWorkerSearch(),
      workerCompareIds: [],
      workerScrollTop: 0,
    }),
  showLibrary: () => set({ screen: 'saves' }),
}));
