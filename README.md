# godo-launcher

<img src="app_icon.ico" align="right" />

A simple **unoffical** open-source Fortnite launcher for Windows made in Rust with multi-account support.

![GitHub License](https://img.shields.io/github/license/Edoardo-b-leiva/godo-launcher?style=for-the-badge)
![GitHub Repo stars](https://img.shields.io/github/stars/Edoardo-B-Leiva/godo-launcher?style=for-the-badge&label=GitHub%20stars)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/Edoardo-B-Leiva/godo-launcher/total?style=for-the-badge&label=Downloads)

> [!Caution]
> Use at your own risk!
> 
> Epic Games and the Fortnite development team do not support the use of third-party software.
>
> We **do not** make responsible for any liability issue *(such as: In-game ban, account ban)*

## Contributors
- ***(Code Owner)*** [@Edoardo-B-Leiva](https://github.com/Edoardo-B-Leiva/) 
- [@G4-Synix](https://github.com/G4-Synix)

### Special thanks:
- [LeleDerGrasshalmi/FortniteEndpointsDocumentation](https://github.com/LeleDerGrasshalmi/FortniteEndpointsDocumentation)
- [MixV2/EpicResearch](https://github.com/MixV2/EpicResearch)

## Getting the launcher
> [!IMPORTANT]
> Currently godo-launcher is *exclusively obtainable by [building the source](README.md#building-from-source)* using cargo.
> Other methods will be implemented in the near future alongside its development.

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
