use ratatui::widgets::ListState;
use crate::config::Config;
use crate::i18n::I18nMessages;

// Represents the current phase of user selection.
#[derive(PartialEq)]
pub enum SelectionPhase {
    Variable,
    Value,
}

// Holds the entire state of the application.
pub struct App<'a> {
    pub should_quit: bool,
    pub config: &'a Config,
    pub i18n: &'a I18nMessages,
    // A sorted list of variable names for consistent display order.
    pub sorted_variable_names: Vec<String>,
    pub variable_list_state: ListState,
    pub value_list_state: ListState,
    pub current_phase: SelectionPhase,
    // The variable name selected in the first phase.
    pub selected_variable: Option<String>,
}

impl<'a> App<'a> {
    // Creates a new App instance with initial state.
    pub fn new(config: &'a Config, i18n: &'a I18nMessages) -> Self {
        let mut variable_list_state = ListState::default();
        let mut sorted_variable_names: Vec<String> = config.keys().cloned().collect();
        sorted_variable_names.sort_unstable(); // Sort keys alphabetically

        if !sorted_variable_names.is_empty() {
            variable_list_state.select(Some(0));
        }

        App {
            should_quit: false,
            config,
            i18n,
            sorted_variable_names,
            variable_list_state,
            value_list_state: ListState::default(),
            current_phase: SelectionPhase::Variable,
            selected_variable: None,
        }
    }
}
