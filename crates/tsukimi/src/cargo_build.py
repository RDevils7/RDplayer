#!/usr/bin/env python3
"""Run cargo build, then copy the built binary where meson expects it.

This replaces the previous `cargo build ... && cp ...` shell construct,
which does not work on Windows: meson invokes custom-target commands
directly (without a shell) there.
"""
import os
import shutil
import subprocess
import sys

cargo, bin_name, output = sys.argv[1], sys.argv[2], sys.argv[3]
options = sys.argv[4:]

subprocess.run([cargo, "build"] + options, check=True)

target_dir = options[options.index("--target-dir") + 1]
profile = "release" if "--release" in options else "debug"
ext = ".exe" if sys.platform == "win32" else ""
src = os.path.join(target_dir, profile, bin_name + ext)
shutil.copyfile(src, output)
