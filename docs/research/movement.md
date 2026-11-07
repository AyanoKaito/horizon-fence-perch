# Movement research

Checked 2026-11-06 against Horizon [`f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd`](https://github.com/horizon-rust/horizon/tree/f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd). This is a research snapshot, not a release target.

## Confirmed source interfaces

Paths are relative to that revision.

- `crates/horizon-client/src/game/native_hooks.rs:2792` — `native_movement_input` exposes forward, strafe, jump and sneak before applying local input. Its game receiver is read-only; this is not a direct mutable movement hook.
- `crates/horizon-client/src/game/native_hooks.rs:9399` — `native_block_collision` returns block-local collision boxes with an empty collision context. Context-sensitive clearance needs further investigation.
- `crates/horizon-client/src/game/native_motion.rs:31` — `native_push_player` adds a client-side velocity impulse. A positive vertical impulse clears the grounded flag. This does not prove correct timing for a normal ground jump.
- `crates/horizon-client/src/game/native_hooks.rs:2526` and `:2532` — client tick hooks run before and after client tick work, including ticks outside an active world. A mod must guard its world and pause state.
- `crates/horizon-client/src/game/native_hooks.rs:6602` — `native_player_jump` reports an accepted guest jump on the server. It has no mutable jump-strength argument.
- `crates/horizon-client/src/motion.rs:2084` — `MotionWorld::jump_from_ground` applies ordinary ground-jump logic before emitting `native_living_jump`. That notification has only entity identity and type.

## Candidate approaches

Historical jump-boost candidates; neither was approved. The collision prototype below supersedes these as the selected experiment.

1. A conditional client impulse near an eligible barrier. Check input timing, ordinary jump interaction, repeat prevention and server correction.
2. A targeted native mixin in the ground-jump calculation. Check available world context, player-only scope, movement prediction and revision compatibility.

Do not change global collision shapes or all living-entity jumps to solve a player-only feature.

## Abuse risk

A barrier-proximity check alone is insufficient: added vertical reach can help the player land on unrelated nearby geometry. Direction, barrier height and clearance checks do not prove that a player cannot redirect after takeoff. The approved collision prototype avoids adding jump strength; no design has been validated as exploit-free.

The ground-jump injection is a candidate, not a demonstrated safer or more reliable implementation. Its timing is easier to reason about than an independent tick impulse, but gameplay evidence is still required.

## Unverified

- Exact impulse or jump-strength change needed to land reliably on top.
- Whether existing named hooks alone cover the mechanic without timing artifacts.
- Stationary starts, walking and sprinting approaches, low ceilings and uneven ground.
- Interactions with movement effects, flight, swimming, climbing and mounts.
- Multiplayer prediction and server movement checks.
- Compilation, installation and gameplay on any platform.

Source inspection establishes possible integration points, not a working mod.

## Player-only collision candidate

Investigated after the jump-height abuse review. Approved as a prototype experiment; approval recorded 2026-11-07.

**Approved prototype policy:** for eligible player collision queries only, intersect supported barrier shapes with the block-local region at or below Y=1.0. Preserve horizontal geometry and empty shapes. Keep ordinary jump physics and all non-player collision queries unchanged.

This is a collision-height change, not a jump bonus. It also changes landing height and player clearance; it does not preserve the original full barrier shape for players. Do not enable it only during ascent: restoring the taller shape while the player stands on top could embed the player.

### Source trace

At the research revision:

- `crates/horizon-client/src/motion.rs:2410` — the player's movement resolver builds collision queries using `movement_collisions`.
- `crates/horizon-client/src/motion.rs:1063` — `no_player_collision` is also used for player clearance. Movement and clearance must agree.
- `crates/horizon-client-render/src/collisions.rs:465` — shared `SceneCollisions::context_shape` obtains block geometry from the registry. Changing this globally would affect other subjects.
- `crates/horizon-world/src/block_collision.rs:199` — contextual shape selection falls back to ordinary geometry for blocks without a contextual rule.
- `crates/horizon-client-render/src/collisions.rs:279` — some scene queries use position-only contexts; other contexts reuse `EntityIdentity(0)`. That identity alone is not a safe player discriminator.

A draft player-scoped adapter now covers local movement, clearance and ground support. Its server validation and movement-prediction paths still need a complete trace. No upstream source changes were made.

### Independent model check

An original, private Python experiment used a ground plane, one synthetic barrier and a fixed-height player body. Ordinary launch speed 0.42, gravity 0.08 and float32 vertical drag 0.98 produce a modeled peak of 1.252203 blocks.

Eight checks passed: original 1.5-high barrier blocks the jump; a 1.0-high barrier permits a stationary jump and landing; walking remains blocked; a low ceiling blocks entry; an unchanged two-block wall remains unreachable; capping preserves horizontal bounds; an ineligible subject keeps its shape; an empty shape stays empty.

These are geometry-model checks, not Horizon tests. The model has no automatic stepping, networking, fluids, effects, entity collisions or actual block resources. It does not verify real fence variants, wall posts, gate states or game integration.

### Remaining tradeoffs

- Lower player collision can admit a player into headroom that the original shape would exclude.
- Reaching otherwise inaccessible places by standing on an allowed barrier remains inherent to the feature.
- Avoiding a launch-speed bonus removes that specific trampoline mechanism, not every possible exploit.
- Server and client geometry must agree. Client-only compatibility is not established.
- Keeping the original 1.5-high standing surface would require another approach and a separate review.

### Prototype evidence (2026-11-07)

- Original dependency-free selection/clipping logic: 8 Rust tests passed; synthetic movement model: 5 passed. These do not test Horizon's solver or actual block resources.
- Draft adapter and three cancellable HEAD injections cover local movement, clearance and support; runtime code has not been type-checked against Horizon.
- Minimal source fixture at the pinned revision: mixin plan succeeded with 3 injections and 2 modules. Planning is not transformation or compilation.
- Separate server movement path: `crates/horizon-client/src/lan/hub.rs:4326-4348` constructs `SceneCollisions` and uses `resolve_with_context`; the prototype does not cover it.
- Current upstream was not rechecked during documentation migration. The pin remains a research snapshot, not a release target.

### Server planning follow-up (2026-11-07)

Re-read `crates/horizon-client/src/lan/hub.rs:4303-4380` at the same exact research revision. The guest `collide` path uses not only `resolve_with_context` (:4339), but also `unobstructed(old)` (:4356) and a `BlockCollisionScan` of new bounds (:4368). Capping only movement resolution would leave separate refusal checks using other geometry. Preserve these checks and make their player geometry agree. This does not establish that the local owner follows the guest path; routing remains to be traced. No source changes, compilation or gameplay checks were performed.
