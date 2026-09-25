# MovieZen-TUI

**Terminal media experience** — a TUI for finding and playing movies, TV shows, anime, and live TV from your terminal, built for mobile-first use (Android / Termux) as well as desktop.

- **Binary:** `moviezen-tui`
- **Creator:** Umaiz Sufiyan
- **Repository:** <https://github.com/sufiyan-sabeel/MovieZen-TUI>
- **License:** MIT OR Apache-2.0
- **Status:** active development — rebranding and playback/image work in progress

## Origin & attribution

MovieZen-TUI is a **fork** of [mesamirh/MovieBox-Tui](https://github.com/mesamirh/MovieBox-Tui)
(v0.1.24, MIT OR Apache-2.0). Upstream copyright and license terms apply to the inherited
code and are preserved in [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE).
Provider integrations such as the MovieBox provider remain named for the services they talk to.

## Build from source

Requires a Rust toolchain ([rustup](https://rustup.rs/) or your package manager's `rust`).

```bash
git clone https://github.com/sufiyan-sabeel/MovieZen-TUI.git
cd MovieZen-TUI
cargo build --release
./target/release/moviezen-tui
```

On Termux (Android):

```bash
pkg update && pkg upgrade
pkg install rust git
git clone https://github.com/sufiyan-sabeel/MovieZen-TUI.git
cd MovieZen-TUI
cargo build --release
./target/release/moviezen-tui
```

## Documentation

- Full usage guide: **this README is being rewritten** — a complete installation, player
  setup, themes, diagnostics, and troubleshooting guide will land here.
- Extended docs live in [`docs/`](docs/).

## Disclaimer

MovieZen-TUI only plays streams it can legitimately access through its configured providers.
It contains no telemetry, no tracking, and no DRM/authentication/paywall bypass.
