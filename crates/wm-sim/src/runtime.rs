use crate::{
    bounded,
    consequences::apply_match,
    planning::{PlanError, validate_plan},
    rng, roll,
};
use serde::{Deserialize, Serialize};
use wm_domain::game::*;
use wm_domain::match_rules::MatchDefinition;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueuedInstruction {
    pub deliver_at: u32,
    pub instruction: LiveInstruction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MatchRuntime {
    pub plan: MatchPlan,
    pub stamina: [i32; 2],
    pub next_action: u32,
    pub control: String,
    pub control_until: u32,
    pub attempts: i32,
    pub successes: i32,
    pub salvaged: i32,
    pub mistakes: i32,
    pub danger: i32,
    pub engagement_sum: i32,
    pub false_finishes: i32,
    pub injured: [bool; 2],
    pub chemistry: i32,
    pub recent_moves: Vec<String>,
    pub used_moves: Vec<(String, String, u32)>,
    pub abandoned: Vec<u32>,
    pub reasons: Vec<String>,
    pub opening_energy: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Session {
    pub seed: String,
    pub rng_position: String,
    pub show_id: i32,
    pub tick: u32,
    pub sequence: u32,
    pub index: usize,
    pub elapsed: u32,
    pub complete: bool,
    pub card: Vec<Segment>,
    pub workers: Vec<Worker>,
    pub agents: Vec<RoadAgent>,
    pub crowd: CrowdState,
    pub current: Option<MatchRuntime>,
    pub reports: Vec<SegmentReport>,
    pub pending: Vec<QueuedInstruction>,
    pub relationships: Vec<(String, String, i32)>,
}

pub fn initial_crowd(trust: i32) -> CrowdState {
    CrowdState {
        energy: 58,
        fatigue: 0,
        expectation: 55,
        trust,
        interference_count: 0,
        peak: 58,
        chant: "Waiting for the opening bell".into(),
        cohorts: vec![
            Cohort {
                name: "Mat & sport".into(),
                preference: "Technical|Sports|Strong style".into(),
                energy: 58,
                trust,
                fatigue: 0,
            },
            Cohort {
                name: "Spectacle".into(),
                preference: "Power|Lucha|Hardcore".into(),
                energy: 58,
                trust,
                fatigue: 0,
            },
            Cohort {
                name: "Characters & stories".into(),
                preference: "Entertainment|Comedy".into(),
                energy: 58,
                trust,
                fatigue: 0,
            },
        ],
    }
}

impl Session {
    pub fn new(
        seed: u64,
        show_id: i32,
        card: Vec<Segment>,
        workers: Vec<Worker>,
        agents: Vec<RoadAgent>,
        trust: i32,
        relationships: Vec<(String, String, i32)>,
    ) -> Result<Self, PlanError> {
        // Validate at the simulator boundary as well as the booking boundary.
        // A non-UI caller must not silently execute an unsupported match label.
        for segment in &card {
            if let SegmentPlan::Match(plan) = &segment.content {
                let participant = |id: &str| {
                    workers
                        .iter()
                        .find(|worker| worker.id == id)
                        .ok_or_else(|| {
                            PlanError(
                                "A booked participant is missing from the show snapshot.".into(),
                            )
                        })
                };
                let agent = agents
                    .iter()
                    .find(|agent| agent.id == plan.agent_id)
                    .ok_or_else(|| {
                        PlanError("The booked road agent is missing from the show snapshot.".into())
                    })?;
                validate_plan(
                    plan,
                    participant(&plan.worker_a)?,
                    participant(&plan.worker_b)?,
                    agent,
                )?;
            }
        }
        Ok(Self {
            seed: seed.to_string(),
            rng_position: "0".into(),
            show_id,
            tick: 0,
            sequence: 0,
            index: 0,
            elapsed: 0,
            complete: false,
            card,
            workers,
            agents,
            crowd: initial_crowd(trust),
            current: None,
            reports: Vec::new(),
            pending: Vec::new(),
            relationships,
        })
    }

    fn emit(
        &mut self,
        events: &mut Vec<SimEvent>,
        kind: &str,
        text: String,
        actor: Option<String>,
        move_id: Option<String>,
        importance: i32,
    ) {
        self.sequence += 1;
        events.push(SimEvent {
            sequence: self.sequence,
            show_second: self.tick,
            segment_id: self.card[self.index].id,
            match_second: self.elapsed,
            kind: kind.into(),
            text,
            actor_id: actor,
            move_id,
            importance,
            crowd_energy: self.crowd.energy,
        });
    }

    fn worker(&self, id: &str) -> Worker {
        self.workers
            .iter()
            .find(|w| w.id == id)
            .expect("validated participant snapshot")
            .clone()
    }
    fn agent(&self, id: i32) -> RoadAgent {
        self.agents
            .iter()
            .find(|a| a.id == id)
            .expect("validated agent snapshot")
            .clone()
    }

    fn begin(&mut self, events: &mut Vec<SimEvent>) {
        let segment = self.card[self.index].clone();
        self.crowd.expectation = (self.crowd.peak - 8).clamp(45, 90);
        match &segment.content {
            SegmentPlan::Match(plan) => {
                let a = self.worker(&plan.worker_a);
                let b = self.worker(&plan.worker_b);
                let chemistry = self
                    .relationships
                    .iter()
                    .find(|(left, right, _)| {
                        (left == &a.id && right == &b.id) || (right == &a.id && left == &b.id)
                    })
                    .map(|v| v.2)
                    .unwrap_or(50);
                self.current = Some(MatchRuntime {
                    plan: plan.clone(),
                    stamina: [
                        1000 - a.condition.fatigue * 5,
                        1000 - b.condition.fatigue * 5,
                    ],
                    next_action: 12,
                    control: a.id.clone(),
                    control_until: 0,
                    attempts: 0,
                    successes: 0,
                    salvaged: 0,
                    mistakes: 0,
                    danger: 0,
                    engagement_sum: 0,
                    false_finishes: 0,
                    injured: [false, false],
                    chemistry,
                    recent_moves: Vec::new(),
                    used_moves: Vec::new(),
                    abandoned: Vec::new(),
                    reasons: Vec::new(),
                    opening_energy: self.crowd.energy,
                });
                self.crowd.chant = format!("{} / {}", a.name, b.name);
                self.emit(
                    events,
                    "entrance",
                    format!(
                        "{} and {} make their entrances. {}",
                        a.name, b.name, plan.purpose
                    ),
                    None,
                    None,
                    2,
                );
            }
            SegmentPlan::Angle(plan) => {
                self.current = None;
                self.emit(
                    events,
                    "angle",
                    format!("{} begins: {}.", segment.title, plan.purpose),
                    None,
                    None,
                    2,
                );
            }
        }
    }

    pub fn queue(&mut self, instruction: LiveInstruction) -> Result<(), PlanError> {
        if self.complete || self.current.is_none() {
            return Err(PlanError(
                "Instructions need an active match. Advance to the opening bell first.".into(),
            ));
        }
        if self.pending.len() >= 4 {
            return Err(PlanError(
                "Let the agent deliver the queued instructions first.".into(),
            ));
        }
        let current = self.current.as_ref().unwrap();
        if instruction
            .worker_id
            .as_ref()
            .is_some_and(|id| id != &current.plan.worker_a && id != &current.plan.worker_b)
        {
            return Err(PlanError("Choose an active participant.".into()));
        }
        if matches!(instruction.kind, InstructionKind::Protect) && instruction.worker_id.is_none() {
            return Err(PlanError("Select the wrestler to protect.".into()));
        }
        if instruction.kind == InstructionKind::AbandonSpot
            && instruction.beat_index.is_none_or(|i| {
                i as usize >= current.plan.beats.len()
                    || current.plan.beats[i as usize].at_second <= self.elapsed
            })
        {
            return Err(PlanError(
                "Choose a planned spot that has not happened yet.".into(),
            ));
        }
        if instruction.kind == InstructionKind::ChangeFinish {
            let Some(ref finish) = instruction.finish else {
                return Err(PlanError("Choose the revised finish.".into()));
            };
            let mut revised = current.plan.clone();
            revised.finish = finish.clone();
            if MatchDefinition::try_from(&revised).is_err() {
                return Err(PlanError("Live communication cannot change the locked result. Choose another finish for that result.".into()));
            }
        }
        let agent = self.agent(current.plan.agent_id);
        self.pending.push(QueuedInstruction {
            deliver_at: self.elapsed + (25 - agent.communication).max(5) as u32,
            instruction,
        });
        Ok(())
    }

    fn deliver(&mut self, events: &mut Vec<SimEvent>) {
        let due: Vec<_> = self
            .pending
            .iter()
            .filter(|q| q.deliver_at <= self.elapsed)
            .cloned()
            .collect();
        self.pending.retain(|q| q.deliver_at > self.elapsed);
        for queued in due {
            let Some(current) = self.current.as_mut() else {
                continue;
            };
            let text = match queued.instruction.kind {
                InstructionKind::SlowDown => {
                    current.plan.pace = (current.plan.pace - 1).max(1);
                    "The referee relays: slow the pace and let the crowd breathe."
                }
                InstructionKind::RaisePace => {
                    current.plan.pace = (current.plan.pace + 1).min(5);
                    "The agent asks for a quicker pace."
                }
                InstructionKind::TakeRisks => {
                    current.plan.risk = (current.plan.risk + 1).min(5);
                    "The agent relays permission for more ambitious offence."
                }
                InstructionKind::Protect => {
                    current.plan.protected_worker_id = queued.instruction.worker_id;
                    current.plan.risk = current.plan.risk.min(2);
                    "The referee asks both workers to protect the named performer and remove avoidable risk."
                }
                InstructionKind::GoHome => {
                    current.plan.duration_seconds =
                        current.plan.duration_seconds.min(self.elapsed + 30);
                    "The referee signals: go to the booked finish within thirty seconds."
                }
                InstructionKind::Extend => {
                    current.plan.duration_seconds = (current.plan.duration_seconds + 120).min(2100);
                    "The agent grants two more minutes, within the broadcast allowance."
                }
                InstructionKind::ChangeFinish => {
                    current.plan.finish = queued.instruction.finish.unwrap();
                    "The referee relays the revised finish. The booked winner remains locked."
                }
                InstructionKind::AbandonSpot => {
                    current
                        .abandoned
                        .push(queued.instruction.beat_index.unwrap());
                    "The agent removes the selected future spot from the working plan."
                }
            };
            self.emit(events, "instruction", text.into(), None, None, 3);
        }
    }

    fn crowd_response(&mut self, style: &str, quality: i32, novel: bool) {
        for cohort in &mut self.crowd.cohorts {
            let fit = cohort.preference.split('|').any(|s| s == style);
            let delta = if quality >= 70 {
                if fit { 3 } else { 1 }
            } else if quality < 40 {
                -3
            } else {
                0
            };
            let repetition = if novel { 0 } else { 2 };
            let fatigue_penalty = if cohort.fatigue > 70 { 1 } else { 0 };
            // Earlier peaks raise the standard; a crowd that distrusts the show is harder to win back.
            let expectation_penalty = i32::from(quality < self.crowd.expectation && delta > 0);
            let distrust_penalty = i32::from(cohort.trust < 35 && delta > 0);
            cohort.energy = bounded(
                cohort.energy + delta
                    - repetition
                    - fatigue_penalty
                    - expectation_penalty
                    - distrust_penalty,
            );
            if quality < 20 {
                cohort.trust = bounded(cohort.trust - 1);
            }
        }
        self.crowd.energy = self.crowd.cohorts.iter().map(|c| c.energy).sum::<i32>()
            / self.crowd.cohorts.len() as i32;
        self.crowd.trust = self.crowd.cohorts.iter().map(|c| c.trust).sum::<i32>()
            / self.crowd.cohorts.len() as i32;
        self.crowd.peak = self.crowd.peak.max(self.crowd.energy);
    }

    fn action(
        &mut self,
        beat: Option<PlannedBeat>,
        random: &mut rand_chacha::ChaCha8Rng,
        events: &mut Vec<SimEvent>,
    ) {
        let mut current = self.current.take().unwrap();
        let a = self.worker(&current.plan.worker_a);
        let b = self.worker(&current.plan.worker_b);
        let agent = self.agent(current.plan.agent_id);
        let index = if let Some(ref beat) = beat {
            usize::from(beat.actor_id == b.id)
        } else if self.elapsed < current.control_until {
            usize::from(current.control == b.id)
        } else {
            let pick = roll(random, 100);
            let threshold = match current.plan.protected_worker_id.as_ref() {
                Some(id) if id == &a.id => 60,
                Some(id) if id == &b.id => 40,
                _ => 50,
            };
            if pick < threshold { 0 } else { 1 }
        };
        let workers = [&a, &b];
        let actor = workers[index];
        let opponent = workers[1 - index];
        current.control = actor.id.clone();
        if let Some(ref beat) = beat {
            match beat.kind {
                BeatKind::Control => {
                    current.control_until = self.elapsed + beat.duration_seconds;
                    self.emit(
                        events,
                        "control",
                        format!(
                            "{} takes control for a sustained stretch, as planned.",
                            actor.name
                        ),
                        Some(actor.id.clone()),
                        None,
                        2,
                    );
                    self.current = Some(current);
                    return;
                }
                BeatKind::FalseFinish => {
                    current.false_finishes += 1;
                    let fresh = current.false_finishes <= 2;
                    self.crowd_response(&current.plan.style, if fresh { 90 } else { 30 }, fresh);
                    self.emit(
                        events,
                        "nearFall",
                        format!(
                            "{} almost has it — {} escapes at two!{}",
                            actor.name,
                            opponent.name,
                            if fresh {
                                " The building rises."
                            } else {
                                " Some fans are starting to see the pattern."
                            }
                        ),
                        Some(actor.id.clone()),
                        None,
                        3,
                    );
                    self.current = Some(current);
                    return;
                }
                BeatKind::Interference | BeatKind::Weapon | BeatKind::RefBump => {
                    self.crowd.interference_count += 1;
                    let repeated = self.crowd.interference_count > 1;
                    self.crowd_response(
                        &current.plan.style,
                        if repeated { 15 } else { 85 },
                        !repeated,
                    );
                    if repeated {
                        for c in &mut self.crowd.cohorts {
                            c.trust = bounded(c.trust - 4);
                        }
                        current.reasons.push("Repeated outside involvement made the crowd less willing to trust the contest.".into());
                    }
                    if beat.kind == BeatKind::Weapon {
                        current.danger += 2;
                        current.stamina[1 - index] = (current.stamina[1 - index] - 40).max(0);
                    }
                    let action = match beat.kind {
                        BeatKind::Weapon => "uses a concealed chair as part of the plan",
                        BeatKind::RefBump => "collides with the referee; the official goes down",
                        _ => "benefits from a planned ringside distraction",
                    };
                    self.emit(
                        events,
                        "interference",
                        format!(
                            "{} {}.{}",
                            actor.name,
                            action,
                            if repeated {
                                " The repeated disruption draws frustrated chants."
                            } else {
                                " The crowd reacts to the sudden change."
                            }
                        ),
                        Some(actor.id.clone()),
                        None,
                        3,
                    );
                    self.current = Some(current);
                    return;
                }
                BeatKind::Callback => {
                    self.crowd_response(&current.plan.style, 75, true);
                    self.emit(events,"callback",format!("{} repeats an earlier exchange, but this time {} is ready with a counter.",actor.name,opponent.name),Some(opponent.id.clone()),None,2);
                    self.current = Some(current);
                    return;
                }
                BeatKind::InjurySell => {
                    self.emit(events,"selling",format!("{} sells the planned injury; {} changes the attack to tell that story. This is a worked sequence.",actor.name,opponent.name),Some(actor.id.clone()),None,2);
                    self.current = Some(current);
                    return;
                }
                BeatKind::Move => {}
            }
        }
        if beat.is_none()
            && current.stamina[index] < 180
            && actor.attributes.psychology + agent.psychology > 23
        {
            current.stamina[index] = (current.stamina[index] + 85).min(1000);
            current.stamina[1 - index] = (current.stamina[1 - index] + 35).min(1000);
            current.next_action = self.elapsed + 18;
            self.emit(events,"recovery",format!("{} slows into a grounded hold. Both wrestlers recover enough breath to build the next exchange.",actor.name),Some(actor.id.clone()),None,1);
            self.current = Some(current);
            return;
        }
        let aggressive = actor.personality == "Impulsive"
            && current.plan.freedom > 70
            && actor.attributes.professionalism < 14;
        let allowed_risk = (current.plan.risk + i32::from(aggressive)).min(5);
        let candidates: Vec<_> = actor
            .moves
            .iter()
            .filter(|m| {
                m.risk <= allowed_risk
                    && m.min_strength <= actor.attributes.strength
                    && (current.stamina[index] > 250 || m.stamina_cost <= 3)
            })
            .collect();
        let movement = beat
            .as_ref()
            .and_then(|b| b.move_id.as_ref())
            .and_then(|id| actor.moves.iter().find(|m| &m.id == id))
            .or_else(|| {
                candidates
                    .get(roll(random, candidates.len().max(1) as u32) as usize)
                    .copied()
            })
            .unwrap_or(&actor.moves[0]);
        let fatigue = (1000 - current.stamina[index]) / 35;
        let size_penalty = if movement.min_strength >= 7 {
            (opponent.weight_kg - actor.weight_kg - 15).max(0) / 3
        } else {
            0
        };
        let capability = actor.attributes.technical * 2
            + actor.attributes.experience / 2
            + movement.proficiency / 5
            + opponent.attributes.safety / 2;
        let preparation = agent.psychology / 4
            + agent.experience / 5
            + agent.communication / 5
            + agent.knowledge / 20
            + current.chemistry / 10;
        let composure = (actor.condition.confidence - 50) / 12 + (actor.condition.morale - 50) / 20;
        let physical_problem = i32::from(current.injured[index]) * 10 + actor.condition.wear / 20;
        let chance = (25 + capability + preparation + composure
            - physical_problem
            - movement.difficulty * 2
            - fatigue
            - size_penalty)
            .clamp(12, 97);
        let impossible = movement.min_strength > actor.attributes.strength + 2;
        let success = !impossible && roll(random, 100) < chance;
        let recovery_skill =
            (actor.attributes.improvisation + opponent.attributes.professionalism) / 2;
        let salvaged = !success && !impossible && roll(random, 30) < recovery_skill;
        let quality = if success {
            75 + roll(random, 21)
        } else if salvaged {
            45 + roll(random, 16)
        } else {
            12 + roll(random, 21)
        };
        let novel = !current
            .recent_moves
            .iter()
            .rev()
            .take(3)
            .any(|id| id == &movement.id);
        current.attempts += 1;
        current.successes += i32::from(success);
        current.salvaged += i32::from(salvaged);
        current.mistakes += i32::from(!success && !salvaged);
        let drain =
            (movement.stamina_cost * current.plan.pace * 2 - actor.attributes.stamina / 5).max(3);
        current.stamina[index] = (current.stamina[index] - drain).max(0);
        current.stamina[1 - index] = (current.stamina[1 - index] - drain / 2).max(0);
        if let Some(used) = current
            .used_moves
            .iter_mut()
            .find(|(id, mv, _)| id == &actor.id && mv == &movement.id)
        {
            used.2 += 1;
        } else {
            current
                .used_moves
                .push((actor.id.clone(), movement.id.clone(), 1));
        }
        current.recent_moves.push(movement.id.clone());
        if current.recent_moves.len() > 8 {
            current.recent_moves.remove(0);
        }
        self.crowd_response(&current.plan.style, quality, novel);
        current.engagement_sum += self.crowd.energy;
        let text = if success {
            format!(
                "{} lands {}. {}{}",
                actor.name,
                movement.name,
                if movement.signature && self.crowd.energy >= 75 {
                    "The signature move brings the crowd to its feet. "
                } else {
                    ""
                },
                if !novel {
                    "The familiar exchange is losing its surprise."
                } else if self.crowd.energy >= 75 {
                    "A roar rolls around the building."
                } else {
                    "The exchange gives the crowd something to follow."
                }
            )
        } else if salvaged {
            format!(
                "{} has to improvise out of {}. {} follows the adjustment and keeps the sequence together.",
                actor.name, movement.name, opponent.name
            )
        } else {
            format!(
                "{} struggles with {}. {}",
                actor.name,
                movement.name,
                if impossible || size_penalty > 5 {
                    "The size and strength mismatch makes the planned lift unconvincing."
                } else if fatigue > 15 {
                    "Fatigue is compromising balance and timing."
                } else {
                    "Their timing breaks down and the audience notices."
                }
            )
        };
        self.emit(
            events,
            if success {
                "move"
            } else if salvaged {
                "improvisation"
            } else {
                "mistake"
            },
            text,
            Some(actor.id.clone()),
            Some(movement.id.clone()),
            if movement.signature {
                3
            } else if !success {
                2
            } else {
                1
            },
        );
        if !success
            && !salvaged
            && movement.risk >= 4
            && roll(random, 100) < (movement.risk * 2 + fatigue / 3 - agent.safety / 3).max(1)
        {
            current.injured[1 - index] = true;
            current.danger += 8;
            current.plan.risk = current.plan.risk.min(2);
            current.reasons.push(format!(
                "An awkward landing forced {} to work around a physical problem.",
                opponent.name
            ));
            self.emit(events,"injury",format!("{} is hurt on the landing. The referee checks in and they simplify the remaining work; the booked result stays in place.",opponent.name),Some(opponent.id.clone()),None,3);
        }
        if self.elapsed > current.plan.duration_seconds * 3 / 4
            && success
            && movement.signature
            && self.elapsed + 25 < current.plan.duration_seconds
            && current.false_finishes < 2
        {
            current.false_finishes += 1;
            self.emit(
                events,
                "nearFall",
                format!(
                    "{} hooks the leg — a late kick-out from {} keeps the match alive!",
                    actor.name, opponent.name
                ),
                Some(actor.id.clone()),
                None,
                3,
            );
        }
        current.next_action =
            self.elapsed + (17 - current.plan.pace * 2 + roll(random, 5)).max(4) as u32;
        self.current = Some(current);
    }

    fn finish_match(&mut self, events: &mut Vec<SimEvent>) {
        let current = self.current.take().unwrap();
        let mut plan = current.plan.clone();
        plan.duration_seconds = self.elapsed;
        let a = self.worker(&plan.worker_a);
        let b = self.worker(&plan.worker_b);
        let agent = self.agent(plan.agent_id);
        let attempts = current.attempts.max(1);
        let mut reasons = current.reasons.clone();
        if current.mistakes > 3 {
            reasons.push(format!("{} exchanges broke down; physical execution could not consistently support the plan.",current.mistakes));
        }
        if current.salvaged > 0 {
            reasons.push(format!(
                "{} improvised recoveries prevented mistakes from derailing the performance.",
                current.salvaged
            ));
        }
        if current.stamina.iter().any(|s| *s < 220) {
            reasons.push("The pace and length left little stamina for the finish.".into());
        }
        if current.chemistry < 45 {
            reasons.push("Limited familiarity made the transitions harder to read.".into());
        }
        if agent.psychology >= 16 {
            reasons.push(format!(
                "{} provided clear structure and a recognisable closing stretch.",
                agent.name
            ));
        }
        if self.crowd.expectation > current.opening_energy + 15 {
            reasons.push("The previous segment raised expectations; this match had to work to regain that energy.".into());
        }
        let protected = plan.protected_worker_id.is_some();
        let performance = Performance {
            execution: bounded((current.successes * 100 + current.salvaged * 55) / attempts),
            psychology: bounded(
                (a.attributes.psychology + b.attributes.psychology) * 2 + agent.psychology
                    - current.mistakes * 2,
            ),
            engagement: bounded(current.engagement_sum / attempts),
            safety: bounded(100 - current.danger - current.mistakes * 2),
            story: bounded(
                55 + current.false_finishes.min(2) * 8 + agent.psychology - current.mistakes,
            ),
            protection: if protected {
                if current.injured.iter().any(|b| *b) {
                    45
                } else {
                    85
                }
            } else {
                60
            },
        };
        let result = match &plan.winner_id {
            Some(id) => format!(
                "{} defeats {} by {}{}.",
                if id == &a.id { &a.name } else { &b.name },
                if id == &a.id { &b.name } else { &a.name },
                match plan.finish {
                    Finish::Pinfall => "pinfall",
                    Finish::Submission => "submission",
                    Finish::CountOut => "count-out",
                    Finish::Disqualification => "disqualification",
                    _ => unreachable!(),
                },
                if plan.clean_finish {
                    ""
                } else {
                    " with a disputed finish"
                }
            ),
            None => {
                if plan.finish == Finish::Draw {
                    "The match reaches its booked time-limit draw.".into()
                } else {
                    "The match ends in the booked no contest.".into()
                }
            }
        };
        if !plan.clean_finish {
            for c in &mut self.crowd.cohorts {
                c.trust = bounded(
                    c.trust
                        - if self.crowd.interference_count > 1 {
                            4
                        } else {
                            1
                        },
                );
            }
        }
        self.crowd_response(&plan.style, performance.execution, true);
        self.emit(
            events,
            "finish",
            result.clone(),
            plan.winner_id.clone(),
            None,
            3,
        );
        if let Some(id) = &plan.protected_worker_id {
            self.emit(events,"aftermath",format!("{} is kept credible in the aftermath; the presentation follows the protection instruction.",self.worker(id).name),Some(id.clone()),None,2);
        }
        let mut changes = Vec::new();
        for (index, id, opponent) in [(0, &a.id, &b), (1, &b.id, &a)] {
            let worker = self.workers.iter_mut().find(|w| &w.id == id).unwrap();
            changes.push(apply_match(
                worker,
                &plan,
                &performance,
                current.stamina[index],
                current.injured[index],
                opponent.attributes.psychology,
                &current.used_moves,
            ));
        }
        let chemistry_change = if performance.safety < 60 {
            -3
        } else if performance.execution > 65 {
            3
        } else {
            1
        };
        if let Some(rel) = self
            .relationships
            .iter_mut()
            .find(|(x, y, _)| (x == &a.id && y == &b.id) || (x == &b.id && y == &a.id))
        {
            rel.2 = bounded(rel.2 + chemistry_change);
        } else {
            self.relationships
                .push((a.id.clone(), b.id.clone(), bounded(50 + chemistry_change)));
        }
        let agent = self
            .agents
            .iter_mut()
            .find(|ag| ag.id == plan.agent_id)
            .unwrap();
        agent.knowledge = bounded(agent.knowledge + 1);
        agent.trust = bounded(agent.trust + if performance.execution > 60 { 1 } else { -1 });
        if reasons.is_empty() {
            reasons.push(
                "The performers kept a familiar plan coherent and gave the finish room to land."
                    .into(),
            );
        }
        self.reports.push(SegmentReport {
            segment_id: self.card[self.index].id,
            title: self.card[self.index].title.clone(),
            winner_id: plan.winner_id,
            result,
            duration_seconds: self.elapsed,
            performance,
            reasons,
            changes,
            chemistry_change,
        });
    }

    fn finish_angle(&mut self, plan: &AnglePlan, events: &mut Vec<SimEvent>) {
        let ability = plan
            .participants
            .iter()
            .map(|id| self.worker(id).attributes.charisma)
            .sum::<i32>()
            / plan.participants.len() as i32;
        let quality = bounded(ability * 4 + 20 - self.crowd.fatigue / 3);
        self.crowd_response("Entertainment", quality, true);
        for cohort in &mut self.crowd.cohorts {
            cohort.fatigue = (cohort.fatigue - 3).max(0);
        }
        let mut changes = Vec::new();
        for id in &plan.participants {
            let worker = self.workers.iter_mut().find(|w| &w.id == id).unwrap();
            let gain = if quality > 65 { 2 } else { -1 };
            worker.condition.confidence = bounded(worker.condition.confidence + gain);
            changes.push(WorkerChange {
                worker_id: id.clone(),
                name: worker.name.clone(),
                fatigue: 0,
                confidence: gain,
                morale: 0,
                momentum: 0,
                popularity: 0,
                development: 0,
                wear: 0,
                injury_days: worker.condition.injury_days,
                note: "Character work changes confidence through the audience response.".into(),
            });
        }
        let result = format!(
            "The {} {}.",
            plan.purpose.to_lowercase(),
            if quality > 65 {
                "connects with the crowd"
            } else {
                "struggles to hold the room"
            }
        );
        self.emit(events, "angleResult", result.clone(), None, None, 2);
        self.reports.push(SegmentReport {
            segment_id: self.card[self.index].id,
            title: self.card[self.index].title.clone(),
            winner_id: None,
            result,
            duration_seconds: self.elapsed,
            performance: Performance {
                execution: quality,
                psychology: quality,
                engagement: self.crowd.energy,
                safety: 100,
                story: quality,
                protection: 70,
            },
            reasons: vec![
                "Microphone ability and the room's accumulated fatigue shaped the response.".into(),
            ],
            changes,
            chemistry_change: 0,
        });
    }

    pub fn advance(&mut self, seconds: u32) -> Vec<SimEvent> {
        let mut random = rng(self.seed.parse().unwrap(), self.show_id as u64);
        random.set_word_pos(self.rng_position.parse().unwrap());
        let mut events = Vec::new();
        for _ in 0..seconds.min(3600) {
            if self.complete {
                break;
            }
            if self.elapsed == 0 {
                self.begin(&mut events);
            }
            self.tick += 1;
            self.elapsed += 1;
            if self.tick.is_multiple_of(60) {
                self.crowd.fatigue = bounded(self.crowd.fatigue + 1);
                for c in &mut self.crowd.cohorts {
                    c.fatigue = bounded(c.fatigue + 1);
                }
            }
            let segment = self.card[self.index].clone();
            match segment.content {
                SegmentPlan::Match(_) => {
                    self.deliver(&mut events);
                    let current = self.current.as_ref().unwrap();
                    let due = current
                        .plan
                        .beats
                        .iter()
                        .enumerate()
                        .find(|(i, b)| {
                            b.at_second == self.elapsed && !current.abandoned.contains(&(*i as u32))
                        })
                        .map(|(_, b)| b.clone());
                    if self.elapsed >= current.plan.duration_seconds {
                        self.finish_match(&mut events);
                        self.next_segment();
                    } else if due.is_some() || self.elapsed >= current.next_action {
                        self.action(due, &mut random, &mut events);
                    }
                }
                SegmentPlan::Angle(plan) => {
                    if self.elapsed.is_multiple_of(30) && self.elapsed < plan.duration_seconds {
                        let worker = self.worker(
                            &plan.participants
                                [(self.elapsed as usize / 30) % plan.participants.len()],
                        );
                        self.emit(
                            &mut events,
                            "promo",
                            format!(
                                "{} {}",
                                worker.name,
                                if worker.attributes.charisma >= 14 {
                                    "holds the room and gives the next line time to land."
                                } else {
                                    "has to work to keep attention on the story."
                                }
                            ),
                            Some(worker.id),
                            None,
                            1,
                        );
                    }
                    if self.elapsed >= plan.duration_seconds {
                        self.finish_angle(&plan, &mut events);
                        self.next_segment();
                    }
                }
            }
        }
        self.rng_position = random.get_word_pos().to_string();
        events
    }

    fn next_segment(&mut self) {
        self.pending.clear();
        self.current = None;
        self.index += 1;
        self.elapsed = 0;
        if self.index >= self.card.len() {
            self.complete = true;
        }
    }

    pub fn view(&self, events: Vec<SimEvent>, report: Option<ShowReport>) -> LiveView {
        let current_match = self.current.as_ref().map(|m| MatchView {
            worker_a: m.plan.worker_a.clone(),
            worker_b: m.plan.worker_b.clone(),
            name_a: self.worker(&m.plan.worker_a).name,
            name_b: self.worker(&m.plan.worker_b).name,
            stamina_a: m.stamina[0] / 10,
            stamina_b: m.stamina[1] / 10,
            second: self.elapsed,
            duration_seconds: m.plan.duration_seconds,
            phase: if self.elapsed < 60 {
                "Opening"
            } else if self.elapsed > m.plan.duration_seconds * 3 / 4 {
                "Closing stretch"
            } else {
                "Working the match"
            }
            .into(),
            pace: m.plan.pace,
            risk: m.plan.risk,
            control: self.worker(&m.control).name,
        });
        LiveView {
            show_id: self.show_id,
            tick: self.tick,
            segment_index: self.index as u32,
            complete: self.complete,
            segment_title: self
                .card
                .get(self.index)
                .map(|s| s.title.clone())
                .unwrap_or_else(|| "Show complete".into()),
            current_match,
            crowd: self.crowd.clone(),
            events,
            pending_instructions: self
                .pending
                .iter()
                .map(|q| {
                    format!(
                        "{:?} · expected in {}s",
                        q.instruction.kind,
                        q.deliver_at.saturating_sub(self.elapsed)
                    )
                })
                .collect(),
            report,
        }
    }
}
