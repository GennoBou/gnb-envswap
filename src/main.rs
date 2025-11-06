mod app;
mod cli;
mod config;
mod i18n;
mod output;
mod tui;

use std::{env, fs};

use app::App;
use clap::Parser;
use cli::{Cli, Commands, EditTarget};
use i18n::I18nMessages;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // Load i18n messages first to ensure all messages are localized.
    let messages = match i18n::load_messages() {
        Ok(messages) => messages,
        Err(err) => {
            eprintln!("Error loading i18n messages: {}", err);
            std::process::exit(1);
        }
    };

    // If a subcommand is given, handle it and exit. Otherwise, run the TUI.
    if let Some(command) = cli.command {
        match command {
            Commands::Edit(args) => {
                handle_edit_command(args.target, &messages);
                return Ok(());
            }
        }
    }

    // --- Default action: Run the TUI ---
    run_tui_mode(&messages)?;

    Ok(())
}

/// The main logic for running the TUI application.
fn run_tui_mode(messages: &I18nMessages) -> std::io::Result<()> {
    let config = match config::load_config() {
        Ok(config) => config,
        Err(err) => {
            if err.contains("No .env.swap.toml file found") {
                eprintln!("{}", messages.get("config_not_found"));
                std::process::exit(1);
            }
            eprintln!("Error loading config: {}", err);
            std::process::exit(1);
        }
    };

    if config.is_empty() {
        eprintln!("{}", messages.get("config_not_found"));
        std::process::exit(1);
    }

    let mut app = App::new(&config, messages);
    tui::run_tui(&mut app)?;

    if let (Some(variable_name), Some(value_index)) =
        (app.selected_variable, app.value_list_state.selected())
    {
        if let Some(env_var) = app.config.get(&variable_name) {
            if let Some(env_value) = env_var.values.get(value_index) {
                let command =
                    output::generate_powershell_command(&variable_name, &env_value.value);
                println!("{}", command);
            }
        }
    }

    Ok(())
}

/// Handles the `edit` subcommand logic.
fn handle_edit_command(target: EditTarget, messages: &I18nMessages) {
    let path = match target {
        EditTarget::Local => env::current_dir().ok().map(|p| p.join(".env.swap.toml")),
        EditTarget::Global => dirs::home_dir().map(|p| p.join(".env.swap.toml")),
    };

    if let Some(path) = path {
        // If the file doesn't exist, create it.
        if !path.exists() {
            if let Err(e) = fs::write(&path, "") {
                let error_message = messages
                    .get("file_creation_failed")
                    .replace("{path}", path.to_str().unwrap_or(""));
                eprintln!("{} ({})", error_message, e);
                std::process::exit(1);
            }
        }

        // Open the file with the default associated application.
        if let Err(e) = open::that(&path) {
            eprintln!("Failed to open file at {:?}: {}", path, e);
            std::process::exit(1);
        }
    } else {
        // This case should be rare (e.g., home directory not found).
        eprintln!("Could not determine the path for the configuration file.");
        std::process::exit(1);
    }
}
