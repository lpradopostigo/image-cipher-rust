from os import write


targets = ["wasm", "lib"]
target = "lib"
common = open("CargoCommon.toml", mode="r", encoding="utf-8").read()


if target in targets:
    cargo_file = open("Cargo.toml", "w+")
    if target == "lib":
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

    elif target == "wasm":
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