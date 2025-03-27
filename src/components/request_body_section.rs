use anathema::{component::Component, state::AnyState};

#[derive(Default)]
pub struct RequestBodySection;

#[derive(Default)]
pub struct RequestBodySectionState {}

impl anathema::state::TypeId for RequestBodySectionState {
    const TYPE: anathema::state::Type = anathema::state::Type::Composite;
}
impl anathema::state::State for RequestBodySectionState {
    fn type_info(&self) -> anathema::state::Type {
        anathema::state::Type::Composite
    }
}
impl anathema::state::AnyMap for RequestBodySectionState {
    fn lookup(&self, _key: &str) -> Option<anathema::state::PendingValue> {
        None
    }
}

impl Component for RequestBodySection {
    type State = RequestBodySectionState;
    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }

    fn receive(
        &mut self,
        ident: &str,
        value: &dyn AnyState,
        _state: &mut Self::State,
        mut elements: anathema::component::Children,
        context: anathema::prelude::Context<'_, '_, Self::State>,
    ) {
        if let "request_body_border" = ident {
            let &focus = value.to::<bool>();
            if focus {
                return;
            }

            let Some(border_color) = context.attribute("border_color") else {
                return;
            };
            let Some(color) = border_color.as_str() else {
                return;
            };

            let color = color.to_string().leak();

            elements
                .elements()
                .by_attribute("id", "request_body_border")
                .each(|_element, attributes| {
                    attributes.set("foreground", &*color);
                });
        }
    }
}
