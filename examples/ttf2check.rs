use std::env;

/// Check if .ttf file can output mesh for input string
fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        println!("usage: ttf2obj <font-file.ttf> <utf8-string>");
        std::process::exit(255);
    }

    let (font, utf8_string) = (&args[1], &args[2]);

    println!("Loading font {:?}...", font);

    let mut font = match ttf2mesh::TTFFile::from_file(font) {
        Ok(font) => font,
        Err(e) => {
            println!(" - font load failed: {:?}", e);
            std::process::exit(255);
        }
    };

    println!("Input string: {:?}", utf8_string);
    for char in utf8_string.chars() {
        println!(
            "- {} = {}",
            char,
            match font.glyph_by_char(char) {
                Ok(mut glyph) => {
                    let len2dvert = match glyph.to_2d_mesh(ttf2mesh::Quality::Medium) {
                        Ok(mesh) => mesh.vertices_len(),
                        Err(_) => 0,
                    };

                    let len3dvert = match glyph.to_3d_mesh(ttf2mesh::Quality::Medium, 0.5) {
                        Ok(mesh) => mesh.vertices_len(),
                        Err(_) => 0,
                    };

                    format!("OK ({}, {})", len2dvert, len3dvert)
                }
                Err(_) => "ERR".to_string(),
            }
        );
    }
    println!("");
    println!("OK (x, y) - x = 2d vertices count, y = 3d vertices count. Both with Medium quality");
    println!("(0, 0) as count means that the glyph is found, but mesh can not be generated");
}
