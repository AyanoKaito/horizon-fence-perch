# Concept

## Confirmed intent

Jump onto a fence, wall or closed fence gate using the ordinary jump control. Land on top, stand there, and descend on either side without opening the gate.

No run-up is required. A jump started from rest directly beside the barrier must allow landing on top after horizontal input.

Primary scenario: enter or leave a crowded animal pen without opening an escape route.

This is not automatic traversal, teleportation or movement through a solid barrier.

## Approved prototype policy

Cap eligible player collision height at block-local Y=1.0. Preserve horizontal collision geometry, ordinary jump strength and non-player collision geometry. The player stands lower than the original 1.5-high collision surface; available headroom changes too. Client and server checks must agree before integration is claimed.

## Proposed boundaries

Pending scope approval:

- Keep ordinary jumps away from fences, walls and gates unchanged.
- Preserve original barrier collision shapes for non-player queries; do not give animals the same movement change.
- Add no separate traversal key or automatic gate interaction.
- Initial validation baseline approved: on-foot survival movement over vanilla fences, walls and fence gates on level ground. Include varied obstacle fixtures; this does not establish support for other movement states.
- Do not promise arbitrary modded blocks or server compatibility.

## Open decisions

- Remaining exploit tolerance: no jump boost is used, but standing on a barrier remains a launch point for another ordinary jump. Do not promise that every bypass is prevented.
- Expected behavior on slopes, from lower ground, while crouching, and under low ceilings.
- Scope for sprinting, active movement effects, creative mode and mounted players.
- Whether any user setting is needed beyond the game's mod toggle.

## Acceptance target

In an isolated survival test, start from rest directly beside each supported fence, wall and closed fence gate. Jump onto it without a run-up, remain on top without continued jump input, and descend to either side. The gate never opens. Nearby animals retain ordinary movement.

No acceptance check has been run.
