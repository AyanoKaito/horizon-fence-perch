//! Synthetic 2D movement proof for the production clipping policy.
//! This deliberately does not claim to reproduce Horizon's collision solver.

use fence_perch::{capped_bounds, supports};

fn simulate(name: &str, open: Option<&str>, player_query: bool, jump: bool, ceiling: f64) -> [f64; 3] {
    let original = [1.0, 0.0, 0.0, 1.25, 1.5, 1.0];
    let barrier = if player_query && supports(name, open) {
        capped_bounds(original).unwrap().unwrap()
    } else {
        original
    };
    let mut x = 0.7_f64;
    let mut y = 0.0_f64;
    let mut speed = if jump { f64::from(0.42_f32) } else { 0.0 };
    let mut peak = 0.0_f64;
    for _ in 0..60 {
        let mut dy = speed;
        if dy < 0.0 {
            dy = dy.max(-y);
            if x + 0.3 > barrier[0] && x - 0.3 < barrier[3] && y >= barrier[4] {
                dy = dy.max(barrier[4] - y);
            }
        } else if ceiling.is_finite() {
            dy = dy.min((ceiling - 1.8 - y).max(0.0));
        }
        y += dy;
        peak = peak.max(y);
        if dy != speed {
            speed = 0.0;
        }
        let mut dx = (1.125 - x).clamp(0.0, 0.08);
        if y < barrier[4] && y + 1.8 > barrier[1] && x + 0.3 <= barrier[0] {
            dx = dx.min(barrier[0] - x - 0.3);
        }
        x += dx;
        speed = (speed - 0.08) * f64::from(0.98_f32);
    }
    [x, y, peak]
}

#[test]
fn stationary_player_lands_and_stays_on_each_barrier_family() {
    for (name, open) in [
        ("minecraft:oak_fence", None),
        ("minecraft:cobblestone_wall", None),
        ("minecraft:oak_fence_gate", Some("false")),
    ] {
        let [x, y, peak] = simulate(name, open, true, true, f64::INFINITY);
        assert!((x - 1.125).abs() < 1e-9, "{name}: x={x}");
        assert!((y - 1.0).abs() < 1e-9, "{name}: y={y}");
        assert!((peak - 1.252203).abs() < 1e-6, "no jump bonus: {peak}");
    }
}

#[test]
fn walking_cannot_pass_through_capped_fence() {
    let [x, y, _] = simulate("minecraft:oak_fence", None, true, false, f64::INFINITY);
    assert!((x - 0.7).abs() < 1e-9);
    assert_eq!(y, 0.0);
}

#[test]
fn non_player_query_keeps_original_obstacle() {
    let [x, y, _] = simulate("minecraft:oak_fence", None, false, true, f64::INFINITY);
    assert!((x - 0.7).abs() < 1e-9);
    assert_eq!(y, 0.0);
}

#[test]
fn unrelated_ledge_does_not_receive_barrier_policy() {
    let [x, y, _] = simulate("minecraft:stone", None, true, true, f64::INFINITY);
    assert!((x - 0.7).abs() < 1e-9);
    assert_eq!(y, 0.0);
}

#[test]
fn low_ceiling_still_blocks_entry() {
    let [x, y, peak] = simulate("minecraft:oak_fence", None, true, true, 2.65);
    assert!((x - 0.7).abs() < 1e-9);
    assert_eq!(y, 0.0);
    assert!(peak < 1.0);
}
