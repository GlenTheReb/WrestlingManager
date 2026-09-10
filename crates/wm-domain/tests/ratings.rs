use serde_json::{Value, json};
use wm_domain::ratings::{
    ALL_DISCIPLINES, Discipline, LegacyAttributes, Rating100, WrestlerAttributes,
    WrestlingStyleProfile,
};

fn legacy() -> LegacyAttributes {
    LegacyAttributes {
        strength: 14,
        technical: 14,
        psychology: 14,
        stamina: 14,
        charisma: 14,
        safety: 14,
        professionalism: 14,
        improvisation: 14,
        experience: 14,
    }
}

fn uniform_attributes(value: i32) -> WrestlerAttributes {
    let mut root =
        serde_json::to_value(WrestlerAttributes::from_legacy(&legacy(), "Technical")).unwrap();
    fn replace_numbers(value: &mut Value, replacement: i32) {
        match value {
            Value::Number(_) => *value = Value::from(replacement),
            Value::Object(values) => {
                for value in values.values_mut() {
                    replace_numbers(value, replacement);
                }
            }
            _ => {}
        }
    }
    replace_numbers(&mut root, value);
    serde_json::from_value(root).unwrap()
}

fn uniform_profile(attributes: &WrestlerAttributes, evidence: i32) -> WrestlingStyleProfile {
    let mut profile = WrestlingStyleProfile::from_legacy(&legacy(), "Technical", attributes);
    for item in &mut profile.evidence {
        item.moveset_readiness = Rating100::new(evidence).unwrap();
        item.mastery = Rating100::new(evidence).unwrap();
        item.proven_performance = Rating100::new(evidence).unwrap();
    }
    profile
}

#[test]
fn rating_boundaries_are_strict_in_json() {
    assert_eq!(serde_json::from_str::<Rating100>("0").unwrap().get(), 0);
    assert_eq!(serde_json::from_str::<Rating100>("100").unwrap().get(), 100);
    for invalid in ["-1", "101", "50.5", "null", "\"80\""] {
        assert!(
            serde_json::from_str::<Rating100>(invalid).is_err(),
            "accepted {invalid}"
        );
    }
}

#[test]
fn canonical_shape_rejects_missing_and_unknown_fields() {
    let attributes = uniform_attributes(50);
    let mut missing = serde_json::to_value(&attributes).unwrap();
    missing["movement"]
        .as_object_mut()
        .unwrap()
        .remove("agility");
    assert!(serde_json::from_value::<WrestlerAttributes>(missing).is_err());

    let mut unknown = serde_json::to_value(&attributes).unwrap();
    unknown["physicality"]["explosiveness"] = json!(70);
    assert!(serde_json::from_value::<WrestlerAttributes>(unknown).is_err());
}

#[test]
fn base_groups_are_rounded_means_of_their_sub_stats() {
    let mut value = serde_json::to_value(uniform_attributes(50)).unwrap();
    value["movement"]["acceleration"] = json!(100);
    let attributes: WrestlerAttributes = serde_json::from_value(value).unwrap();
    assert_eq!(attributes.groups().movement.get(), 57);
    assert_eq!(attributes.groups().physicality.get(), 50);
}

#[test]
fn every_discipline_fit_uses_bounded_canonical_evidence() {
    for value in [0, 100] {
        let attributes = uniform_attributes(value);
        let profile = uniform_profile(&attributes, value);
        profile.validate().unwrap();
        for discipline in ALL_DISCIPLINES {
            assert_eq!(profile.discipline_fit(&attributes, discipline), value);
        }
    }
}

#[test]
fn style_profile_requires_exactly_one_evidence_row_per_discipline() {
    let attributes = uniform_attributes(50);
    let mut profile = uniform_profile(&attributes, 50);
    profile.evidence.push(profile.evidence[0].clone());
    assert!(profile.validate().is_err());
}

#[test]
fn secondaries_require_fit_readiness_mastery_and_proximity() {
    let attributes = uniform_attributes(80);
    let mut profile = uniform_profile(&attributes, 50);
    profile.current_primary = Discipline::TechnicalGrappling;
    profile
        .adjust_discipline_evidence(Discipline::TechnicalGrappling, 40, 40, 40)
        .unwrap();
    profile
        .adjust_discipline_evidence(Discipline::SubmissionWrestling, 30, 30, 30)
        .unwrap();
    profile
        .adjust_discipline_evidence(Discipline::LuchaLibre, 30, 30, 30)
        .unwrap();

    let summary = profile.summary(&attributes);
    assert_eq!(
        summary.secondaries,
        vec![Discipline::SubmissionWrestling, Discipline::LuchaLibre]
    );
    assert_eq!(summary.archetype, "Submission Technician");
}

#[test]
fn developing_and_all_rounder_are_exceptional_labels() {
    let developing_attributes = uniform_attributes(20);
    let developing = uniform_profile(&developing_attributes, 20).summary(&developing_attributes);
    assert!(developing.developing);
    assert_eq!(developing.archetype, "Developing Wrestler");
    assert!(developing.secondaries.is_empty());

    let elite_attributes = uniform_attributes(90);
    let elite = uniform_profile(&elite_attributes, 90).summary(&elite_attributes);
    assert!(!elite.developing);
    assert_eq!(elite.archetype, "All-Rounder");
    assert_eq!(elite.overall.get(), 90);

    let capable_attributes = uniform_attributes(60);
    let mut transitioning = uniform_profile(&capable_attributes, 0);
    transitioning
        .adjust_discipline_evidence(Discipline::Striking, 100, 100, 100)
        .unwrap();
    assert!(!transitioning.summary(&capable_attributes).developing);
}

#[test]
fn primary_change_obeys_immediate_and_sustained_thresholds() {
    let attributes = uniform_attributes(80);
    let mut sustained = uniform_profile(&attributes, 60);
    sustained.current_primary = Discipline::TechnicalGrappling;
    sustained
        .adjust_discipline_evidence(Discipline::Striking, 20, 0, 0)
        .unwrap();
    for _ in 0..7 {
        sustained.register_match(&attributes);
        assert_eq!(sustained.current_primary, Discipline::TechnicalGrappling);
    }
    sustained.register_match(&attributes);
    assert_eq!(sustained.current_primary, Discipline::Striking);

    let mut immediate = uniform_profile(&attributes, 60);
    immediate.current_primary = Discipline::TechnicalGrappling;
    immediate
        .adjust_discipline_evidence(Discipline::Striking, 40, 0, 0)
        .unwrap();
    immediate.register_day(&attributes);
    assert_eq!(immediate.current_primary, Discipline::Striking);
}

#[test]
fn training_evidence_changes_are_clamped_and_recalculated() {
    let attributes = uniform_attributes(70);
    let mut profile = uniform_profile(&attributes, 50);
    let before = profile.discipline_fit(&attributes, Discipline::AerialWrestling);
    profile
        .adjust_discipline_evidence(Discipline::AerialWrestling, 80, 80, -80)
        .unwrap();
    let evidence = profile.evidence_for(Discipline::AerialWrestling);
    assert_eq!(evidence.moveset_readiness.get(), 100);
    assert_eq!(evidence.mastery.get(), 100);
    assert_eq!(evidence.proven_performance.get(), 0);
    assert!(profile.discipline_fit(&attributes, Discipline::AerialWrestling) > before);
}
