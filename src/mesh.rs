use core::slice;
use std::{convert::TryInto, marker::PhantomData};

use ttf2mesh_sys as sys;

use crate::{
    output::{DataIterator, Value},
    Error,
};

pub type Mesh3d = ttf2mesh_sys::ttf_mesh3d;
pub type Mesh2d = ttf2mesh_sys::ttf_mesh;

type Vert2d = sys::ttf_mesh__bindgen_ty_1;
type Face2d = sys::ttf_mesh__bindgen_ty_2;
type Vert3d = sys::ttf_mesh3d__bindgen_ty_1;
type Face3d = sys::ttf_mesh3d__bindgen_ty_2;
type Normal = sys::ttf_mesh3d__bindgen_ty_3;

#[allow(unused_imports)]
use crate::Glyph;

/// A (2d or 3d) mesh that has been generated from a [`Glyph`]
///
/// Usage:
/// ```rust
/// # use ttf2mesh::{TTFFile, Quality, Value};
/// # let mut ttf = TTFFile::from_file("./fonts/FiraMono-Medium.ttf").unwrap();
/// # let mut glyph = ttf.glyph_from_char('€').unwrap();
/// # let mesh_3d = glyph.to_3d_mesh(Quality::Medium, 2.).unwrap();
/// #
/// let vertices_3d = mesh_3d.iter_vertices()
///     .map(|v| v.val())
///     .collect::<Vec<(f32, f32, f32)>>();
///
/// let faces = mesh_3d.iter_faces()
///     .map(|v| v.val())
///     .collect::<Vec<(i32, i32, i32)>>();
///
/// let normals = mesh_3d.iter_normals().unwrap()
///     .map(|v| v.val())
///     .collect::<Vec<(f32, f32, f32)>>();
/// ```
pub struct Mesh<'a, T: InnerMesh<'a>> {
    inner: *mut T,
    _phantom: &'a PhantomData<T>,
}

/// Representation of `ttf2mesh` internal mesh structure
///
/// Do not use methods directly, but rather use [`Value`] methods
pub trait InnerMesh<'a> {
    /// Value type for a vertices iterator
    type VertStruct: Value<'a>;

    /// Value type for a faces iterator
    type FaceStruct: Value<'a>;

    /// Value type for a normals iterator
    type NormalStruct: Value<'a>;

    fn vertices_len(&self) -> usize;
    fn faces_len(&self) -> usize;
    fn normals_len(&self) -> usize;

    fn vert_ptr(&self) -> *mut Self::VertStruct;
    fn face_ptr(&self) -> *mut Self::FaceStruct;
    fn normals_ptr(&self) -> Option<*mut Self::NormalStruct>;

    unsafe fn free(&mut self);
}

impl<'a> InnerMesh<'a> for Mesh2d {
    type VertStruct = Vert2d;
    type FaceStruct = Face2d;
    type NormalStruct = Normal;

    fn vert_ptr(&self) -> *mut Self::VertStruct {
        self.vert
    }

    fn vertices_len(&self) -> usize {
        self.nvert.try_into().unwrap()
    }

    fn face_ptr(&self) -> *mut Self::FaceStruct {
        self.faces
    }

    fn faces_len(&self) -> usize {
        self.nfaces.try_into().unwrap()
    }

    fn normals_ptr(&self) -> Option<*mut Self::NormalStruct> {
        None
    }

    fn normals_len(&self) -> usize {
        0
    }

    unsafe fn free(&mut self) {
        sys::ttf_free_mesh(&mut *self)
    }
}

impl<'a> InnerMesh<'a> for Mesh3d {
    type VertStruct = Vert3d;
    type FaceStruct = Face3d;
    type NormalStruct = Normal;

    fn vert_ptr(&self) -> *mut Self::VertStruct {
        self.vert
    }

    fn vertices_len(&self) -> usize {
        self.nvert.try_into().unwrap()
    }

    fn face_ptr(&self) -> *mut Self::FaceStruct {
        self.faces
    }

    fn faces_len(&self) -> usize {
        self.nfaces.try_into().unwrap()
    }

    fn normals_ptr(&self) -> Option<*mut Self::NormalStruct> {
        Some(self.normals)
    }

    fn normals_len(&self) -> usize {
        self.nvert.try_into().unwrap()
    }

    unsafe fn free(&mut self) {
        sys::ttf_free_mesh3d(&mut *self)
    }
}

impl<'a, T: InnerMesh<'a>> Mesh<'a, T> {
    pub(crate) fn from_raw(mesh: *mut T) -> Result<Self, Error> {
        Ok(Mesh {
            inner: mesh,
            _phantom: &PhantomData,
        })
    }

    /// Get an iterator of mesh vertices
    ///
    /// Produces `(x: f32, y: f32, z: f32)` tuples for 3d mesh and `(x: f32, y: f32)` tuples for 2d mesh
    pub fn iter_vertices(&'a self) -> DataIterator<'a, <T as InnerMesh>::VertStruct> {
        let vertices =
            unsafe { slice::from_raw_parts((&*self.inner).vert_ptr(), self.vertices_len()) };

        DataIterator::new(vertices)
    }

    /// Get an iterator of mesh faces (indices)
    ///
    /// Produces `(v1: i32, v2: i32, v3: i32)` tuples
    pub fn iter_faces<'b>(&'a self) -> DataIterator<'a, <T as InnerMesh>::FaceStruct> {
        let faces = unsafe { slice::from_raw_parts((&*self.inner).face_ptr(), self.faces_len()) };

        DataIterator::new(faces)
    }

    /// Get an iterator of mesh normals. Only for 3d mesh, always None for 2d mesh
    ///
    /// Produces `(x: f32, y: f32, z: f32)` tuples for 3d mesh
    pub fn iter_normals<'b>(&'a self) -> Option<DataIterator<'a, <T as InnerMesh>::NormalStruct>> {
        let ptr = match unsafe { &*self.inner }.normals_ptr() {
            Some(ptr) => ptr,
            None => return None,
        };

        let normals = unsafe { slice::from_raw_parts(ptr, self.normals_len()) };

        Some(DataIterator::new(normals))
    }

    /// Get the count of vertices
    pub fn vertices_len(&self) -> usize {
        unsafe { &*self.inner }.vertices_len()
    }

    /// Get the count of faces (indices)
    pub fn faces_len(&self) -> usize {
        unsafe { &*self.inner }.faces_len()
    }

    /// Get the count of normals (always zero for 2d meshes)
    pub fn normals_len(&self) -> usize {
        unsafe { &*self.inner }.normals_len()
    }
}

impl<'a, T: InnerMesh<'a>> Drop for Mesh<'a, T> {
    fn drop(&mut self) {
        unsafe { (&mut *self.inner).free() }
    }
}
