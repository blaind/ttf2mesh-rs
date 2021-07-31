use core::slice;

use std::{convert::TryInto, mem::MaybeUninit, path::Path};

use ttf2mesh_sys as sys;

use crate::{path_to_cstring, Error, Glyph, Quality};

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
        if !Path::new(path.as_ref().as_os_str()).exists() {
            return Err(Error::FileNotFound);
        }

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
            unsafe { sys::ttf_export_to_obj(self.ttf, file_name.as_ptr(), quality.as_u8()) };

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
