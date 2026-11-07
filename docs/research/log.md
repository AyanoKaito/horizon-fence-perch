# Research log

## 2026-11-08 — Release candidate on the newest Nightly

**Confirmed:** Repinned 1.0.0 to Nightly 2026-11-07 `2ebf00a0281c592f79191e76fce0e7d522c6f425` without code changes. Dev build, 22 single-player and 34 LAN checks (three runs), package and official check passed. See [validation](../../VALIDATION.md).

## 2026-11-07 — Nightly repin, LAN and package

**Confirmed:** Repinned to Nightly `6560641ba51a452f6bc2407baa4eb1b3be517ca8` without code changes; target functions unchanged. Dev build, 19 single-player and 34 LAN host/guest checks, deterministic package and official package check passed. See [validation](../../VALIDATION.md).

**Not confirmed:** Nightly `2ebf00a` and later, other movement states, auto-jump, other native mods, dedicated server.

## 2026-11-07 — Merged bundle, Dev build and first gameplay

**Confirmed:** Merged the RETURN server prototype into the production bundle with one shared player-scoped provider. Full-workspace apply, AST audit, launcher-equivalent Dev build and 19 isolated single-player checks passed; the same checks without the mod fail on the barriers. See [validation](../../VALIDATION.md).

**Not confirmed:** LAN host/guest, other movement states, auto-jump, other native mods, current Nightly, packaging.

## 2026-11-07 — Implementation route comparison

**Confirmed:** Reviewed native-integration routes and alternative mechanics against scope, server validation, privileges, performance and maintenance. Found an already-installed newer official tool (`4342a0bfbe71d4d14185528d64e496d6022f16a1`) that supports ordinary-closure RETURN targets. No-op RETURN and HEAD source applications passed on the pinned hub fixture; pristine source unchanged. The original inner-expression modifier still fails. Current-source and policy snapshots are pinned in [implementation-options.md](implementation-options.md).

**Recommendation:** Investigate RETURN geometry revalidation first; conditional HEAD remains a fallback, not the only route. Both need real parity/cost proof. No architecture automatically approved.

**Not confirmed:** real handler correctness, compilation, measured performance, native-mod coexistence or gameplay. No new code in the production bundle, tool install/build, game launch or public message.

## 2026-11-07 — Local policy implementation and tool blocker

**Confirmed:** User authorized the proposed fixes and local tests, excluding native builds and game runs. Added explicit-coordinate clipping and a refusal decision helper to the dependency-free core; added 6 regressions. `cargo test --locked --offline`: 19 passed. Native runtime adapter and manifest unchanged.

**Blocked:** The pinned hub capability probe rejected the proposed expression modifier with `E_UNSUPPORTED_CONTROL_FLOW` (closures in the target function). A no-op cancellable HEAD probe planned successfully (1 injection, 0 modules). Replacing the collision calculation at that boundary is a different design and remains unapproved.

**Not done:** shared runtime extraction, new server integration, source apply, Horizon compilation or gameplay. See [validation](../../VALIDATION.md).

## 2026-11-07 — Player routing and narrow integration design

**Confirmed:** At `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd`, the integrated owner connects as a client and still reaches collision validation; the speed-check exemption does not bypass collisions. Mapped local movement/clearance/support and server movement, old-body, new-body and optional ground probes. Scan hits use world coordinates. See [player-collisions.md](player-collisions.md).

**Proposed:** Share the player-scoped adapter, wrap server calls and modify only the new-collision predicate. Preserve original server logic rather than overwrite it or disable corrections.

**Not confirmed:** mixin selector/capture feasibility for the new hooks, runtime compilation, current Nightly compatibility or gameplay. No gameplay code changed, tests rerun, native build or game launch.

## 2026-11-07 — Initial validation planning

**Confirmed:** User approved proceeding on foot over level ground and allowed varied obstacles. Drafted controlled fixtures without creating a world. Rechecked the pinned guest collision function: movement resolution, old-body obstruction and new-block scanning all need consistent player geometry.

**Not confirmed:** local-owner routing, final adapter seam, buildable validation target or runtime correctness. Technical planning stops at the integration research gate; no new code, tests, native builds or game runs.

## 2026-11-07 — Prototype and documentation reconciliation

**Confirmed:**

- User approved trying player-only barrier collision capping at Y=1.0, keeping ordinary jump strength unchanged.
- Existing prototype contains selection/clipping logic, a draft runtime adapter and three HEAD injections.
- Previous run: 8 core tests and 5 synthetic movement tests passed. Minimal-fixture source planning succeeded with 3 injections and 2 modules at `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd`.
- User approved local planning setup and status corrections. No source code changed during reconciliation.

**Not confirmed:** runtime adapter type checking, source transformation, Horizon compilation, client/server geometry agreement or gameplay. No tests or builds rerun in this block. Historical status entries below describe earlier stages, not current implementation state.

## 2026-11-06 — Collision-height alternative

- Traced player movement and clearance queries into shared collision-shape selection at the recorded Horizon revision.
- Found that position-only contexts and reused entity identity values prevent a global identity-based player check from being treated as safe.
- Investigated player-only shape capping at local Y=1.0, with ordinary jump physics unchanged.
- Ran an independent synthetic geometry model: 8 checks passed. This is not Horizon or gameplay validation.
- Recorded changed standing height, headroom and server-consistency risks. The approach remains pending user review.
- No gameplay code, native build or game launch.

## 2026-11-06 — Implementation review

- User authorized continuing toward implementation; native builds and game runs remain separately gated.
- Inspected the player ground-jump path and the shared living-jump calculation at the recorded research revision.
- Proposed a player-specific ground-jump mixin. User raised abuse concerns instead of approving that architecture.
- Recorded the adjacent-ledge risk. Architecture selection and gameplay implementation remain paused for design review.

## 2026-11-06 — Scope decisions

- User approved walls alongside fences and closed fence gates.
- User required stationary starts directly beside the barrier, without a run-up.
- User selected MIT for original code and documentation; added `LICENSE`.
- Updated acceptance targets. No new technical tests were run.

## 2026-11-06 — Scope and initial source check

- Selected Fence Perch as the project name.
- Clarified the mechanic: land and stand on the barrier, not automatically cross it.
- Checked the official Horizon index: 94 projects; no exact Fence Perch name or `fence_perch` ID. Earlier description searches found no dedicated equivalent. Unlisted projects remain possible.
- Modrinth and GitHub repository searches returned no results for the proposed name. This is not a guarantee of uniqueness or trademark clearance.
- Inspected Horizon movement integration points; recorded the exact snapshot in [movement.md](movement.md).
- Created documentation with private Russian review material separated from public English files.

**Not done at that stage:** implementation, dependency installation, native build, game launch, multiplayer checks, remote repository creation or publication.
