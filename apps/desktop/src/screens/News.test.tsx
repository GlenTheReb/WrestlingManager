import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, expect, it, vi } from 'vitest';
import type { NewsItem } from '@wm/contracts';
import { gameApi } from '../api';
import { News } from './News';
import { useNavigation } from '../navigation';

vi.mock('../api', () => ({
  gameApi: { news: vi.fn(), setNewsRead: vi.fn() },
  errorMessage: (error: { message: string }) => error.message,
}));
const story: NewsItem = {
  id: 7,
  category: 'Results',
  title: 'Thursday Night: the result',
  body: 'Mercer won by submission.\n\nThe crowd stayed engaged.',
  date: '2026-01-01',
  showId: 12,
  workerId: null,
  read: false,
};
beforeEach(() => {
  vi.resetAllMocks();
  useNavigation.setState({ screen: 'news', reportShowId: null });
});
function renderNews() {
  render(
    <QueryClientProvider
      client={
        new QueryClient({
          defaultOptions: {
            queries: { retry: false },
            mutations: { retry: false },
          },
        })
      }
    >
      <News saveId="career" />
    </QueryClientProvider>,
  );
}

it('marks an opened story read and links to its exact historical show', async () => {
  vi.mocked(gameApi.news).mockResolvedValue({
    items: [story],
    total: 1,
    unread: 1,
  });
  vi.mocked(gameApi.setNewsRead).mockResolvedValue();
  const user = userEvent.setup();
  renderNews();
  await user.click(
    await screen.findByRole('button', { name: /Thursday Night: the result/ }),
  );
  await waitFor(() =>
    expect(gameApi.setNewsRead).toHaveBeenCalledWith('career', 7, true),
  );
  await user.click(screen.getByRole('button', { name: 'Open show report' }));
  expect(useNavigation.getState().reportShowId).toBe(12);
  expect(useNavigation.getState().screen).toBe('reports');
});

it('does not claim a story is read when saving the acknowledgement fails', async () => {
  vi.mocked(gameApi.news).mockResolvedValue({
    items: [story],
    total: 1,
    unread: 1,
  });
  vi.mocked(gameApi.setNewsRead).mockRejectedValue({
    message: 'Could not save read state.',
  });
  const user = userEvent.setup();
  renderNews();
  await user.click(
    await screen.findByRole('button', { name: /Thursday Night: the result/ }),
  );
  expect(await screen.findByRole('alert')).toHaveTextContent(
    'Could not save read state.',
  );
  expect(screen.getByRole('button', { name: 'Mark read' })).toBeEnabled();
});
