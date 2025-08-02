# ClipVault
A clipboard manager written in Rust with offline local storage.

<img width="400" height="400" alt="image" src="https://github.com/user-attachments/assets/0fe02b0b-6855-4ad5-b60d-61ef83d546e5" />

## Features:
- Persistent local storage of clipboard history using SQLite
- User-configurable settings saved via TOML config files
- Runs as a background system tray application
- Filter clipboard entries by date and custom user tags

## Usage

- **To open the clipboard GUI:**
    ```sh
    cargo run --bin gui
    ```

- **To run the main application:**
    ```sh
    cargo run --bin clipvault
    ```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## License

MIT
