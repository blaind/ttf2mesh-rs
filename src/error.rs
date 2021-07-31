/// Represents an error by the library
#[derive(Debug)]
pub enum Error {
    /// Font could not be loaded. The library doesn't support all font types
    FontLoadError,

    /// Font could not be exported to an obj file
    ObjExportError,

    /// Mesh generation failed
    Glyph2MeshError,

    /// Glyph is not found in the font file
    GlyphNotFound,

    /// Quality could not be parsed from input
    QualityParse,

    /// File to be opened was not found
    FileNotFound,
}
