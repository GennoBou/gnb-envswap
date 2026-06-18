use ratatui::widgets::ListState;
use crate::config::Config;
use crate::i18n::I18nMessages;

// Represents the current phase of user selection.
#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub search_query: String,
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
            search_query: String::new(),
        }
    }

    /// Returns a list of variable names that match the current search query.
    pub fn filtered_variables(&self) -> Vec<&String> {
        let query = self.search_query.to_lowercase();
        self.sorted_variable_names
            .iter()
            .filter(|name| name.to_lowercase().contains(&query))
            .collect()
    }

    /// Returns a list of values for the selected variable that match the current search query.
    pub fn filtered_values(&self) -> Vec<&crate::config::EnvValue> {
        if let Some(var_name) = &self.selected_variable {
            if let Some(env_var) = self.config.get(var_name) {
                let query = self.search_query.to_lowercase();
                return env_var
                    .values
                    .iter()
                    .filter(|v| {
                        v.label.to_lowercase().contains(&query)
                            || v.value.to_lowercase().contains(&query)
                    })
                    .collect();
            }
        }
        vec![]
    }

    /// Select the next variable in the filtered list, wrapping around.
    pub fn next_variable(&mut self) {
        let items = self.filtered_variables();
        if items.is_empty() {
            return;
        }
        let i = match self.variable_list_state.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.variable_list_state.select(Some(i));
    }

    /// Select the previous variable in the filtered list, wrapping around.
    pub fn previous_variable(&mut self) {
        let items = self.filtered_variables();
        if items.is_empty() {
            return;
        }
        let i = match self.variable_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.variable_list_state.select(Some(i));
    }

    /// Select the next value in the filtered list, wrapping around.
    pub fn next_value(&mut self) {
        let items = self.filtered_values();
        if items.is_empty() {
            return;
        }
        let i = match self.value_list_state.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.value_list_state.select(Some(i));
    }

    /// Select the previous value in the filtered list, wrapping around.
    pub fn previous_value(&mut self) {
        let items = self.filtered_values();
        if items.is_empty() {
            return;
        }
        let i = match self.value_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.value_list_state.select(Some(i));
    }

    /// Adjust the selection index if it's out of bounds after filtering.
    pub fn adjust_selection(&mut self) {
        match self.current_phase {
            SelectionPhase::Variable => {
                let count = self.filtered_variables().len();
                if count == 0 {
                    self.variable_list_state.select(None);
                } else if let Some(selected) = self.variable_list_state.selected() {
                    if selected >= count {
                        self.variable_list_state.select(Some(count - 1));
                    }
                } else {
                    self.variable_list_state.select(Some(0));
                }
            }
            SelectionPhase::Value => {
                let count = self.filtered_values().len();
                if count == 0 {
                    self.value_list_state.select(None);
                } else if let Some(selected) = self.value_list_state.selected() {
                    if selected >= count {
                        self.value_list_state.select(Some(count - 1));
                    }
                } else {
                    self.value_list_state.select(Some(0));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{EnvVar, EnvValue};
    use std::collections::HashMap;

    #[test]
    fn test_app_initialization() {
        let mut config = HashMap::new();
        config.insert("VAR1".to_string(), EnvVar {
            values: vec![EnvValue { label: "L1".to_string(), value: "V1".to_string() }]
        });
        
        let i18n = crate::i18n::load_messages().unwrap();
        let app = App::new(&config, &i18n);

        assert_eq!(app.current_phase, SelectionPhase::Variable);
        assert_eq!(app.sorted_variable_names, vec!["VAR1".to_string()]);
        assert_eq!(app.variable_list_state.selected(), Some(0));
        assert_eq!(app.selected_variable, None);
        assert_eq!(app.search_query, "");
    }

    #[test]
    fn test_app_filtering_variables() {
        let mut config = HashMap::new();
        config.insert("APPLE".to_string(), EnvVar { values: vec![] });
        config.insert("BANANA".to_string(), EnvVar { values: vec![] });
        config.insert("CHERRY".to_string(), EnvVar { values: vec![] });
        let i18n = crate::i18n::load_messages().unwrap();
        let mut app = App::new(&config, &i18n);

        app.search_query = "a".to_string();
        let filtered = app.filtered_variables();
        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains(&&"APPLE".to_string()));
        assert!(filtered.contains(&&"BANANA".to_string()));

        app.search_query = "CHER".to_string();
        let filtered = app.filtered_variables();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0], "CHERRY");
    }

    #[test]
    fn test_app_filtering_values() {
        let mut config = HashMap::new();
        config.insert("VAR".to_string(), EnvVar {
            values: vec![
                EnvValue { label: "Development".to_string(), value: "dev".to_string() },
                EnvValue { label: "Production".to_string(), value: "prod".to_string() },
            ]
        });
        let i18n = crate::i18n::load_messages().unwrap();
        let mut app = App::new(&config, &i18n);

        app.selected_variable = Some("VAR".to_string());
        app.search_query = "dev".to_string();
        let filtered = app.filtered_values();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].label, "Development");

        app.search_query = "prod".to_string();
        let filtered = app.filtered_values();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].label, "Production");
    }

    #[test]
    fn test_app_variable_loop_navigation_with_filter() {
        let mut config = HashMap::new();
        config.insert("APPLE".to_string(), EnvVar { values: vec![] });
        config.insert("BANANA".to_string(), EnvVar { values: vec![] });
        config.insert("CHERRY".to_string(), EnvVar { values: vec![] });
        let i18n = crate::i18n::load_messages().unwrap();
        let mut app = App::new(&config, &i18n);

        app.search_query = "a".to_string(); // APPLE, BANANA
        app.variable_list_state.select(Some(0)); // APPLE
        
        app.next_variable(); // APPLE -> BANANA
        assert_eq!(app.filtered_variables()[app.variable_list_state.selected().unwrap()], &"BANANA".to_string());
        
        app.next_variable(); // BANANA -> APPLE (Loop)
        assert_eq!(app.filtered_variables()[app.variable_list_state.selected().unwrap()], &"APPLE".to_string());
    }
}
