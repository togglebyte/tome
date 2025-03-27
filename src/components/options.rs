use crate::{
    options::{save_options, Options},
    templates::template,
    theme::{get_app_theme_by_name, AppTheme},
};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anathema::{
    component::{Component, ComponentId},
    prelude::Context,
    runtime::Builder,
    state::{AnyState, State, Value},
};

use crate::options::get_options;

use super::{
    app_layout::AppLayoutMessages, dashboard::DashboardMessages,
    response_renderer::ResponseRendererMessages, send_message,
};

#[derive(Default)]
pub struct OptionsView {
    #[allow(dead_code)]
    component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
}

#[derive(Default, State)]
pub struct OptionsViewState {
    app_theme: Value<AppTheme>,
    options: Value<OptionsState>,
    options_window: Value<OptionsWindows>,
}

#[derive(Default, State)]
struct OptionsState {
    app_theme_name: Value<String>,
    syntax_theme: Value<String>,
}

impl From<Options> for OptionsState {
    fn from(val: Options) -> Self {
        OptionsState {
            app_theme_name: val.app_theme_name.into(),
            syntax_theme: val.syntax_theme.into(),
        }
    }
}

impl OptionsViewState {
    pub fn new(options: Options) -> Self {
        let app_theme = get_app_theme_by_name(&options.app_theme_name);
        let options_state: OptionsState = options.into();

        OptionsViewState {
            app_theme: app_theme.into(),
            options: options_state.into(),
            options_window: OptionsWindows::None.into(),
        }
    }
}

#[derive(Default)]
enum OptionsWindows {
    #[default]
    None,
    SyntaxThemeSelector,
    AppThemeSelector,
}

impl State for OptionsWindows {
    fn type_info(&self) -> anathema::state::Type {
        anathema::state::Type::String
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Self::None => Some("None"),
            Self::SyntaxThemeSelector => Some("SyntaxThemeSelector"),
            Self::AppThemeSelector => Some("AppThemeSelector"),
        }
    }
}

impl OptionsView {
    pub fn new(component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>) -> Self {
        OptionsView { component_ids }
    }

    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut Builder<()>,
    ) -> anyhow::Result<()> {
        let options = get_options();
        let id = builder.component(
            "options",
            template("templates/options"),
            OptionsView::new(ids.clone()),
            OptionsViewState::new(options),
        )?;

        let mut ids_ref = ids.borrow_mut();
        ids_ref.insert(String::from("options"), id);

        Ok(())
    }

    fn go_back(&self, context: anathema::prelude::Context<'_, '_, OptionsViewState>) {
        let component_ids = self.component_ids.try_borrow();
        if component_ids.is_err() {
            return;
        }

        let component_ids = component_ids.unwrap();
        let Some(app_id) = component_ids.get("app") else {
            return;
        };

        let Ok(msg) = serde_json::to_string(&AppLayoutMessages::OpenDashboard) else {
            return;
        };

        context.emit(*app_id, msg);
    }

    fn open_theme_selector(
        &self,
        state: &mut OptionsViewState,
        mut context: anathema::prelude::Context<'_, '_, OptionsViewState>,
    ) {
        state
            .options_window
            .set(OptionsWindows::SyntaxThemeSelector);
        context.components.by_name("syntax_theme_selector").focus();
        context.components.by_name("syntax_theme_selector").focus();
    }

    fn open_app_theme_selector(
        &self,
        state: &mut OptionsViewState,
        mut context: anathema::prelude::Context<'_, '_, OptionsViewState>,
    ) {
        state.options_window.set(OptionsWindows::AppThemeSelector);
        context.components.by_name("app_theme_selector").focus();
        context.components.by_name("app_theme_selector").focus();
    }

    fn update_app_theme(
        &self,
        state: &mut OptionsViewState,
        context: Context<'_, '_, OptionsViewState>,
    ) {
        let app_theme_name = state.options.to_ref().app_theme_name.to_ref().clone();
        let app_theme = get_app_theme_by_name(&app_theme_name);
        state.app_theme.set(app_theme);

        // Update Dashboard theme
        let _ = self.component_ids.try_borrow().map(|ids| {
            if let Ok(msg) = serde_json::to_string(&DashboardMessages::ThemeUpdate) {
                let _ = send_message("dashboard", msg, &ids, context.emitter);
            }
        });

        // Update Request Body Section
        let _ = self.component_ids.try_borrow().map(|ids| {
            let _ = send_message(
                "request_body_section",
                "theme_update".to_string(),
                &ids,
                context.emitter,
            );
        });

        // Update URL Input
        let _ = self.component_ids.try_borrow().map(|ids| {
            let _ = send_message(
                "url_input",
                "theme_update".to_string(),
                &ids,
                context.emitter,
            );
        });

        let _ = self.component_ids.try_borrow().map(|ids| {
            if let Ok(msg) = serde_json::to_string(&ResponseRendererMessages::ThemeUpdate) {
                let _ = send_message("response_renderer", msg, &ids, context.emitter);
            }
        });
    }

    fn send_error_message(&self, error_message: &str, context: Context<'_, '_, OptionsViewState>) {
        let dashboard_msg = DashboardMessages::ShowError(error_message.to_string());
        let Ok(msg) = serde_json::to_string(&dashboard_msg) else {
            return;
        };
        let Ok(ids) = self.component_ids.try_borrow() else {
            return;
        };
        let _ = send_message("dashboard", msg, &ids, context.emitter);
    }
}

impl Component for OptionsView {
    type State = OptionsViewState;
    type Message = String;

    fn accept_focus(&self) -> bool {
        true
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        _: anathema::component::Children,
        context: anathema::prelude::Context<'_, '_, Self::State>,
    ) {
        match key.code {
            #[allow(clippy::single_match)]
            anathema::component::KeyCode::Char(char) => match char {
                'b' => self.go_back(context),
                'x' => self.open_theme_selector(state, context),
                'a' => self.open_app_theme_selector(state, context),

                _ => {}
            },
            anathema::component::KeyCode::Esc => self.go_back(context),

            _ => {}
        }
    }

    fn receive(
        &mut self,
        ident: &str,
        value: &dyn AnyState,
        state: &mut Self::State,
        _: anathema::component::Children,
        mut context: anathema::prelude::Context<'_, '_, Self::State>,
    ) {
        match ident {
            "syntax_theme_selector__selection" => {
                let mut options = get_options();

                let new_theme = value.as_str().unwrap().replace(".tmTheme", "");
                options.syntax_theme = new_theme.clone();

                match save_options(options) {
                    Ok(_) => {
                        state.options.to_mut().syntax_theme.set(new_theme);
                    }
                    Err(error) => {
                        let error_message =
                            format!("Error saving selected syntax highlighting theme: {}", error);

                        self.send_error_message(&error_message, context);
                    }
                }
            }
            "syntax_theme_selector__cancel" => {
                state.options_window.set(OptionsWindows::None);
                context.components.by_name("options").focus();
            }

            "app_theme_selector__cancel" => {
                state.options_window.set(OptionsWindows::None);
                context.components.by_name("options").focus();
            }

            "app_theme_selector__selection" => {
                let mut options = get_options();

                let name = value.as_str().unwrap();
                options.app_theme_name = name.to_string();

                match save_options(options) {
                    Ok(_) => {
                        state.options.to_mut().app_theme_name.set(name.to_string());
                        self.update_app_theme(state, context);
                    }
                    Err(error) => {
                        let error_message = format!("Error saving selected theme: {}", error);

                        self.send_error_message(&error_message, context);
                    }
                }
            }

            _ => {}
        }
    }
}
