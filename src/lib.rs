#![feature(test)]
extern crate test;

use core::slice;

use std::{
    convert::TryInto, ffi::CString, mem::MaybeUninit, os::unix::prelude::OsStrExt, path::Path,
};

use ttf2mesh_sys as sys;

// TODO: support TTF_FEATURE_IGN_ERR as bitflag

pub struct TTFFile {
    ttf: *mut sys::ttf_file,
}

impl TTFFile {
    //pub fn from_system_font() -> Result<TTFFile, Error> {
    /*
    // list all system fonts by filename mask:

    ttf_t **list = ttf_list_system_fonts("DejaVuSans*|Ubuntu*|FreeSerif*|Arial*|Cour*");
    if (list == NULL) return false; // no memory in system
    if (list[0] == NULL) return false; // no fonts were found

    // load the first font from the list

    ttf_load_from_file(list[0]->filename, &font, false);
    ttf_free_list(list);
    if (font == NULL) return false;

    printf("font \"%s\" loaded\n", font->names.full_name);
    return true;
        */
    //}

    // data needs to be mutable (modified by ttf_*), hence vec
    pub fn from_buffer_vec(data: Vec<u8>) -> Result<TTFFile, Error> {
        let mut ttf = MaybeUninit::uninit();
        let error = unsafe {
            sys::ttf_load_from_mem(
                data.as_ptr(),
                data.len().try_into().unwrap(),
                ttf.as_mut_ptr(),
                false,
            )
        };
        Self::load(ttf, error)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<TTFFile, Error> {
        let file_name = path_to_cstring(path);

        let mut ttf = MaybeUninit::uninit();
        let error = unsafe { sys::ttf_load_from_file(file_name.as_ptr(), ttf.as_mut_ptr(), false) };

        Self::load(ttf, error)
    }

    fn load(ttf: MaybeUninit<*mut sys::ttf_file>, error: i32) -> Result<TTFFile, Error> {
        if error != ttf2mesh_sys::TTF_DONE as i32 {
            // fprintf(stderr, "Unable to load font: %s\n", ttf_error_str[error]);
            return Err(Error::FontLoadError);
        }

        Ok(Self {
            ttf: unsafe { ttf.assume_init() },
        })
    }

    pub fn export_to_obj<P: AsRef<Path>>(
        &mut self,
        obj_path: P,
        quality: Quality,
    ) -> Result<(), Error> {
        let file_name = path_to_cstring(obj_path);

        let error =
            unsafe { sys::ttf_export_to_obj(self.ttf, file_name.as_ptr(), quality.as_u8()?) };

        if error != ttf2mesh_sys::TTF_DONE as i32 {
            // fprintf(stderr, "Unable to export font: %s\n", ttf_error_str[error]);
            return Err(Error::ObjExportError);
        }

        Ok(())
    }

    pub fn glyph_by_char<'a>(&'a mut self, char: char) -> Result<Glyph<'a>, Error> {
        let mut bytes = [0; 2];
        char.encode_utf16(&mut bytes);

        let index = unsafe { sys::ttf_find_glyph(self.ttf, bytes[0]) };

        if index < 0 {
            return Err(Error::GlyphNotFound);
        }

        self.glyph_by_index(index.try_into().unwrap())
    }

    pub fn glyph_count(&self) -> usize {
        unsafe { *self.ttf }.nglyphs.try_into().unwrap()
    }

    pub fn glyph_by_index<'a>(&'a mut self, index: usize) -> Result<Glyph<'a>, Error> {
        let glyphs = unsafe { slice::from_raw_parts_mut((*self.ttf).glyphs, self.glyph_count()) };

        match glyphs.get_mut(index) {
            Some(glyph) => Ok(Glyph::from_raw(glyph)),
            None => Err(Error::GlyphNotFound),
        }
    }
}

impl Drop for TTFFile {
    fn drop(&mut self) {
        unsafe { sys::ttf_free(self.ttf) }
    }
}

pub struct Glyph<'a> {
    inner: &'a mut sys::ttf_glyph,
}

impl<'a> Glyph<'a> {
    fn from_raw(raw: &'a mut sys::ttf_glyph) -> Self {
        Self { inner: raw }
    }

