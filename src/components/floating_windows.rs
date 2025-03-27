use anathema::state::State;

pub mod add_project_variable;
pub mod app_theme_selector;
pub mod body_mode_selector;
pub mod code_gen;
pub mod commands;
pub mod edit_endpoint_name;
pub mod edit_project_name;
pub mod endpoints_selector;
pub mod file_selector;
pub mod project_variables;
pub mod syntax_theme_selector;

#[derive(PartialEq, Eq)]
pub enum FloatingWindow {
    None,
    Method,
    AddHeader,
    Error,
    EditHeaderSelector,
    Project,
    ConfirmAction,
    Message,
    ChangeEndpointName,
    ChangeProjectName,
    EndpointsSelector,
    Commands,
    CodeGen,
    PostmanFileSelector,
    BodyModeSelector,
    AddProjectVariable,
    ViewProjectVariables,
}

impl State for FloatingWindow {
    fn type_info(&self) -> anathema::state::Type {
        anathema::state::Type::String
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Self::None => Some("None"),
            Self::Method => Some("Method"),
            Self::AddHeader => Some("AddHeader"),
            Self::Error => Some("Error"),
            Self::EditHeaderSelector => Some("EditHeaderSelector"),
            Self::Project => Some("Project"),
            Self::ConfirmAction => Some("ConfirmAction"),
            Self::Message => Some("Message"),
            Self::ChangeEndpointName => Some("ChangeEndpointName"),
            Self::ChangeProjectName => Some("ChangeProjectName"),
            Self::EndpointsSelector => Some("EndpointsSelector"),
            Self::Commands => Some("Commands"),
            Self::CodeGen => Some("CodeGen"),
            Self::PostmanFileSelector => Some("PostmanFileSelector"),
            Self::BodyModeSelector => Some("BodyModeSelector"),
            Self::AddProjectVariable => Some("AddProjectVariable"),
            Self::ViewProjectVariables => Some("ViewProjectVariables"),
        }
    }
}
