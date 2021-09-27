//! # Overview
//!
//! A high-level Rust wrapper API for [fetisov's ttf2mesh](https://github.com/fetisov/ttf2mesh/)
//! library for generating a 2d/3d mesh (vertices, indices and normals [only for 3D])
//! from TrueType (`.ttf`) glyphs.
//!
//! Usage:
//! ```rust
//! # use ttf2mesh::{TTFFile, Quality, Value, Mesh, Mesh2d, Mesh3d};
//! #
//! let mut ttf = TTFFile::from_file("./fonts/FiraMono-Medium.ttf").unwrap();
//!
//! // export all glyphs as 2d meshes to a .obj file
//! ttf.export_to_obj("./fonts/FiraMono-Medium.obj", Quality::Low).unwrap();
//!
//! // generate 2d mesh for a glyph
//! let mut glyph = ttf.glyph_from_char('€').unwrap();
//! let mesh_2d: Mesh<Mesh2d> = glyph.to_2d_mesh(Quality::Medium).unwrap();
//!
//! // work with Mesh vertices, faces (indices). See Mesh documentation for more
//! assert_eq!(mesh_2d.vertices_len(), 56);
//! assert_eq!(mesh_2d.iter_vertices().next().unwrap().val(), (0.555, 0.656));
//!
//! assert_eq!(mesh_2d.faces_len(), 54);
//! assert_eq!(mesh_2d.iter_faces().next().unwrap().val(), (53, 52, 5));
//!
//! // 3d mesh with depth of 0.5
//! let mesh_3d: Mesh<Mesh3d> = glyph.to_3d_mesh(Quality::Medium, 0.5).unwrap();
//! ```
#![cfg_attr(feature = "unstable", feature(test))]

use std::{ffi::CString, path::Path};

mod error;
mod glyph;
mod mesh;
mod output;
mod quality;
mod ttf;

pub use error::Error;
pub use glyph::Glyph;
pub use mesh::{Mesh, Mesh2d, Mesh3d};
pub use output::{DataIterator, Value};
pub use quality::Quality;
pub use ttf::TTFFile;

// TODO: support TTF_FEATURE_IGN_ERR as bitflag

#[cfg(not(windows))]
fn path_to_cstring<P: AsRef<Path>>(path: P) -> CString {
    use std::os::unix::ffi::OsStrExt;
    CString::new(path.as_ref().as_os_str().as_bytes()).unwrap()
}

