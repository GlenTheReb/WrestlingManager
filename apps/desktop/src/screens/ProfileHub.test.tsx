import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { WorkerProfile } from '@wm/contracts';
import { gameApi } from '../api';
import { defaultWorkerSearch, useNavigation } from '../navigation';
import { ProfilePanel } from './ProfileHub';

vi.mock('../api', () => ({
  errorMessage: (error: unknown) => String(error),
  gameApi: { profile: vi.fn() },
}));

const scoreGroups = {
  movement: 80,
  physicality: 70,
  ringcraft: 84,
  psychology: 77,
  fundamentals: 82,
  entertainment: 75,
};

const profile = {
  company: {
    id: 'uwf',
    name: 'Ultimate Wrestling Federation',
    initials: 'UWF',
    region: 'United Kingdom',
  },
  worker: {
    id: 'worker-1',
    name: 'Atlas King',
    age: 31,
    nationality: 'British',
    language: 'English',
    school: 'Northside Academy',
    background: 'A decorated touring wrestler.',
    personality: 'Focused professional',
    ambition: 'Competitive achievement',
    identity: {
      personality: {},
      qualities: {},
      motivations: null,
      languages: [],
      hobbies: [],
      biography: { mode: 'generated' },
    },
    style: 'Technical',
    weightKg: 98,
    attributes: {
      movement: { agility: 80 },
      physicality: { strength: 70 },
      ringcraft: { technicalGrappling: 84 },
      psychology: { matchStorytelling: 77 },
      fundamentals: { safety: 82 },
      entertainment: { presence: 75 },
    },
    wrestlingStyle: {
      approach: { pace: 'measured' },
      specialisations: ['counterWrestling'],
    },
    condition: {
      fatigue: 20,
      confidence: 76,
      morale: 72,
      momentum: 68,
      popularity: 65,
      wear: 18,
      injuryDays: 0,
      development: 55,
      matches: 14,
    },
    moves: [
      {
        id: 'bridge-suplex',
        name: 'Bridge Suplex',
        style: 'Technical',
        difficulty: 12,
        risk: 2,
        staminaCost: 8,
        minStrength: 40,
        proficiency: 88,
        signature: true,
      },
    ],
    appearanceFee: 120000,
  },
  wrestling: {
    groups: scoreGroups,
    disciplineFits: [{ discipline: 'technicalGrappling', score: 86 }],
    primary: 'technicalGrappling',
    secondaries: ['counterWrestling'],
    archetype: 'Technical Ace',
    overall: 84,
    developing: false,
  },
  history: [
    {
      showId: 4,
      showName: 'UWF Collision Course',
      date: '2026-01-08',
      segmentId: 12,
      kind: 'match',
      title: 'Atlas King vs Rowan Vale',
      participants: [
        { workerId: 'worker-1', name: 'Atlas King' },
        { workerId: 'worker-2', name: 'Rowan Vale' },
      ],
      winnerId: 'worker-1',
      result: 'Atlas King won by pinfall.',
      durationSeconds: 735,
      performance: {
        execution: 85,
        psychology: 79,
        engagement: 82,
        safety: 91,
        story: 78,
        protection: 74,
      },
      reasons: ['Clean execution supported the planned finish.'],
    },
  ],
  nextBooking: {
    showId: 5,
    showName: 'UWF Thursday Night',
    date: '2026-01-15',
    segmentId: 18,
    kind: 'match',
    title: 'Atlas King vs Rowan Vale II',
    participants: [
      { workerId: 'worker-1', name: 'Atlas King' },
      { workerId: 'worker-2', name: 'Rowan Vale' },
    ],
  },
  recentNews: [
    {
      id: 9,
      category: 'People',
      title: 'Atlas King responds to management',
      body: 'The discussion ended constructively.',
      date: '2026-01-09',
      showId: null,
      workerId: 'worker-1',
      read: false,
    },
  ],
  personalityDescription: {
    text: 'Focused professional',
    partial: false,
    reasons: [],
  },
  biography: 'Atlas King is a British technical wrestler.',
  exceptionalTraits: { states: [], history: [], hasEvidence: false },
  relationships: {
    ruleVersion: 1,
    management: {
      revision: 0,
      summary: 'Professional respect',
      signals: { affinity: 60, respect: 70, trust: 65, tension: 10 },
    },
    personal: [],
    interactionOptions: [],
    interactionHistory: [],
    attentionRemaining: 4,
    attentionLimit: 4,
  },
  character: {
    legalName: 'Alex King',
    active: {
      id: 'character-1',
      revision: 0,
      ringName: 'Atlas King',
      alignmentIntent: 'hero',
      status: 'active',
      masked: false,
      concealed: false,
      gimmick: { name: 'The Standard Bearer' },
      companyId: 'uwf',
      brand: null,
      startedOn: '2026-01-01',
      endedOn: null,
      aliases: [],
      identityKnowledge: 'public',
    },
    history: [],
    audienceResponses: [],
    pendingChange: null,
  },
} as unknown as WorkerProfile;

