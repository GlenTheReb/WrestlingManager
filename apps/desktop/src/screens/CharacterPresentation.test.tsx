import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { expect, it, vi } from 'vitest';
import type { CharacterProfile } from '@wm/contracts';
import { CharacterPresentation } from './CharacterPresentation';

vi.mock('../api', () => ({
  errorMessage: (error: unknown) => String(error),
  gameApi: {
    proposeCharacterChange: vi.fn(),
    launchCharacterChange: vi.fn(),
    setCharacterRetired: vi.fn(),
  },
}));

const character: CharacterProfile = {
  legalName: 'Jordan Vale',
  active: {
    id: 'character-worker-001',
    revision: 1,
    ringName: 'Jordan Vale',
    alignmentIntent: 'face',
    status: 'active',
    masked: false,
    concealed: false,
    companyId: 'uwf',
    brand: null,
    startedOn: '2026-01-01',
    endedOn: null,
    aliases: ['Jordan Vale'],
    identityKnowledge: 'public',
    gimmick: {
      name: 'Authentic competitor',
      description: 'A grounded professional wrestler.',
      coreFantasy: 'Credible professional wrestler',
      tags: ['authentic'],
      tone: 'Serious',
      promoVoice: 'Natural',
      presentationIntensity: 'Balanced',
      entranceAndMatchBehavior: 'Focused',
      attireAndMask: 'Standard ring attire',
      catchphrasesAndGestures: '',
      traitsToEmphasize: [],
    },
  },
  history: [],
  audienceResponses: [],
  pendingChange: null,
};

it('presents a wrestling-language character workflow without editable stat bonuses', async () => {
  render(
    <QueryClientProvider client={new QueryClient()}>
      <CharacterPresentation
        saveId="career"
        workerId="worker-001"
        character={character}
      />
    </QueryClientProvider>,
  );

  expect(
    screen.getByRole('heading', { name: 'Jordan Vale' }),
  ).toBeInTheDocument();
  expect(screen.getByText('Face')).toBeInTheDocument();
  await userEvent.click(screen.getByRole('button', { name: /plan a change/i }));
  expect(screen.getByLabelText('Ring name')).toHaveValue('Jordan Vale');
  expect(
    screen.getByLabelText('What should the audience understand?'),
  ).toBeInTheDocument();
  expect(screen.queryByLabelText(/bonus|modifier/i)).not.toBeInTheDocument();
});
