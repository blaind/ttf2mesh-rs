use std::{convert::TryInto, mem::MaybeUninit};

use ttf2mesh_sys as sys;

use crate::{Error, Mesh2d, Mesh3d, Quality};

pub struct Glyph<'a> {
    inner: &'a mut sys::ttf_glyph,
}

impl<'a> Glyph<'a> {
    pub(crate) fn from_raw(raw: &'a mut sys::ttf_glyph) -> Self {
        Self { inner: raw }
    }

    pub fn to_2d_mesh(&mut self, quality: Quality) -> Result<Mesh2d, Error> {
        let mut mesh = MaybeUninit::uninit();

        let features = sys::TTF_FEATURES_DFLT;

        let error = unsafe {
            sys::ttf_glyph2mesh(
                self.inner,
                mesh.as_mut_ptr(),
                quality.as_u8(),
                features.try_into().unwrap(),
            )
        };

        if error != ttf2mesh_sys::TTF_DONE as i32 {
            return Err(Error::Glyph2MeshError);
        }

        let mesh = unsafe { mesh.assume_init() };
        Ok(Mesh2d::from_raw(mesh)?)
    }

    pub fn to_3d_mesh(&mut self, quality: Quality, depth: f32) -> Result<Mesh3d, Error> {
        let mut mesh = MaybeUninit::uninit();

        let features = sys::TTF_FEATURES_DFLT;

        let error = unsafe {
            sys::ttf_glyph2mesh3d(
                self.inner,
                mesh.as_mut_ptr(),
                quality.as_u8(),
                features.try_into().unwrap(),
                depth,
            )
        };

        if error != ttf2mesh_sys::TTF_DONE as i32 {
            return Err(Error::Glyph2MeshError);
        }

        let mesh = unsafe { mesh.assume_init() };
        Ok(Mesh3d::from_raw(mesh)?)
    }
}
