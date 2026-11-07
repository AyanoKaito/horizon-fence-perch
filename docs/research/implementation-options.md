# Implementation options

2026-11-07. Research comparison, not implementation approval or gameplay evidence.

## Evidence scope

- Prototype/source fixtures: Horizon `f4dcf8a2fb4a68b2d28422f4d34d2be2970e26bd`.
- Current-source cross-check: Horizon `2303a445fe43e2930d2ffcc665cb6d40a57748b1`.
- Current tool source: horizon-mixins `5a64af938ec86b00d53d16a408aabc0038fb3e57`.
- Additional installed tool used for source-only proof: `4342a0bfbe71d4d14185528d64e496d6022f16a1`.
- Both older and newer tools report 0.6.0; that version string alone does not establish capabilities.

## Required invariants

Ordinary jump input; stationary starts; land and stand on fences, walls and closed gates;
descend either side. Keep animals, unrelated blocks and jump strength unchanged. Server geometry
must agree with the client. No weakened movement checks or compatibility claim for unmodified servers.

Player-only collision capping inherently changes standing height and headroom. This is approved
prototype behavior, not identical vanilla movement and not a guarantee against every possible bypass.

## Native integration routes

### A. Postprocess the server collision result (RETURN)

Keep the original validator and adapt its returned geometry result only for eligible queries.
The official newer tool permits ordinary closures for RETURN. A no-op RETURN probe successfully
applied to the original pinned `Hub::collide`; pristine source stayed byte-identical.

Pros: no whole-body source replacement; original validation executes; official installed tool available.
Risks: original plus capped revalidation can duplicate collision work. Modifying a returned refusal
can still weaken checks if implemented incorrectly. Geometry logic and exemptions must remain aligned
with upstream; other mods modifying that result require composition tests. No real adapter implemented.

### B. Conditional entry handler (cancellable HEAD)

Handle only relevant player/barrier geometry; otherwise continue the original method.
A no-op HEAD probe successfully planned and applied; pristine source stayed byte-identical.

Pros: already-supported boundary; can reuse Horizon's solver and scanners; no need to duplicate
the original collision calculation on the handled path.
Risks: handled paths skip the original body and later hooks. The adapter must preserve all geometry
validation, exemptions, errors and ground results; upstream drift and native-mod conflicts need tests.
This is not equivalent to blindly accepting movement, but it is not implemented or proven safe.

### C. Scoped guard around original validation

A thread-local RAII guard could mark player validation while recursively invoking the original path;
a shared shape hook would cap only inside that scope.

Pros: retains the original validation algorithm; no global world-shape mutation required.
Risks: implicit scope can affect nested unrelated queries; recursive calls can repeat other mods' hooks;
panic restoration, reentrancy and thread isolation require proof. Proposed only, not a preferred default.

### D. Separate immutable player collision binding

Return the same scene type with a player-specific binding instead of altering validation results.

Pros: stock validator could run once; capped geometry could be cached rather than recomputed per hit.
Risks: cached registry fields are private; constructors use checked data tables. Lifetime-safe ownership,
registry identity, dynamic/contextual rules, memory bounds and reload behavior are unresolved.
Do not mutate the shared binding, leak registries on each query or use unsafe lifetime casts.
Not an immediately available implementation.

### E. Extend the mixin tool for precise safe sites

Support the required local receiver/captures and leave unrelated closures untouched.

Pros: could preserve stock validation and change only the intended shape-query seam.
Risks: this needs tested AST/borrow/control-flow support, not simply disabling safety checks.
It introduces a tool-maintenance and distribution obligation. No tool fork or change was made.

### F. Add an explicit player-query seam upstream

A player-specific provider or trustworthy query-kind hook would distinguish players without
identity guesses and keep authoritative validation in Horizon.

Pros: preferred long-term ownership boundary; least need to reproduce validation rules.
Risks: requires upstream source changes/review and alignment across player routes. Neither acceptance
nor a schedule is known; it is not a prerequisite that should silently stall native-mod work.
No upstream message or patch was submitted. A separately maintained game fork is not an ordinary
drop-in native bundle and is not the chosen product.

## Different mechanics and rejected shortcuts

- **Jump boost/impulse:** preserves full-height barriers, but adds reach toward unrelated ledges;
  timing, repeated activation, ceilings and server validation remain risks. Not the approved policy.
- **Conditional airborne stepping/lift:** may retain original standing height, but changes movement,
  can enable unintended air steps, and still needs matching server validation. Not a shortcut around integration.
- **Global caps or temporary block replacement:** can affect animals, unrelated queries and world state;
  violates player-only scope. Reject.
- **Client-only adjustment or disabled server refusal:** geometry disagrees or protection is weakened;
  reject as a product route. Server-owner consent and matching client/server rules are required.
- **Teleport/automatic crossing:** changes the required interaction and introduces clipping/correction
  risks. Reject for this feature.
- **Player body/pose tricks:** shrinking height does not increase ordinary jump reach; shifting the
  collider can create false ceiling/side collisions. Unproven and inferior to explicit barrier geometry.

## Rules, security and performance

Registry policy permits native mods subject to rights, source correspondence and no harmful behavior;
it does not guarantee listing or authorize use on someone else's server. No movement-mod ban was found
in the checked policy. Server-specific rules remain independent.

Every native route has the game's user-level privileges; RETURN does not grant extra privileges and
HEAD is not malware merely because it cancels a method. Limit code to geometry; no account, network,
filesystem or save-format changes are needed. Reviews and isolated tests remain necessary.

No route has measured runtime cost here. Revalidation adds potential collision work; caches trade
memory/startup work for query cost; duplicating validation increases update burden. No numerical
performance or risk score is justified by current evidence.

## Recommendation and next proof

Investigate A first with the already-installed newer official tool; retain B as a bounded fallback,
not the only possible route. Compare actual validator parity and query cost before selecting either.
Consider E/F if neither can preserve the invariants cleanly. C/D remain research alternatives, not
shortcuts to declare safe. Do not silently switch the approved collision policy to another mechanic.

Next proof: real adapter in an isolated probe, differential tests against original validation on
unsupported geometry and unchanged states, exact fallback/error cases and operation counts. Then,
with separate consent, Horizon compilation and isolated gameplay. No option is a working game mod yet.

## Sources

- [Current RETURN/local receiver contracts](https://github.com/horizon-rust/horizon-mixins/blob/5a64af938ec86b00d53d16a408aabc0038fb3e57/docs/advanced-injections.md).
- [Tool safety implementation](https://github.com/horizon-rust/horizon-mixins/blob/5a64af938ec86b00d53d16a408aabc0038fb3e57/crates/horizon-mixins-core/src/plan.rs#L944-L970).
- [Current launcher tool pin](https://github.com/horizon-rust/horizon/blob/2303a445fe43e2930d2ffcc665cb6d40a57748b1/launcher/src/native_mods.rs#L40-L42).
- [Current collision registry](https://github.com/horizon-rust/horizon/blob/2303a445fe43e2930d2ffcc665cb6d40a57748b1/crates/horizon-world/src/collision.rs#L58-L72).
- [Native privilege/compatibility contract](https://github.com/horizon-rust/horizon/blob/2303a445fe43e2930d2ffcc665cb6d40a57748b1/docs/native-mods.md).
- [Registry policy snapshot](https://github.com/horizon-rust/horizon-mods/blob/63daa89bcbdcb80282c29970d743d6d00918a6ce/MOD-POLICY.md).
