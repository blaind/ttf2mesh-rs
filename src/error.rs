use core::fmt;

/// Represents an error in the library
#[derive(Debug)]
pub enum Error {
    /// Font could not be loaded. The library doesn't support all font types
    FontLoadError(Ttf2MeshError),

    /// Font could not be exported to an obj file
    ObjExportError(Ttf2MeshError),

    /// Mesh generation failed
    Glyph2MeshError(Ttf2MeshError),

    /// Glyph is not found in the font file
    GlyphNotFound,

    /// Quality could not be parsed from input
    QualityParse(String),

    /// File to be opened was not found
    FileNotFound(Option<String>),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FontLoadError(error) => write!(f, "could not load font: {}", error),
            Error::ObjExportError(error) => write!(f, "could not export obj file: {}", error),
            Error::Glyph2MeshError(error) => {
                write!(f, "glyph could not be converted to mesh: {}", error)
            }
            Error::GlyphNotFound => write!(f, "the font file does not contain requested glyph"),
            Error::QualityParse(value) => write!(
                f,
                "quality could not be parsed from input ({}), please set to high, medium or low",
                value
            ),
            Error::FileNotFound(path) => write!(f, "file {:?} was not found", path),
        }
    }
}

/// Error from ttf2mesh library
#[derive(Debug)]
pub enum Ttf2MeshError {
    // not enough memory
    NoMem,

    // file size > TTF_MAX_FILE
    Size,

    // error opening file
    Open,

    // unsupported file version
    Ver,

    // invalid file structure
    Fmt,

    // no required tables in file
    NoTab,

    // invalid file or table checksum
    CSum,

    // unsupported table format
    UTab,

    // unable to create mesh
    Mesher,

    // glyph has no outline
    NoOutline,

    // error writing file
    Writing,

    // unknown error
    Unknown,
}

impl std::error::Error for Ttf2MeshError {}

impl fmt::Display for Ttf2MeshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ttf2MeshError::NoMem => write!(f, "not enough memory (malloc failed)"),
            Ttf2MeshError::Size => write!(f, "file size > TTF_MAX_FILE"),
            Ttf2MeshError::Open => write!(f, "error opening file"),
            Ttf2MeshError::Ver => write!(f, "unsupported file version"),
            Ttf2MeshError::Fmt => write!(f, "invalid file structure"),
            Ttf2MeshError::NoTab => write!(f, "no required tables in file"),
            Ttf2MeshError::CSum => write!(f, "invalid file or table checksum"),
            Ttf2MeshError::UTab => write!(f, "unsupported table format"),
            Ttf2MeshError::Mesher => write!(f, "unable to create mesh"),
            Ttf2MeshError::NoOutline => write!(f, "glyph has no outline"),
            Ttf2MeshError::Writing => write!(f, "error writing file"),
            Ttf2MeshError::Unknown => write!(f, "unknown error"),
        }
    }
}

impl From<i32> for Ttf2MeshError {
    fn from(value: i32) -> Self {
        match value {
            1 => Ttf2MeshError::NoMem,
            2 => Ttf2MeshError::Size,
            3 => Ttf2MeshError::Open,
            4 => Ttf2MeshError::Ver,
            5 => Ttf2MeshError::Fmt,
            6 => Ttf2MeshError::NoTab,
            7 => Ttf2MeshError::CSum,
            8 => Ttf2MeshError::UTab,
            9 => Ttf2MeshError::Mesher,
            10 => Ttf2MeshError::NoOutline,
            11 => Ttf2MeshError::Writing,
            _ => Ttf2MeshError::Unknown,
        }
    }
}
