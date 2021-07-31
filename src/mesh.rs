use core::slice;
use std::convert::TryInto;

use ttf2mesh_sys as sys;

use crate::{
    outputs::{FacesIterator, Vertex3dIterator, VertexIterator},
    Error,
};

pub struct Mesh2d {
    inner: *mut sys::ttf_mesh,
}

// TODO: handle        ttf_outline_t *outline;       /* see ttf_linear_outline() */ ?
impl Mesh2d {
    pub(crate) fn from_raw(mesh: *mut sys::ttf_mesh) -> Result<Mesh2d, Error> {
        Ok(Mesh2d { inner: mesh })
    }

    pub fn iter_vertices<'a>(&'a self) -> VertexIterator<'a> {
        let vertices = unsafe {
            slice::from_raw_parts((*self.inner).vert, (*self.inner).nvert.try_into().unwrap())
        };

        VertexIterator { index: 0, vertices }
    }

    pub fn vertex_len(&self) -> usize {
        unsafe { *self.inner }.nvert.try_into().unwrap()
    }

    pub fn face_len(&self) -> usize {
        unsafe { *self.inner }.nfaces.try_into().unwrap()
    }

    pub fn iter_faces<'a>(&'a self) -> FacesIterator<'a, sys::ttf_mesh__bindgen_ty_2> {
        let faces = unsafe {
            slice::from_raw_parts(
                (*self.inner).faces,
                (*self.inner).nfaces.try_into().unwrap(),
            )
        };

        FacesIterator { index: 0, faces }
    }
}

impl Drop for Mesh2d {
    fn drop(&mut self) {
        unsafe { sys::ttf_free_mesh(self.inner) }
    }
}

pub struct Mesh3d {
    inner: *mut sys::ttf_mesh3d,
}

// TODO: handle        ttf_outline_t *outline;       /* see ttf_linear_outline() */ ?
impl Mesh3d {
    pub(crate) fn from_raw(mesh: *mut sys::ttf_mesh3d) -> Result<Mesh3d, Error> {
        Ok(Mesh3d { inner: mesh })
    }

    pub fn iter_vertices<'a>(&'a self) -> Vertex3dIterator<'a> {
        let vertices = unsafe {
            slice::from_raw_parts((*self.inner).vert, (*self.inner).nvert.try_into().unwrap())
        };

        Vertex3dIterator { index: 0, vertices }
    }

    pub fn vertex_len(&self) -> usize {
        unsafe { *self.inner }.nvert.try_into().unwrap()
    }

    pub fn face_len(&self) -> usize {
        unsafe { *self.inner }.nfaces.try_into().unwrap()
    }

    pub fn iter_faces<'a>(&'a self) -> FacesIterator<'a, sys::ttf_mesh3d__bindgen_ty_2> {
        let faces = unsafe {
            slice::from_raw_parts(
                (*self.inner).faces,
                (*self.inner).nfaces.try_into().unwrap(),
            )
        };

        FacesIterator { index: 0, faces }
    }

    pub fn iter_normals<'a>(&'a self) -> FacesIterator<'a, sys::ttf_mesh3d__bindgen_ty_3> {
        let normals = unsafe {
            slice::from_raw_parts(
                (*self.inner).normals,
                (*self.inner).nvert.try_into().unwrap(), // FIXME check ok?
            )
        };

        FacesIterator {
            index: 0,
            faces: normals,
        }
    }
}

impl Drop for Mesh3d {
    fn drop(&mut self) {
        unsafe { sys::ttf_free_mesh3d(self.inner) }
    }
}
