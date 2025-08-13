# ClipVault
A clipboard manager written in Rust with offline local storage.

<img width="500" height="500" alt="image" src="https://github.com/user-attachments/assets/321af03c-39f3-4679-b3ed-103fa5b0c3e4" />

## Features:
- Persistent local storage of clipboard history using SQLite
- User-configurable settings saved via TOML config files
- Runs as a background system tray application
- Filter clipboard entries by date and customizable user tags

## Usage

- **To open the clipboard GUI:**
    ```sh
    cargo run --bin gui
    ```

- **To run the main application:**
    ```sh
    cargo build --release
    cargo run --bin clipvault
    ```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## License

MIT
