use crate::{rng, roll};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use wm_domain::game::*;
use wm_domain::ratings::{LegacyAttributes, WrestlerAttributes, WrestlingStyleProfile};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Background {
    pub nationality: String,
    pub language: String,
    pub school: String,
    pub style: String,
    pub history: String,
    pub first_names: Vec<String>,
    pub last_names: Vec<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContentPack {
    pub format_version: u32,
    pub name: String,
    pub backgrounds: Vec<Background>,
    pub moves: Vec<Move>,
}

pub fn parse_pack(json: &str) -> Result<ContentPack, String> {
    let pack: ContentPack =
        serde_json::from_str(json).map_err(|e| format!("Invalid content pack: {e}"))?;
    if pack.format_version != 1
        || pack.name.trim().is_empty()
        || pack.backgrounds.is_empty()
        || pack.backgrounds.len() > 100
    {
        return Err("Unsupported or empty content pack.".into());
    }
    let mut ids = BTreeSet::new();
    for m in &pack.moves {
        if m.id.is_empty()
            || !ids.insert(&m.id)
            || m.name.is_empty()
            || !(1..=20).contains(&m.difficulty)
            || !(1..=5).contains(&m.risk)
            || !(1..=20).contains(&m.min_strength)
            || !(1..=10).contains(&m.stamina_cost)
            || !(0..=100).contains(&m.proficiency)
        {
            return Err("A move has duplicate identity or invalid capability/risk values.".into());
        }
    }
    for b in &pack.backgrounds {
        if b.first_names.is_empty()
            || b.last_names.is_empty()
            || b.style.is_empty()
            || !pack.moves.iter().any(|m| m.style == b.style && m.signature)
            || b.first_names
                .iter()
                .chain(&b.last_names)
                .any(|n| n.trim().is_empty() || n.len() > 80)
        {
            return Err("Each background needs names and a style with a signature move.".into());
        }
    }
    if !pack.moves.iter().any(|m| m.style == "Basic") {
        return Err("A basic move repertoire is required.".into());
    }
    Ok(pack)
}

pub fn base_pack() -> ContentPack {
    parse_pack(include_str!("../../../content/base/world.json"))
        .expect("shipped content is validated in tests")
}

fn roman_ordinal(mut value: usize) -> String {
    let mut result = String::new();
    for (number, numeral) in [(10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")] {
        while value >= number {
            result.push_str(numeral);
            value -= number;
        }
    }
    result
}

pub fn generate_world(seed: u64, pack: &ContentPack) -> Vec<Worker> {
    let mut random = rng(seed, 0);
    let mut name_counts = BTreeMap::new();
    (0..40)
        .map(|i| {
            let b = &pack.backgrounds[i % pack.backgrounds.len()];
            let first = &b.first_names[roll(&mut random, b.first_names.len() as u32) as usize];
            let last = &b.last_names[roll(&mut random, b.last_names.len() as u32) as usize];
            let base_name = format!("{first} {last}");
            let occurrence = name_counts.entry(base_name.clone()).or_insert(0);
            *occurrence += 1;
            let name = if *occurrence == 1 {
                base_name
            } else {
                format!("{base_name} {}", roman_ordinal(*occurrence))
            };
            let age = match i {
                0 => 19,
                1 => 43,
                _ => 18 + roll(&mut random, 27),
            };
            let strength = if b.style == "Power" {
                15 + roll(&mut random, 5)
            } else {
                6 + roll(&mut random, 9)
            };
            let legacy = LegacyAttributes {
                strength,
                technical: (8 + roll(&mut random, 7) + i32::from(b.style == "Technical") * 3)
                    .min(20),
                psychology: (6 + age / 5 + roll(&mut random, 5)).min(20),
                stamina: (17 - age / 10 + roll(&mut random, 4)).min(20),
                charisma: 7 + roll(&mut random, 11),
                safety: 10 + roll(&mut random, 9),
                professionalism: 9 + roll(&mut random, 11),
                improvisation: 6 + roll(&mut random, 12),
                experience: (age - 16).clamp(1, 20),
            };
            let attributes = WrestlerAttributes::from_legacy(&legacy, &b.style);
            let wrestling_style =
                WrestlingStyleProfile::from_legacy(&legacy, &b.style, &attributes);
            let moves = pack
                .moves
                .iter()
                .filter(|m| m.style == "Basic" || m.style == b.style)
                .map(|m| {
                    let mut mv = m.clone();
                    mv.proficiency = (40 + age + roll(&mut random, 15)).min(96);
                    mv
                })
                .collect();
            Worker {
                id: format!("worker-{:03}", i + 1),
                name,
                age,
                nationality: b.nationality.clone(),
                language: b.language.clone(),
                school: b.school.clone(),
                background: b.history.clone(),
                style: b.style.clone(),
                personality: ["Methodical", "Ambitious", "Generous", "Impulsive"]
                    [roll(&mut random, 4) as usize]
                    .into(),
                ambition: if age < 25 {
                    "Earn a lasting place on the main card."
                } else if age > 38 {
                    "Leave a legacy and help the next generation."
                } else {
                    "Become the performer the company builds around."
                }
                .into(),
                weight_kg: if b.style == "Power" {
                    108 + roll(&mut random, 28)
                } else {
                    74 + roll(&mut random, 28)
                },
                attributes,
                wrestling_style,
                condition: Condition {
                    fatigue: roll(&mut random, 12),
                    confidence: 50 + roll(&mut random, 26),
                    morale: 60 + roll(&mut random, 21),
                    momentum: 35 + roll(&mut random, 26),
                    popularity: 20 + roll(&mut random, 46),
                    wear: (age - 18) / 2,
                    injury_days: 0,
                    development: 0,
                    matches: 0,
                },
                moves,
                appearance_fee: 18_000 + (age * 400),
            }
        })
        .collect()
}

pub fn initial_agents() -> Vec<RoadAgent> {
    vec![
        RoadAgent {
            id: 1,
            name: "Marion Locke".into(),
            psychology: 18,
            communication: 17,
            experience: 19,
            safety: 18,
            philosophy: "Build the stakes; give the finish room to breathe.".into(),
            knowledge: 55,
            trust: 65,
        },
        RoadAgent {
            id: 2,
            name: "Theo Calder".into(),
            psychology: 13,
            communication: 14,
            experience: 12,
            safety: 14,
            philosophy: "Energy, movement and a strong closing stretch.".into(),
            knowledge: 38,
            trust: 55,
        },
        RoadAgent {
            id: 3,
            name: "Kieran Pike".into(),
            psychology: 9,
            communication: 10,
            experience: 8,
            safety: 11,
            philosophy: "Give the workers freedom to find the moment.".into(),
            knowledge: 24,
            trust: 50,
        },
    ]
}
