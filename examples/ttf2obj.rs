use std::env;

/// Create a .obj file from an input .ttf file
///
/// Obj will contain all glyphs as separate 2d meshes
fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        println!("usage: ttf2obj <font-file.ttf> <output-file.obj> [quality]");
        std::process::exit(255);
    }

    let (font, obj_file) = (&args[1], &args[2]);

    let quality = if args.len() > 3 {
        let quality_str = &args[3];

        match ttf2mesh::Quality::from_str(&quality_str) {
            Ok(q) => q,
            Err(e) => {
                println!(
                    "Can not parse quality ({}): {:?}. Try 'low', 'medium' or 'high'",
                    quality_str, e
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
        Err(e) => {
            println!(" - font load failed: {:?}", e);
            std::process::exit(255);
        }
    };

    println!("Export to obj {:?} with quality={:?}...", obj_file, quality);

    match font.export_to_obj(obj_file, quality) {
        Ok(_) => println!("Done"),
        Err(e) => {
            println!(" - export failed: {:?}", e);
            std::process::exit(255);
        }
    }
}