    pub fn to_2d_mesh(&mut self, quality: Quality) -> Result<Mesh2d, Error> {
        let mut mesh = MaybeUninit::uninit();

        let features = sys::TTF_FEATURES_DFLT;

        let error = unsafe {
            sys::ttf_glyph2mesh(
                self.inner,
                mesh.as_mut_ptr(),
                quality.as_u8()?,
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
                quality.as_u8()?,
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

pub struct Mesh2d {
    inner: *mut sys::ttf_mesh,
}

// TODO: handle        ttf_outline_t *outline;       /* see ttf_linear_outline() */ ?
impl Mesh2d {
    fn from_raw(mesh: *mut sys::ttf_mesh) -> Result<Mesh2d, Error> {
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
    fn from_raw(mesh: *mut sys::ttf_mesh3d) -> Result<Mesh3d, Error> {
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
pub struct VertexIterator<'a> {
    index: usize,
    vertices: &'a [sys::ttf_mesh__bindgen_ty_1],
}

impl<'a> Iterator for VertexIterator<'a> {
    type Item = Vertex2d<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.vertices.get(self.index) {
            Some(vertex) => {
                self.index += 1;

                Some(Vertex2d { vertex })
            }
            None => None,
        }
    }
}

pub struct Vertex2d<'a> {
    vertex: &'a sys::ttf_mesh__bindgen_ty_1,
}

impl<'a> Vertex2d<'a> {
    pub fn get(&self) -> (f32, f32) {
        (self.vertex.x, self.vertex.y)
    }
}

impl<'a> std::fmt::Debug for Vertex2d<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.get();
        write!(f, "Vertex2d {{ x={:.5}, y={:.5} }}", x, y)
    }
}

pub struct Vertex3dIterator<'a> {
    index: usize,
    vertices: &'a [sys::ttf_mesh3d__bindgen_ty_1],
}

impl<'a> Iterator for Vertex3dIterator<'a> {
    type Item = Vertex3d<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.vertices.get(self.index) {
            Some(vertex) => {
                self.index += 1;

                Some(Vertex3d { vertex })
            }
            None => None,
        }
    }
}

pub struct Vertex3d<'a> {
    vertex: &'a sys::ttf_mesh3d__bindgen_ty_1,
}

impl<'a> Vertex3d<'a> {
    pub fn get(&self) -> (f32, f32, f32) {
        (self.vertex.x, self.vertex.y, self.vertex.z)
    }
}

impl<'a> std::fmt::Debug for Vertex3d<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y, z) = self.get();
        write!(f, "Vertex3d {{ x={:.5}, y={:.5}, z={:.5} }}", x, y, z)
    }
}

pub struct FacesIterator<'a, T> {
    index: usize,
    faces: &'a [T],
}

impl<'a, T> Iterator for FacesIterator<'a, T> {
    type Item = Face<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.faces.get(self.index) {
            Some(face) => {
                self.index += 1;

                Some(Face { face })
            }
            None => None,
        }
    }
}

pub struct Face<'a, T> {
    face: &'a T,
}

impl<'a, T: FaceValues> Face<'a, T> {
    pub fn get(&self) -> (i32, i32, i32) {
        self.face.get()
    }

    pub fn get_f32(&self) -> (f32, f32, f32) {
        self.face.get_f32()
    }
}

impl<'a, T: FaceValues> std::fmt::Debug for Face<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vals = self.get();
        write!(f, "Face[{}, {}, {}]", vals.0, vals.1, vals.2)
    }
}

pub trait FaceValues {
    fn get(&self) -> (i32, i32, i32) {
        (0, 0, 0)
    }
    fn get_f32(&self) -> (f32, f32, f32) {
        (0., 0., 0.)
    }
}

impl FaceValues for sys::ttf_mesh3d__bindgen_ty_2 {
    fn get(&self) -> (i32, i32, i32) {
        (self.v1, self.v2, self.v3)
    }
}

