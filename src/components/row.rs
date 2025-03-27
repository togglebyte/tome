use anathema::component::Component;

#[derive(Default)]
pub struct Row;

#[derive(Default)]
pub struct RowState {}

impl anathema::state::TypeId for RowState {
    const TYPE: anathema::state::Type = anathema::state::Type::Unit;
}
impl anathema::state::State for RowState {
    fn type_info(&self) -> anathema::state::Type {
        anathema::state::Type::Unit
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
