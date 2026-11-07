"""Perch checks against an already running, isolated Horizon instance.

Uses horizon-ctl's script command to batch requests. No builds, downloads or
game launches. Commands and replies stay beside the isolated test world.
"""

import argparse
import json
import os
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parents[1]
FLOOR = 64  # top surface of the prepared stone floor
BARRIER_X = 3
# Barrier state, near face (block-local x) along the +X approach.
BARRIERS = (
    ("oak_fence", "minecraft:oak_fence", 0.375),
    ("cobblestone_wall", "minecraft:cobblestone_wall", 0.25),
    ("closed_gate", "minecraft:oak_fence_gate[facing=east,open=false]", 0.375),
)


class Batch:
    def __init__(self, session):
        self.run = Path(session["run"]).resolve()
        if not self.run.is_relative_to(ROOT / ".local" / "runs"):
            raise ValueError("Only project-local isolated runs are accepted")
        self.home = Path(session["gameHome"]).resolve()
        if not self.home.is_relative_to(self.run):
            raise ValueError("Game home must be inside the isolated run")
        self.ctl = session["ctl"]
        self.socket = session["socket"]
        self.env = os.environ.copy()
        for key, child in {
            "HORIZON_HOME": "game", "HORIZON_LAUNCHER_HOME": "launcher",
            "XDG_DATA_HOME": "xdg-data", "XDG_CONFIG_HOME": "xdg-config",
            "XDG_RUNTIME_DIR": "runtime", "HORIZON_LAUNCHER_TRASH": "trash",
            "HORIZON_LAUNCHER_DESKTOP": "desktop",
        }.items():
            self.env[key] = str(self.run / child)
        self.env.update(HORIZON_LAUNCHER_NO_SELF_UPDATE="1", HORIZON_NO_AUDIO="1",
                        HORIZON_LAUNCHER_CLIPBOARD=str(self.run / "clipboard.txt"))
        self.passed = []
        self.failed = []
        self.server_edits = False

    def script(self, label, commands, timeout=90):
        if self.server_edits:
            # A LAN guest's ctl edits are sent as chat commands; use server commands instead.
            commands = [
                "command " + c.replace(" --confirm", "") if c.startswith(("fill ", "setblock ")) else c
                for c in commands
            ]
        path = self.run / f"batch-{label}.ctl"
        path.write_text("\n".join(commands) + "\n", encoding="utf-8")
        result = subprocess.run(
            [self.ctl, "--socket", self.socket, "script", str(path)],
            cwd=self.home, env=self.env, capture_output=True, text=True,
            encoding="utf-8", timeout=timeout,
        )
        (self.run / f"batch-{label}.jsonl").write_text(result.stdout, encoding="utf-8")
        (self.run / f"batch-{label}.err.txt").write_text(result.stderr, encoding="utf-8")
        replies = [json.loads(line) for line in result.stdout.splitlines() if line.strip()]
        rejected = any(
            "Incorrect argument" in line or "Unknown or incomplete command" in line
            for r in replies for line in r.get("chat", [])
        )
        if (result.returncode or len(replies) != len(commands)
                or any("error" in r for r in replies) or rejected):
            raise AssertionError(f"Control batch failed: {label}; inspect local receipts")
        return replies

    def check(self, name, ok, detail):
        (self.passed if ok else self.failed).append(name)
        print(f"{'PASS' if ok else 'FAIL'} {name}: {detail}")


def position(reply):
    player = reply["player"]
    pos = player.get("position") or player.get("pos")
    return [float(v) for v in pos], bool(player.get("on_ground"))


def fixture(block):
    return [
        "command gamemode creative",
        f"fill -2 {FLOOR - 1} -3 9 {FLOOR - 1} 3 minecraft:stone --confirm",
        f"fill -2 {FLOOR} -3 9 {FLOOR + 4} 3 minecraft:air --confirm",
        f"setblock {BARRIER_X} {FLOOR} 0 {block} --confirm",
    ]


