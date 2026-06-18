# gnb-envswap

English | [日本語](./README.ja.md)

`gnb-envswap` is a command-line tool with a Text-based UI (TUI) to quickly and safely switch environment variables in a PowerShell session. It's designed to streamline the workflow for developers who frequently switch between different API keys, database connections, or other environment-dependent settings.

## Features

-   **Interactive TUI:** A user-friendly interface for selecting environment variables and their values.
-   **Real-time Search:** Instantly filter variables and values by simply typing in the TUI.
-   **Configuration via TOML:** Define your environment variable sets in a `.env.swap.toml` file.
-   **Smart Merging:** Automatically merges local (workspace) and global (home) configurations, clearly distinguishing them with `<Work>` and `<Home>` colored prefixes.
-   **Fast and Built with Rust:** A single, lightweight executable built with Rust 2024 Edition.
-   **i18n Support:** UI messages are available in English and Japanese (auto-detected from your OS locale).

## Installation

The recommended way to install `gnb-envswap` on Windows is via [Scoop](https://scoop.sh/).

```powershell
# Add the developer's bucket (replace with the actual bucket URL)
scoop bucket add gennobou https://github.com/gennobou/scoop-bucket
# Install the app
scoop install gennobou/gnb-envswap
```

## Usage

1.  **Create a Configuration File:**
    Create a `.env.swap.toml` file. For detailed configuration options and how local/global merging works, please see the [Configuration Reference](docs/configuration.md).

2.  **Run `envswap` in PowerShell:**

    If you installed with Scoop, a convenient `envswap` function is automatically added to your PowerShell profile. Just run `envswap`:

    ```powershell
    envswap
    ```

    This will open the TUI. 
    *   **Type letters** to search/filter the list.
    *   Use the **Up/Down arrow keys** to navigate (the list loops!).
    *   Press **Enter** to select.
    *   Press **Esc** to go back or quit.

    **How it works (and manual setup):**

    The `envswap` function is a simple wrapper that executes `gnb-envswap | Invoke-Expression`. The `gnb-envswap` command itself outputs a PowerShell command to set the variable, and `Invoke-Expression` applies it.

    If you didn't install with Scoop, you can use the tool by running the full command:

    ```powershell
    gnb-envswap | Invoke-Expression
    ```

    Or, you can add the `envswap` function to your PowerShell profile (`$PROFILE`) manually for convenience.

### `show` Subcommand

The `show` subcommand is used to check the current status of the environment variables defined in your configuration file. All output is sent to `stderr` to prevent accidental piping to `Invoke-Expression`.

```powershell
# Display the current status (values are masked)
gnb-envswap show
```

Example output:
```
API_KEY: Development Server 🚀
DB_HOST: not set
SECRET_TOKEN: <custom value>
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