function renderProfile(onClose = vi.fn()) {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  return {
    onClose,
    ...render(
      <QueryClientProvider client={client}>
        <ProfilePanel
          saveId="profile-test"
          workerId="worker-1"
          onClose={onClose}
        />
      </QueryClientProvider>,
    ),
  };
}

beforeEach(() => {
  vi.mocked(gameApi.profile).mockResolvedValue(profile);
  useNavigation.setState({
    profileId: 'worker-1',
    profileMode: 'quick',
    profileTab: 'overview',
    workerSearch: defaultWorkerSearch(),
    workerScrollTop: 320,
  });
});

describe('Person profile hub', () => {
  it('expands quick reference into the complete profile without losing search state', async () => {
    const user = userEvent.setup();
    renderProfile();

    expect(await screen.findByText('QUICK PROFILE')).toBeInTheDocument();
    expect(
      await screen.findByRole('heading', { name: 'Atlas King' }),
    ).toBeInTheDocument();
    expect(screen.getByText('Atlas King vs Rowan Vale II')).toBeInTheDocument();
    await user.click(
      screen.getAllByRole('button', { name: 'Full profile' })[0]!,
    );

    expect(screen.getByText('PERSON HUB')).toBeInTheDocument();
    expect(
      screen.getByRole('navigation', { name: 'Person profile sections' }),
    ).toBeInTheDocument();
    expect(
      screen.getByRole('button', { name: /Matches & appearances/ }),
    ).toBeInTheDocument();
    expect(useNavigation.getState().workerScrollTop).toBe(320);
  });

  it('shows detailed attributes and show-aware appearance evidence', async () => {
    useNavigation.setState({ profileMode: 'full' });
    const user = userEvent.setup();
    renderProfile();
    await screen.findByRole('heading', { name: 'Atlas King' });

    await user.click(screen.getByRole('button', { name: /Attributes/ }));
    expect(screen.getByText('Working repertoire')).toBeInTheDocument();
    expect(screen.getByText('Bridge Suplex')).toBeInTheDocument();

    await user.click(
      screen.getByRole('button', { name: /Matches & appearances/ }),
    );
    expect(
      screen.getByText('UWF Collision Course · with Rowan Vale'),
    ).toBeInTheDocument();
    expect(screen.getByText('12:15')).toBeInTheDocument();
    expect(screen.getByText('Atlas King won by pinfall.')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Open show report' }));
    expect(useNavigation.getState().screen).toBe('reports');
    expect(useNavigation.getState().reportShowId).toBe(4);
  });

  it('closes without mutating the originating workspace', async () => {
    const user = userEvent.setup();
    const { onClose } = renderProfile();
    await screen.findByText('QUICK PROFILE');
    await user.click(
      screen.getByRole('button', { name: 'Close person profile' }),
    );
    expect(onClose).toHaveBeenCalledOnce();
    expect(useNavigation.getState().workerSearch.text).toBe('');
    expect(useNavigation.getState().workerScrollTop).toBe(320);
  });

  it('does not silently evict a worker when comparison is full', async () => {
    const selected = ['worker-2', 'worker-3', 'worker-4', 'worker-5'];
    useNavigation.setState({ workerCompareIds: selected });
    const user = userEvent.setup();
    renderProfile();
    await screen.findByRole('heading', { name: 'Atlas King' });

    await user.click(screen.getByText('Actions'));
    expect(
      screen.getByRole('button', { name: 'Comparison full' }),
    ).toBeDisabled();
    expect(useNavigation.getState().workerCompareIds).toEqual(selected);
  });
});
