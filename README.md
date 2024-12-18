# GodoLauncher
![GitHub Repo stars](https://img.shields.io/github/stars/Edoardo-B-Leiva/godo-launcher?style=for-the-badge&label=GitHub%20stars)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/Edoardo-B-Leiva/godo-launcher/total?style=for-the-badge&label=Downloads)

A simple Fortnite launcher for Windows made in Rust.

## Contributors
- @G4-Synix
## Getting the launcher
Currently GodoLauncher is *only obtainable via building the source code* using cargo.
Other methods will be implemented in the near future alongside its development.
### Building from source
#### Requirements
- [Cargo](https://rustup.rs/)
#### Dependencies
- [reqwest](https://github.com/seanmonstar/reqwest)
- [serde](https://serde.rs/)
- [colog](https://github.com/chrivers/rust-colog)
#### Procedure
```powershell
git clone https://github.com/Edoardo-B-Leiva/godo-launcher --depth 1 &&
cd godo-launcher &&
cargo build --release
```
