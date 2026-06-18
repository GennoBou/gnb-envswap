use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Terminal,
};
use std::io::{self, stderr, Stderr};
use std::time::Duration;

use crate::app::{App, SelectionPhase};

// A wrapper around the ratatui Terminal.
pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl Tui {
    // Constructor for Tui.
    pub fn new() -> io::Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
        Ok(Self { terminal })
    }

    // Initialize the terminal for TUI display.
    pub fn enter(&self) -> io::Result<()> {
        enable_raw_mode()?;
        stderr().execute(EnterAlternateScreen)?;
        Ok(())
    }

    // Restore the terminal to its original state.
    fn exit(&self) -> io::Result<()> {
        disable_raw_mode()?;
        stderr().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}

// Automatically restore the terminal when Tui goes out of scope.
impl Drop for Tui {
    fn drop(&mut self) {
        let _ = self.exit();
    }
}

// Main event loop for the TUI.
pub fn run_tui(app: &mut App) -> io::Result<()> {
    let mut tui = Tui::new()?;
    tui.enter()?;

    while !app.should_quit {
        // Draw the UI
        tui.terminal.draw(|frame| draw(frame, app))?;
        // Handle user input
        handle_events(app)?;
    }

    Ok(())
}

// Handle user input events.
fn handle_events(app: &mut App) -> io::Result<()> {
    // Poll for an event with a timeout.
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.current_phase {
                    SelectionPhase::Variable => handle_variable_selection_keys(key, app),
                    SelectionPhase::Value => handle_value_selection_keys(key, app),
                }
            }
        }
    }
    Ok(())
}

// Handle key presses during the variable selection phase.
fn handle_variable_selection_keys(key: event::KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Esc => app.should_quit = true,
        KeyCode::Up => app.previous_variable(),
        KeyCode::Down => app.next_variable(),
        KeyCode::Enter => {
            if let Some(selected_index) = app.variable_list_state.selected() {
                let items = app.filtered_variables();
                if let Some(variable_name) = items.get(selected_index) {
                    app.selected_variable = Some((*variable_name).clone());
                    app.current_phase = SelectionPhase::Value;
                    app.search_query.clear(); // Clear search for the next phase
                    app.value_list_state.select(Some(0));
                    app.adjust_selection();
                }
            }
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.adjust_selection();
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.adjust_selection();
        }
        _ => {}
    }
}

// Handle key presses during the value selection phase.
fn handle_value_selection_keys(key: event::KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Esc => {
            app.current_phase = SelectionPhase::Variable;
            app.selected_variable = None;
            app.search_query.clear(); // Clear search when going back
            app.value_list_state.select(None);
            app.adjust_selection();
        }
        KeyCode::Up => app.previous_value(),
        KeyCode::Down => app.next_value(),
        KeyCode::Enter => {
            // Final selection is made, quit the TUI to output the command.
            if app.value_list_state.selected().is_some() && !app.filtered_values().is_empty() {
                app.should_quit = true;
            }
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.adjust_selection();
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.adjust_selection();
        }
        _ => {}
    }
}

// Draw the UI widgets.
fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    let current_phase = app.current_phase;
    let i18n = app.i18n;

    match current_phase {
        SelectionPhase::Variable => {
            let (list_items, items_count) = {
                let items = app.filtered_variables();
                let count = items.len();
                let list_items: Vec<ListItem> = if items.is_empty() {
                    vec![ListItem::new(i18n.get("no_results")).italic()]
                } else {
                    items
                        .iter()
                        .map(|name| ListItem::new(name.to_string()))
                        .collect()
                };
                (list_items, count)
            };
            let title = i18n.get("select_variable");
            let key_hint = i18n.get("key_hint_variable_selection");
            let list_widget = List::new(list_items)
                .block(Block::default().title(title).borders(Borders::ALL))
                .highlight_style(Style::default().bold().reversed())
                .highlight_symbol("> ");

            render_layout(
                frame,
                &app.search_query,
                app.i18n,
                items_count,
                &mut app.variable_list_state,
                list_widget,
                key_hint,
            );
        }
        SelectionPhase::Value => {
            let (list_items, items_count) = {
                let items = app.filtered_values();
                let count = items.len();
                let list_items: Vec<ListItem> = if items.is_empty() {
                    vec![ListItem::new(i18n.get("no_results")).italic()]
                } else {
                    items
                        .iter()
                        .map(|v| {
                            let label = &v.label;
                            if label.starts_with("<Work> ") {
                                ListItem::new(Line::from(vec![
                                    Span::styled("<Work>", Style::default().fg(Color::Cyan).bold()),
                                    Span::raw(label["<Work>".len()..].to_string()),
                                ]))
                            } else if label.starts_with("<Home> ") {
                                ListItem::new(Line::from(vec![
                                    Span::styled("<Home>", Style::default().fg(Color::Yellow).bold()),
                                    Span::raw(label["<Home>".len()..].to_string()),
                                ]))
                            } else {
                                ListItem::new(label.to_string())
                            }
                        })
                        .collect()
                };
                (list_items, count)
            };
            let title = i18n.get("select_value");
            let key_hint = i18n.get("key_hint_value_selection");
            let list_widget = List::new(list_items)
                .block(Block::default().title(title).borders(Borders::ALL))
                .highlight_style(Style::default().bold().reversed())
                .highlight_symbol("> ");

            render_layout(
                frame,
                &app.search_query,
                app.i18n,
                items_count,
                &mut app.value_list_state,
                list_widget,
                key_hint,
            );
        }
    };
}

/// Helper function to render the common layout.
fn render_layout(
    frame: &mut ratatui::Frame,
    search_query: &str,
    i18n: &crate::i18n::I18nMessages,
    items_count: usize,
    state: &mut ratatui::widgets::ListState,
    list_widget: List,
    key_hint: &str,
) {
    // Height calculation:
    // Search Box: 3 lines (border top + text + border bottom)
    // List Borders: 2 lines (top + bottom)
    // Key Hint (Footer): 1 line
    // Total margin: 6 lines
    let list_height = if items_count == 0 { 1 } else { items_count as u16 };
    let max_height = frame.area().height * 80 / 100;
    let total_height = (list_height + 6).min(max_height);
    let area = centered_rect(80, total_height, frame.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    let search_area = chunks[0];
    let main_area = chunks[1];
    let footer_area = chunks[2];

    // Search Box
    let search_text = format!("{}{}", i18n.get("search_placeholder"), search_query);
    let search_box = Paragraph::new(search_text)
        .block(Block::default().borders(Borders::ALL).title("Search"));

    let key_hint_paragraph = Paragraph::new(Line::from(key_hint).centered());

    frame.render_widget(Clear, area);
    frame.render_widget(search_box, search_area);
    frame.render_stateful_widget(list_widget, main_area, state);
    frame.render_widget(key_hint_paragraph, footer_area);
}

/// Helper function to create a centered rect.
fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    // Ensure that the height does not exceed the available area's height.
    let height = height.min(r.height);
    // Ensure that the percentage for width does not exceed 100.
    let percent_x = percent_x.min(100);

    let popup_width = r.width * percent_x / 100;

    let x_margin = (r.width - popup_width) / 2;
    let y_margin = (r.height - height) / 2;

    Rect {
        x: r.x + x_margin,
        y: r.y + y_margin,
        width: popup_width,
        height,
    }
}
