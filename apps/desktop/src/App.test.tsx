import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { invoke, isTauri } from '@tauri-apps/api/core';
import type { PromotionOverview, CareerOffice } from '@wm/contracts';
import { App } from './App';
import { useNavigation } from './navigation';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(), isTauri: vi.fn() }));
const promotion: PromotionOverview = {
  saveId: 'uwf-career',
  promotionId: 'uwf',
  name: 'Ultimate Wrestling Federation',
  initials: 'UWF',
  region: 'United Kingdom',
  foundedOn: '2026-01-01',
  currentDate: '2026-01-01',
  cashPence: 25000000,
  seed: '18446744073709551615',
  engineVersion: '0.2.0',
  schemaVersion: 2,
};
const office: CareerOffice = {
  promotion,
  show: {
    id: 1,
    name: 'UWF Thursday Night',
    date: '2026-01-01',
    status: 'draft',
    revision: 0,
    capacitySeconds: 7200,
    segments: [],
  },
  agents: [],
  rosterCount: 40,
  media: [],
  recentShows: [],
};

function renderApp() {
  const cache = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  return render(
    <QueryClientProvider client={cache}>
      <App />
    </QueryClientProvider>,
  );
}

beforeEach(() => {
  vi.mocked(invoke).mockReset();
  vi.mocked(isTauri).mockReturnValue(true);
  useNavigation.setState({
    screen: 'saves',
    activeSaveId: null,
    plannerDirty: false,
    reportShowId: null,
  });
});

describe('career workflow', () => {
  it('uses WM branding and exits through the native command', async () => {
    vi.mocked(invoke).mockResolvedValue([]);
    const user = userEvent.setup();
    renderApp();
    expect(
      screen.getByRole('button', { name: /WM.*WRESTLING.*MANAGER/ }),
    ).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Game menu' }));
    await user.click(screen.getByRole('button', { name: 'Exit game' }));
    expect(invoke).toHaveBeenCalledWith('exit_game');
  });

  it('keeps the native exit command behind the unsaved-planner choice', async () => {
    vi.mocked(invoke).mockResolvedValue([]);
    useNavigation.setState({ plannerDirty: true });
    const user = userEvent.setup();
    renderApp();
    await user.click(screen.getByRole('button', { name: 'Game menu' }));
    await user.click(screen.getByRole('button', { name: 'Exit game' }));
    expect(invoke).not.toHaveBeenCalledWith('exit_game');
    expect(
      screen.getByText('Your match instructions have unsaved changes.'),
    ).toBeInTheDocument();
    await user.click(
      screen.getByRole('button', { name: 'Return to my booking' }),
    );
    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });
  it('creates a career through IPC with full seed precision and shows returned data', async () => {
    vi.mocked(invoke).mockImplementation(async (command) =>
      command === 'list_saves'
        ? []
        : command === 'career_office'
          ? office
          : promotion,
    );
    const user = userEvent.setup();
    renderApp();
    await user.click(screen.getByRole('button', { name: '+ New career' }));
    await user.clear(screen.getByLabelText('World seed'));
    await user.type(screen.getByLabelText('World seed'), promotion.seed);
    await user.click(screen.getByRole('button', { name: 'Create career' }));
    expect(
      await screen.findByRole('heading', { name: 'Decision centre' }),
    ).toBeInTheDocument();
    expect(invoke).toHaveBeenCalledWith('create_game', {
      request: { saveId: 'uwf-career', seed: promotion.seed },
    });
    expect(screen.getAllByText('£250,000').length).toBeGreaterThan(0);
    expect(screen.getByText(promotion.seed)).toBeInTheDocument();
  });

  it('loads an existing career and can navigate back to its library', async () => {
    vi.mocked(invoke).mockImplementation(async (command) =>
      command === 'list_saves'
        ? [
            {
              saveId: 'uwf-career',
              promotionName: promotion.name,
              currentDate: promotion.currentDate,
              seed: promotion.seed,
            },
          ]
        : command === 'career_office'
          ? office
          : promotion,
    );
    const user = userEvent.setup();
    renderApp();
    await user.click(
      await screen.findByRole('button', { name: 'Open career uwf-career' }),
    );
    expect(
      await screen.findByRole('heading', { name: 'Decision centre' }),
    ).toBeInTheDocument();
    expect(invoke).toHaveBeenCalledWith('load_game', { saveId: 'uwf-career' });
    await user.click(screen.getByRole('button', { name: 'Game menu' }));
    await user.click(screen.getByRole('button', { name: 'Save library' }));
    expect(
      screen.getByRole('heading', { name: 'Save library' }),
    ).toBeInTheDocument();
  });

  it('keeps the form and displays a duplicate-save error without navigating', async () => {
    vi.mocked(invoke).mockImplementation(async (command) => {
      if (command === 'create_game')
        throw {
          code: 'save_already_exists',
          message: 'A save with this ID already exists.',
        };
      return [];
    });
    const user = userEvent.setup();
    renderApp();
    await user.click(screen.getByRole('button', { name: '+ New career' }));
    await user.click(screen.getByRole('button', { name: 'Create career' }));
    expect(await screen.findByRole('alert')).toHaveTextContent(
      'already exists',
    );
    expect(screen.getByLabelText('Save name')).toHaveValue('uwf-career');
    expect(
      screen.queryByRole('heading', { name: promotion.name }),
    ).not.toBeInTheDocument();
  });

  it('prevents repeated creation while the request is pending', async () => {
    let finish: (value: PromotionOverview) => void = () => {};
    vi.mocked(invoke).mockImplementation((command) =>
      command === 'create_game'
        ? new Promise((resolve) => {
            finish = resolve;
          })
        : Promise.resolve(command === 'career_office' ? office : []),
    );
    const user = userEvent.setup();
    renderApp();
    await user.click(screen.getByRole('button', { name: '+ New career' }));
    await user.click(screen.getByRole('button', { name: 'Create career' }));
    expect(
      screen.getByRole('button', { name: 'Creating career…' }),
    ).toBeDisabled();
    finish(promotion);
    await screen.findByRole('heading', { name: 'Decision centre' });
    expect(
      vi
        .mocked(invoke)
        .mock.calls.filter(([command]) => command === 'create_game'),
    ).toHaveLength(1);
  });

  it('retries a failed save listing', async () => {
    vi.mocked(invoke)
      .mockRejectedValueOnce({ message: 'Save storage unavailable.' })
      .mockResolvedValue([]);
    const user = userEvent.setup();
    renderApp();
    expect(await screen.findByRole('alert')).toHaveTextContent(
      'Save storage unavailable.',
    );
    await user.click(
      screen.getByRole('button', { name: 'Retry loading saves' }),
    );
    await waitFor(() =>
      expect(screen.queryByRole('alert')).not.toBeInTheDocument(),
    );
    expect(screen.getByText(/No careers yet/)).toBeInTheDocument();
  });

  it('explains desktop-only storage without invoking unavailable commands in a browser', () => {
    vi.mocked(isTauri).mockReturnValue(false);
    renderApp();
    expect(
      screen.getByRole('heading', {
        name: 'Open Wrestling Manager on your desktop',
      }),
    ).toBeInTheDocument();
    expect(invoke).not.toHaveBeenCalled();
  });
});