impl FaceValues for sys::ttf_mesh3d__bindgen_ty_3 {
    fn get_f32(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

impl FaceValues for sys::ttf_mesh__bindgen_ty_2 {
    fn get(&self) -> (i32, i32, i32) {
        (self.v1, self.v2, self.v3)
    }
}

#[derive(Debug)]
pub enum Error {
    FontLoadError,
    ObjExportError,
    Glyph2MeshError,
    GlyphNotFound,
    NoCharacterFound,
}

pub enum Quality {
    // 10
    Low,
    // 20
    Medium,
    // 50
    High,
    // customly set
    Custom(usize),
}

impl Quality {
    fn as_u8(&self) -> Result<u8, Error> {
        Ok(match self {
            Quality::Low => 10,
            Quality::Medium => 20,
            Quality::High => 50,
            Quality::Custom(value) => (*value).try_into().unwrap(),
        })
    }
}

fn path_to_cstring<P: AsRef<Path>>(path: P) -> CString {
    CString::new(path.as_ref().as_os_str().as_bytes()).unwrap()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use test::Bencher;

    fn get_font_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts")
    }

    fn get_font(font_file: Option<&str>) -> PathBuf {
        match font_file {
            Some(file) => get_font_path().join(file),
            None => get_font_path().join("FiraMono-Medium.ttf"),
        }
    }

    fn read_font(font_file: Option<&str>) -> Vec<u8> {
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
    fn test_get_glyph_by_char() {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();
        let char = "A".chars().next().unwrap();
        let _ = font.glyph_by_char(char).unwrap();
    }

    #[test]
    fn test_to_3d_mesh() {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();
        let mut glyph = font.glyph_by_char("€".chars().next().unwrap()).unwrap();
        let mesh = glyph.to_3d_mesh(Quality::Low, 0.5).unwrap();

        let mut sizes = Vec::new();
        sizes.extend_from_slice(&[
            mesh.iter_vertices().collect::<Vec<_>>().len(),
            mesh.iter_normals().collect::<Vec<_>>().len(),
            mesh.iter_faces().collect::<Vec<_>>().len(),
        ]);

        let mesh = glyph.to_3d_mesh(Quality::High, 1.5).unwrap();

        sizes.extend_from_slice(&[
            mesh.iter_vertices().collect::<Vec<_>>().len(),
            mesh.iter_normals().collect::<Vec<_>>().len(),
            mesh.iter_faces().collect::<Vec<_>>().len(),
        ]);

        let mesh = glyph.to_3d_mesh(Quality::Custom(255), 0.5).unwrap();

        sizes.extend_from_slice(&[
            mesh.iter_vertices().collect::<Vec<_>>().len(),
            mesh.iter_normals().collect::<Vec<_>>().len(),
            mesh.iter_faces().collect::<Vec<_>>().len(),
        ]);

        assert_eq!(sizes, &[246, 246, 160, 552, 552, 364, 1164, 1164, 772]);
    }

    #[test]
    fn test_to_2d_mesh() {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();
        let mut glyph = font.glyph_by_char("€".chars().next().unwrap()).unwrap();

        let mut sizes = Vec::new();
        let mesh = glyph.to_2d_mesh(Quality::Low).unwrap();
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

    #[bench]
    fn bench_open_font(b: &mut Bencher) {
        let buffer = read_font(None);

        b.iter(|| {
            let _ = TTFFile::from_buffer_vec(buffer.clone()).unwrap();
        });
    }

    #[bench]
    fn bench_get_glyph(b: &mut Bencher) {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();
        let char = "€".chars().next().unwrap();

        b.iter(|| {
            let _ = font.glyph_by_char(char).unwrap();
        });
    }

    #[bench]
    fn bench_glyph_to_3d_mesh_low_quality(b: &mut Bencher) {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();

        let char = "€".chars().next().unwrap();
        let mut glyph = font.glyph_by_char(char).unwrap();

        b.iter(|| {
            let _ = glyph.to_3d_mesh(Quality::Low, 0.1).unwrap();
        });
    }

    #[bench]
    fn bench_glyph_to_3d_mesh_high_quality(b: &mut Bencher) {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();

        let char = "€".chars().next().unwrap();
        let mut glyph = font.glyph_by_char(char).unwrap();

        b.iter(|| {
            let _ = glyph.to_3d_mesh(Quality::High, 0.1).unwrap();
        });
    }

    #[bench]
    fn bench_glyph_to_2d_mesh_low_quality(b: &mut Bencher) {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();

        let char = "€".chars().next().unwrap();
        let mut glyph = font.glyph_by_char(char).unwrap();

        b.iter(|| {
            let _ = glyph.to_2d_mesh(Quality::Low).unwrap();
        });
    }

    #[bench]
    fn bench_glyph_to_2d_mesh_high_quality(b: &mut Bencher) {
        let mut font = TTFFile::from_buffer_vec(read_font(None)).unwrap();

        let char = "€".chars().next().unwrap();
        let mut glyph = font.glyph_by_char(char).unwrap();

        b.iter(|| {
            let _ = glyph.to_2d_mesh(Quality::High).unwrap();
        });
    }
}
