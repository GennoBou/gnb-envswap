use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
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
                    SelectionPhase::Variable => handle_variable_selection_keys(key.code, app),
                    SelectionPhase::Value => handle_value_selection_keys(key.code, app),
                }
            }
        }
    }
    Ok(())
}

// Handle key presses during the variable selection phase.
fn handle_variable_selection_keys(key_code: KeyCode, app: &mut App) {
    match key_code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(selected) = app.variable_list_state.selected() {
                if selected > 0 {
                    app.variable_list_state.select(Some(selected - 1));
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(selected) = app.variable_list_state.selected() {
                if selected < app.sorted_variable_names.len() - 1 {
                    app.variable_list_state.select(Some(selected + 1));
                }
            }
        }
        KeyCode::Enter => {
            if let Some(selected_index) = app.variable_list_state.selected() {
                let variable_name = &app.sorted_variable_names[selected_index];
                app.selected_variable = Some(variable_name.clone());
                app.current_phase = SelectionPhase::Value;
                app.value_list_state.select(Some(0)); // Select the first value by default
            }
        }
        _ => {}
    }
}

// Handle key presses during the value selection phase.
fn handle_value_selection_keys(key_code: KeyCode, app: &mut App) {
    match key_code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Esc => {
            app.current_phase = SelectionPhase::Variable;
            app.selected_variable = None;
            app.value_list_state.select(None);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let (Some(_selected_var), Some(selected_index)) =
                (&app.selected_variable, app.value_list_state.selected())
            {
                if selected_index > 0 {
                    app.value_list_state.select(Some(selected_index - 1));
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let (Some(selected_var), Some(selected_index)) =
                (&app.selected_variable, app.value_list_state.selected())
            {
                let values_count = app.config[selected_var].values.len();
                if selected_index < values_count - 1 {
                    app.value_list_state.select(Some(selected_index + 1));
                }
            }
        }
        KeyCode::Enter => {
            // Final selection is made, quit the TUI to output the command.
            app.should_quit = true;
        }
        _ => {}
    }
}


// Draw the UI widgets.
fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    let (list, state, title, key_hint) = match app.current_phase {
        SelectionPhase::Variable => {
            let items: Vec<ListItem> = app
                .sorted_variable_names
                .iter()
                .map(|name| ListItem::new(name.as_str()))
                .collect();
            let list = List::new(items);
            (
                list,
                &mut app.variable_list_state,
                app.i18n.get("select_variable"),
                app.i18n.get("key_hint_variable_selection"),
            )
        }
        SelectionPhase::Value => {
            let items: Vec<ListItem> = if let Some(var) = &app.selected_variable {
                app.config[var]
                    .values
                    .iter()
                    .map(|v| ListItem::new(v.label.as_str()))
                    .collect()
            } else {
                vec![]
            };
            let list = List::new(items);
            (
                list,
                &mut app.value_list_state,
                app.i18n.get("select_value"),
                app.i18n.get("key_hint_value_selection"),
            )
        }
    };

    // +2 for list borders, +1 for the key hint line.
    let total_height = list.len() as u16 + 3;
    let area = centered_rect(80, total_height, frame.area());

    // Split the area into a main section and a footer for the key hint.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let main_area = chunks[0];
    let footer_area = chunks[1];

    let styled_list = list
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_style(Style::default().bold().reversed())
        .highlight_symbol("> ");

    let key_hint_paragraph = Paragraph::new(Line::from(key_hint).centered());

    frame.render_widget(Clear, area); // Clear the background
    frame.render_stateful_widget(styled_list, main_area, state);
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
