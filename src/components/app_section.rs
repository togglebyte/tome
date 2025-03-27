use anathema::{
    component::Component,
    state::{AnyState, State, Value},
};

use crate::theme::{get_app_theme, AppTheme};

#[derive(Default)]
pub struct AppSection;

impl AppSection {
    // TODO: Add a message so the theme can update since this doesn't have focus
    #[allow(unused)]
    fn update_app_theme(&self, state: &mut AppSectionState) {
        let app_theme = get_app_theme();
        state.app_theme.set(app_theme);
    }
}

#[derive(Default, State)]
pub struct AppSectionState {
    section_id: Value<Option<String>>,
    section_text_id: Value<Option<String>>,
    app_theme: Value<AppTheme>,
}

impl AppSectionState {
    pub fn new() -> Self {
        let app_theme = get_app_theme();

        AppSectionState {
            section_id: None.into(),
            section_text_id: None.into(),
            app_theme: app_theme.into(),
        }
    }
}

impl Component for AppSection {
    type State = AppSectionState;
    type Message = ();

    fn tick(
        &mut self,
        state: &mut Self::State,
        elements: anathema::component::Children,
        context: anathema::prelude::Context<'_, '_, Self::State>,
        _dt: std::time::Duration,
    ) {
        if state.section_id.to_ref().is_none() {
            let Some(section_id) = context.attribute("section_id") else {
                return;
            };

            let Some(section_text_id) = context.attribute("section_text_id") else {
                return;
            };

            if let Some(id) = section_id.as_str() {
                state.section_id.set(Some(id.to_string()));
            }

            if let Some(section_text_id) = section_text_id.as_str() {
                state.section_text_id.set(Some(section_text_id.to_string()));
            }
        }

        self.resize(state, elements, context);
    }

    fn receive(
        &mut self,
        ident: &str,
        value: &dyn AnyState,
        state: &mut Self::State,
        mut elements: anathema::component::Children,
        mut context: anathema::prelude::Context<'_, '_, Self::State>,
    ) {
        if ident == "input_focus" {
            let &focus = value.to::<bool>();
            if !focus {
                context.components.by_name("app").focus();
            }

            let section_id = state.section_id.to_ref().clone();
            let Some(section_id) = section_id else { return };

            match focus {
                true => elements.elements().by_attribute("id", &*section_id).each(
                    |_element, attributes| {
                        attributes.set("foreground", "#ffff00");
                    },
                ),
                false => elements.elements().by_attribute("id", &*section_id).each(
                    |_element, attributes| {
                        attributes.set("foreground", "#ff0000");
                    },
                ),
            }
        }
    }

    fn accept_focus(&self) -> bool {
        false
    }
}