#[cfg(windows)]
fn path_to_cstring<P: AsRef<Path>>(path: P) -> CString {
    CString::new(path.as_ref().as_os_str().to_str().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn get_font_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts")
    }

    fn get_font(font_file: Option<&str>) -> PathBuf {
        match font_file {
            Some(file) => get_font_path().join(file),
            None => get_font_path().join("FiraMono-Medium.ttf"),
        }
    }

    pub(crate) fn read_font(font_file: Option<&str>) -> Vec<u8> {
        std::fs::read(get_font(font_file)).unwrap()
    }

    #[test]
    fn test_from_buffer_vec() {
        let _ = TTFFile::from_buffer_vec(read_font(None)).unwrap();
    }

    #[test]
    fn test_from_file() {
        let _ = TTFFile::from_file(get_font(None)).unwrap();
    }

    #[test]
    fn test_get_glyph_from_char() {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();
        let _ = font.glyph_from_char('A').unwrap();
    }

    #[test]
    fn test_to_3d_mesh() {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();
        let mut glyph = font.glyph_from_char('€').unwrap();
        let mesh = glyph.to_3d_mesh(Quality::Low, 0.5).unwrap();

        let mut sizes = Vec::new();
        sizes.extend_from_slice(&[
            mesh.iter_vertices().collect::<Vec<_>>().len(),
            mesh.iter_normals().unwrap().collect::<Vec<_>>().len(),
            mesh.iter_faces().collect::<Vec<_>>().len(),
        ]);

        let mesh = glyph.to_3d_mesh(Quality::High, 1.5).unwrap();

        sizes.extend_from_slice(&[
            mesh.iter_vertices().collect::<Vec<_>>().len(),
            mesh.iter_normals().unwrap().collect::<Vec<_>>().len(),
            mesh.iter_faces().collect::<Vec<_>>().len(),
        ]);

        let mesh = glyph.to_3d_mesh(Quality::Custom(255), 0.5).unwrap();

        sizes.extend_from_slice(&[
            mesh.iter_vertices().collect::<Vec<_>>().len(),
            mesh.iter_normals().unwrap().collect::<Vec<_>>().len(),
            mesh.iter_faces().collect::<Vec<_>>().len(),
        ]);

        assert_eq!(sizes, &[246, 246, 160, 552, 552, 364, 1164, 1164, 772]);
    }

    #[test]
    fn test_to_2d_mesh() {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();
        let mut glyph = font.glyph_from_char('€').unwrap();

        let mut sizes = Vec::new();
        let mesh = glyph.to_2d_mesh(Quality::Low).unwrap();
        assert!(mesh.iter_normals().is_none());
        sizes.extend_from_slice(&[
            mesh.iter_vertices().collect::<Vec<_>>().len(),
            mesh.iter_faces().collect::<Vec<_>>().len(),
        ]);

        let mesh = glyph.to_2d_mesh(Quality::High).unwrap();
        sizes.extend_from_slice(&[
            mesh.iter_vertices().collect::<Vec<_>>().len(),
            mesh.iter_faces().collect::<Vec<_>>().len(),
        ]);

        let mesh = glyph.to_2d_mesh(Quality::Custom(255)).unwrap();
        sizes.extend_from_slice(&[
            mesh.iter_vertices().collect::<Vec<_>>().len(),
            mesh.iter_faces().collect::<Vec<_>>().len(),
        ]);

        assert_eq!(sizes, &[41, 39, 92, 90, 194, 192]);
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_open_font(b: &mut Bencher) {
        let buffer = tests::read_font(None);

        b.iter(|| {
            let _ = TTFFile::from_buffer_vec(buffer.clone()).unwrap();
        });
    }

    #[bench]
    fn bench_get_glyph(b: &mut Bencher) {
        let mut font = TTFFile::from_buffer_vec(tests::read_font(None)).unwrap();

        b.iter(|| {
            let _ = font.glyph_from_char('€').unwrap();
        });
    }

    #[bench]
    fn bench_glyph_to_3d_mesh_low_quality(b: &mut Bencher) {
        let mut font = TTFFile::from_buffer_vec(tests::read_font(None)).unwrap();
        let mut glyph = font.glyph_from_char('€').unwrap();

        b.iter(|| {
            let _ = glyph.to_3d_mesh(Quality::Low, 0.1).unwrap();
        });
    }

    #[bench]
    fn bench_glyph_to_3d_mesh_high_quality(b: &mut Bencher) {
        let mut font = TTFFile::from_buffer_vec(tests::read_font(None)).unwrap();
        let mut glyph = font.glyph_from_char('€').unwrap();

        b.iter(|| {
            let _ = glyph.to_3d_mesh(Quality::High, 0.1).unwrap();
        });
    }

    #[bench]
    fn bench_glyph_to_2d_mesh_low_quality(b: &mut Bencher) {
        let mut font = TTFFile::from_buffer_vec(tests::read_font(None)).unwrap();
        let mut glyph = font.glyph_from_char('€').unwrap();

        b.iter(|| {
            let _ = glyph.to_2d_mesh(Quality::Low).unwrap();
        });
    }

    #[bench]
    fn bench_glyph_to_2d_mesh_high_quality(b: &mut Bencher) {
        let mut font = TTFFile::from_buffer_vec(tests::read_font(None)).unwrap();
        let mut glyph = font.glyph_from_char('€').unwrap();

        b.iter(|| {
            let _ = glyph.to_2d_mesh(Quality::High).unwrap();
        });
    }
}