def perch(batch, label, block, face, forward_ticks):
    """Stationary start touching the barrier, one jump while walking toward it."""
    start_x = BARRIER_X + face - 0.3 - 0.01
    commands = fixture(block) + [
        f"command tp @s {start_x:.3f} {FLOOR} 0.5 -90 0",
        "command gamemode survival", "wait 10", f"block {BARRIER_X} {FLOOR} 0", "state",
        "key w down", "key space down", "wait 3", "key space up",
        f"wait {forward_ticks}", "key w up",
        "wait 30", "state",          # landed
        "wait 40", "state",          # still standing (no server correction)
        "key w down", "wait 12", "key w up", "wait 30", "state",  # stepped off
    ]
    replies = batch.script(label, commands)
    placed = next(r for c, r in zip(commands, replies) if c.startswith("block "))
    expected = block.split("[")[0]
    batch.check(f"{label}: barrier placed", placed.get("block") == expected, f"block {placed.get('state')}")
    states = [r for c, r in zip(commands, replies) if c == "state"]
    (start, _), (landed, landed_ground), (stood, stood_ground), (after, after_ground) = (
        position(s) for s in states
    )
    top = FLOOR + 1.0
    batch.check(f"{label}: start beside barrier",
                abs(start[1] - FLOOR) < 1e-6 and abs(start[0] - start_x) < 0.01, f"start {start}")
    batch.check(f"{label}: lands on top", abs(landed[1] - top) < 0.01 and landed_ground,
                f"landed {landed} ground={landed_ground}")
    batch.check(f"{label}: keeps standing", abs(stood[1] - top) < 0.01 and stood_ground
                and abs(stood[0] - landed[0]) < 0.01, f"after 2 s {stood}")
    batch.check(f"{label}: steps off far side",
                abs(after[1] - FLOOR) < 0.01 and after_ground and after[0] > BARRIER_X + 1.0,
                f"after {after}")


def open_gate_passes(batch):
    commands = fixture("minecraft:oak_fence_gate[facing=east,open=true]") + [
        f"command tp @s {BARRIER_X - 1.5:.3f} {FLOOR} 0.5 -90 0",
        "command gamemode survival", "wait 10",
        "key w down", "wait 25", "key w up", "wait 20", "state",
    ]
    replies = batch.script("open-gate", commands)
    pos, ground = position(replies[-1])
    batch.check("open gate: walk through at floor level",
                abs(pos[1] - FLOOR) < 0.01 and pos[0] > BARRIER_X + 1.0 and ground,
                f"after {pos}")


def ordinary_jump_height(batch):
    """A jump on open floor peaks near vanilla's 1.2522 blocks; the mod must not raise it."""
    commands = fixture("minecraft:air") + [
        "command tp @s 0.5 64 0.5 -90 0", "command gamemode survival", "wait 10",
        "key space down", "wait 3", "key space up",
    ]
    commands += ["wait 1", "state"] * 10
    replies = batch.script("ordinary-jump", commands)
    peak = max(position(r)[0][1] for c, r in zip(commands, replies) if c == "state") - FLOOR
    batch.check("ordinary jump: peak unchanged", 1.15 < peak < 1.26, f"peak {peak:.4f}")


def higher_ledge_stays_out_of_reach(batch, forward_ticks):
    """A 1.5-block ledge next to nothing supported stays unreachable."""
    commands = fixture("minecraft:stone") + [
        f"setblock {BARRIER_X} {FLOOR + 1} 0 minecraft:stone_slab[type=bottom] --confirm",
        f"command tp @s {BARRIER_X - 0.31:.3f} {FLOOR} 0.5 -90 0",
        "command gamemode survival", "wait 10",
        "key w down", "key space down", "wait 3", "key space up",
        f"wait {forward_ticks}", "key w up", "wait 30", "state",
    ]
    replies = batch.script("ledge-1-5", commands)
    pos, ground = position(replies[-1])
    batch.check("1.5-block ledge: still unreachable", abs(pos[1] - FLOOR) < 0.01 and ground,
                f"after {pos}")


def low_ceiling_blocks_perch(batch, forward_ticks):
    """Capping must not let a player stand where the ceiling leaves no headroom."""
    commands = fixture("minecraft:oak_fence") + [
        f"setblock {BARRIER_X} {FLOOR + 2} 0 minecraft:stone --confirm",
        f"command tp @s {BARRIER_X + 0.375 - 0.31:.3f} {FLOOR} 0.5 -90 0",
        "command gamemode survival", "wait 10",
        "key w down", "key space down", "wait 3", "key space up",
        f"wait {forward_ticks}", "key w up", "wait 30", "state",
    ]
    replies = batch.script("low-ceiling", commands)
    pos, ground = position(replies[-1])
    health = replies[-1]["player"]["health"]
    batch.check("low ceiling: no standing on fence, no damage",
                abs(pos[1] - FLOOR) < 0.01 and ground and health == 20.0,
                f"after {pos} health {health}")


