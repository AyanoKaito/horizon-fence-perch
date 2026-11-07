# Installation

## Release package

Download `fence_perch-1.0.0.hnmod` from the [official 1.0.0 release](https://github.com/AyanoKaito/horizon-fence-perch/releases/tag/v1.0.0), and compare its SHA-256 with the digest GitHub shows for that file on the release page. Version `1.0.0` targets Horizon Nightly 2026-11-07, revision `2ebf00a0281c592f79191e76fce0e7d522c6f425`; compatibility with newer Nightlies is not claimed.

Official repository: <https://github.com/AyanoKaito/horizon-fence-perch>

### Verify the signature

The release also provides `fence_perch-1.0.0.hnmod.minisig`. Download it beside the `.hnmod` and verify with [Minisign](https://jedisct1.github.io/minisign/) and the project's public key:

```powershell
minisign -V -P RWRDywNNoT8kYatbQtkHxdvUPbWE3YZyJqEAQWJQPIg8sWpTNfBoqSb2 -m fence_perch-1.0.0.hnmod
```

The expected success message is `Signature and comment signature verified`. The public key above (ID `61243FA14D03CB43`) is dedicated to this project. A valid signature proves the package was signed by the matching key, not that its code is harmless.

## Horizon launcher

Horizon native mods run as native code with the player's privileges. Read the source and accept Horizon's trust warning only if you trust this project.

Use the launcher's Mods page and **Add from GitHub** with the official repository URL. Horizon resolves a release to an exact `.hnmod`, verifies its GitHub digest, and builds a separate native-mod game instance after the player explicitly approves the build. A GitHub release is separate from a listing in Horizon's reviewed mod registry.

The first native build may download Horizon's Rust toolchain and take substantially longer than later builds.

## Multiplayer

Install the mod on every player's game. In single-player and on a LAN game hosted with the mod, the host's integrated server validates player movement with the same barrier shape. Joining a world or server without the mod is not supported: an unmodified server validates movement against full-height barriers. Dedicated servers are not supported.

## Updates and removal

Use Horizon's normal GitHub-mod update and removal controls. The mod stores no settings and changes no blocks; worlds load normally without it. Back up important worlds before changing any native mod build.
