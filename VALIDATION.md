# Validation

2026-11-08. Release pin: Nightly 2026-11-07 `2ebf00a0281c592f79191e76fce0e7d522c6f425`; earlier pins `6560641ba51a452f6bc2407baa4eb1b3be517ca8`, `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd`.

## 1.0.0 Release Candidate on Nightly 2026-11-07 · 2ebf00a

Target: Minecraft 26.3 on Horizon Nightly `2026-11-07` (`2ebf00a0281c592f79191e76fce0e7d522c6f425`),
the newest Nightly at release time; horizon-mixins `4342a0bfbe71d4d14185528d64e496d6022f16a1`;
Rust 1.99.1. Checked on 2026-11-07 and 2026-11-08. Mixin target functions and the server call
site are unchanged from the previous pin.

| Check | Result |
|---|---|
| Standalone Rust tests | Passed: 35 |
| Mixin apply, full workspace | Passed: 4 injections, 5 modules |
| AST preservation audit | Passed: 3 files; 5 modules, 3 HEAD hooks, 7 RETURN exits |
| Native Horizon build | Passed: launcher-equivalent Dev build, 15.1 min |
| Single-player gameplay | Passed: 22 checks (19 behaviour checks plus 3 fixture checks); no "moved wrongly" warnings |
| Same checks on the unmodified Nightly client | 11 barrier checks fail as expected; the other 11 pass |
| LAN host and guest | Passed: 34 checks (17 per actor) in three consecutive runs; no "moved wrongly" warnings |
| Manual test polygon | Reopened with this build; perch confirmed |
| Package | `fence_perch-1.0.0.hnmod`, 11 entries, 9,395 bytes, SHA-256 `d4b3cef31bc299deb5ea1910ed25059ec5a48bd4eff7af36262c216d278bb266`; a second build was byte-identical; bundle files byte-identical to the committed sources. One bundle file had Windows line endings during the build; after normalising to LF, a fresh apply produced byte-identical generated Horizon files |
| Official `cargo horizon-mod check` (built at the same Nightly) | Passed: `native-mod`; with the project key and signature, `signed: true` |
| Minisign signature | The author signed the unchanged package; `minisign -V` verifies it with key `61243FA14D03CB43` |
| Dedicated server | Not claimed |

Harness note: with a 4-tick forward input, two of four LAN runs had one guest closed-gate case
overshoot the 0.25-block gate top and land on the far side (no server correction). The LAN harness
now uses a 2-tick forward input; three consecutive runs then passed. Single-player uses 4 ticks.

Game runs used the installed Nightly's `horizon-ctl` and prepared 26.3 resources read-only, with
isolated homes and test worlds. Auto-jump was disabled. Not covered: auto-jump, sprint-jumps,
swimming, mounted movement, other native mods, joining an unmodified server.

## 0.1.0 Candidate on Nightly 2026-11-07 (2026-11-07)

Horizon Nightly `2026-11-07` (`6560641ba51a452f6bc2407baa4eb1b3be517ca8`), horizon-mixins
`4342a0bfbe71d4d14185528d64e496d6022f16a1`, Rust 1.99.1. Bundle code unchanged from the section
below; only the pinned revision changed. All mixin target functions and the server call site
are unchanged between the two revisions.

| Check | Result |
|---|---|
| Standalone Rust tests | Passed: 35 |
| Mixin apply, full workspace | Passed: 4 injections, 5 modules |
| AST preservation audit | Passed: 3 files; 5 modules, 3 HEAD hooks, 7 RETURN exits |
| Native Horizon build | Passed: launcher-equivalent Dev build, 19.7 min |
| Single-player gameplay | Passed: 19 checks; no "moved wrongly" warnings |
| Same checks on the unmodified Nightly client | 11 barrier checks fail as expected; 8 control checks pass |
| LAN host and guest | Passed: 34 checks, 17 per actor (barrier placed, perch, keep standing and step off on fence, wall and closed gate; fence-line walk; low ceiling). No "moved wrongly" warnings on the host |
| Manual test polygon | Reopened with this build; perch confirmed |
| Package | `tools/package.py`: `fence_perch-0.1.0.hnmod`, 11 entries, 9,406 bytes, SHA-256 `6eccc6f114c7b6e76f7648b511f03bdc0df205fe519cc989b5ce3b06d25c7b59`; a second build was byte-identical |
| Official `cargo horizon-mod check` (built at the same Nightly) | Passed: `native-mod`, unsigned |
| Dedicated server | Not claimed |

The mod version was renamed to 1.0.0 after these checks; the release package will be rebuilt and
revalidated on the newest Nightly.

Game runs used the installed Nightly's `horizon-ctl` and prepared 26.3 resources read-only, with
isolated homes and test worlds. The next Nightly (`2ebf00a`) changes `client_lib.rs`, a module
insertion file; it has not been tested.

## Merged Bundle, Dev Build and Gameplay (2026-11-07)

Horizon `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd`, horizon-mixins `4342a0bfbe71d4d14185528d64e496d6022f16a1`, Rust 1.99.1.
One bundle now covers client movement and the server collision result; both use one shared
player-scoped collision provider.

