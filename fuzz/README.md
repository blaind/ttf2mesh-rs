# ttf2mesh fuzzing

This fuzz project fuzzes the underlying `ttf2mesh` C api through the provided Rust bindings.

To fuzz:

Install cargo afl

    cargo install cargo-fuzz

Initialize folders & copy a corpus of ttf-files to data input folder
	mkdir -p data/in data/out
    find /usr/ -name "*.ttf" -exec cp {} fuzzing/data/in \;

Build & fuzz

	cargo fuzz build
    cargo fuzz run fuzz_target_1
