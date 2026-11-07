# Decisions

- **2026-11-06 — Name.** Fence Perch. Local repository directory: `horizon-fence-perch`; mod ID: `fence_perch`.
- **2026-11-06 — Core behavior.** Jump onto fences, walls and closed fence gates, stand on top, then descend. Do not substitute automatic crossing.
- **2026-11-06 — Stationary start.** No run-up is required. Jumping from rest directly beside a supported barrier must allow landing on top after horizontal input.
- **2026-11-06 — License.** Original code and documentation use the MIT License. Third-party material retains its applicable notices.
- **2026-11-06 — Documentation.** Public files are English-only, concise reference material. Russian review notes and agent context stay in ignored paths. Keep findings, proposals and approved decisions separate.
- **2026-11-06 — Originality.** Do not inspect or reuse Java mod implementation code. Public documentation describes this project's behavior and Horizon integration, not other mods. Any later third-party reuse requires approval and the applicable notices.
- **2026-11-06 — Review gate.** Initially documentation only; superseded for the approved prototype below. Native builds, game runs and publication remain separately gated.
- **2026-11-07 — Collision prototype.** User approved trying a player-only collision-height cap at block-local Y=1.0, without increasing jump strength. Preserve horizontal geometry and non-player collision. Changed player standing height and headroom are explicit tradeoffs, not a promise of zero exploits.
- **2026-11-07 — Spec Kit.** User approved local Spec Kit setup and status reconciliation. Keep agent infrastructure ignored and public references concise; no new gameplay code or build is authorized by this documentation block.
- **2026-11-07 — Initial validation baseline.** Proceed on foot in survival, on level ground, using vanilla barriers. Varied obstacle fixtures are allowed, including connections, corners and ceiling/ledge controls. Mounted and vehicle movement are outside this first check; build/run approval and network-support claims remain separate.
