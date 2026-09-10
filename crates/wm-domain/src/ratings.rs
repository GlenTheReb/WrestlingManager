use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use std::collections::BTreeSet;
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, TS)]
#[serde(transparent)]
pub struct Rating100(u8);

impl Rating100 {
    pub fn new(value: i32) -> Result<Self, String> {
        if (0..=100).contains(&value) {
            Ok(Self(value as u8))
        } else {
            Err(format!("rating must be between 0 and 100, got {value}"))
        }
    }

    pub fn get(self) -> i32 {
        i32::from(self.0)
    }

    pub fn adjusted(self, change: i32) -> Self {
        Self((self.get() + change).clamp(0, 100) as u8)
    }
}

impl<'de> Deserialize<'de> for Rating100 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Self::new(value).map_err(D::Error::custom)
    }
}

fn rating(value: i32) -> Rating100 {
    Rating100::new(value.clamp(0, 100)).expect("clamped rating")
}

fn scaled(value: i32, adjustment: i32) -> Rating100 {
    rating(value * 5 + adjustment)
}

fn blend(a: i32, b: i32, adjustment: i32) -> Rating100 {
    rating((a + b) * 5 / 2 + adjustment)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LegacyAttributes {
    pub strength: i32,
    pub technical: i32,
    pub psychology: i32,
    pub stamina: i32,
    pub charisma: i32,
    pub safety: i32,
    pub professionalism: i32,
    pub improvisation: i32,
    pub experience: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MovementAttributes {
    pub acceleration: Rating100,
    pub ring_speed: Rating100,
    pub agility: Rating100,
    pub acrobatics: Rating100,
    pub jumping: Rating100,
    pub flexibility: Rating100,
    pub balance: Rating100,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PhysicalityAttributes {
    pub strength: Rating100,
    pub stamina: Rating100,
    pub toughness: Rating100,
    pub recovery: Rating100,
    pub injury_resistance: Rating100,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RingcraftAttributes {
    pub technical_grappling: Rating100,
    pub chain_wrestling: Rating100,
    pub striking: Rating100,
    pub brawling: Rating100,
    pub submissions: Rating100,
    pub aerial_wrestling: Rating100,
    pub power_offense: Rating100,
    pub hardcore_weapons: Rating100,
    pub countering: Rating100,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PsychologyAttributes {
    pub match_storytelling: Rating100,
    pub pacing: Rating100,
    pub selling: Rating100,
    pub crowd_reading: Rating100,
    pub match_calling: Rating100,
    pub adaptability: Rating100,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FundamentalsAttributes {
    pub timing: Rating100,
    pub consistency: Rating100,
    pub safety: Rating100,
    pub bumping: Rating100,
    pub positioning: Rating100,
    pub cooperation: Rating100,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntertainmentAttributes {
    pub presence: Rating100,
    pub microphone: Rating100,
    pub character_performance: Rating100,
    pub crowd_connection: Rating100,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WrestlerAttributes {
    pub movement: MovementAttributes,
    pub physicality: PhysicalityAttributes,
    pub ringcraft: RingcraftAttributes,
    pub psychology: PsychologyAttributes,
    pub fundamentals: FundamentalsAttributes,
    pub entertainment: EntertainmentAttributes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum Discipline {
    TechnicalGrappling,
    SubmissionWrestling,
    LuchaLibre,
    AerialWrestling,
    Striking,
    PowerWrestling,
    Brawling,
    HardcoreWrestling,
    ShootFighting,
}

pub const ALL_DISCIPLINES: [Discipline; 9] = [
    Discipline::TechnicalGrappling,
    Discipline::SubmissionWrestling,
    Discipline::LuchaLibre,
    Discipline::AerialWrestling,
    Discipline::Striking,
    Discipline::PowerWrestling,
    Discipline::Brawling,
    Discipline::HardcoreWrestling,
    Discipline::ShootFighting,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum Tempo {
    Methodical,
    Balanced,
    FastPaced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum Structure {
    Traditional,
    Escalating,
    SpotDriven,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum Presentation {
    SportRealistic,
    Dramatic,
    Spectacle,
    Comedy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum Contact {
    Light,
    Standard,
    Stiff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum RiskApproach {
    Conservative,
    Balanced,
    Daredevil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum Specialisation {
    ChainWrestling,
    CatchWrestling,
    MatControl,
    CounterWrestling,
    JointLocks,
    Chokes,
    StretchHolds,
    LimbTargeting,
    Ropework,
    LuchaGrappling,
    LuchaSequences,
    MultiPersonFlow,
    Diving,
    Springboards,
    Slingshots,
    AerialCounters,
    Punches,
    Kicks,
    KneesElbows,
    ChopsForearms,
    Lifts,
    Presses,
    ImpactSlams,
    Throws,
    ClinchFighting,
    DirtyFighting,
    ArenaBrawling,
    Weapons,
    TablesLadders,
    EnvironmentalOffense,
    Deathmatch,
    Takedowns,
    GroundControl,
    GroundAndPound,
    MartialArtsIntegration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WrestlingApproach {
    pub tempo: Tempo,
    pub structure: Structure,
    pub presentation: Presentation,
    pub contact: Contact,
    pub risk: RiskApproach,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DisciplineEvidence {
    pub discipline: Discipline,
    pub moveset_readiness: Rating100,
    pub mastery: Rating100,
    pub proven_performance: Rating100,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WrestlingStyleProfile {
    pub evidence: Vec<DisciplineEvidence>,
    pub approach: WrestlingApproach,
    pub specialisations: Vec<Specialisation>,
    pub ring_experience: Rating100,
    pub performance_modifier: i8,
    pub current_primary: Discipline,
    pub pending_primary: Option<Discipline>,
    pub pending_days: u16,
    pub pending_matches: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GroupScores {
    pub movement: Rating100,
    pub physicality: Rating100,
    pub ringcraft: Rating100,
    pub psychology: Rating100,
    pub fundamentals: Rating100,
    pub entertainment: Rating100,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DisciplineFit {
    pub discipline: Discipline,
    pub score: Rating100,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WrestlingSummary {
    pub groups: GroupScores,
    pub discipline_fits: Vec<DisciplineFit>,
    pub primary: Discipline,
    pub secondaries: Vec<Discipline>,
    pub archetype: String,
    pub overall: Rating100,
    pub developing: bool,
}

fn mean(values: &[Rating100]) -> Rating100 {
    rating(
        (values.iter().map(|value| value.get()).sum::<i32>() + values.len() as i32 / 2)
            / values.len() as i32,
    )
}

impl WrestlerAttributes {
    pub fn from_legacy(legacy: &LegacyAttributes, source_style: &str) -> Self {
        let fast = matches!(source_style, "Lucha" | "Strong style");
        let aerial = source_style == "Lucha";
        let power = source_style == "Power";
        let hardcore = source_style == "Hardcore";
        let technical = matches!(source_style, "Technical" | "Sports");
        let entertainer = matches!(source_style, "Entertainment" | "Comedy");
        let speed_bonus = if fast {
            10
        } else if power {
            -15
        } else {
            0
        };
        Self {
            movement: MovementAttributes {
                acceleration: blend(legacy.stamina, legacy.improvisation, speed_bonus),
                ring_speed: blend(legacy.stamina, legacy.technical, speed_bonus),
                agility: blend(
                    legacy.technical,
                    legacy.improvisation,
                    if fast { 10 } else { 0 },
                ),
                acrobatics: blend(
                    legacy.technical,
                    legacy.stamina,
                    if aerial { 18 } else { -12 },
                ),
                jumping: blend(
                    legacy.strength,
                    legacy.stamina,
                    if aerial { 12 } else { -3 },
                ),
                flexibility: blend(
                    legacy.technical,
                    legacy.safety,
                    if technical { 8 } else { 0 },
                ),
                balance: blend(legacy.technical, legacy.safety, 5),
            },
            physicality: PhysicalityAttributes {
                strength: scaled(legacy.strength, 0),
                stamina: scaled(legacy.stamina, 0),
                toughness: blend(
                    legacy.strength,
                    legacy.professionalism,
                    if hardcore { 12 } else { 2 },
                ),
                recovery: blend(legacy.stamina, legacy.professionalism, 0),
                injury_resistance: blend(legacy.safety, legacy.professionalism, 0),
            },
            ringcraft: RingcraftAttributes {
                technical_grappling: scaled(legacy.technical, if technical { 8 } else { 0 }),
                chain_wrestling: blend(
                    legacy.technical,
                    legacy.psychology,
                    if technical { 8 } else { -2 },
                ),
                striking: blend(
                    legacy.technical,
                    legacy.strength,
                    if source_style == "Strong style" {
                        15
                    } else {
                        0
                    },
                ),
                brawling: blend(
                    legacy.strength,
                    legacy.improvisation,
                    if hardcore { 15 } else { 0 },
                ),
                submissions: blend(
                    legacy.technical,
                    legacy.strength,
                    if technical { 6 } else { -5 },
                ),
                aerial_wrestling: blend(
                    legacy.technical,
                    legacy.stamina,
                    if aerial { 20 } else { -15 },
                ),
                power_offense: scaled(legacy.strength, if power { 12 } else { -4 }),
                hardcore_weapons: blend(
                    legacy.safety,
                    legacy.improvisation,
                    if hardcore { 20 } else { -15 },
                ),
                countering: blend(legacy.technical, legacy.improvisation, 4),
            },
            psychology: PsychologyAttributes {
                match_storytelling: scaled(legacy.psychology, 0),
                pacing: blend(legacy.psychology, legacy.professionalism, 0),
                selling: blend(legacy.psychology, legacy.safety, 0),
                crowd_reading: blend(
                    legacy.psychology,
                    legacy.charisma,
                    if entertainer { 8 } else { 0 },
                ),
                match_calling: blend(legacy.psychology, legacy.experience, 0),
                adaptability: scaled(legacy.improvisation, 0),
            },
            fundamentals: FundamentalsAttributes {
                timing: blend(legacy.technical, legacy.psychology, 0),
                consistency: scaled(legacy.professionalism, 0),
                safety: scaled(legacy.safety, 0),
                bumping: blend(legacy.safety, legacy.technical, 0),
                positioning: blend(legacy.psychology, legacy.experience, 0),
                cooperation: scaled(legacy.professionalism, 0),
            },
            entertainment: EntertainmentAttributes {
                presence: scaled(legacy.charisma, if entertainer { 10 } else { 0 }),
                microphone: blend(
                    legacy.charisma,
                    legacy.improvisation,
                    if entertainer { 8 } else { 0 },
                ),
                character_performance: blend(
                    legacy.charisma,
                    legacy.improvisation,
                    if entertainer { 12 } else { 0 },
                ),
                crowd_connection: blend(
                    legacy.charisma,
                    legacy.psychology,
                    if entertainer { 8 } else { 0 },
                ),
            },
        }
    }

    pub fn groups(&self) -> GroupScores {
        GroupScores {
            movement: mean(&[
                self.movement.acceleration,
                self.movement.ring_speed,
                self.movement.agility,
                self.movement.acrobatics,
                self.movement.jumping,
                self.movement.flexibility,
                self.movement.balance,
            ]),
            physicality: mean(&[
                self.physicality.strength,
                self.physicality.stamina,
                self.physicality.toughness,
                self.physicality.recovery,
                self.physicality.injury_resistance,
            ]),
            ringcraft: mean(&[
                self.ringcraft.technical_grappling,
                self.ringcraft.chain_wrestling,
                self.ringcraft.striking,
                self.ringcraft.brawling,
                self.ringcraft.submissions,
                self.ringcraft.aerial_wrestling,
                self.ringcraft.power_offense,
                self.ringcraft.hardcore_weapons,
                self.ringcraft.countering,
            ]),
            psychology: mean(&[
                self.psychology.match_storytelling,
                self.psychology.pacing,
                self.psychology.selling,
                self.psychology.crowd_reading,
                self.psychology.match_calling,
                self.psychology.adaptability,
            ]),
            fundamentals: mean(&[
                self.fundamentals.timing,
                self.fundamentals.consistency,
                self.fundamentals.safety,
                self.fundamentals.bumping,
                self.fundamentals.positioning,
                self.fundamentals.cooperation,
            ]),
            entertainment: mean(&[
                self.entertainment.presence,
                self.entertainment.microphone,
                self.entertainment.character_performance,
                self.entertainment.crowd_connection,
            ]),
        }
    }

    pub fn sim_strength(&self) -> i32 {
        (self.physicality.strength.get() + 2) / 5
    }
    pub fn sim_technical(&self) -> i32 {
        (self.ringcraft.technical_grappling.get() + 2) / 5
    }
    pub fn sim_psychology(&self) -> i32 {
        (self.psychology.match_storytelling.get() + 2) / 5
    }
    pub fn sim_stamina(&self) -> i32 {
        (self.physicality.stamina.get() + 2) / 5
    }
    pub fn sim_charisma(&self) -> i32 {
        (self.entertainment.presence.get() + 2) / 5
    }
    pub fn sim_safety(&self) -> i32 {
        (self.fundamentals.safety.get() + 2) / 5
    }
    pub fn sim_professionalism(&self) -> i32 {
        (self.fundamentals.consistency.get() + 2) / 5
    }
    pub fn sim_improvisation(&self) -> i32 {
        (self.psychology.adaptability.get() + 2) / 5
    }
}

impl Discipline {
    pub fn label(self) -> &'static str {
        match self {
            Self::TechnicalGrappling => "Technical Grappling",
            Self::SubmissionWrestling => "Submission Wrestling",
            Self::LuchaLibre => "Lucha Libre",
            Self::AerialWrestling => "Aerial Wrestling",
            Self::Striking => "Striking",
            Self::PowerWrestling => "Power Wrestling",
            Self::Brawling => "Brawling",
            Self::HardcoreWrestling => "Hardcore Wrestling",
            Self::ShootFighting => "Shoot Fighting",
        }
    }
}

fn weighted(values: &[(Rating100, i32)]) -> i32 {
    (values
        .iter()
        .map(|(value, weight)| value.get() * weight)
        .sum::<i32>()
        + 50)
        / 100
}

pub fn attribute_fit(attributes: &WrestlerAttributes, discipline: Discipline) -> i32 {
    let a = attributes;
    match discipline {
        Discipline::TechnicalGrappling => weighted(&[
            (a.ringcraft.technical_grappling, 25),
            (a.ringcraft.chain_wrestling, 15),
            (a.ringcraft.countering, 15),
            (a.fundamentals.timing, 10),
            (a.movement.balance, 10),
            (a.fundamentals.positioning, 10),
            (a.psychology.adaptability, 5),
            (a.fundamentals.safety, 5),
            (a.physicality.stamina, 5),
        ]),
        Discipline::SubmissionWrestling => weighted(&[
            (a.ringcraft.submissions, 30),
            (a.ringcraft.technical_grappling, 15),
            (a.ringcraft.countering, 10),
            (a.physicality.strength, 10),
            (a.movement.flexibility, 10),
            (a.fundamentals.timing, 10),
            (a.fundamentals.positioning, 5),
            (a.physicality.toughness, 5),
            (a.physicality.stamina, 5),
        ]),
        Discipline::LuchaLibre => weighted(&[
            (a.movement.ring_speed, 12),
            (a.movement.agility, 12),
            (a.ringcraft.chain_wrestling, 12),
            (a.ringcraft.aerial_wrestling, 12),
            (a.movement.balance, 10),
            (a.ringcraft.technical_grappling, 10),
            (a.movement.acrobatics, 8),
            (a.ringcraft.countering, 8),
            (a.fundamentals.timing, 8),
            (a.fundamentals.positioning, 4),
            (a.fundamentals.cooperation, 4),
        ]),
        Discipline::AerialWrestling => weighted(&[
            (a.ringcraft.aerial_wrestling, 24),
            (a.movement.acrobatics, 15),
            (a.movement.jumping, 12),
            (a.movement.balance, 10),
            (a.movement.agility, 10),
            (a.fundamentals.timing, 8),
            (a.fundamentals.positioning, 7),
            (a.fundamentals.safety, 6),
            (a.movement.ring_speed, 4),
            (a.fundamentals.bumping, 4),
        ]),
        Discipline::Striking => weighted(&[
            (a.ringcraft.striking, 30),
            (a.fundamentals.timing, 15),
            (a.movement.balance, 10),
            (a.ringcraft.countering, 10),
            (a.physicality.toughness, 10),
            (a.movement.acceleration, 5),
            (a.physicality.stamina, 5),
            (a.fundamentals.positioning, 5),
            (a.fundamentals.safety, 5),
            (a.psychology.adaptability, 5),
        ]),
        Discipline::PowerWrestling => weighted(&[
            (a.ringcraft.power_offense, 30),
            (a.physicality.strength, 25),
            (a.movement.balance, 10),
            (a.fundamentals.timing, 10),
            (a.fundamentals.safety, 10),
            (a.physicality.stamina, 5),
            (a.fundamentals.positioning, 5),
            (a.fundamentals.consistency, 5),
        ]),
        Discipline::Brawling => weighted(&[
            (a.ringcraft.brawling, 30),
            (a.ringcraft.striking, 15),
            (a.physicality.toughness, 15),
            (a.physicality.stamina, 10),
            (a.fundamentals.timing, 10),
            (a.psychology.selling, 5),
            (a.psychology.adaptability, 5),
            (a.psychology.crowd_reading, 5),
            (a.fundamentals.safety, 5),
        ]),
        Discipline::HardcoreWrestling => weighted(&[
            (a.ringcraft.hardcore_weapons, 25),
            (a.physicality.toughness, 15),
            (a.fundamentals.safety, 15),
            (a.psychology.adaptability, 10),
            (a.ringcraft.brawling, 10),
            (a.psychology.selling, 10),
            (a.fundamentals.timing, 5),
            (a.physicality.recovery, 5),
            (a.psychology.crowd_reading, 5),
        ]),
        Discipline::ShootFighting => weighted(&[
            (a.ringcraft.technical_grappling, 15),
            (a.ringcraft.submissions, 15),
            (a.ringcraft.striking, 15),
            (a.ringcraft.countering, 15),
            (a.movement.balance, 10),
            (a.physicality.stamina, 10),
            (a.physicality.strength, 5),
            (a.physicality.toughness, 5),
            (a.fundamentals.timing, 5),
            (a.fundamentals.positioning, 5),
        ]),
    }
}

impl WrestlingStyleProfile {
    pub fn from_legacy(
        legacy: &LegacyAttributes,
        source_style: &str,
        attributes: &WrestlerAttributes,
    ) -> Self {
        let (primary, secondaries, approach, specialisations) = legacy_style(source_style);
        let proven = attributes.groups().psychology.get();
        let evidence = ALL_DISCIPLINES
            .into_iter()
            .map(|discipline| {
                let fit = attribute_fit(attributes, discipline);
                let rank = if discipline == primary {
                    0
                } else if secondaries.contains(&discipline) {
                    1
                } else {
                    2
                };
                DisciplineEvidence {
                    discipline,
                    moveset_readiness: rating(match rank {
                        0 => 92,
                        1 => 72,
                        _ => (fit - 12).clamp(25, 65),
                    }),
                    mastery: rating(match rank {
                        0 => (legacy.experience * 5 + 10).clamp(60, 98),
                        1 => (legacy.experience * 5).clamp(55, 90),
                        _ => (fit - 15).clamp(20, 65),
                    }),
                    proven_performance: rating((proven + legacy.experience * 5) / 2),
                }
            })
            .collect();
        Self {
            evidence,
            approach,
            specialisations,
            ring_experience: scaled(legacy.experience, 0),
            performance_modifier: 0,
            current_primary: primary,
            pending_primary: None,
            pending_days: 0,
            pending_matches: 0,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(-3..=3).contains(&self.performance_modifier) {
            return Err("performance modifier must be between -3 and 3".into());
        }
        let disciplines = self
            .evidence
            .iter()
            .map(|value| value.discipline)
            .collect::<BTreeSet<_>>();
        if self.evidence.len() != ALL_DISCIPLINES.len()
            || disciplines.len() != ALL_DISCIPLINES.len()
            || ALL_DISCIPLINES.iter().any(|d| !disciplines.contains(d))
        {
            return Err("style evidence must contain each Discipline exactly once".into());
        }
        let specialisations = self
            .specialisations
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if specialisations.len() != self.specialisations.len() {
            return Err("specialisations must be unique".into());
        }
        Ok(())
    }

    pub fn evidence_for(&self, discipline: Discipline) -> &DisciplineEvidence {
        self.evidence
            .iter()
            .find(|value| value.discipline == discipline)
            .expect("validated complete Discipline evidence")
    }

    pub fn adjust_discipline_evidence(
        &mut self,
        discipline: Discipline,
        readiness_change: i32,
        mastery_change: i32,
        performance_change: i32,
    ) -> Result<(), String> {
        let evidence = self
            .evidence
            .iter_mut()
            .find(|value| value.discipline == discipline)
            .ok_or_else(|| format!("missing evidence for {}", discipline.label()))?;
        evidence.moveset_readiness = evidence.moveset_readiness.adjusted(readiness_change);
        evidence.mastery = evidence.mastery.adjusted(mastery_change);
        evidence.proven_performance = evidence.proven_performance.adjusted(performance_change);
        Ok(())
    }

    pub fn discipline_fit(&self, attributes: &WrestlerAttributes, discipline: Discipline) -> i32 {
        let e = self.evidence_for(discipline);
        (attribute_fit(attributes, discipline) * 55
            + e.moveset_readiness.get() * 25
            + e.mastery.get() * 15
            + e.proven_performance.get() * 5
            + 50)
            / 100
    }

    pub fn summary(&self, attributes: &WrestlerAttributes) -> WrestlingSummary {
        let mut fits = ALL_DISCIPLINES
            .into_iter()
            .map(|discipline| (discipline, self.discipline_fit(attributes, discipline)))
            .collect::<Vec<_>>();
        fits.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        let primary_score = fits
            .iter()
            .find(|(d, _)| *d == self.current_primary)
            .map(|(_, score)| *score)
            .unwrap_or(fits[0].1);
        let developing = fits[0].1 < 55;
        let mut secondaries = fits
            .iter()
            .filter_map(|(discipline, score)| {
                let evidence = self.evidence_for(*discipline);
                (*discipline != self.current_primary
                    && *score >= 65
                    && primary_score - *score <= 12
                    && evidence.moveset_readiness.get() >= 60
                    && evidence.mastery.get() >= 60)
                    .then_some(*discipline)
            })
            .take(2)
            .collect::<Vec<_>>();
        let blend = match secondaries.as_slice() {
            [] => primary_score,
            [secondary] => {
                (primary_score * 75 + self.discipline_fit(attributes, *secondary) * 25 + 50) / 100
            }
            [first, second] => {
                (primary_score * 70
                    + self.discipline_fit(attributes, *first) * 20
                    + self.discipline_fit(attributes, *second) * 10
                    + 50)
                    / 100
            }
            _ => unreachable!(),
        };
        let groups = attributes.groups();
        let overall = (blend * 50
            + groups.psychology.get() * 20
            + groups.fundamentals.get() * 20
            + groups.entertainment.get() * 10
            + 50)
            / 100
            + i32::from(self.performance_modifier);
        let all_rounder = fits.iter().take(5).all(|(discipline, score)| {
            let evidence = self.evidence_for(*discipline);
            *score >= 85 && evidence.moveset_readiness.get() >= 80 && evidence.mastery.get() >= 80
        }) && fits[0].1 - fits[4].1 <= 5;
        if developing {
            secondaries.clear();
        }
        WrestlingSummary {
            groups,
            discipline_fits: ALL_DISCIPLINES
                .into_iter()
                .map(|discipline| DisciplineFit {
                    discipline,
                    score: rating(self.discipline_fit(attributes, discipline)),
                })
                .collect(),
            primary: self.current_primary,
            secondaries: secondaries.clone(),
            archetype: if developing {
                "Developing Wrestler".into()
            } else if all_rounder {
                "All-Rounder".into()
            } else {
                archetype_name(
                    self.current_primary,
                    secondaries.first().copied(),
                    self.approach.presentation,
                )
            },
            overall: rating(overall),
            developing,
        }
    }

    pub fn register_match(&mut self, attributes: &WrestlerAttributes) {
        self.ring_experience = self.ring_experience.adjusted(1);
        self.refresh_primary(attributes, false);
        if self.pending_primary.is_some() {
            self.pending_matches = self.pending_matches.saturating_add(1);
            self.refresh_primary(attributes, false);
        }
    }

    pub fn register_day(&mut self, attributes: &WrestlerAttributes) {
        self.refresh_primary(attributes, false);
        if self.pending_primary.is_some() {
            self.pending_days = self.pending_days.saturating_add(1);
            self.refresh_primary(attributes, false);
        }
    }

    fn refresh_primary(&mut self, attributes: &WrestlerAttributes, force: bool) {
        let mut fits = ALL_DISCIPLINES
            .into_iter()
            .map(|d| (d, self.discipline_fit(attributes, d)))
            .collect::<Vec<_>>();
        fits.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let (candidate, candidate_score) = fits[0];
        let current_score = self.discipline_fit(attributes, self.current_primary);
        if candidate == self.current_primary || candidate_score < current_score + 4 {
            self.pending_primary = None;
            self.pending_days = 0;
            self.pending_matches = 0;
            return;
        }
        if self.pending_primary != Some(candidate) {
            self.pending_primary = Some(candidate);
            self.pending_days = 0;
            self.pending_matches = 0;
        }
        if force
            || candidate_score >= current_score + 8
            || self.pending_days >= 90
            || self.pending_matches >= 8
        {
            self.current_primary = candidate;
            self.pending_primary = None;
            self.pending_days = 0;
            self.pending_matches = 0;
        }
    }
}

fn legacy_style(
    source: &str,
) -> (
    Discipline,
    Vec<Discipline>,
    WrestlingApproach,
    Vec<Specialisation>,
) {
    use Discipline::*;
    use Specialisation::*;
    match source {
        "Power" => (
            PowerWrestling,
            vec![Brawling, Striking],
            WrestlingApproach {
                tempo: Tempo::Methodical,
                structure: Structure::Escalating,
                presentation: Presentation::Dramatic,
                contact: Contact::Standard,
                risk: RiskApproach::Balanced,
            },
            vec![Lifts, ImpactSlams],
        ),
        "Lucha" => (
            LuchaLibre,
            vec![AerialWrestling, TechnicalGrappling],
            WrestlingApproach {
                tempo: Tempo::FastPaced,
                structure: Structure::SpotDriven,
                presentation: Presentation::Spectacle,
                contact: Contact::Standard,
                risk: RiskApproach::Daredevil,
            },
            vec![Ropework, LuchaSequences, Diving],
        ),
        "Strong style" => (
            Striking,
            vec![ShootFighting, TechnicalGrappling],
            WrestlingApproach {
                tempo: Tempo::Balanced,
                structure: Structure::Escalating,
                presentation: Presentation::SportRealistic,
                contact: Contact::Stiff,
                risk: RiskApproach::Balanced,
            },
            vec![Kicks, KneesElbows, ChopsForearms],
        ),
        "Sports" => (
            ShootFighting,
            vec![SubmissionWrestling, TechnicalGrappling],
            WrestlingApproach {
                tempo: Tempo::Methodical,
                structure: Structure::Traditional,
                presentation: Presentation::SportRealistic,
                contact: Contact::Standard,
                risk: RiskApproach::Conservative,
            },
            vec![Takedowns, GroundControl, JointLocks],
        ),
        "Hardcore" => (
            HardcoreWrestling,
            vec![Brawling, Striking],
            WrestlingApproach {
                tempo: Tempo::Balanced,
                structure: Structure::Escalating,
                presentation: Presentation::Spectacle,
                contact: Contact::Stiff,
                risk: RiskApproach::Daredevil,
            },
            vec![Weapons, EnvironmentalOffense, ArenaBrawling],
        ),
        "Comedy" => (
            Brawling,
            vec![LuchaLibre, TechnicalGrappling],
            WrestlingApproach {
                tempo: Tempo::Balanced,
                structure: Structure::SpotDriven,
                presentation: Presentation::Comedy,
                contact: Contact::Light,
                risk: RiskApproach::Balanced,
            },
            vec![DirtyFighting, Ropework],
        ),
        "Entertainment" => (
            Brawling,
            vec![TechnicalGrappling, Striking],
            WrestlingApproach {
                tempo: Tempo::Balanced,
                structure: Structure::Traditional,
                presentation: Presentation::Dramatic,
                contact: Contact::Light,
                risk: RiskApproach::Conservative,
            },
            vec![Punches, DirtyFighting],
        ),
        _ => (
            TechnicalGrappling,
            vec![SubmissionWrestling, ShootFighting],
            WrestlingApproach {
                tempo: Tempo::Balanced,
                structure: Structure::Traditional,
                presentation: Presentation::SportRealistic,
                contact: Contact::Standard,
                risk: RiskApproach::Conservative,
            },
            vec![ChainWrestling, MatControl, CounterWrestling],
        ),
    }
}

fn archetype_name(
    primary: Discipline,
    secondary: Option<Discipline>,
    presentation: Presentation,
) -> String {
    use Discipline::*;
    if presentation == Presentation::Comedy {
        return match primary {
            Brawling => "Comedy Brawler",
            LuchaLibre => "Comedy Luchador",
            _ => "Comedy Wrestler",
        }
        .into();
    }
    match (primary, secondary) {
        (TechnicalGrappling, Some(SubmissionWrestling))
        | (SubmissionWrestling, Some(TechnicalGrappling)) => "Submission Technician",
        (TechnicalGrappling, Some(Striking)) | (Striking, Some(TechnicalGrappling)) => {
            "Technical Striker"
        }
        (LuchaLibre, Some(AerialWrestling)) | (AerialWrestling, Some(LuchaLibre)) => {
            "Lucha Aerialist"
        }
        (AerialWrestling, Some(Striking)) | (Striking, Some(AerialWrestling)) => "Aerial Striker",
        (PowerWrestling, Some(Brawling)) | (Brawling, Some(PowerWrestling)) => "Power Brawler",
        (PowerWrestling, Some(Striking)) => "Striking Powerhouse",
        (Striking, Some(ShootFighting)) | (ShootFighting, Some(Striking)) => "Shoot Striker",
        (HardcoreWrestling, Some(Brawling)) | (Brawling, Some(HardcoreWrestling)) => {
            "Hardcore Brawler"
        }
        (TechnicalGrappling, _) => "Technical Wrestler",
        (SubmissionWrestling, _) => "Submission Specialist",
        (LuchaLibre, _) => "Luchador",
        (AerialWrestling, _) => "Aerialist",
        (Striking, _) => "Striker",
        (PowerWrestling, _) => "Powerhouse",
        (Brawling, _) => "Brawler",
        (HardcoreWrestling, _) => "Hardcore Wrestler",
        (ShootFighting, _) => "Shoot Fighter",
    }
    .into()
}
