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

2.  **Run in PowerShell:**

    Execute `gnb-envswap` in your PowerShell terminal. It will present a TUI to select an environment variable.

    ```powershell
    gnb-envswap
    ```

3.  **Select and Apply:**

    Use the arrow keys to navigate and `Enter` to select. After choosing a variable and a value, the tool will output a PowerShell command. To apply it to your current session, use `Invoke-Expression`.

    ```powershell
    gnb-envswap | Invoke-Expression
    ```

    You might want to create a PowerShell function for convenience:

    ```powershell
    function envswap {
        gnb-envswap | Invoke-Expression
    }
    ```

    Now you can just run `envswap`.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
