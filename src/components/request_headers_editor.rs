use anathema::{component::Component, state::State};

#[derive(Default)]
pub struct RequestHeadersEditor;

#[derive(Default)]
pub struct RequestHeadersEditorState {}

impl anathema::state::TypeId for RequestHeadersEditorState {
    const TYPE: anathema::state::Type = anathema::state::Type::Composite;
}
impl anathema::state::State for RequestHeadersEditorState {
    fn type_info(&self) -> anathema::state::Type {
        anathema::state::Type::Composite
    }
}
impl anathema::state::AnyMap for RequestHeadersEditorState {
    fn lookup(&self, key: &str) -> Option<anathema::state::PendingValue> {
        match key {
            _ => None,
        }
    }
}

impl RequestHeadersEditorState {
    pub fn new() -> Self {
        RequestHeadersEditorState {}
    }
}

impl Component for RequestHeadersEditor {
    type State = RequestHeadersEditorState;
    type Message = ();
}
