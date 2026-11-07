# Fence Perch

A prototype mod for [Horizon](https://playhorizon.dev): jump onto fences, walls and closed fence gates without a run-up, stand on top, then step off on either side. Gates stay closed.

## Status

Player-only collision-height prototype; ordinary jump strength is unchanged. One bundle now covers client movement and server movement validation (a result hook on the server collision check). A release candidate for Horizon Nightly 2026-11-07 passed single-player and LAN host/guest gameplay checks. Not yet published. See [validation](VALIDATION.md).

## Documentation

- [Concept](docs/concept.md): intended behavior and scope questions.
- [Decisions](docs/decisions.md): approved choices.
- [Movement research](docs/research/movement.md): source findings and limits.
- [Research log](docs/research/log.md): dated evidence.
- [Development plan](docs/development.md): review gates and validation.

## Project

- Name: Fence Perch.
- Repository directory: `horizon-fence-perch`.
- Mod ID: `fence_perch`.
- Original implementation; no third-party mod code has been inspected or used.
- Developed with AI assistance under human direction.
- License: [MIT](LICENSE).
