# Player collision integration

Source check: 2026-11-07, Horizon `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd`.
Research and proposed design only; no runtime compilation or gameplay evidence.

**Capability update:** The inner-expression design below failed planning with horizon-mixins 0.6.0:
`E_UNSUPPORTED_CONTROL_FLOW` because `Hub::collide` contains closures. A no-op cancellable HEAD
probe resolves this boundary, but a replacement collision handler requires separate approval.
See [validation](../../VALIDATION.md). Do not implement the rejected inner hooks as if supported.

## Confirmed routing

- `game/integrated.rs:848-863`: the integrated server registers its owner and connects the local client through an in-memory connection.
- `game.rs:7925-7929`: a client window uses `tick_remote`; the server processes its players as guests.
- `game/remote.rs:6301-6324`: an alive, awake, unmounted local player runs `MotionWorld::controlled_simulation_tick_with_sprint_window`.
- `lan/hub.rs:4070-4123`: the owner bypasses the moved-too-quickly check, not `Hub::collide`. Collision validation therefore matters in single-player too.
- `game/lan_actions.rs:4654-4709,4793-4802`: server-side guest bookkeeping/support and travel also use `MotionWorld`.

Paths above are under `crates/horizon-client/src/`.

## Geometry queries

- `motion.rs:2410`: player movement resolution; preserve entity boxes, world border, step height, context and query budget.
- `motion.rs:1063`: player clearance; edge backoff at :2347 delegates here.
- `motion.rs:2455`: player ground-support lookup; existing prototype adapts this path.
- `lan/hub.rs:4339`: authoritative movement resolution.
- `lan/hub.rs:4356`: old-body obstruction check.
- `lan/hub.rs:4368-4379`: new-body collision scan and old-shape overlap predicate.
- `lan/anticheat.rs:401-410`: optional ground validation has another downward scene probe. Do not disable it; include its geometry in any supported configuration.
- `motion.rs:2264-2281`: auto-jump uses separate original-shape queries. Keep auto-jump disabled in the first manual-jump fixtures; compatibility with this optional behavior remains unverified.

## Proposed narrow integration

Share the existing player-query adapter and clipping operation inside `horizon-client-lib`.
Keep local `MotionWorld` hooks; add call wrappers for server movement and old-body clearance.
Use `binding.registry()` for server state selection; `BlockCollisionRegistry::registry` is public
at `crates/horizon-world/src/block_collision.rs:138`.

Modify only the new-collision refusal predicate inside `Hub::collide`, not its whole body:

1. Preserve the original predicate result for unsupported blocks and on unexpected geometry errors.
2. For an eligible barrier hit, cap the shape in world coordinates at `hit.position.y + 1.0`.
3. If the capped hit does not intersect the new body, it cannot justify refusal.
4. Otherwise retain the original old-body overlap rule using the capped shape.

The original scan is a conservative candidate set: capping removes volume and cannot create a
collision absent from the original geometry. Re-test new-body overlap after clipping; merely
clipping the old-body comparison would falsely reject hits whose remaining volume misses the player.

`crates/horizon-world/src/block_collision_scan.rs:425-460` confirms that scan hits are already
translated to world coordinates. Block-local Y=1.0 must not be used as a world-space height.

Use exact-count native mixin selectors; verify wrappers and local captures in a source fixture
before runtime compilation. Preserve original packet handling, error fallback, exemptions,
warnings, corrections and ground bookkeeping. No global shape mutation, identity-only player
detection, thread-local mode flag or anti-cheat bypass is proposed.

## Remaining proof

The design is proposed, not approved implementation. Exact mixin selectors/captures still need
planning/apply validation. Regression checks must show identical results on unsupported geometry,
correct local/world clipping at different elevations, unchanged animal queries and consistent
movement/clearance/support. The optional ground probe and auto-jump need explicit coverage before
support claims. The same revision is a candidate for the first full-workspace check, not a proven
buildable or release target. Native compilation and isolated gameplay require separate consent.
