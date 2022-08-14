# game-of-life

[![pages-build-deployment](https://github.com/artslob/game-of-life/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/artslob/game-of-life/actions/workflows/pages/pages-build-deployment)

<div align="center">
    <img alt="Game logo" align="center" src="/logo.png?raw=true" title="Game logo"/>
</div>

Logo created with Canva.

Deployed as wasm: https://artslob.github.io/game-of-life/

## Development

```bash
# macroquad dependencies on ubuntu:
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

Run locally: `cargo run`

To run locally as wasm:
```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
cargo install basic-http-server
basic-http-server .
```

### TODO (optional)
1. select map size
1. use theme/skin for ui
1. read map from file
1. select map from predefined
1. allow creating custom map
1. add option to seed random generator
