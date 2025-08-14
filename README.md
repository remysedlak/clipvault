# ClipVault
A clipboard manager written in Rust with offline local storage.

<img width="400" height="400" alt="main screen" src="https://github.com/user-attachments/assets/3cf3296d-c29b-4089-8cd7-588dbb31de64" />
<img width="400" height="400" alt="tag manager" src="https://github.com/user-attachments/assets/e51f668e-4e70-42f9-bc3d-41eee8a7d1df" />

## Features:
- Persistent local storage of clipboard history using SQLite
- User-configurable settings saved via TOML config files
- Runs as a background system tray application
- Filter clipboard entries by date, customizable user tags, or searching

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
