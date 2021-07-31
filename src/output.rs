//! Entities for interacting with mesh-produced output (vertices, faces(indices) and normals)
use std::fmt;

use ttf2mesh_sys as sys;

/// Value produced by a mesh data iterator. Access with `.val()`
///
/// Values produced by `.val()` depend on the mesh & data type:
/// * 2d vertices: `(f32, f32)` which represent x, y
/// * 3d vertices: `(f32, f32, f32)` which represent x, y, z
/// * indices (both 2d & 3d): `(i32, i32, i32)`
/// * normals (only for 3d): `(f32, f32, f32)`
pub trait Value<'a> {
    type Output: fmt::Debug;

    fn val(&self) -> Self::Output;
}

impl<'a> Value<'a> for sys::ttf_mesh__bindgen_ty_1 {
    type Output = (f32, f32);

    fn val(&self) -> Self::Output {
        (self.x, self.y)
    }
}

impl<'a> Value<'a> for sys::ttf_mesh__bindgen_ty_2 {
    type Output = (i32, i32, i32);

    fn val(&self) -> Self::Output {
        (self.v1, self.v2, self.v3)
    }
}

impl<'a> Value<'a> for sys::ttf_mesh3d__bindgen_ty_1 {
    type Output = (f32, f32, f32);

    fn val(&self) -> Self::Output {
        (self.x, self.y, self.z)
    }
}

impl<'a> Value<'a> for sys::ttf_mesh3d__bindgen_ty_2 {
    type Output = (i32, i32, i32);

    fn val(&self) -> Self::Output {
        (self.v1, self.v2, self.v3)
    }
}

impl<'a> Value<'a> for sys::ttf_mesh3d__bindgen_ty_3 {
    type Output = (f32, f32, f32);

    fn val(&self) -> Self::Output {
        (self.x, self.y, self.z)
    }
}

/// An iterator over an array of mesh values (separate output type for 2d vertices, 3d vertices, faces and normals)
///
/// Produces [`Value`]'s, where internal value can be accessed by `.val()` method.
///
/// Values produced by `.val()` depend on the mesh & data type:
/// * 2d vertices: `(f32, f32)` which represent x, y
/// * 3d vertices: `(f32, f32, f32)` which represent x, y, z
/// * indices (both 2d & 3d): `(i32, i32, i32)`
/// * normals (only for 3d): `(f32, f32, f32)`
///
/// Example:
/// ```rust
/// # use ttf2mesh::{TTFFile, Quality, Value};
/// # let mut ttf = TTFFile::from_file("./fonts/FiraMono-Medium.ttf").unwrap();
/// # let mut glyph = ttf.glyph_from_char('€').unwrap();
/// # let mut mesh_2d = glyph.to_2d_mesh(Quality::Medium).unwrap();
/// #
/// let vertices = mesh_2d.iter_vertices();
/// for vertice in vertices {
///     let value: (f32, f32) = vertice.val();
/// }
/// ```
pub struct DataIterator<'a, T: Value<'a>> {
    index: usize,
    iterable: &'a [T],
}

impl<'a, T: Value<'a>> DataIterator<'a, T> {
    pub(crate) fn new(iterable: &'a [T]) -> Self {
        Self { index: 0, iterable }
    }
}

impl<'a, T: Value<'a>> Iterator for DataIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterable.get(self.index) {
            Some(item) => {
                self.index += 1;

                Some(item)
            }
            None => None,
        }
    }
}
