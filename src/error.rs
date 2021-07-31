#[derive(Debug)]
pub enum Error {
    FontLoadError,
    ObjExportError,
    Glyph2MeshError,
    GlyphNotFound,
    NoCharacterFound,
    QualityParse,
    FileNotFound,
}
