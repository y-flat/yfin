# yfin 0.1.2 ![Rust](https://img.shields.io/github/workflow/status/jakeroggenbuck/yfin/Rust?style=for-the-badge)
[Y-flat](https://github.com/adamhutchings/yfc) Installer

## Usage
```
 yfin <SUBCOMMAND>
```

#### Flags
```
 -h, --help       Prints help information
 -V, --version    Prints version information
```

#### Subcommands
```
 help                Prints this message or the help of the given subcommand(s)
 init                Initialize a package
 install             Install from git repo url
 install-compiler    Install compiler yfin install-compiler
 install-yflib       Install yflib yfin install-yflib
 uninstall           Uninstall package
 upgrade             Install newer version of package
```

## Install Latest
If you have cargo on your machine, skip to step 3

1. Install [`rustup.rs`](https://rustup.rs/).

2. Setup rust
   ```sh
   rustup override set stable
   rustup update stable
   ```

3. Install from crates
   ```
   cargo install --git https://github.com/JakeRoggenbuck/yfin
   ```

# Join the discussion
<a href="https://discord.gg/v27SpPyj">![discord](https://img.shields.io/badge/Discord-7289DA?style=for-the-badge&logo=discord&logoColor=white)</a>

# Video
[![Watch the video](https://i.imgur.com/nqnaLqW.png)](https://www.youtube.com/watch?v=IW0TiN3d7FI)

