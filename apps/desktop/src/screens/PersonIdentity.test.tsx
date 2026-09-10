import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { expect, it } from 'vitest';
import type {
  PersonIdentity,
  PersonalityDescription,
  TraitOverview,
} from '@wm/contracts';
import { PersonIdentityPanel } from './PersonIdentity';

const populatedIdentity: PersonIdentity = {
  personality: {
    ambition: {
      value: 81,
      visibility: { kind: 'public' },
      source: 'Scout report',
    },
    sociability: {
      value: null,
      visibility: { kind: 'hidden' },
      source: 'Not yet observed',
    },
  },
  qualities: {
    professionalism: {
      value: 18,
      visibility: { kind: 'public' },
      source: 'Employer reference',
    },
    leadership: {
      value: 60,
      visibility: { kind: 'public' },
      source: 'Interview',
    },
  },
  motivations: {
    primary: 'achievement',
    secondary: ['fame', 'wrestlingCraft'],
  },
  languages: [
    { name: 'English', proficiency: 'fluent', native: true },
    { name: 'Japanese', proficiency: null, native: false },
  ],
  hobbies: [
    { hobby: 'gaming', involvement: 'casual' },
    { hobby: 'combatSports', involvement: 'passionate' },
  ],
  biography: {
    mode: 'hybrid',
    authoredText: 'A former amateur champion.',
    birthplace: 'Bristol',
    debutYear: 2018,
    trainers: ['M. Stone'],
    previousOccupations: ['Teacher'],
  },
};

const populatedDescription: PersonalityDescription = {
  ruleVersion: 1,
  text: 'Driven but difficult to read.',
  partial: true,
  reasons: [
    {
      field: 'ambition',
      label: 'Ambition inferred from interviews',
      source: 'Scout report',
    },
  ],
};

const populatedTraits: TraitOverview = {
  states: [
    {
      traitId: 'companyIcon',
      companyId: null,
      visibility: { kind: 'public' },
      status: 'active',
      evidenceIds: ['show-1'],
    },
    {
      traitId: 'wellnessComplianceRisk',
      companyId: 'company-a',
      visibility: { kind: 'company', companyId: 'company-a' },
      status: 'elevated',
      evidenceIds: ['incident-1', 'incident-2'],
    },
  ],
  history: [
    {
      date: '2026-01-04',
      ruleVersion: 1,
      from: 'inactive',
      state: {
        traitId: 'companyIcon',
        companyId: null,
        visibility: { kind: 'public' },
        status: 'active',
        evidenceIds: ['show-1'],
      },
      reason: 'Three strong public appearances.',
    },
  ],
  hasEvidence: true,
};

function renderPanel(
  identity: PersonIdentity = populatedIdentity,
  description: PersonalityDescription = populatedDescription,
  biography = 'A former amateur champion.',
  traits: TraitOverview = populatedTraits,
) {
  render(
    <PersonIdentityPanel
      identity={identity}
      description={description}
      biography={biography}
      traits={traits}
    />,
  );
}

it('renders assessment bands, source tooltips, partial reasons, and identity details', async () => {
  renderPanel();
  const personality = screen
    .getByRole('heading', { name: 'Personality' })
    .closest('section') as HTMLElement;
  const qualities = screen
    .getByRole('heading', { name: 'Shared qualities' })
    .closest('section') as HTMLElement;

  expect(
    within(personality).getByText('Driven but difficult to read.'),
  ).toBeInTheDocument();
  expect(personality).toHaveTextContent('Partial assessment');
  expect(within(personality).getByText('Very high')).toHaveAttribute(
    'title',
    '81/100 · Scout report',
  );
  expect(within(personality).getByText('Unknown')).toHaveAttribute(
    'title',
    'Not yet known',
  );
  expect(within(qualities).getByText('Very low')).toBeInTheDocument();
  expect(within(qualities).getByText('Moderate')).toBeInTheDocument();

  const user = userEvent.setup();
  await user.click(within(personality).getByText('Why this description?'));
  expect(
    within(personality).getByText(
      /Ambition inferred from interviews: Ambition · Scout report/,
    ),
  ).toBeInTheDocument();

  expect(screen.getByText('Achievement')).toBeInTheDocument();
  expect(screen.getByText('Fame, Wrestling Craft')).toBeInTheDocument();
  expect(screen.getByText('English · Fluent · Native')).toBeInTheDocument();
  expect(
    screen.getByText('Japanese · Proficiency unknown'),
  ).toBeInTheDocument();
  expect(screen.getByText('Gaming · Casual')).toBeInTheDocument();
  expect(screen.getByText('Combat Sports · Passionate')).toBeInTheDocument();
  expect(screen.getByText('Hybrid biography')).toBeInTheDocument();
});

it('renders exceptional trait states and history, including confidential company context', async () => {
  renderPanel();
  const traits = screen
    .getByRole('heading', { name: 'Exceptional traits' })
    .closest('section') as HTMLElement;

  expect(within(traits).getByText('Company Icon')).toBeInTheDocument();
  expect(
    within(traits).getByText('Wellness Compliance Risk'),
  ).toBeInTheDocument();
  expect(traits).toHaveTextContent(/Company Icon\s*·\s*Active/);
  expect(traits).toHaveTextContent(
    /Wellness Compliance Risk\s*·\s*Elevated\s*· company-a/,
  );
  expect(traits).toHaveTextContent('Confidential company information');
  expect(traits).toHaveTextContent('2 evidence item(s)');

  const user = userEvent.setup();
  await user.click(within(traits).getByText('Trait history (latest 30)'));
  expect(
    within(traits).getByText('2026-01-04 · Company Icon'),
  ).toBeInTheDocument();
  expect(within(traits).getByText('Inactive → Active')).toBeInTheDocument();
  expect(
    within(traits).getByText('Three strong public appearances.'),
  ).toBeInTheDocument();
});

it('renders explicit empty states when identity and trait evidence are unknown', () => {
  const identity: PersonIdentity = {
    personality: {},
    qualities: {},
    motivations: null,
    languages: [],
    hobbies: [],
    biography: {
      mode: 'organic',
      authoredText: '',
      birthplace: null,
      debutYear: null,
      trainers: [],
      previousOccupations: [],
    },
  };
  renderPanel(
    identity,
    {
      ruleVersion: 1,
      text: 'Personality not yet established',
      partial: false,
      reasons: [],
    },
    '',
    { states: [], history: [], hasEvidence: false },
  );

  expect(screen.getByText('Motivations not yet known.')).toBeInTheDocument();
  expect(screen.getByText('No languages recorded.')).toBeInTheDocument();
  expect(screen.getByText('No interests recorded.')).toBeInTheDocument();
  expect(
    screen.getByText('No biographical facts recorded.'),
  ).toBeInTheDocument();
  expect(screen.getByText('Organic biography')).toBeInTheDocument();
  expect(
    screen.getByText('No exceptional-trait evidence recorded yet.'),
  ).toBeInTheDocument();
});
