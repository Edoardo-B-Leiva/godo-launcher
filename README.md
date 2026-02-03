# godo-launcher

<img src="app_icon.ico" align="right" />

A simple **unoffical** open-source Fortnite launcher for Windows made in Rust with multi-account support.

![GitHub License](https://img.shields.io/github/license/a-mayb3/godo-launcher?style=for-the-badge)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/a-mayb3/godo-launcher/total?style=for-the-badge&label=Downloads)

![GitHub Repo stars](https://img.shields.io/github/stars/a-mayb3/godo-launcher?style=for-the-badge&label=GitHub%20stars)
<img alt="Gitea Stars" src="https://img.shields.io/gitea/stars/a-mayb3/godo-launcher?gitea_url=https%3A%2F%2Fgit.vollex.cc&style=for-the-badge&logo=forgejo&label=Stars%20on%20git.vollex.cc&link=https%3A%2F%2Fgit.vollex.cc%2Fa-mayb3%2Fgodo-launcher">

> [!Caution]
> Use at your own risk!
> 
> Epic Games and the Fortnite development team do not support the use of third-party software.
>
> We **do not take responsibility** for any liability issue *(such as: In-game ban, account ban)*

## Contributors
- ***(Code Owner)*** [@a-mayb3](https://github.com/a-mayb3/) 
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
```bash
git clone --depth 1 https://github.com/a-mayb3/godo-launcher
cd godo-launcher
cargo install --path . --locked
```
```powershell
git clone --depth 1 https://github.com/a-mayb3/godo-launcher
set-location godo-launcher
cargo install --path . --locked
```
