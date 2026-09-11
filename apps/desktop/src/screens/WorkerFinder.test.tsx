import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { gameApi } from '../api';
import { defaultWorkerSearch, useNavigation } from '../navigation';
import { WorkerFinder } from './WorkerFinder';

vi.mock('../api', () => ({
  errorMessage: (error: unknown) => String(error),
  gameApi: {
    workerSearch: vi.fn(),
    workerFilterOptions: vi.fn(),
    workerDiscoveryLists: vi.fn(),
    compareWorkers: vi.fn(),
    saveWorkerView: vi.fn(),
    createWorkerShortlist: vi.fn(),
    deleteWorkerView: vi.fn(),
    deleteWorkerShortlist: vi.fn(),
    setWorkerShortlistMember: vi.fn(),
    setWorkerBlacklisted: vi.fn(),
  },
}));

const emptyOptions = {
  nationalities: [],
  languages: [],
  schools: [],
  archetypes: [],
  primaryDisciplines: [],
};
const originalLists = {
  savedViews: [],
  shortlists: [{ id: 2, name: 'Prospects', memberCount: 0 }],
  blacklistCount: 0,
};

function renderFinder() {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  return render(
    <QueryClientProvider client={client}>
      <WorkerFinder saveId="finder-test" />
    </QueryClientProvider>,
  );
}

beforeEach(() => {
  vi.mocked(gameApi.workerSearch).mockResolvedValue({ rows: [], total: 0 });
  vi.mocked(gameApi.workerFilterOptions).mockResolvedValue(emptyOptions);
  vi.mocked(gameApi.workerDiscoveryLists).mockResolvedValue(originalLists);
  vi.mocked(gameApi.createWorkerShortlist).mockResolvedValue({
    ...originalLists,
    shortlists: [
      { id: 1, name: 'Alpha', memberCount: 0 },
      ...originalLists.shortlists,
    ],
  });
  useNavigation.setState({
    workerSearch: defaultWorkerSearch(),
    workerColumns: ['name'],
    workerCompareIds: [],
    workerScrollTop: 0,
  });
});

describe('Worker Finder', () => {
  it('selects the shortlist that was just created rather than the last sorted list', async () => {
    const user = userEvent.setup();
    renderFinder();
    await user.type(screen.getByLabelText('New shortlist name'), 'Alpha');
    await user.click(screen.getByRole('button', { name: 'Create' }));

    await waitFor(() =>
      expect(screen.getByLabelText('Active shortlist')).toHaveValue('1'),
    );
  });
});
