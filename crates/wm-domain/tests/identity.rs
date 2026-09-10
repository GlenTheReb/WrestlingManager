use wm_domain::{identity::*, ratings::Rating100, traits::*};

fn assessment(value: i32) -> Assessment {
    Assessment {
        value: Some(Rating100::new(value).unwrap()),
        visibility: IdentityVisibility::Public,
        source: "Fictional test fixture".into(),
    }
}
fn neutral() -> PersonIdentity {
    let mut p = PersonIdentity::default();
    for a in p.personality.values_mut().chain(p.qualities.values_mut()) {
        *a = assessment(50);
    }
    p
}
#[test]
fn personality_distinguishes_effort_conduct_and_deterministic_ties() {
    let mut p = neutral();
    p.qualities.insert(QualityField::WorkEthic, assessment(95));
    p.qualities
        .insert(QualityField::Professionalism, assessment(90));
    assert_eq!(p.describe(None).text, "Hard-working, dependable");
    p.qualities.insert(QualityField::WorkEthic, assessment(20));
    p.qualities
        .insert(QualityField::Professionalism, assessment(95));
    assert_eq!(p.describe(None).text, "Dependable, low training effort");
    let mut p = neutral();
    p.personality
        .insert(PersonalityField::Sociability, assessment(10));
    p.personality
        .insert(PersonalityField::Loyalty, assessment(90));
    assert_eq!(p.describe(None).text, "Reserved, loyal");
    assert_eq!(p.describe(None), p.describe(None));
}
#[test]
fn unknown_hidden_and_partial_are_not_average_or_secret_hints() {
    assert_eq!(
        PersonIdentity::default().describe(None).text,
        "Personality not yet established"
    );
    assert_eq!(neutral().describe(None).text, "No pronounced tendencies");
    let mut p = neutral();
    let mut secret = assessment(0);
    secret.visibility = IdentityVisibility::Hidden;
    secret.source = "Secret evidence".into();
    p.personality.insert(PersonalityField::Integrity, secret);
    let public = p.for_viewer(None);
    assert!(
        public.personality[&PersonalityField::Integrity]
            .value
            .is_none()
    );
    assert!(
        !serde_json::to_string(&public)
            .unwrap()
            .contains("Secret evidence")
    );
    assert!(p.describe(None).partial);
    assert_eq!(p.describe(None).text, "Personality not yet established");
    p.personality
        .insert(PersonalityField::Loyalty, assessment(90));
    assert_eq!(p.describe(None).text, "Loyal");
}
#[test]
fn company_assessments_do_not_leak_to_other_employers() {
    let mut p = neutral();
    let mut private = assessment(100);
    private.visibility = IdentityVisibility::Company("a".into());
    p.personality.insert(PersonalityField::Ego, private);
    assert_eq!(p.describe(Some("a")).text, "Status-conscious");
    assert_eq!(
        p.describe(Some("b")).text,
        "Personality not yet established"
    );
}
#[test]
fn naming_boundaries_and_invalid_identity_inputs() {
    let mut p = neutral();
    for (value, expected) in [
        (69, "No pronounced tendencies"),
        (70, "Ambitious"),
        (30, "Content with current position"),
        (31, "No pronounced tendencies"),
    ] {
        p.personality
            .insert(PersonalityField::Ambition, assessment(value));
        assert_eq!(p.describe(None).text, expected);
    }
    assert!(serde_json::from_str::<Rating100>("101").is_err());
    assert!(serde_json::from_str::<Rating100>("-1").is_err());
    p.motivations = Some(Motivations {
        primary: Motivation::Legacy,
        secondary: vec![Motivation::Legacy],
    });
    assert!(p.validate().is_err());
    p.motivations = None;
    p.hobbies = vec![
        Interest {
            hobby: Hobby::Music,
            involvement: Involvement::Casual
        };
        2
    ];
    assert!(p.validate().is_err());
    p.hobbies.clear();
    p.languages = vec![SpokenLanguage {
        name: "English".into(),
        native: true,
        proficiency: Some(LanguageLevel::Basic),
    }];
    assert!(p.validate().is_err());
    p.languages.clear();
    p.personality.remove(&PersonalityField::Ego);
    assert!(p.validate().is_err());
}
#[test]
fn biography_modes_preserve_authored_text_and_only_render_facts() {
    let mut p = neutral();
    p.biography.mode = BiographyMode::Authored;
    p.biography.authored_text = "An authored introduction.".into();
    assert_eq!(
        p.biography_text("Mara", "Canada", "School", "Background", 2),
        "An authored introduction."
    );
    p.biography.mode = BiographyMode::Hybrid;
    let text = p.biography_text("Mara", "Canada", "School", "Background", 2);
    assert!(text.starts_with("An authored introduction."));
    assert!(text.contains("2 matches recorded"));
    assert!(!text.contains("Birthplace"));
    assert!(!text.contains("Entered wrestling"));
    p.biography.mode = BiographyMode::Organic;
    assert!(
        !p.biography_text("Mara", "Canada", "School", "Background", 0)
            .contains("authored introduction")
    );
}

