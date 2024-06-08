use crate::nbt_view::{NbtEndian, NbtHeader};

#[derive(Debug, Clone)]
pub enum BEditorMessage {
    NbtViewSetPath(String),
    NbtViewSetEndian(NbtEndian),
    NbtViewSetHeader(NbtHeader),
    NbtViewRefresh,
}
