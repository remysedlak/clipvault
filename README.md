# ClipVault
A clipboard manager written in Rust with offline local storage.

<img width="400" height="400" alt="image" src="https://github.com/user-attachments/assets/524cf4bc-8a00-428d-ab70-0b52d2676069" />

## Features:
- Persistent local storage of clipboard history using SQLite
- User-configurable settings saved via TOML config files
- Runs as a background system tray application
- Filters clipboard entries by date

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
