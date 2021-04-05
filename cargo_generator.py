from os import write


targets = ["wasm", "lib"]
target = "lib"
common = open("CargoCommon.toml", mode="r", encoding="utf-8").read()


if target in targets:
    cargo_file = open("Cargo.toml", "w+")
    if target == "lib":
        extra = (
            "[lib]\n"
            "crate-type"
            ' = ["lib"]\n'
            "[dependencies]\n"
            'image = "0.23.14"\n'
        )
        cargo_file.write(extra + common)
