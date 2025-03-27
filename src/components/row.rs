use anathema::{component::Component, derive::State};

#[derive(Default)]
pub struct Row;

#[derive(Default)]
pub struct RowState {}

impl anathema::state::TypeId for RowState {
    const TYPE: anathema::state::Type = anathema::state::Type::Composite;
}
impl anathema::state::State for RowState {
    fn type_info(&self) -> anathema::state::Type {
        anathema::state::Type::Composite
    }
}
impl anathema::state::AnyMap for RowState {
    fn lookup(&self, key: &str) -> Option<anathema::state::PendingValue> {
        match key {
            _ => None,
        }
    }
}
impl RowState {
    pub fn new() -> Self {
        RowState {}
    }
}

impl Component for Row {
    type State = RowState;
    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }
}
