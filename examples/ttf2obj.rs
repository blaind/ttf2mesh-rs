use std::{char, env};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        println!("usage: ttf2obj <font-file.ttf> <output-file.obj> [quality]");
    }
    let (font, obj_file) = (&args[1], &args[2]);

    let quality = 10;

    println!("Loading font {:?}...", font);

    let mut font = match ttf2mesh::TTFFile::from_file(font) {
        Ok(font) => font,
        Err(e) => {
            println!("FONT LOAD ERR");
            return;
        }
    };

    println!("Loaded");
    return;

    println!(
        "Export to wavefront {:?} with quality={}...",
        obj_file, quality
    );

    font.export_to_obj(obj_file, ttf2mesh::Quality::High)
        .unwrap();

    let mut glyph = font.glyph_by_char("0".chars().next().unwrap()).unwrap();

    let mesh = glyph.to_2d_mesh(ttf2mesh::Quality::High).unwrap();
    let vertices = mesh.iter_vertices().collect::<Vec<_>>();
    println!("VERTICES: {:?}", vertices);

    let faces = mesh.iter_faces().collect::<Vec<_>>();
    println!("FACES: {:?}", faces);

    println!("All done!");
}
