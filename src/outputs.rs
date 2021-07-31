use std::fmt;

use ttf2mesh_sys as sys;

pub trait IteratorValue<'a> {
    type Output: fmt::Debug;

    fn value(&self) -> Self::Output;
}

impl<'a> IteratorValue<'a> for sys::ttf_mesh__bindgen_ty_1 {
    type Output = (f32, f32);

    fn value(&self) -> Self::Output {
        (self.x, self.y)
    }
}

impl<'a> IteratorValue<'a> for sys::ttf_mesh__bindgen_ty_2 {
    type Output = (i32, i32, i32);

    fn value(&self) -> Self::Output {
        (self.v1, self.v2, self.v3)
    }
}

impl<'a> IteratorValue<'a> for sys::ttf_mesh3d__bindgen_ty_1 {
    type Output = (f32, f32, f32);

    fn value(&self) -> Self::Output {
        (self.x, self.y, self.z)
    }
}

impl<'a> IteratorValue<'a> for sys::ttf_mesh3d__bindgen_ty_2 {
    type Output = (i32, i32, i32);

    fn value(&self) -> Self::Output {
        (self.v1, self.v2, self.v3)
    }
}

impl<'a> IteratorValue<'a> for sys::ttf_mesh3d__bindgen_ty_3 {
    type Output = (f32, f32, f32);

    fn value(&self) -> Self::Output {
        (self.x, self.y, self.z)
    }
}

pub struct MeshIterator<'a, T: IteratorValue<'a>> {
    index: usize,
    iterable: &'a [T],
}

impl<'a, T: IteratorValue<'a>> MeshIterator<'a, T> {
    pub fn new(iterable: &'a [T]) -> Self {
        Self { index: 0, iterable }
    }
}

impl<'a, T: IteratorValue<'a>> Iterator for MeshIterator<'a, T> {
    type Item = Value<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterable.get(self.index) {
            Some(item) => {
                self.index += 1;

                Some(Value { inner: item })
            }
            None => None,
        }
    }
}

pub struct Value<'a, T> {
    inner: &'a T,
}

impl<'a, T: IteratorValue<'a>> Value<'a, T> {
    pub fn value(&self) -> T::Output {
        self.inner.value()
    }
}

impl<'a, T: IteratorValue<'a>> fmt::Debug for Value<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value {{ {:?} }}", self.value())
    }
}
