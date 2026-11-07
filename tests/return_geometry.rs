#[path = "../mixins/fence_perch/validator.rs"]
mod validator;
// Independent axis-aligned model, not Horizon's solver: no stepping, entities or fluids.
use fence_perch::{capped_bounds_at, supports};
use validator::{adapt, Bounds, Cells, Geometry, Request, Verdict};

#[derive(Clone)]
struct Block {
    name: &'static str,
    open: Option<&'static str>,
    position: [i32; 3],
    bounds: Bounds,
}

struct World {
    blocks: Vec<Block>,
    changed: bool,
    solves: usize,
}

fn overlaps(a: Bounds, b: Bounds) -> bool {
    (0..3).all(|axis| a[axis] < b[axis + 3] && a[axis + 3] > b[axis])
}

fn solve(mut body: Bounds, mut movement: [f64; 3], shapes: &[Bounds]) -> [f64; 3] {
    for axis in [1, 0, 2] {
        for shape in shapes {
            if !(0..3).filter(|&other| other != axis).all(|other|
                body[other] < shape[other + 3] && body[other + 3] > shape[other]) {
                continue;
            }
            if movement[axis] > 0.0 && body[axis + 3] <= shape[axis] {
                movement[axis] = movement[axis].min(shape[axis] - body[axis + 3]);
            } else if movement[axis] < 0.0 && body[axis] >= shape[axis + 3] {
                movement[axis] = movement[axis].max(shape[axis + 3] - body[axis]);
            }
        }
        body[axis] += movement[axis]; body[axis + 3] += movement[axis];
    }
    movement
}

fn new_hit(old: Bounds, new: Bounds, shapes: &[Bounds]) -> bool {
    shapes.iter().any(|&shape| overlaps(new, shape) && !overlaps(old, shape))
}

fn deflate(mut body: Bounds) -> Bounds {
    let e = f64::from(1.0e-5_f32);
    for axis in 0..3 { body[axis] += e; body[axis + 3] -= e; }
    body
}

fn original(request: Request, shapes: &[Bounds]) -> Verdict {
    let result = solve(request.old, request.movement, shapes);
    let x = request.movement[0] - result[0];
    let z = request.movement[2] - result[2];
    let wrong = !request.creative && x * x + z * z > 0.0625;
    let ground = request.movement[1] < 0.0 && result[1] != request.movement[1];
    if wrong && !shapes.iter().any(|&s| overlaps(request.old, s)) {
        (true, true, ground)
    } else {
        (wrong, new_hit(deflate(request.old), deflate(request.new), shapes), ground)
    }
}

impl World {
    fn shapes(&mut self) -> Vec<Bounds> {
        self.blocks.iter().filter_map(|block| {
            if supports(block.name, block.open) {
                let clipped = capped_bounds_at(block.bounds, f64::from(block.position[1]) + 1.0).unwrap();
                self.changed |= clipped != Some(block.bounds);
                clipped
            } else { Some(block.bounds) }
        }).collect()
    }
}

impl Geometry for World {
    fn eligible_nearby(&mut self, cells: Cells) -> Result<bool, ()> {
        Ok(self.blocks.iter().any(|block| supports(block.name, block.open)
            && (0..3).all(|a| block.position[a] >= cells.min[a] && block.position[a] <= cells.max[a])))
    }
    fn resolve(&mut self, request: &Request) -> Result<[f64; 3], ()> {
        self.solves += 1;
        Ok(solve(request.old, request.movement, &self.shapes()))
    }
    fn changed(&self) -> bool { self.changed }
    fn old_clear(&mut self, old: Bounds) -> Result<bool, ()> {
        Ok(!self.shapes().iter().any(|&s| overlaps(old, s)))
    }
    fn new_collision(&mut self, old: Bounds, new: Bounds) -> Result<bool, ()> {
        Ok(new_hit(old, new, &self.shapes()))
    }
}

fn approach(y: f64) -> Request {
    Request { old: [0.775, y + 1.1, 0.2, 1.375, y + 2.9, 0.8],
        new: [0.875, y + 1.02, 0.2, 1.475, y + 2.82, 0.8],
        movement: [0.1, -0.08, 0.0], creative: false, enabled: true }
}

