# Bevy Tutorial

- Youtube tutorial [here](https://www.youtube.com/watch?v=TQt-v_bFdao&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd)
- Bevy engine [here](https://bevyengine.org/news/introducing-bevy)
- Quick start guide [here](https://bevy.org/learn/quick-start/introduction/)

## Bevy
A bevy is a group of birds!

Bevy is an ECS: A custom Entity Component System

## Tutorial
[tutorial guide](https://bevy.org/learn/quick-start/getting-started/)

### Requirements
```bash
sudo apt update

sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
# optional if you disabled the wayland feature
sudo apt-get install libwayland-dev libxkbcommon-dev 
# linker optimalization
sudo apt-get install clang ldd
```

### Building
See [setup](https://bevy.org/learn/quick-start/getting-started/setup/)

Dynamic linking

This is the most impactful compilation time decrease! You can compile bevy as dynamic library, preventing it from having to be statically linked each time you rebuild your project. You can enable this with the dynamic_linking feature flag.

```bash
cargo run --features bevy/dynamic_linking
```

If you don't want to add the --features bevy/dynamic_linking to each run, this flag can permanently be set with this command (edits Cargo.toml for you)

```bash
cargo add bevy -F dynamic_linking
```

This changes Cargo.toml and adds

```toml
[dependencies]
bevy = { version = "0.19.0", features = ["dynamic_linking"] }
```