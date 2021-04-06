#!/usr/bin/python
from os import write
from argparse import ArgumentParser

# arg parser section
targets = ["wasm", "lib"]

parser = ArgumentParser(description="generate Cargo.toml for a specified target")
parser.add_argument(
    "target",
    metavar="target",
    type=str,
    help="target for generate the Cargo.toml",
    choices=targets,
)
args = parser.parse_args()

# file generation section
common = open("CargoCommon.toml", mode="r", encoding="utf-8").read()
cargo_file = open("Cargo.toml", "w+")

if args.target == "lib":
    lib = (
        "[lib]\n"
        "crate-type"
        ' = ["lib"]\n'
        "[dependencies]\n"
        'image = "0.23.14"\n'
        "[profile.release]\n"
        "lto = true\n"
    )
    cargo_file.write(common + lib)

elif args.target == "wasm":
    wasm = (
        "[lib]\n"
        "crate-type"
        ' = ["cdylib"]\n'
        "[dependencies]\n"
        'image = "0.23.14"\n'
        'wasm-bindgen = "0.2.73"\n'
        "[dev-dependencies]\n"
        'wasm-bindgen-test = "0.2"\n'
        "[profile.release]\n"
        "lto = true\n"
        'opt-level = "s"\n'
    )
    cargo_file.write(common + wasm)

cargo_file.close()
