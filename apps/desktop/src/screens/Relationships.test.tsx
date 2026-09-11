import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { invoke } from '@tauri-apps/api/core';
import type {
  InteractionOutcome,
  InteractionTargetPage,
  RelationshipProfile,
} from '@wm/contracts';
import { beforeEach, expect, it, vi } from 'vitest';
import { RelationshipsPanel } from './Relationships';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(), isTauri: vi.fn() }));

const relationships: RelationshipProfile = {
  ruleVersion: 1,
  management: {
    summary: 'Neutral colleague',
    signals: {
      affinity: 'Neutral',
      respect: 'Slightly respectful',
      trust: 'Neutral',
      tension: 'Calm',
    },
    revision: 3,
  },
  personal: [
    {
      otherId: 'worker-002',
      otherName: 'Morgan Vale',
      summary: 'Professional respect',
      signals: {
        affinity: 'Slightly likes',
        respect: 'Strongly respectful',
        trust: 'Slightly trusting',
        tension: 'Calm',
      },
      memories: [
        {
          id: 'memory-1',
          occurredOn: '2026-01-01',
          kind: 'sharedBackground',
          summary: 'They share a training background.',
          active: true,
        },
      ],
    },
  ],
  interactionOptions: [
    {
      kind: 'checkIn',
      label: 'Check in',
      description: 'Ask how they are doing without forcing an agenda.',
      enabled: true,
      unavailableReason: null,
      attentionCost: 1,
      cooldownUntil: null,
      requiresTarget: false,
    },
    {
      kind: 'praiseRecentWork',
      label: 'Praise recent work',
      description: 'Recognise a recent performance.',
      enabled: false,
      unavailableReason:
        'There is no performance from the last 28 days to discuss.',
      attentionCost: 1,
      cooldownUntil: null,
      requiresTarget: false,
    },
    {
      kind: 'discussColleague',
      label: 'Discuss a colleague',
      description: 'Ask privately how they feel about another wrestler.',
      enabled: true,
      unavailableReason: null,
      attentionCost: 1,
      cooldownUntil: null,
      requiresTarget: true,
    },
  ],
  interactionHistory: [],
  attentionRemaining: 4,
  attentionLimit: 4,
};

const targets: InteractionTargetPage = {
  rows: [
    {
      workerId: 'worker-002',
      name: 'Morgan Vale',
      relationship: 'Professional respect',
    },
  ],
  total: 1,
};

function renderPanel() {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  return render(
    <QueryClientProvider client={client}>
      <RelationshipsPanel
        saveId="career"
        workerId="worker-001"
        relationships={relationships}
      />
    </QueryClientProvider>,
  );
}

beforeEach(() => {
  vi.mocked(invoke).mockReset();
  vi.mocked(invoke).mockResolvedValue(targets);
});

it('explains directional relationships and unavailable conversations', () => {
  renderPanel();
  expect(screen.getByText('Professional respect')).toBeInTheDocument();
  expect(
    screen.getByText(/other person may feel differently/i),
  ).toBeInTheDocument();
  expect(
    screen.getByText(/no performance from the last 28 days/i),
  ).toBeInTheDocument();
  expect(
    screen.getByRole('button', { name: 'Praise recent work' }),
  ).toBeDisabled();
  expect(
    screen.getByRole('button', { name: 'Discuss a colleague' }),
  ).toBeDisabled();
});

it('sends a contextual conversation and explains the returned response', async () => {
  const outcome: InteractionOutcome = {
    requestId: 'request-1',
    workerId: 'worker-001',
    kind: 'discussColleague',
    label: 'Discuss a colleague',
    occurredOn: '2026-01-01',
    tone: 'open',
    response: 'Alex: "Morgan is someone I respect professionally."',
    factors: ['Their candid nature made them willing to talk.'],
    relationshipDelta: { affinity: 0, respect: 0, trust: 1, tension: 0 },
    moraleDelta: 0,
    confidenceDelta: 0,
    effects: ['Trust in management improved.'],
    contextWorkerId: 'worker-002',
  };
  vi.mocked(invoke).mockImplementation((command) =>
    Promise.resolve(command === 'interact_with_worker' ? outcome : targets),
  );
  const user = userEvent.setup();
  renderPanel();
  await screen.findByRole('option', { name: /Morgan Vale/ });
  await user.selectOptions(screen.getByLabelText('Colleague'), 'worker-002');
  await user.click(screen.getByRole('button', { name: 'Discuss a colleague' }));
  expect(await screen.findByText(outcome.response)).toBeInTheDocument();
  expect(invoke).toHaveBeenCalledWith('interact_with_worker', {
    request: expect.objectContaining({
      saveId: 'career',
      workerId: 'worker-001',
      kind: 'discussColleague',
      contextWorkerId: 'worker-002',
      expectedRevision: 3,
    }),
  });
  await user.click(screen.getByText('Why did they respond this way?'));
  expect(
    screen.getByText('Their candid nature made them willing to talk.'),
  ).toBeInTheDocument();
});
