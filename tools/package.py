#!/usr/bin/env python3
"""Build a deterministic source-only Horizon native mod package."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import tomllib
import zipfile


ROOT = Path(__file__).resolve().parents[1]
GAME_VERSION = "26.3"
ID_PATTERN = re.compile(r"[a-z0-9_]{1,64}")
REVISION_PATTERN = re.compile(r"[0-9a-f]{40}")
ZIP_TIMESTAMP = (1980, 1, 1, 0, 0, 0)


def metadata(root: Path) -> tuple[Path, dict, dict]:
    compatibility = json.loads((root / "compatibility.json").read_text(encoding="utf-8"))
    manifests = sorted((root / "mixins").glob("*/horizon-mixin.toml"))
    if len(manifests) != 1:
        raise ValueError("exactly one mixin bundle is required")

    manifest_path = manifests[0]
    mixin = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    mod = mixin["mod"]
    mod_id = mod["id"]
    version = mod["version"]
    bundle = manifest_path.parent

    if ID_PATTERN.fullmatch(mod_id) is None:
        raise ValueError("the mod ID must contain only lowercase letters, digits and underscores")
    if bundle.name != mod_id:
        raise ValueError("the mixin bundle directory must match the mod ID")
    if not version:
        raise ValueError("the mod version cannot be empty")

    horizon = compatibility["horizon_revision"]
    mixins = compatibility["horizon_mixins_revision"]
    if REVISION_PATTERN.fullmatch(horizon) is None or REVISION_PATTERN.fullmatch(mixins) is None:
        raise ValueError("Horizon and horizon-mixins must use exact 40-character revisions")
    if mixin.get("compatibility", {}).get("horizon_revision") != horizon:
        raise ValueError("the bundle and compatibility metadata use different Horizon revisions")

    native_manifest = {
        "game": GAME_VERSION,
        "horizon": horizon,
        "id": mod_id,
        "mixins": mixins,
        "schema": 1,
        "version": version,
    }
    return bundle, mixin, native_manifest


def payload(root: Path) -> tuple[dict[str, bytes], dict]:
    bundle, _, native_manifest = metadata(root)
    files = {
        "horizon-native.json": (
            json.dumps(native_manifest, indent=2, sort_keys=True) + "\n"
        ).encode("utf-8")
    }
    for path in sorted(bundle.rglob("*")):
        if not path.is_file():
            continue
        relative = path.relative_to(root).as_posix()
        in_data = "data" in path.relative_to(bundle).parts[:-1]
        if path.suffix not in {".rs", ".toml"} and not in_data:
            raise ValueError(f"unsupported bundle file: {relative}")
        files[relative] = path.read_bytes()
    return files, native_manifest


def archive_info(name: str) -> zipfile.ZipInfo:
    info = zipfile.ZipInfo(name, ZIP_TIMESTAMP)
    info.compress_type = zipfile.ZIP_DEFLATED
    info.create_system = 3
    info.external_attr = 0o100644 << 16
    return info


def build(root: Path, output: Path) -> dict:
    files, native_manifest = payload(root)
    output.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for name in sorted(files):
            archive.writestr(archive_info(name), files[name])

    # Release assets are the package and its signature only; the digest is printed.
    digest = hashlib.sha256(output.read_bytes()).hexdigest()
    return {
        "file": str(output.resolve()),
        "sha256": digest,
        "size": output.stat().st_size,
        "entries": sorted(files),
        "manifest": native_manifest,
    }


def default_output(root: Path) -> Path:
    _, _, native_manifest = metadata(root)
    return root / "dist" / f"{native_manifest['id']}-{native_manifest['version']}.hnmod"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, help="output .hnmod path")
    args = parser.parse_args()
    output = args.output or default_output(ROOT)
    if output.suffix != ".hnmod":
        parser.error("the output file must use the .hnmod extension")
    result = build(ROOT, output)
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
