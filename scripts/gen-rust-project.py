#!/usr/bin/env python3
"""Generate rust-project.json for rust-analyzer with kernel module support.

Wraps the kernel's generate_rust_analyzer.py and fixes sysroot crate duplication
that causes rust-analyzer to fail resolving inherent methods on primitive types.
"""

import json
import subprocess
import sys
import os

KERNEL_SRC = "/usr/src/linux-6.18.12-gentoo"
SCRIPT = f"{KERNEL_SRC}/scripts/generate_rust_analyzer.py"
VELES_KERNEL = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "veles-kernel")

def main():
    rustc = os.path.expanduser("~/.cargo/bin/rustc")
    sysroot = subprocess.check_output([rustc, "--print", "sysroot"]).decode().strip()
    sysroot_src = f"{sysroot}/lib/rustlib/src/rust/library"

    # Run kernel's generate_rust_analyzer.py
    env = os.environ.copy()
    env["RUSTC"] = rustc
    result = subprocess.run(
        [
            sys.executable, SCRIPT,
            "--cfgs=core=--cfg no_fp_fmt_parse",
            "2024",
            KERNEL_SRC, KERNEL_SRC,
            sysroot, sysroot_src,
            VELES_KERNEL,
        ],
        capture_output=True, text=True, env=env,
    )
    if result.returncode != 0:
        print(f"Error: {result.stderr}", file=sys.stderr)
        sys.exit(1)

    data = json.loads(result.stdout)

    # Add sysroot_src (required for rust-analyzer to resolve core inherent methods)
    data["sysroot_src"] = sysroot_src

    # Remove explicit sysroot crates (core, alloc, std, proc_macro) from crates array.
    # rust-analyzer loads these from sysroot automatically. Having them both as explicit
    # crates AND sysroot crates creates duplicate type instances (e.g. two different
    # core::fmt::Arguments) causing false type-mismatch errors.
    sysroot_names = {"core", "alloc", "std", "proc_macro"}
    sysroot_indices = {
        i for i, c in enumerate(data["crates"])
        if c["display_name"] in sysroot_names and not c["is_workspace_member"]
    }

    old_to_new = {}
    new_crates = []
    for i, c in enumerate(data["crates"]):
        if i in sysroot_indices:
            continue
        old_to_new[i] = len(new_crates)
        c["deps"] = [
            {"crate": old_to_new[dep["crate"]], "name": dep["name"]}
            for dep in c["deps"]
            if dep["crate"] not in sysroot_indices
        ]
        new_crates.append(c)

    data["crates"] = new_crates
    json.dump(data, sys.stdout, indent=4, sort_keys=True)


if __name__ == "__main__":
    main()
