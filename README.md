# STFAR - Some Touhou Fangame Attempt in Rust

## Development

### Build Dependencies

#### Gentoo

- `llvm-core/clang`
- `x11-libs/libX11`
- `dev-util/pkgconf`
- `media-libs/alsa-lib`
- `wild-linker`

#### Arch

- `clang`
- `libx11`
- `pkgconf`
- `alsa-lib`
- `libxcursor`
- `libxrandr`
- `libxi`
- `wild-linker`

#### Debian

- `clang`
- `libstdc++-dev`
- `pkg-config`
- `libx11-dev`
- `libasound2-dev`
- `libudev-dev`
- `libxkbcommon-x11-0`
- `libwayland-dev`
- `libxkbcommon-dev`
- `wild-linker`

#### Void

- `clang`
- `pkgconf`
- `alsa-lib-devel`
- `libX11-devel`
- `eudev-libudev-devel`
- `wild-linker`

#### Fedora

- `clang`
- `libX11-devel`
- `alsa-lib-devel`
- `systemd-devel`
- `wayland-devel`
- `libxkbcommon-devel`
- `wild-linker`

#### Other Distros

Refer to [Bevy Linux dependencies](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md). Install `clang` from you distro's official repos. Install `just` and `wild-linker` with cargo.

#### MacOS
Install Xcode command line tools with `xcode-select --install`.

#### Windows
Refer to [Bevy Windows depedencies](https://bevy.org/learn/quick-start/getting-started/setup/#windows).


### Dev Dependencies

- `just` (optional)


### Build

```sh
# build in debug mode
just build dev # or `just build`

# build in release mode
just build release

# run in debug mode
just run dev # or `just`

# run in release mode
just run release

# perform clippy check
just check
```

Alternatively, `cargo` can be used directly:
```sh
# build in debug mode
cargo build

# build in release mode
cargo build --release

# run in debug mode
cargo run

# run in release mode
cargo run --release

# perform clippy check
cargo clippy check --all-targets
```

> `--features bevy/dynamic_linking` can be paired with above cargo commands to use dynamic linking, which provided just commands use by default. It will reduce subsequent build times.
