import { invoke, isTauri } from '@tauri-apps/api/core';
import type {
  CreateGameRequest,
  PromotionOverview,
  SaveSummary,
  CareerOffice,
  RosterPage,
  WorkerProfile,
  ShowCard,
  MatchPlan,
  AgentAdvice,
  SaveSegmentRequest,
  LiveView,
  AdvanceRequest,
  LiveInstruction,
  ShowReport,
  NewsPage,
} from '@wm/contracts';

export const desktopAvailable = () => isTauri();

// Rust owns these operations. Browser development deliberately has no fake saves.
export const gameApi = {
  createGame: (request: CreateGameRequest) =>
    invoke<PromotionOverview>('create_game', { request }),
  loadGame: (saveId: string) =>
    invoke<PromotionOverview>('load_game', { saveId }),
  overview: (saveId: string) =>
    invoke<PromotionOverview>('get_promotion_overview', { saveId }),
  listSaves: () => invoke<SaveSummary[]>('list_saves'),
  office: (saveId: string) => invoke<CareerOffice>('career_office', { saveId }),
  roster: (saveId: string, search = '', offset = 0, limit = 64) =>
    invoke<RosterPage>('roster_page', { saveId, search, offset, limit }),
  profile: (saveId: string, workerId: string) =>
    invoke<WorkerProfile>('worker_profile', { saveId, workerId }),
  card: (saveId: string, showId: number) =>
    invoke<ShowCard>('show_card', { saveId, showId }),
  agent: (saveId: string, plan: MatchPlan) =>
    invoke<AgentAdvice>('agent_advice', { saveId, plan }),
  saveSegment: (request: SaveSegmentRequest) =>
    invoke<ShowCard>('save_segment', { request }),
  reorder: (saveId: string, showId: number, revision: number, ids: number[]) =>
    invoke<ShowCard>('rearrange_card', { saveId, showId, revision, ids }),
  deleteSegment: (
    saveId: string,
    showId: number,
    revision: number,
    segmentId: number,
  ) =>
    invoke<ShowCard>('delete_segment', { saveId, showId, revision, segmentId }),
  startShow: (saveId: string, showId: number) =>
    invoke<LiveView>('start_show', { saveId, showId }),
  live: (saveId: string, showId: number) =>
    invoke<LiveView>('live_show', { saveId, showId }),
  advance: (request: AdvanceRequest) =>
    invoke<LiveView>('advance_show', { request }),
  instruct: (saveId: string, showId: number, instruction: LiveInstruction) =>
    invoke<LiveView>('live_instruction', { saveId, showId, instruction }),
  report: (saveId: string, showId: number) =>
    invoke<ShowReport>('show_report', { saveId, showId }),
  continueDay: (saveId: string) =>
    invoke<CareerOffice>('continue_day', { saveId }),
  fullscreen: () => invoke<boolean>('toggle_fullscreen'),
  exit: () => invoke<void>('exit_game'),
  news: (
    saveId: string,
    category = '',
    unreadOnly = false,
    offset = 0,
    limit = 30,
  ) =>
    invoke<NewsPage>('news_page', {
      saveId,
      category,
      unreadOnly,
      offset,
      limit,
    }),
  setNewsRead: (saveId: string, id: number, read: boolean) =>
    invoke<void>('set_news_read', { saveId, id, read }),
};

export function errorMessage(error: unknown): string {
  if (
    typeof error === 'object' &&
    error !== null &&
    'message' in error &&
    typeof error.message === 'string'
  ) {
    return error.message;
  }
  return 'The operation could not be completed. Please try again.';
}
