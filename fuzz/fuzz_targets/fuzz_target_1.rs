#![no_main]
use libfuzzer_sys::fuzz_target;

use rand::{thread_rng, Rng};
use ttf2mesh::{Quality, TTFFile};

fuzz_target!(|data: &[u8]| {
    let mut font = match TTFFile::from_buffer_vec(data.to_vec()) {
        Ok(font) => font,
        Err(_) => {
            //println!("Font load failed");
            return;
        }
    };

    let glyph_count = font.glyph_count();

    let mut rng = thread_rng();
    let index = rng.gen_range(0..glyph_count);

    let mut glyph = match font.glyph_by_index(index) {
        Ok(g) => g,
        Err(_) => {
            // this should not happen
            println!("Glyph not found");
            return;
        }
    };

    match glyph.to_2d_mesh(Quality::High) {
        Ok(_) => (),  //println!("2d mesh ok"),
        Err(_) => (), //println!("To 2d mesh failed"),
    }

    match glyph.to_3d_mesh(Quality::High, 0.5) {
        Ok(_) => (),  //println!("3d mesh ok"),
        Err(_) => (), // println!("To 3d mesh failed"),
    }
});
