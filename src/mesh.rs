use core::slice;
use std::{convert::TryInto, marker::PhantomData};

use ttf2mesh_sys as sys;

use crate::{
    output::{IteratorValue, MeshIterator},
    Error,
};

#[allow(unused_imports)]
use crate::Glyph;

/// A (2d or 3d) mesh that has been generated from a [`Glyph`]
///
/// Usage:
/// ```rust
/// # use ttf2mesh::{TTFFile, Quality};
/// # let mut ttf = TTFFile::from_file("./fonts/FiraMono-Medium.ttf").unwrap();
/// # let mut glyph = ttf.glyph_from_char('€').unwrap();
/// # let mesh_3d = glyph.to_3d_mesh(Quality::Medium, 2.).unwrap();
///
/// // vertices with for-loop
/// for vertex in mesh_3d.iter_vertices() {
///     let values: (f32, f32, f32) = vertex.val();
///     // do something
/// }
///
/// // or copying to a new vector
/// let vertices = mesh_3d.iter_vertices()
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
pub struct Mesh<'a, T: MeshPointer<'a>> {
    inner: *mut T,
    _phantom: &'a PhantomData<T>,
}

pub trait MeshPointer<'a> {
    type VertStruct: IteratorValue<'a>;
    type FaceStruct: IteratorValue<'a>;
    type NormalStruct: IteratorValue<'a>;

    fn get_vert_ptr(&self) -> *mut Self::VertStruct;
    fn get_vert_len(&self) -> usize;

    fn get_face_ptr(&self) -> *mut Self::FaceStruct;
    fn get_face_len(&self) -> usize;

    fn get_normals_ptr(&self) -> Option<*mut Self::NormalStruct>;
    fn get_normals_len(&self) -> usize;

    fn free(&mut self);
}

impl<'a> MeshPointer<'a> for sys::ttf_mesh {
    type VertStruct = sys::ttf_mesh__bindgen_ty_1;
    type FaceStruct = sys::ttf_mesh__bindgen_ty_2;
    type NormalStruct = sys::ttf_mesh__bindgen_ty_2;

    fn get_vert_ptr(&self) -> *mut Self::VertStruct {
        self.vert
    }

    fn get_vert_len(&self) -> usize {
        self.nvert.try_into().unwrap()
    }

    fn get_face_ptr(&self) -> *mut Self::FaceStruct {
        self.faces
    }

    fn get_face_len(&self) -> usize {
        self.nfaces.try_into().unwrap()
    }

    fn get_normals_ptr(&self) -> Option<*mut Self::NormalStruct> {
        None
    }

    fn get_normals_len(&self) -> usize {
        0
    }

    fn free(&mut self) {
        unsafe { sys::ttf_free_mesh(&mut *self) }
    }
}

impl<'a> MeshPointer<'a> for sys::ttf_mesh3d {
    type VertStruct = sys::ttf_mesh3d__bindgen_ty_1;
    type FaceStruct = sys::ttf_mesh3d__bindgen_ty_2;
    type NormalStruct = sys::ttf_mesh3d__bindgen_ty_3;

    fn get_vert_ptr(&self) -> *mut Self::VertStruct {
        self.vert
    }

    fn get_vert_len(&self) -> usize {
        self.nvert.try_into().unwrap()
    }

    fn get_face_ptr(&self) -> *mut Self::FaceStruct {
        self.faces
    }

    fn get_face_len(&self) -> usize {
        self.nfaces.try_into().unwrap()
    }

    fn get_normals_ptr(&self) -> Option<*mut Self::NormalStruct> {
        Some(self.normals)
    }

    fn get_normals_len(&self) -> usize {
        self.nvert.try_into().unwrap()
    }

    fn free(&mut self) {
        unsafe { sys::ttf_free_mesh3d(&mut *self) }
    }
}

impl<'a, T: MeshPointer<'a>> Mesh<'a, T> {
    pub(crate) fn from_raw(mesh: *mut T) -> Result<Self, Error> {
        Ok(Mesh {
            inner: mesh,
            _phantom: &PhantomData,
        })
    }

    /// Get an iterator of mesh vertices
    ///
    /// Produces `(x: f32, y: f32, z: f32)` tuples for 3d mesh and `(x: f32, y: f32)` tuples for 2d mesh
    pub fn iter_vertices(&'a self) -> MeshIterator<'a, <T as MeshPointer>::VertStruct> {
        let vertices =
            unsafe { slice::from_raw_parts((&*self.inner).get_vert_ptr(), self.vertices_len()) };

        MeshIterator::new(vertices)
    }

    /// Get an iterator of mesh faces (indices)
    ///
    /// Produces `(v1: i32, v2: i32, v3: i32)` tuples
    pub fn iter_faces<'b>(&'a self) -> MeshIterator<'a, <T as MeshPointer>::FaceStruct> {
        let faces =
            unsafe { slice::from_raw_parts((&*self.inner).get_face_ptr(), self.faces_len()) };

        MeshIterator::new(faces)
    }

    /// Get an iterator of mesh normals. Only for 3d mesh, always None for 2d mesh
    ///
    /// Produces `(x: f32, y: f32, z: f32)` tuples for 3d mesh
    pub fn iter_normals<'b>(
        &'a self,
    ) -> Option<MeshIterator<'a, <T as MeshPointer>::NormalStruct>> {
        let ptr = match unsafe { &*self.inner }.get_normals_ptr() {
            Some(ptr) => ptr,
            None => return None,
        };

        let normals = unsafe { slice::from_raw_parts(ptr, self.normals_len()) };

        Some(MeshIterator::new(normals))
    }

    /// Get the count of vertices
    pub fn vertices_len(&self) -> usize {
        unsafe { &*self.inner }.get_vert_len()
    }

    /// Get the count of faces (indices)
    pub fn faces_len(&self) -> usize {
        unsafe { &*self.inner }.get_face_len()
    }

    /// Get the count of normals (always zero for 2d meshes)
    pub fn normals_len(&self) -> usize {
        unsafe { &*self.inner }.get_normals_len()
    }
}

impl<'a, T: MeshPointer<'a>> Drop for Mesh<'a, T> {
    fn drop(&mut self) {
        unsafe { &mut *self.inner }.free();
    }
}
