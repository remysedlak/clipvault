# ClipVault
A clipboard manager written in Rust with offline local storage.

<img width="550" height="550" alt="image" src="https://github.com/user-attachments/assets/b2ef2909-7f52-4896-855d-59c1590f9015" />


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