fn barrier(name: &'static str, y: i32) -> Block {
    Block { name, open: Some("false"), position: [1, y, 0],
        bounds: [1.375, f64::from(y), 0.375, 1.625, f64::from(y) + 1.5, 0.625] }
}

#[test]
fn capped_families_remove_only_the_modeled_upper_refusal_at_multiple_heights() {
    for name in ["minecraft:oak_fence", "minecraft:cobblestone_wall", "minecraft:oak_fence_gate"] {
        for y in [-64, 0, 73] {
            let req = approach(f64::from(y));
            let block = barrier(name, y);
            let native = original(req, &[block.bounds]);
            assert!(native.1);
            let mut world = World { blocks: vec![block], changed: false, solves: 0 };
            assert!(!adapt(native, req, &mut world).1);
            assert_eq!(world.solves, 1);
        }
    }
}

#[test]
fn nearby_barrier_does_not_waive_low_ceiling_or_unrelated_wall() {
    for obstruction in [
        [1.375, 2.6, 0.0, 2.375, 3.6, 1.0],
        [1.375, 0.0, 0.0, 2.375, 3.0, 1.0],
    ] {
        let req = approach(0.0);
        let blocks = vec![barrier("minecraft:oak_fence", 0),
            Block { name: "minecraft:stone", open: None, position: [1, 2, 0], bounds: obstruction }];
        let native = original(req, &blocks.iter().map(|b| b.bounds).collect::<Vec<_>>());
        assert!(native.1);
        let mut world = World { blocks, changed: false, solves: 0 };
        assert!(adapt(native, req, &mut world).1);
    }
}

#[test]
fn no_barrier_skips_second_solver_and_preserves_original_geometry_decisions() {
    for creative in [false, true] {
        let mut req = approach(0.0); req.creative = creative;
        let block = barrier("minecraft:stone", 0);
        let native = original(req, &[block.bounds]);
        let mut world = World { blocks: vec![block], changed: false, solves: 0 };
        assert_eq!(adapt(native, req, &mut world), native);
        assert_eq!(world.solves, 0);
    }
}

#[test]
fn movement_into_remaining_fence_volume_stays_blocked() {
    let mut req = approach(0.0);
    for axis in [1, 4] { req.old[axis] -= 1.1; req.new[axis] -= 1.1; }
    req.movement[1] = 0.0;
    let block = barrier("minecraft:oak_fence", 0);
    let native = original(req, &[block.bounds]);
    let mut world = World { blocks: vec![block], changed: false, solves: 0 };
    assert!(adapt(native, req, &mut world).1);
}

#[test]
fn differential_model_matches_direct_capped_geometry() {
    let mut checked = 0;
    for y in [-64, 0, 73] {
        for height in [0.0, 1.02, 1.1, 1.6] {
            for dx in [0.0, 0.1, 0.5] {
                for dy in [-0.2, 0.0, 0.2] {
                    for creative in [false, true] {
                        for wall in [false, true] {
                            let mut req = approach(f64::from(y));
                            req.old[1] = f64::from(y) + height;
                            req.old[4] = req.old[1] + 1.8;
                            req.movement = [dx, dy, 0.0]; req.creative = creative;
                            req.new = req.old;
                            for a in 0..3 { req.new[a] += req.movement[a]; req.new[a + 3] += req.movement[a]; }
                            let mut blocks = vec![barrier("minecraft:oak_fence", y)];
                            if wall {
                                blocks.push(Block { name: "minecraft:stone", open: None, position: [2, y, 0],
                                    bounds: [2.0, f64::from(y), 0.0, 3.0, f64::from(y) + 3.0, 1.0] });
                            }
                            let native = original(req, &blocks.iter().map(|b| b.bounds).collect::<Vec<_>>());
                            let mut world = World { blocks, changed: false, solves: 0 };
                            let expected = original(req, &world.shapes());
                            world.changed = false;
                            assert_eq!(adapt(native, req, &mut world), expected,
                                "y={y} height={height} dx={dx} dy={dy} creative={creative} wall={wall}");
                            checked += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(checked, 432);
}
