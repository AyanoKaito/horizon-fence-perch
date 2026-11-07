// Synthetic server geometry checks using the production clipping/refusal policy.
use fence_perch::{capped_bounds_at, refusal_after_cap, supports};

#[test]
fn world_cap_tracks_positive_and_negative_block_elevations() {
    for y in [-64.0, -1.0, 0.0, 73.0, 320.0] {
        let shape = [10.375, y, -3.0, 10.625, y + 1.5, -2.0];
        assert_eq!(
            capped_bounds_at(shape, y + 1.0),
            Ok(Some([10.375, y, -3.0, 10.625, y + 1.0, -2.0]))
        );
    }
}

#[test]
fn removed_upper_volume_does_not_refuse_new_body() {
    assert!(!refusal_after_cap(true, Some(false), Some(false)));
    assert!(!refusal_after_cap(true, Some(false), None));
}

#[test]
fn remaining_new_collision_refuses_only_without_old_overlap() {
    assert!(refusal_after_cap(false, Some(true), Some(false)));
    assert!(!refusal_after_cap(true, Some(true), Some(true)));
}

#[test]
fn geometry_errors_preserve_original_refusal() {
    for original in [false, true] {
        assert_eq!(refusal_after_cap(original, None, None), original);
        assert_eq!(refusal_after_cap(original, Some(true), None), original);
    }
}

#[test]
fn nonfinite_world_caps_are_rejected() {
    let shape = [0.0, 0.0, 0.0, 1.0, 1.5, 1.0];
    for cap in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(capped_bounds_at(shape, cap).is_err());
    }
}

#[test]
fn open_gate_and_unrelated_block_keep_original_policy() {
    assert!(!supports("minecraft:oak_fence_gate", Some("true")));
    assert!(!supports("minecraft:stone", None));
    assert!(!supports("other:oak_fence", None));
}
