use std::{convert::TryInto, mem::MaybeUninit};

use ttf2mesh_sys as sys;

use crate::{mesh::Mesh, Error, Quality};

pub struct Glyph<'a> {
    inner: &'a mut sys::ttf_glyph,
}

impl<'a> Glyph<'a> {
    pub(crate) fn from_raw(raw: &'a mut sys::ttf_glyph) -> Self {
        Self { inner: raw }
    }

    /// Generate a 2d mesh from the glyph with desired [`Quality`]
    pub fn to_2d_mesh<'b>(&mut self, quality: Quality) -> Result<Mesh<'b, sys::ttf_mesh>, Error> {
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
        Ok(Mesh::from_raw(mesh)?)
    }

    /// Generate a 3d mesh from the glyph with desired [`Quality`] and `depth`
    pub fn to_3d_mesh<'b>(
        &mut self,
        quality: Quality,
        depth: f32,
    ) -> Result<Mesh<'b, sys::ttf_mesh3d>, Error> {
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
        Ok(Mesh::from_raw(mesh)?)
    }
}
