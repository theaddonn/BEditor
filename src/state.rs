use crate::nbt_view::NbtView;

pub enum BEditorState {
    /// Start Screen
    Idle,
    NbtView(NbtView),
}
