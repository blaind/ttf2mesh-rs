use ttf2mesh_sys as sys;

pub struct VertexIterator<'a> {
    pub(crate) index: usize,
    pub(crate) vertices: &'a [sys::ttf_mesh__bindgen_ty_1],
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
    pub(crate) index: usize,
    pub(crate) vertices: &'a [sys::ttf_mesh3d__bindgen_ty_1],
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
    pub(crate) index: usize,
    pub(crate) faces: &'a [T],
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
