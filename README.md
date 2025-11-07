# gnb-envswap

English | [日本語](./README.ja.md)

`gnb-envswap` is a command-line tool with a Text-based UI (TUI) to quickly and safely switch environment variables in a PowerShell session. It's designed to streamline the workflow for developers who frequently switch between different API keys, database connections, or other environment-dependent settings.

## Features

-   **Interactive TUI:** A user-friendly interface for selecting environment variables and their values.
-   **Configuration via TOML:** Define your environment variable sets in a simple `.env.swap.toml` file.
-   **Fast and Built with Rust:** A single, lightweight executable built with Rust.
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

    Create a file named `.env.swap.toml` in your project's root directory or your home directory (`~`).

    ```toml
    [API_KEY]
    [[API_KEY.values]]
    label = "Development Server 🚀"
    value = "dev_api_key_xxxxxxxxx"

    [[API_KEY.values]]
    label = "Production Server"
    value = "prod_api_key_yyyyyyyy"

    [DB_HOST]
    [[DB_HOST.values]]
    label = "Local Database"
    value = "localhost"
    ```

2.  **Run `envswap` in PowerShell:**

    If you installed with Scoop, a convenient `envswap` function is automatically added to your PowerShell profile. Just run `envswap`:

    ```powershell
    envswap
    ```

    This will open the TUI. Use the arrow keys to navigate and `Enter` to select. After choosing a variable and a value, the environment variable will be set in your current session.

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
