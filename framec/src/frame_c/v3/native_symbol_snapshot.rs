use crate::frame_c::visitors::TargetLanguage;

#[derive(Debug, Clone, Default)]
pub struct NativeParamsSnapshot {
    pub handler_params: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NativeSymbolSnapshotV3 {
    pub language: TargetLanguage,
    pub params: Option<NativeParamsSnapshot>,
}

/// Build a native symbol snapshot for a spliced body. For Stage 10A this is a
/// scaffold that returns an empty snapshot; later we will populate handler
/// parameter metadata for Py/TS using native parsers mapped via splice_map.
pub fn build_native_symbol_snapshot_for_spliced(
    language: TargetLanguage,
    _spliced_text: &str,
) -> NativeSymbolSnapshotV3 {
    NativeSymbolSnapshotV3 { language, params: None }
}
