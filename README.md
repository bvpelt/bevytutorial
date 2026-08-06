# Bevy Tutorial

- Youtube tutorial [here](https://www.youtube.com/watch?v=TQt-v_bFdao&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd)
- Bevy engine [here](https://bevyengine.org/news/introducing-bevy)
- Quick start guide [here](https://bevy.org/learn/quick-start/introduction/)

## Bevy
A bevy is a group of birds!

Bevy is an ECS: A custom Entity Component System

## Tutorial
[tutorial guide](https://bevy.org/learn/quick-start/getting-started/)
[index](./Index.md)

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

## App

### Minimal bevy program

```rust
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

When running ```cargo run``` nothing appens.

An App contains our World, and our World contains our game's data. An App also contains the logic for controlling the outer loop of our game, allowing us to orchestrate the data in our World into the gameplay we want to create.

App is typically only used to setup the structure of your game, which is done by chaining its methods with the builder pattern. Using these App methods, you'll be able to add systems, insert unique resources, and create the entities and components needed for your gameplay.

App provides us with tools for:
- Initializing resources in the World to store globally available data that we only need a single copy of.
- Adding systems to our Schedule, which can read and modify resources and our entities' components, according to our game logic.
- Importing other blocks of App-modifying code using Plugins.

## ECS
All app logic in Bevy uses the Entity Component System paradigm, which is often shortened to ECS. ECS is a software pattern that involves breaking your program up into *Entities, Components, and Systems*. **Entities** are unique "things" that are assigned groups of **Components**, which are then processed using **Systems**.

In rust this is implemented as

Components: Rust structs that implement the Component trait

```rust
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
```

Systems: normal Rust functions

```rust
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}
```

Entities: a simple type containing a unique integer

```rust
struct Entity(u64);
```

### Your First System
Create your main.rs file:

```rust
use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new().add_systems(Update, hello_world).run();
}
```

The add_systems function adds the system to your App's Update Schedule, but we'll cover that more later.

Now run your app again using cargo run. You should see hello world! printed once in your terminal.