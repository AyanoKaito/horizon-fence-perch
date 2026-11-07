# Development plan

Draft. Approval of a document does not authorize later builds or publication.

## Steps

- [x] Record core behavior, MIT license and approved player-only collision-cap prototype policy.
- [x] Implement standalone clipping logic, synthetic tests and a draft player-scoped adapter.
- [x] Review initial validation baseline: on-foot survival, vanilla barriers, level ground and varied fixtures; broader release scope remains open.
- [x] Trace initial player collision routes and propose narrow client/server integration.
- [x] Review integration options; first candidate: postprocess the server collision result (RETURN), sharing one player-scoped provider with client movement.
- [x] Full-workspace source transformation at the research revision (4 injections, 5 modules).
- [x] AST preservation audit and type-check of the transformed workspace.
- [x] Isolated Dev build and single-player gameplay checks.
- [x] LAN host and guest gameplay checks.
- [x] Repin to Nightly 2026-11-07; deterministic package; official package check.
- [x] Release decisions: 1.0.0, dedicated signing key, private repository until release.
- [x] Repin to the newest Nightly (2026-11-07, `2ebf00a`) and full revalidation.
- [ ] Media, signature, GitHub release, registry pull request.
- [ ] Record supported revisions and tested environments; prepare packaging after review.

## Review gates

- Current block updates documentation and draft technical planning only. Resolve the integration research gate and review the design before gameplay-code changes.
- Compare proposed integration approaches without assuming client-only server compatibility.
- Native builds need explicit consent. Never modify a normal installation or real saves.
- Remote creation, commits, pushes, releases and public messages require approval.

## Validation targets

- Land on and remain atop fences and closed gates; descend to both sides.
- Include walls. A stationary start directly beside every supported barrier must work without a run-up.
- Gate state and animal movement remain unchanged.
- Ordinary jumps away from supported barriers remain unchanged.
- Check stationary, walking and sprinting starts, held jump, repeated attempts and barrier corners.
- Check low ceilings, uneven ground and unsupported movement states.
- Test multiplayer roles separately before claiming support.

Report source checks, unit tests, compilation and gameplay separately. Record skipped checks and failures. A successful build is not gameplay evidence.

## Existing evidence (2026-11-07)

Previous prototype run: 8 core tests and 5 synthetic movement tests passed with `cargo test --offline`. A minimal source fixture produced a plan with 3 injections and 2 modules at research revision `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd`. This does not type-check the runtime adapter. Source transformation, Horizon compilation, server consistency and gameplay remain unverified. No tests or builds were rerun during documentation reconciliation.

Initial obstacle coverage: straight fences, walls and closed gates; both gate axes; connected sections, corners and junctions; low ceilings; nearby unrelated ledges; unchanged open gates and animal controls. These are planned fixtures, not completed tests.