def walk_along_line(batch, label, block, face, forward_ticks):
    """Jump onto the end of a connected line, then walk along its top."""
    start_x = BARRIER_X + face - 0.3 - 0.01
    commands = fixture(block) + [
        f"fill {BARRIER_X} {FLOOR} 0 {BARRIER_X + 4} {FLOOR} 0 {block} --confirm",
        f"command tp @s {start_x:.3f} {FLOOR} 0.5 -90 0",
        "command gamemode survival", "wait 10",
        "key w down", "key space down", "wait 3", "key space up",
        f"wait {forward_ticks}", "key w up", "wait 20", "state",
        "key w down", "wait 8", "key w up", "wait 20", "state",
    ]
    replies = batch.script(label, commands)
    states = [r for c, r in zip(commands, replies) if c == "state"]
    (landed, landed_ground), (walked, walked_ground) = (position(s) for s in states)
    top = FLOOR + 1.0
    batch.check(f"{label}: walks along the top",
                abs(landed[1] - top) < 0.01 and landed_ground and abs(walked[1] - top) < 0.01
                and walked_ground and walked[0] > landed[0] + 1.0,
                f"landed {landed}, walked {walked}")


def animals_stay_in_pen(batch):
    """Mobs keep full-height barriers: pigs wandering for 30 s stay inside a fence ring."""
    commands = fixture("minecraft:air") + [
        "command kill @e[type=minecraft:pig]",
        f"fill 1 {FLOOR} -3 9 {FLOOR} 3 minecraft:oak_fence --confirm",
        f"fill 2 {FLOOR} -2 8 {FLOOR} 2 minecraft:air --confirm",
        f"command tp @s -1.5 {FLOOR} 0.5 -90 0", "command gamemode survival",
    ] + [f"summon pig {x} {FLOOR} {z}" for x, z in ((3, -1), (5, 0), (7, 1), (4, 1), (6, -1), (5, 2))] + [
        "wait 600", "entities 24 minecraft:pig",
    ]
    replies = batch.script("animal-pen", commands, timeout=120)
    pigs = replies[-1]["entities"]
    inside = [p for p in pigs if 2.0 < p["pos"][0] < 9.0 and -2.0 < p["pos"][2] < 3.0]
    batch.check("animal pen: all pigs stay inside", len(pigs) == 6 and len(inside) == 6,
                f"{len(inside)} of {len(pigs)} inside: {[p['pos'] for p in pigs]}")
    batch.script("animal-pen-cleanup", ["command kill @e[type=minecraft:pig]"])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--session", default=str(ROOT / ".local" / "build" / "active-test-run.json"))
    parser.add_argument("--forward-ticks", type=int, default=4)
    parser.add_argument("--only", choices=[b[0] for b in BARRIERS] + ["open_gate"])
    args = parser.parse_args()
    session = json.loads(Path(args.session).read_text(encoding="utf-8-sig"))
    batch = Batch(session)
    batch.script("setup", ["options-set autoJump false", "command gamerule minecraft:spawn_mobs false"])
    for name, block, face in BARRIERS:
        if args.only in (None, name):
            perch(batch, name, block, face, args.forward_ticks)
    if args.only in (None, "open_gate"):
        open_gate_passes(batch)
    if args.only is None:
        walk_along_line(batch, "fence line", "minecraft:oak_fence", 0.375, args.forward_ticks)
        walk_along_line(batch, "wall line", "minecraft:cobblestone_wall", 0.25, args.forward_ticks)
        ordinary_jump_height(batch)
        higher_ledge_stays_out_of_reach(batch, args.forward_ticks)
        low_ceiling_blocks_perch(batch, args.forward_ticks)
        animals_stay_in_pen(batch)
    print(f"passed {len(batch.passed)}, failed {len(batch.failed)}")
    raise SystemExit(1 if batch.failed else 0)


if __name__ == "__main__":
    main()
