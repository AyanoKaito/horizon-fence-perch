#[path = "../mixins/fence_perch/validator.rs"]
mod validator;
use validator::{adapt, preflight_cells, Bounds, Cells, Geometry, Request, Verdict, PREFLIGHT_BUDGET};

struct Probe {
    nearby: Result<bool, ()>,
    resolved: Result<[f64; 3], ()>,
    changed: bool,
    old_clear: Result<bool, ()>,
    new_hit: Result<bool, ()>,
    calls: Vec<&'static str>,
    scanned: usize,
}

impl Geometry for Probe {
    fn eligible_nearby(&mut self, cells: Cells) -> Result<bool, ()> {
        self.calls.push("nearby"); self.scanned = cells.count; self.nearby
    }
    fn resolve(&mut self, _: &Request) -> Result<[f64; 3], ()> {
        self.calls.push("resolve"); self.resolved
    }
    fn changed(&self) -> bool { self.changed }
    fn old_clear(&mut self, _: Bounds) -> Result<bool, ()> {
        self.calls.push("old"); self.old_clear
    }
    fn new_collision(&mut self, old: Bounds, new: Bounds) -> Result<bool, ()> {
        self.calls.push("new");
        assert!(old[0] > 0.2 && new[0] > 0.2);
        self.new_hit
    }
}

fn request() -> Request {
    Request { old: [0.2, 0.0, 0.2, 0.8, 1.8, 0.8],
        new: [0.28, -0.08, 0.2, 0.88, 1.72, 0.8],
        movement: [0.08, -0.08, 0.0], creative: false, enabled: true }
}

fn probe() -> Probe {
    Probe { nearby: Ok(true), resolved: Ok([0.08, 0.0, 0.0]), changed: true,
        old_clear: Ok(true), new_hit: Ok(false), calls: vec![], scanned: 0 }
}

fn all_verdicts() -> impl Iterator<Item = Verdict> {
    (0..8).map(|n| (n & 1 != 0, n & 2 != 0, n & 4 != 0))
}

#[test]
fn no_barrier_preserves_every_original_verdict_without_extra_solver() {
    for original in all_verdicts() {
        let mut p = probe(); p.nearby = Ok(false);
        assert_eq!(adapt(original, request(), &mut p), original);
        assert_eq!(p.calls, ["nearby"]);
        assert!(p.scanned <= PREFLIGHT_BUDGET);
    }
}

#[test]
fn exempt_player_does_no_additional_queries() {
    let mut req = request(); req.enabled = false;
    for original in all_verdicts() {
        let mut p = probe();
        assert_eq!(adapt(original, req, &mut p), original);
        assert!(p.calls.is_empty());
    }
}

#[test]
fn unchanged_geometry_does_not_override_other_results() {
    for original in all_verdicts() {
        let mut p = probe(); p.changed = false;
        assert_eq!(adapt(original, request(), &mut p), original);
        assert_eq!(p.calls, ["nearby", "resolve"]);
    }
}

#[test]
fn landing_can_correct_original_refusal_and_support() {
    let mut p = probe();
    assert_eq!(adapt((false, true, false), request(), &mut p), (false, false, true));
    assert_eq!(p.calls, ["nearby", "resolve", "new"]);
}

#[test]
fn real_horizontal_obstruction_remains_refused() {
    let mut req = request(); req.movement[0] = 0.5;
    let mut p = probe(); p.resolved = Ok([0.0, 0.0, 0.0]);
    assert_eq!(adapt((true, true, false), req, &mut p), (true, true, true));
    assert_eq!(p.calls, ["nearby", "resolve", "old"]);
}

#[test]
fn creative_skips_only_wrongly_flag_not_new_collision() {
    let mut req = request(); req.creative = true; req.movement[0] = 0.5;
    let mut p = probe(); p.resolved = Ok([0.0, 0.0, 0.0]); p.new_hit = Ok(true);
    assert_eq!(adapt((false, true, false), req, &mut p), (false, true, true));
    assert_eq!(p.calls, ["nearby", "resolve", "new"]);
}

#[test]
fn failures_preserve_all_original_decisions() {
    for original in all_verdicts() {
        for failure in 0..4 {
            let mut req = request(); let mut p = probe();
            match failure {
                0 => p.nearby = Err(()),
                1 => p.resolved = Err(()),
                2 => { req.movement[0] = 0.5; p.resolved = Ok([0.0, 0.0, 0.0]); p.old_clear = Err(()); }
                _ => p.new_hit = Err(()),
            }
            assert_eq!(adapt(original, req, &mut p), original);
        }
    }
}

#[test]
fn malformed_and_excessive_requests_do_not_scan() {
    for mode in 0..4 {
        let mut req = request();
        match mode {
            0 => req.old[0] = f64::NAN,
            1 => req.new[3] = 1000000.0,
            2 => req.old[3] = req.old[0],
            _ => req.movement[0] = f64::INFINITY,
        }
        let mut p = probe();
        assert_eq!(adapt((true, true, false), req, &mut p), (true, true, false));
        assert!(p.calls.is_empty());
    }
}

#[test]
fn narrow_queries_have_bounded_preflight_at_negative_heights() {
    let mut req = request();
    for y in [-64.0, -1.0, 0.0, 256.0] {
        req.old[1] = y; req.old[4] = y + 1.8;
        req.new[1] = y - 0.08; req.new[4] = y + 1.72;
        let cells = preflight_cells(&req).unwrap();
        assert!(cells.count <= 64);
        assert!(f64::from(cells.min[1]) < y);
    }
}

#[test]
fn native_horizontal_threshold_and_ground_direction_match() {
    for creative in [false, true] {
        for horizontal in [0.0, 0.25, 0.250001, 0.5] {
            for vertical in [-0.08, 0.0, 0.42] {
                for old_clear in [false, true] {
                    for new_hit in [false, true] {
                        let mut req = request();
                        req.creative = creative; req.movement = [horizontal, vertical, 0.0];
                        let mut p = probe(); p.resolved = Ok([0.0, 0.0, 0.0]);
                        p.old_clear = Ok(old_clear); p.new_hit = Ok(new_hit);
                        let actual = adapt((false, false, false), req, &mut p);
                        let wrong = !creative && horizontal * horizontal > 0.0625;
                        let expected = (wrong, (wrong && old_clear) || new_hit, vertical < 0.0);
                        assert_eq!(actual, expected);
                    }
                }
            }
        }
    }
}

#[test]
fn nonfinite_solver_result_preserves_original_verdict() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut p = probe(); p.resolved = Ok([value, 0.0, 0.0]);
        assert_eq!(adapt((true, true, false), request(), &mut p), (true, true, false));
        assert_eq!(p.calls, ["nearby", "resolve"]);
    }
}
