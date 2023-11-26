use std::env;
use ttf2mesh::Value;

/// Output 2d mesh data from a glyph in an input .ttf file
fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        println!("usage: ttf2obj <font-file.ttf> <utf8-string> [quality]");
        std::process::exit(255);
    }

    let (font, utf8_string) = (&args[1], &args[2]);

    let quality = if args.len() > 3 {
        let quality_str = &args[3];

        match ttf2mesh::Quality::from_str(&quality_str) {
            Ok(q) => q,
            Err(error) => {
                println!(
                    "Can not parse quality ({}): {}. Try 'low', 'medium' or 'high'",
                    quality_str, error
                );
                std::process::exit(255);
            }
        }
    } else {
        ttf2mesh::Quality::Medium
    };

    println!("Loading font {:?}...", font);

    let mut font = match ttf2mesh::TTFFile::from_file(font) {
        Ok(font) => font,
        Err(error) => {
            println!(" - font load failed: {}", error);
            std::process::exit(255);
        }
    };

    for char in utf8_string.chars() {
        println!("Mesh data char {:?}", char);
        let mut glyph = match font.glyph_from_char(char) {
            Ok(g) => g,
            Err(error) => {
                println!("- can not find glyph in the font file: {}", error);
                continue;
            }
        };

        match glyph.to_2d_mesh(quality) {
            Ok(mesh) => {
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
                println!("");
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
            Err(error) => {
                println!(" - could not generate 2d mesh: {}", error);
            }
        }

        println!("");
    }
}
