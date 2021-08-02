# ttf2mesh-rs &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs Version]][docs]

[Build Status]: https://img.shields.io/github/workflow/status/blaind/ttf2mesh-rs/test
[actions]: https://github.com/blaind/ttf2mesh/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/ttf2mesh.svg
[crates.io]: https://crates.io/crates/ttf2mesh

[Docs Version]: https://docs.rs/ttf2mesh/badge.svg
[docs]: https://docs.rs/ttf2mesh

A high-level Rust wrapper API for [fetisov's ttf2mesh](https://github.com/fetisov/ttf2mesh/) library for generating a 2d/3d mesh (vertices, indices and normals [only for 3D]) from TrueType (`.ttf`) glyphs.

## Installing

Prequisites:

    apt-get install build-essential patch

Add to `Cargo.toml`:

    [dependencies]
    ttf2mesh-rs = "*" # change with latest version

## Examples

See [examples](/examples) -folder and crate docs.

## Security

The API surface (mainly `.ttf` loading) has been fuzzed with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz). Current findings have been reported upstream and being applied at build time.

Still, be careful if planning to use this library for parsing untrusted fonts.

## Development

Install prequisites (see above).

Clone repository:

    git clone https://github.com/blaind/ttf2mesh-rs.git

Update submodules

    git submodule update --init

Develop

## License

Licensed under <a href="LICENSE">MIT license</a>

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the software by you,  shall be licensed as above, without any additional terms or conditions.
