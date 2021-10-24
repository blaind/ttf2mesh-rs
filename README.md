# ttf2mesh &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs Version]][docs]

[build status]: https://img.shields.io/github/workflow/status/blaind/ttf2mesh-rs/test
[actions]: https://github.com/blaind/ttf2mesh/actions?query=branch%3Amain
[latest version]: https://img.shields.io/crates/v/ttf2mesh.svg
[crates.io]: https://crates.io/crates/ttf2mesh
[docs version]: https://docs.rs/ttf2mesh/badge.svg
[docs]: https://docs.rs/ttf2mesh

A high-level Rust wrapper API for [fetisov's ttf2mesh](https://github.com/fetisov/ttf2mesh/) library for generating a 2d/3d mesh (vertices, indices and normals [only for 3D]) from TrueType (`.ttf`) glyphs.

## Installing

Prequisites:

    apt-get install build-essential patch

Add to `Cargo.toml`:

    [dependencies]
    ttf2mesh = "*" # change to latest version

## Examples

See [examples](/examples) -folder and crate docs.

Simple usage:

```rust
use ttf2mesh::{Quality, TTFFile, Value};

let mut font = TTFFile::from_file("fonts/FiraMono-Medium.ttf").unwrap();

for char in "Hello_World".chars() {
    let mut glyph = font.glyph_from_char(char).unwrap();
    let mesh = glyph.to_2d_mesh(Quality::Medium).unwrap();

    println!("Mesh data char {:?}", char);
    println!(
        "- vertices: [{}]",
        mesh.iter_vertices()
            .map(|v| {
                let v = v.val();
                format!("({:.3}, {:.2})", v.0, v.1)
            })
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "- faces: [{}]",
        mesh.iter_faces()
            .map(|v| {
                let v = v.val();
                format!("({}, {}, {})", v.0, v.1, v.2)
            })
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("");
}


```

## Security

The API surface (mainly `.ttf` loading) has been fuzzed with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

## Development

Install prequisites (see above).

Clone repository:

    git clone https://github.com/blaind/ttf2mesh-rs.git

Update submodules

    git submodule update --init

Develop

## Rust Version Support

The minimum supported Rust version is 1.43

## License

Licensed under <a href="LICENSE">MIT license</a>

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the software by you, shall be licensed as above, without any additional terms or conditions.