| Check | Result |
|---|---|
| Standalone Rust tests | Passed: 35 (8 core, 5 movement model, 6 server policy, 11 RETURN orchestration, 5 RETURN differential) |
| Mixin apply, full workspace | Passed: 4 injections (3 HEAD, 1 RETURN), 5 modules |
| AST preservation audit | Passed: `client_lib.rs`, `motion.rs`, `lan/hub.rs` match pristine source after removing the declared modules, 3 HEAD hooks and 7 RETURN exits. Audit tool tests: 5 passed |
| Native Horizon build | Passed: isolated build matching the launcher's Dev build (`cargo build --locked --offline --profile fast --config profile.fast.package.horizon-client.opt-level=0 -p horizon-client --bin horizon-client`), 9.7 min. No mod warnings |
| Single-player gameplay | Passed: 19 checks (`tools/test_perch.py`) |
| Same checks without the mod (installed Nightly `66f3981` client) | 11 barrier checks fail as expected (no perch or line walk on fence, wall or closed gate); the 8 control checks pass |
| Manual single-player check | Passed: the author played the isolated test polygon (all stations, animal pen) with the same build |
| LAN host and guest | Not run |
| Dedicated server | Not claimed |

Gameplay checks, each from a stationary start touching the barrier, one jump while walking toward it:
land on top of an oak fence, a cobblestone wall and a closed oak fence gate (feet at floor + 1.0,
on ground), keep standing for 2 s with no server correction, step off the far side; walk along the
top of a connected fence line and wall line. Controls: an open gate is walked through; an ordinary
jump peaks at 1.2522 blocks; a 1.5-block ledge stays unreachable; a fence under a one-block ceiling
gives no perch and no damage; six pigs pressed against a fence ring for 30 s stay inside.
The server log reported no "moved wrongly" warnings during these runs.

Game runs used the installed Nightly `66f3981` `horizon-ctl` and prepared 26.3 resources read-only,
with isolated homes and test worlds. Auto-jump was disabled. Not covered: other movement states
(sprinting, swimming, sneaking edge cases), auto-jump, other native mods, multiplayer roles, packaging.

## Standalone Logic

`cargo test --locked --offline`: 19 passed (8 core, 5 movement-model, 6 server-policy checks).
New checks cover world-space cap elevations, removed upper volume, old/new overlap rules,
invalid caps, geometry-error fallback and unsupported selectors. The new policy test first failed
to compile because its two policy helpers did not exist; it passed after adding them.

These tests exercise dependency-free logic and synthetic geometry only. The new server policy
helpers are not connected to Horizon's authoritative movement path.

Formatting check unavailable: the installed launcher toolchain has no `rustfmt.exe`.
No toolchain components were installed to bypass this limitation.

## Source Capability Checks

Using horizon-mixins 0.6.0 and a minimal indexing fixture with the original pinned hub source:

- Proposed `MODIFY_EXPRESSION_VALUE` of the new-collision predicate: rejected with
  `E_UNSUPPORTED_CONTROL_FLOW` because the target contains closures. Planned server injection
  is therefore not viable with this tool; no production manifest change was made.
- A no-op cancellable HEAD probe for the same `Hub::collide` target: planning passed,
  1 injection, 0 modules. This establishes boundary-hook resolution only, not a working replacement.

The alternative would replace the narrow server collision calculation through a boundary handler.
It requires design approval and regression proof preserving existing exemptions, corrections,
ground bookkeeping and error behavior. No check may be disabled to bypass the tool limitation.

## Not Done

Shared runtime extraction, authoritative handler implementation, source apply for new server hooks,
Horizon type checking/build, game launch, actual barrier resources, multiplayer and release validation.
No native Horizon build or game run was executed.

## Alternative-route Research (2026-11-07)

An already-installed newer official tool at revision `4342a0bfbe71d4d14185528d64e496d6022f16a1`
successfully applied a no-op RETURN hook to the pinned original hub fixture (1 injection, 0 modules).
The older-tool no-op HEAD probe also passed source apply. Pristine hub SHA-256 stayed
`4769ef9d791ef106be0cb699d605038f2d2bc2d9a6895414b7b9d5e50b9bf17c` in both checks.
The inner-expression probe still fails with the newer tool. No real geometry handler was added.

Current tool source `5a64af938ec86b00d53d16a408aabc0038fb3e57` documents the ordinary-closure
RETURN support. Current Horizon source `2303a445fe43e2930d2ffcc665cb6d40a57748b1` pins that tool
in its launcher; this is not evidence that the user's launcher binary has been updated.
Both locally tested tools report 0.6.0; exact revisions matter.

## RETURN Prototype (2026-11-07)

An isolated experimental bundle postprocesses the `Hub::collide` result with a player-scoped
capped geometry provider. Dependency-free orchestration tests: 16 passed (11 call-order, budget and
failure-fallback checks; 5 differential checks against an independent axis-aligned model).
Root suite rerun: 19 passed. Every probe error, unchanged geometry or absent barrier returns the
original verdict. Source apply with tool `4342a0bfbe71d4d14185528d64e496d6022f16a1`: 1 RETURN
injection, 4 modules. Horizon API names were checked against pinned source, not type-checked.
The launcher at the research pin selects mixins `5a64af938ec86b00d53d16a408aabc0038fb3e57`.

Not done: merged client/server bundle, full-workspace apply, compilation, gameplay.

See [implementation options](docs/research/implementation-options.md). Source apply establishes
only transformation feasibility, not runtime compilation, correctness, performance or security.
No tool was downloaded, installed or built for these probes; no gameplay source changed.