fn event(id: &str, kind: IdentityEventKind) -> IdentityEvent {
    IdentityEvent {
        id: id.into(),
        incident_id: format!("incident-{id}"),
        person_id: "worker-001".into(),
        kind,
        occurred_on: "2026-01-01".into(),
        recorded_on: "2026-01-01".into(),
        company_id: None,
        visibility: IdentityVisibility::Public,
        outcome: EvidenceOutcome::Confirmed,
        source: "Fictional verified fixture".into(),
        supersedes: None,
        recognition: None,
        service_days: None,
    }
}
fn record(l: &mut TraitLedger, e: IdentityEvent) {
    l.record(e, "worker-001", "2026-01-01").unwrap();
}
fn state(l: &TraitLedger, t: ExceptionalTrait) -> TraitStatus {
    l.states
        .iter()
        .find(|s| s.trait_id == t)
        .map(|s| s.status)
        .unwrap_or(TraitStatus::Inactive)
}
#[test]
fn traits_count_incidents_not_delivery_and_record_escalation() {
    let mut l = TraitLedger::default();
    let e = event("a", IdentityEventKind::MissedMediaBooking);
    record(&mut l, e.clone());
    let before = l.clone();
    assert!(!l.record(e.clone(), "worker-001", "2026-01-01").unwrap());
    assert_eq!(l, before);
    let mut conflict = e.clone();
    conflict.source = "different payload".into();
    assert!(l.record(conflict, "worker-001", "2026-01-01").is_err());
    for id in ["b", "c"] {
        record(&mut l, event(id, IdentityEventKind::MissedMediaBooking));
    }
    assert_eq!(
        state(&l, ExceptionalTrait::MediaLiability),
        TraitStatus::Active
    );
    for id in ["d", "e"] {
        record(&mut l, event(id, IdentityEventKind::ConfidentialityBreach));
    }
    assert_eq!(
        state(&l, ExceptionalTrait::MediaLiability),
        TraitStatus::Elevated
    );
    assert_eq!(l.history.len(), 2);
    let mut duplicate = event("new-report", IdentityEventKind::MissedMediaBooking);
    duplicate.incident_id = e.incident_id;
    assert!(l.record(duplicate, "worker-001", "2026-01-01").is_err());
}
#[test]
fn criticism_and_private_evidence_cannot_create_public_media_risk() {
    let mut l = TraitLedger::default();
    for i in 0..5 {
        record(
            &mut l,
            event(&i.to_string(), IdentityEventKind::PublicComment),
        );
    }
    for i in 5..10 {
        let mut e = event(&i.to_string(), IdentityEventKind::MissedMediaBooking);
        e.company_id = Some("uwf".into());
        e.visibility = IdentityVisibility::Company("uwf".into());
        record(&mut l, e);
    }
    assert_eq!(
        state(&l, ExceptionalTrait::MediaLiability),
        TraitStatus::Inactive
    );
    let mut private = TraitLedger::default();
    for id in ["a", "b"] {
        let mut e = event(id, IdentityEventKind::PolicyBreach);
        e.company_id = Some("uwf".into());
        e.visibility = IdentityVisibility::Company("uwf".into());
        record(&mut private, e);
    }
    assert!(private.view(None).states.is_empty());
    assert!(!private.view(None).has_evidence);
    assert_eq!(private.view(Some("uwf")).states.len(), 1);
    assert!(private.view(Some("other")).states.is_empty());
}
#[test]
fn corrections_and_expiry_remove_risk_without_rehabilitation_claims() {
    let mut l = TraitLedger::default();
    for id in ["a", "b", "c"] {
        record(&mut l, event(id, IdentityEventKind::MissedMediaBooking));
    }
    let mut correction = event("a", IdentityEventKind::MissedMediaBooking);
    correction.id = "a-corrected".into();
    correction.supersedes = Some("a".into());
    correction.outcome = EvidenceOutcome::Retracted;
    record(&mut l, correction);
    assert_eq!(
        state(&l, ExceptionalTrait::MediaLiability),
        TraitStatus::Inactive
    );
    assert!(l.history.last().unwrap().reason.contains("correction"));
    let mut l = TraitLedger::default();
    for id in ["a", "b", "c"] {
        record(&mut l, event(id, IdentityEventKind::MissedMediaBooking));
    }
    let d = chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    l.evaluate("worker-001", &(d + chrono::Days::new(539)).to_string())
        .unwrap();
    assert_eq!(
        state(&l, ExceptionalTrait::MediaLiability),
        TraitStatus::Active
    );
    l.evaluate("worker-001", &(d + chrono::Days::new(540)).to_string())
        .unwrap();
    assert_eq!(
        state(&l, ExceptionalTrait::MediaLiability),
        TraitStatus::Inactive
    );
    let count = l.history.len();
    l.evaluate("worker-001", &(d + chrono::Days::new(540)).to_string())
        .unwrap();
    assert_eq!(l.history.len(), count);
}
#[test]
fn malformed_evidence_and_rule_versions_fail_without_partial_mutation() {
    let mut l = TraitLedger::default();
    let before = l.clone();
    let mut e = event("a", IdentityEventKind::MissedMediaBooking);
    e.occurred_on = "2026-02-30".into();
    assert!(l.record(e, "worker-001", "2026-01-01").is_err());
    assert_eq!(l, before);
    let mut e = event("a", IdentityEventKind::PolicyBreach);
    assert!(l.record(e.clone(), "worker-001", "2026-01-01").is_err());
    e.company_id = Some("uwf".into());
    assert!(l.record(e, "wrong-person", "2026-01-01").is_err());
    l.rule_version = 99;
    assert!(l.evaluate("worker-001", "2026-01-01").is_err());
}
#[test]
fn sponsor_and_company_icon_have_different_lifecycles() {
    let mut l = TraitLedger::default();
    for id in ["a", "b", "c"] {
        let mut e = event(id, IdentityEventKind::CampaignCompleted);
        e.outcome = EvidenceOutcome::Positive;
        record(&mut l, e);
    }
    assert_eq!(
        state(&l, ExceptionalTrait::SponsorFriendly),
        TraitStatus::Active
    );
    for id in ["d", "e", "f"] {
        let mut e = event(id, IdentityEventKind::CompanyMilestone);
        e.company_id = Some("a".into());
        record(&mut l, e);
    }
    let mut standing = event("standing", IdentityEventKind::CompanyStanding);
    standing.company_id = Some("a".into());
    standing.recognition = Some(Rating100::new(80).unwrap());
    standing.service_days = Some(1825);
    record(&mut l, standing);
    assert_eq!(
        state(&l, ExceptionalTrait::CompanyIcon),
        TraitStatus::Historical
    );
    l.evaluate("worker-001", "2030-01-01").unwrap();
    assert_eq!(
        state(&l, ExceptionalTrait::SponsorFriendly),
        TraitStatus::Inactive
    );
    assert_eq!(
        state(&l, ExceptionalTrait::CompanyIcon),
        TraitStatus::Historical
    );
    assert!(
        l.states
            .iter()
            .filter(|s| s.trait_id == ExceptionalTrait::CompanyIcon)
            .all(|s| s.company_id.as_deref() == Some("a"))
    );
}
#[test]
fn celebrity_retention_threshold_and_replay_survive_serialization() {
    let mut l = TraitLedger::default();
    for id in ["a", "b"] {
        let mut e = event(id, IdentityEventKind::ExternalProject);
        e.outcome = EvidenceOutcome::Positive;
        record(&mut l, e);
    }
    let mut recognition = event("recognition", IdentityEventKind::MainstreamRecognition);
    recognition.recognition = Some(Rating100::new(70).unwrap());
    record(&mut l, recognition);
    assert_eq!(
        state(&l, ExceptionalTrait::CrossoverCelebrity),
        TraitStatus::Active
    );
    let mut next = event("z-recognition", IdentityEventKind::MainstreamRecognition);
    next.recognition = Some(Rating100::new(60).unwrap());
    record(&mut l, next);
    assert_eq!(
        state(&l, ExceptionalTrait::CrossoverCelebrity),
        TraitStatus::Active
    );
    let restored: TraitLedger = serde_json::from_str(&serde_json::to_string(&l).unwrap()).unwrap();
    assert_eq!(restored, l);
    let mut replay = TraitLedger::default();
    for e in &l.events {
        record(&mut replay, e.clone());
    }
    assert_eq!(replay, l);
    l.evaluate("worker-001", "2030-01-01").unwrap();
    assert_eq!(
        state(&l, ExceptionalTrait::CrossoverCelebrity),
        TraitStatus::Historical
    );
    let mut correction = event("a", IdentityEventKind::ExternalProject);
    correction.id = "a-retracted".into();
    correction.supersedes = Some("a".into());
    correction.recorded_on = "2030-01-01".into();
    correction.outcome = EvidenceOutcome::Retracted;
    l.record(correction, "worker-001", "2030-01-01").unwrap();
    assert_eq!(
        state(&l, ExceptionalTrait::CrossoverCelebrity),
        TraitStatus::Inactive
    );
}
