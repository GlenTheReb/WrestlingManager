use proptest::prelude::*;
use wm_domain::{CreateGameRequest, SaveId, Seed};

#[test]
fn accepts_boundary_save_ids_and_seeds() {
    assert!(SaveId::parse("a").is_ok());
    assert!(SaveId::parse(format!("a{}", "-".repeat(47))).is_ok());
    assert!(Seed::parse("0").is_ok());
    assert!(Seed::parse(&u64::MAX.to_string()).is_ok());
}

#[test]
fn rejects_non_canonical_inputs() {
    for value in ["", "-save", "UPPER", "with_space", "é", &"a".repeat(49)] {
        assert!(SaveId::parse(value).is_err(), "accepted {value:?}");
    }

    for value in ["", "00", "01", "+1", "-1", " 1", "18446744073709551616"] {
        assert!(Seed::parse(value).is_err(), "accepted {value:?}");
    }
}

#[test]
fn request_validation_returns_typed_values() {
    let spec = CreateGameRequest {
        save_id: "career-1".into(),
        seed: "42".into(),
    }
    .validate()
    .unwrap();

    assert_eq!(spec.save_id.as_str(), "career-1");
    assert_eq!(spec.seed.get(), 42);
}

proptest! {
    #[test]
    fn accepted_save_ids_obey_every_path_safe_invariant(value in ".{0,60}") {
        if let Ok(save_id) = SaveId::parse(value) {
            let bytes = save_id.as_str().as_bytes();
            prop_assert!((1..=48).contains(&bytes.len()));
            prop_assert!(bytes[0].is_ascii_alphanumeric());
            prop_assert!(bytes.iter().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-'));
            prop_assert!(!save_id.as_str().contains(".."));
            prop_assert!(!save_id.as_str().contains('/') && !save_id.as_str().contains('\\'));
        }
    }

    #[test]
    fn every_u64_decimal_string_is_canonical(value in any::<u64>()) {
        let text = value.to_string();
        let parsed = Seed::parse(&text).unwrap();
        prop_assert_eq!(parsed.get(), value);
        prop_assert_eq!(parsed.canonical(), text);
    }
}
