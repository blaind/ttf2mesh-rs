use ttf2mesh::{Quality, TTFFile, Value};

fn main() {
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
}
