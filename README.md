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

### ECS
Create your main.rs file:

```rust
use bevy::{prelude::*, ui::update};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
```

Note that we have used .chain() on the two systems. This is because we want both of them to run in exactly the order they're listed in the code: with update_people occurring before greet_people. If they weren’t, the name might change after we greet the people.

### Plugins
One of Bevy's core principles is modularity. All Bevy engine features are implemented as plugins---collections of code that modify an App. This includes internal features like the renderer, but games themselves are also implemented as plugins! This empowers developers to pick and choose which features they want. Don't need a UI? Don't register the UiPlugin. Want to build a headless server? Don't register the RenderPlugin.

A valuable place to find more is [here](https://bevy.org/assets/)

[Default plugins](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html)

Create your main.rs file:

```rust
use bevy::{prelude::*, ui::update};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

```

### Resources
The Entity and Component data types are great for representing complex, query-able groups of data. But most Apps will also require "globally unique" data of some kind. In Bevy ECS, we represent globally unique data using the Resource trait.

Here are some examples of data that could be encoded as a Resource:
- Elapsed Time
- Asset Collections (sounds, textures, meshes)
- Renderers

Add chrono = "0.4" to dependencies in Cargo.toml.

Create your main.rs file:

```rust
use bevy::{prelude::*, ui::update};
use chrono::Utc;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone

    if timer.0.tick(time.delta()).just_finished() {
        let now = Utc::now();
        let iso_string = now.to_rfc3339();
        println!("Now: {} Time elapsed: {:?}", iso_string, time.delta());
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

```

## Dependencies

```text
$ cargo tree
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /home/bvpelt/Develop/bevytutorial/my_bevy_game/Cargo.toml
workspace: /home/bvpelt/Develop/bevytutorial/Cargo.toml
my_bevy_game v0.1.0 (/home/bvpelt/Develop/bevytutorial/my_bevy_game)
├── bevy v0.19.0
│   ├── bevy_dylib v0.19.0
│   │   └── bevy_internal v0.19.0
│   │       ├── bevy_a11y v0.19.0
│   │       │   ├── accesskit v0.24.1
│   │       │   │   └── uuid v1.24.0
│   │       │   │       ├── getrandom v0.4.3
│   │       │   │       │   ├── cfg-if v1.0.4
│   │       │   │       │   └── libc v0.2.189
│   │       │   │       └── serde_core v1.0.229
│   │       │   ├── bevy_app v0.19.0
│   │       │   │   ├── bevy_derive v0.19.0 (proc-macro)
│   │       │   │   │   ├── bevy_macro_utils v0.19.0
│   │       │   │   │   │   ├── proc-macro2 v1.0.107
│   │       │   │   │   │   │   └── unicode-ident v1.0.24
│   │       │   │   │   │   ├── quote v1.0.47
│   │       │   │   │   │   │   └── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   ├── syn v2.0.119
│   │       │   │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   └── unicode-ident v1.0.24
│   │       │   │   │   │   └── toml_edit v0.25.13+spec-1.1.0
│   │       │   │   │   │       ├── indexmap v2.14.0
│   │       │   │   │   │       │   ├── equivalent v1.0.2
│   │       │   │   │   │       │   └── hashbrown v0.17.1
│   │       │   │   │   │       ├── toml_datetime v1.1.1+spec-1.1.0
│   │       │   │   │   │       ├── toml_parser v1.1.3+spec-1.1.0
│   │       │   │   │   │       │   └── winnow v1.0.4
│   │       │   │   │   │       └── winnow v1.0.4
│   │       │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   └── syn v2.0.119 (*)
│   │       │   │   ├── bevy_ecs v0.19.0
│   │       │   │   │   ├── arrayvec v0.7.8
│   │       │   │   │   ├── bevy_ecs_macros v0.19.0 (proc-macro)
│   │       │   │   │   │   ├── bevy_ecs_macro_logic v0.19.0
│   │       │   │   │   │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   ├── bevy_platform v0.19.0
│   │       │   │   │   │   ├── foldhash v0.2.0
│   │       │   │   │   │   ├── futures-lite v2.6.1
│   │       │   │   │   │   │   ├── fastrand v2.5.0
│   │       │   │   │   │   │   ├── futures-core v0.3.33
│   │       │   │   │   │   │   ├── futures-io v0.3.33
│   │       │   │   │   │   │   ├── parking v2.2.1
│   │       │   │   │   │   │   └── pin-project-lite v0.2.17
│   │       │   │   │   │   ├── hashbrown v0.16.1
│   │       │   │   │   │   │   ├── allocator-api2 v0.2.21
│   │       │   │   │   │   │   ├── equivalent v1.0.2
│   │       │   │   │   │   │   ├── foldhash v0.2.0
│   │       │   │   │   │   │   └── serde_core v1.0.229
│   │       │   │   │   │   ├── serde v1.0.229
│   │       │   │   │   │   │   ├── serde_core v1.0.229
│   │       │   │   │   │   │   └── serde_derive v1.0.229 (proc-macro)
│   │       │   │   │   │   │       ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │       ├── quote v1.0.47 (*)
│   │       │   │   │   │   │       └── syn v3.0.3
│   │       │   │   │   │   │           ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │           ├── quote v1.0.47 (*)
│   │       │   │   │   │   │           └── unicode-ident v1.0.24
│   │       │   │   │   │   └── spin v0.10.1
│   │       │   │   │   ├── bevy_ptr v0.19.0
│   │       │   │   │   ├── bevy_reflect v0.19.0
│   │       │   │   │   │   ├── assert_type_match v0.1.1 (proc-macro)
│   │       │   │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_ptr v0.19.0
│   │       │   │   │   │   ├── bevy_reflect_derive v0.19.0 (proc-macro)
│   │       │   │   │   │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   │   │   │   ├── indexmap v2.14.0 (*)
│   │       │   │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   │   ├── bevy_utils v0.19.0
│   │       │   │   │   │   │   ├── async-channel v2.5.0
│   │       │   │   │   │   │   │   ├── concurrent-queue v2.5.0
│   │       │   │   │   │   │   │   │   └── crossbeam-utils v0.8.22
│   │       │   │   │   │   │   │   ├── event-listener-strategy v0.5.4
│   │       │   │   │   │   │   │   │   ├── event-listener v5.4.2
│   │       │   │   │   │   │   │   │   │   ├── parking v2.2.1
│   │       │   │   │   │   │   │   │   │   └── pin-project-lite v0.2.17
│   │       │   │   │   │   │   │   │   └── pin-project-lite v0.2.17
│   │       │   │   │   │   │   │   ├── futures-core v0.3.33
│   │       │   │   │   │   │   │   └── pin-project-lite v0.2.17
│   │       │   │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   │   ├── disqualified v1.0.0
│   │       │   │   │   │   │   ├── indexmap v2.14.0
│   │       │   │   │   │   │   │   ├── equivalent v1.0.2
│   │       │   │   │   │   │   │   ├── hashbrown v0.17.1
│   │       │   │   │   │   │   │   │   └── foldhash v0.2.0
│   │       │   │   │   │   │   │   └── serde_core v1.0.229
│   │       │   │   │   │   │   └── thread_local v1.1.10
│   │       │   │   │   │   │       └── cfg-if v1.0.4
│   │       │   │   │   │   ├── derive_more v2.1.1
│   │       │   │   │   │   │   └── derive_more-impl v2.1.1 (proc-macro)
│   │       │   │   │   │   │       ├── convert_case v0.10.0
│   │       │   │   │   │   │       │   └── unicode-segmentation v1.13.3
│   │       │   │   │   │   │       ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │       ├── quote v1.0.47 (*)
│   │       │   │   │   │   │       ├── syn v2.0.119 (*)
│   │       │   │   │   │   │       └── unicode-xid v0.2.6
│   │       │   │   │   │   │       [build-dependencies]
│   │       │   │   │   │   │       └── rustc_version v0.4.1
│   │       │   │   │   │   │           └── semver v1.0.28
│   │       │   │   │   │   ├── disqualified v1.0.0
│   │       │   │   │   │   ├── downcast-rs v2.0.2
│   │       │   │   │   │   ├── erased-serde v0.4.10
│   │       │   │   │   │   │   ├── serde_core v1.0.229
│   │       │   │   │   │   │   └── typeid v1.0.3
│   │       │   │   │   │   ├── foldhash v0.2.0
│   │       │   │   │   │   ├── glam v0.32.1
│   │       │   │   │   │   │   ├── bytemuck v1.25.2
│   │       │   │   │   │   │   │   └── bytemuck_derive v1.11.0 (proc-macro)
│   │       │   │   │   │   │   │       ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   │       ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   │       └── syn v2.0.119 (*)
│   │       │   │   │   │   │   ├── encase v0.12.0
│   │       │   │   │   │   │   │   ├── const_panic v0.2.15
│   │       │   │   │   │   │   │   │   └── typewit v1.15.2
│   │       │   │   │   │   │   │   ├── encase_derive v0.12.0 (proc-macro)
│   │       │   │   │   │   │   │   │   └── encase_derive_impl v0.12.0
│   │       │   │   │   │   │   │   │       ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   │   │       ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   │   │       └── syn v2.0.119 (*)
│   │       │   │   │   │   │   │   └── thiserror v2.0.19
│   │       │   │   │   │   │   │       └── thiserror-impl v2.0.19 (proc-macro)
│   │       │   │   │   │   │   │           ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   │           ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   │           └── syn v3.0.3 (*)
│   │       │   │   │   │   │   ├── libm v0.2.16
│   │       │   │   │   │   │   ├── rand v0.10.2
│   │       │   │   │   │   │   │   └── rand_core v0.10.1
│   │       │   │   │   │   │   └── serde_core v1.0.229
│   │       │   │   │   │   ├── indexmap v2.14.0 (*)
│   │       │   │   │   │   ├── inventory v0.3.24
│   │       │   │   │   │   ├── petgraph v0.8.3
│   │       │   │   │   │   │   ├── fixedbitset v0.5.7
│   │       │   │   │   │   │   ├── hashbrown v0.15.5
│   │       │   │   │   │   │   │   └── foldhash v0.1.5
│   │       │   │   │   │   │   ├── indexmap v2.14.0 (*)
│   │       │   │   │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   │   │   └── serde_derive v1.0.229 (proc-macro) (*)
│   │       │   │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   │   ├── smallvec v1.15.2
│   │       │   │   │   │   ├── smol_str v0.2.2
│   │       │   │   │   │   │   └── serde v1.0.229 (*)
│   │       │   │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   │   ├── uuid v1.24.0 (*)
│   │       │   │   │   │   ├── variadics_please v1.1.0 (proc-macro)
│   │       │   │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   │   └── wgpu-types v29.0.4
│   │       │   │   │   │       ├── bitflags v2.13.1
│   │       │   │   │   │       │   ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │       │   └── serde_core v1.0.229
│   │       │   │   │   │       ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │       ├── log v0.4.33
│   │       │   │   │   │       ├── raw-window-handle v0.6.2
│   │       │   │   │   │       └── serde v1.0.229 (*)
│   │       │   │   │   ├── bevy_tasks v0.19.0
│   │       │   │   │   │   ├── async-channel v2.5.0 (*)
│   │       │   │   │   │   ├── async-executor v1.14.0
│   │       │   │   │   │   │   ├── async-task v4.7.1
│   │       │   │   │   │   │   ├── concurrent-queue v2.5.0 (*)
│   │       │   │   │   │   │   ├── fastrand v2.5.0
│   │       │   │   │   │   │   ├── futures-lite v2.6.1 (*)
│   │       │   │   │   │   │   ├── pin-project-lite v0.2.17
│   │       │   │   │   │   │   └── slab v0.4.12
│   │       │   │   │   │   ├── async-task v4.7.1
│   │       │   │   │   │   ├── atomic-waker v1.1.2
│   │       │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   ├── concurrent-queue v2.5.0 (*)
│   │       │   │   │   │   ├── crossbeam-queue v0.3.13
│   │       │   │   │   │   │   └── crossbeam-utils v0.8.22
│   │       │   │   │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   │   │   └── futures-lite v2.6.1 (*)
│   │       │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   ├── bumpalo v3.20.3
│   │       │   │   │   ├── concurrent-queue v2.5.0 (*)
│   │       │   │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   │   ├── fixedbitset v0.5.7
│   │       │   │   │   ├── indexmap v2.14.0 (*)
│   │       │   │   │   ├── log v0.4.33
│   │       │   │   │   ├── nonmax v0.5.5
│   │       │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   ├── slotmap v1.1.1
│   │       │   │   │   │   [build-dependencies]
│   │       │   │   │   │   └── version_check v0.9.5
│   │       │   │   │   ├── smallvec v1.15.2
│   │       │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   └── variadics_please v1.1.0 (proc-macro) (*)
│   │       │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   ├── bevy_tasks v0.19.0 (*)
│   │       │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   ├── ctrlc v3.5.2
│   │       │   │   │   └── nix v0.31.3
│   │       │   │   │       ├── bitflags v2.13.1 (*)
│   │       │   │   │       ├── cfg-if v1.0.4
│   │       │   │   │       └── libc v0.2.189
│   │       │   │   │       [build-dependencies]
│   │       │   │   │       └── cfg_aliases v0.2.2
│   │       │   │   ├── downcast-rs v2.0.2
│   │       │   │   ├── log v0.4.33
│   │       │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   └── variadics_please v1.1.0 (proc-macro) (*)
│   │       │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   └── bevy_reflect v0.19.0 (*)
│   │       ├── bevy_animation v0.19.0
│   │       │   ├── bevy_animation_macros v0.19.0 (proc-macro)
│   │       │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   ├── quote v1.0.47 (*)
│   │       │   │   └── syn v2.0.119 (*)
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_asset v0.19.0
│   │       │   │   ├── async-broadcast v0.7.2
│   │       │   │   │   ├── event-listener v5.4.2 (*)
│   │       │   │   │   ├── event-listener-strategy v0.5.4 (*)
│   │       │   │   │   ├── futures-core v0.3.33
│   │       │   │   │   └── pin-project-lite v0.2.17
│   │       │   │   ├── async-channel v2.5.0 (*)
│   │       │   │   ├── async-fs v2.2.0
│   │       │   │   │   ├── async-lock v3.4.2
│   │       │   │   │   │   ├── event-listener v5.4.2 (*)
│   │       │   │   │   │   ├── event-listener-strategy v0.5.4 (*)
│   │       │   │   │   │   └── pin-project-lite v0.2.17
│   │       │   │   │   ├── blocking v1.6.2
│   │       │   │   │   │   ├── async-channel v2.5.0 (*)
│   │       │   │   │   │   ├── async-task v4.7.1
│   │       │   │   │   │   ├── futures-io v0.3.33
│   │       │   │   │   │   ├── futures-lite v2.6.1 (*)
│   │       │   │   │   │   └── piper v0.2.5
│   │       │   │   │   │       ├── atomic-waker v1.1.2
│   │       │   │   │   │       ├── fastrand v2.5.0
│   │       │   │   │   │       └── futures-io v0.3.33
│   │       │   │   │   └── futures-lite v2.6.1 (*)
│   │       │   │   ├── async-io v2.6.0
│   │       │   │   │   ├── cfg-if v1.0.4
│   │       │   │   │   ├── concurrent-queue v2.5.0 (*)
│   │       │   │   │   ├── futures-io v0.3.33
│   │       │   │   │   ├── futures-lite v2.6.1 (*)
│   │       │   │   │   ├── parking v2.2.1
│   │       │   │   │   ├── polling v3.11.0
│   │       │   │   │   │   ├── cfg-if v1.0.4
│   │       │   │   │   │   └── rustix v1.1.4
│   │       │   │   │   │       ├── bitflags v2.13.1 (*)
│   │       │   │   │   │       └── linux-raw-sys v0.12.1
│   │       │   │   │   ├── rustix v1.1.4 (*)
│   │       │   │   │   └── slab v0.4.12
│   │       │   │   │   [build-dependencies]
│   │       │   │   │   └── autocfg v1.5.1
│   │       │   │   ├── async-lock v3.4.2 (*)
│   │       │   │   ├── atomicow v1.2.0
│   │       │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   ├── bevy_asset_macros v0.19.0 (proc-macro)
│   │       │   │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   └── syn v2.0.119 (*)
│   │       │   │   ├── bevy_diagnostic v0.19.0
│   │       │   │   │   ├── atomic-waker v1.1.2
│   │       │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   ├── bevy_tasks v0.19.0 (*)
│   │       │   │   │   ├── bevy_time v0.19.0
│   │       │   │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   │   ├── crossbeam-channel v0.5.16
│   │       │   │   │   │   │   └── crossbeam-utils v0.8.22
│   │       │   │   │   │   └── log v0.4.33
│   │       │   │   │   ├── const-fnv1a-hash v1.1.0
│   │       │   │   │   ├── log v0.4.33
│   │       │   │   │   └── sysinfo v0.38.4
│   │       │   │   │       ├── libc v0.2.189
│   │       │   │   │       └── memchr v2.8.3
│   │       │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   ├── bevy_tasks v0.19.0 (*)
│   │       │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   ├── blake3 v1.8.6
│   │       │   │   │   ├── arrayref v0.3.9
│   │       │   │   │   ├── arrayvec v0.7.8
│   │       │   │   │   ├── cfg-if v1.0.4
│   │       │   │   │   ├── constant_time_eq v0.4.2
│   │       │   │   │   └── cpufeatures v0.3.0
│   │       │   │   │   [build-dependencies]
│   │       │   │   │   └── cc v1.4.0
│   │       │   │   │       ├── find-msvc-tools v0.1.9
│   │       │   │   │       └── shlex v2.0.1
│   │       │   │   ├── crossbeam-channel v0.5.16 (*)
│   │       │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   ├── disqualified v1.0.0
│   │       │   │   ├── downcast-rs v2.0.2
│   │       │   │   ├── either v1.17.0
│   │       │   │   ├── futures-io v0.3.33
│   │       │   │   ├── futures-lite v2.6.1 (*)
│   │       │   │   ├── futures-util v0.3.33
│   │       │   │   │   ├── futures-core v0.3.33
│   │       │   │   │   ├── futures-macro v0.3.33 (proc-macro)
│   │       │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   ├── futures-task v0.3.33
│   │       │   │   │   ├── pin-project-lite v0.2.17
│   │       │   │   │   └── slab v0.4.12
│   │       │   │   ├── ron v0.12.2
│   │       │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   ├── once_cell v1.21.4
│   │       │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   ├── serde_derive v1.0.229 (proc-macro) (*)
│   │       │   │   │   ├── typeid v1.0.3
│   │       │   │   │   └── unicode-ident v1.0.24
│   │       │   │   ├── serde v1.0.229 (*)
│   │       │   │   ├── stackfuture v0.3.1
│   │       │   │   │   └── const_panic v0.2.15 (*)
│   │       │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   ├── tracing v0.1.44
│   │       │   │   │   ├── pin-project-lite v0.2.17
│   │       │   │   │   ├── tracing-attributes v0.1.31 (proc-macro)
│   │       │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   └── tracing-core v0.1.36
│   │       │   │   │       └── once_cell v1.21.4
│   │       │   │   └── uuid v1.24.0 (*)
│   │       │   ├── bevy_color v0.19.0
│   │       │   │   ├── bevy_math v0.19.0
│   │       │   │   │   ├── arrayvec v0.7.8
│   │       │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   │   ├── glam v0.32.1 (*)
│   │       │   │   │   ├── itertools v0.14.0
│   │       │   │   │   │   └── either v1.17.0
│   │       │   │   │   ├── libm v0.2.16
│   │       │   │   │   ├── rand v0.10.2 (*)
│   │       │   │   │   ├── rand_distr v0.6.0
│   │       │   │   │   │   ├── num-traits v0.2.19
│   │       │   │   │   │   │   └── libm v0.2.16
│   │       │   │   │   │   │   [build-dependencies]
│   │       │   │   │   │   │   └── autocfg v1.5.1
│   │       │   │   │   │   └── rand v0.10.2 (*)
│   │       │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   └── variadics_please v1.1.0 (proc-macro) (*)
│   │       │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   ├── encase v0.12.0 (*)
│   │       │   │   ├── serde v1.0.229 (*)
│   │       │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   └── wgpu-types v29.0.4 (*)
│   │       │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_math v0.19.0 (*)
│   │       │   ├── bevy_mesh v0.19.0
│   │       │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   ├── bevy_encase_derive v0.19.0 (proc-macro)
│   │       │   │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   │   └── encase_derive_impl v0.12.0 (*)
│   │       │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   ├── bevy_mikktspace v1.0.0
│   │       │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   ├── bevy_transform v0.19.0
│   │       │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   ├── bevy_tasks v0.19.0 (*)
│   │       │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   │   └── thiserror v2.0.19 (*)
│   │       │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   ├── encase v0.12.0 (*)
│   │       │   │   ├── glam v0.32.1 (*)
│   │       │   │   ├── half v2.7.1
│   │       │   │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   │   ├── cfg-if v1.0.4
│   │       │   │   │   ├── num-traits v0.2.19 (*)
│   │       │   │   │   └── zerocopy v0.8.56
│   │       │   │   │       └── zerocopy-derive v0.8.56 (proc-macro)
│   │       │   │   │           ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │           ├── quote v1.0.47 (*)
│   │       │   │   │           └── syn v2.0.119 (*)
│   │       │   │   ├── hexasphere v18.0.0
│   │       │   │   │   ├── constgebra v0.1.4
│   │       │   │   │   │   └── const_soft_float v0.1.4
│   │       │   │   │   └── glam v0.32.1 (*)
│   │       │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   ├── tracing v0.1.44 (*)
│   │       │   │   └── wgpu-types v29.0.4 (*)
│   │       │   ├── bevy_platform v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_time v0.19.0 (*)
│   │       │   ├── bevy_transform v0.19.0 (*)
│   │       │   ├── bevy_utils v0.19.0 (*)
│   │       │   ├── blake3 v1.8.6 (*)
│   │       │   ├── derive_more v2.1.1 (*)
│   │       │   ├── downcast-rs v2.0.2
│   │       │   ├── either v1.17.0
│   │       │   ├── petgraph v0.8.3 (*)
│   │       │   ├── ron v0.12.2 (*)
│   │       │   ├── serde v1.0.229 (*)
│   │       │   ├── smallvec v1.15.2
│   │       │   ├── thiserror v2.0.19 (*)
│   │       │   ├── thread_local v1.1.10 (*)
│   │       │   ├── tracing v0.1.44 (*)
│   │       │   └── uuid v1.24.0 (*)
│   │       ├── bevy_anti_alias v0.19.0
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_asset v0.19.0 (*)
│   │       │   ├── bevy_camera v0.19.0
│   │       │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   ├── bevy_image v0.19.0
│   │       │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   │   ├── futures-lite v2.6.1 (*)
│   │       │   │   │   ├── guillotiere v0.6.2
│   │       │   │   │   │   ├── euclid v0.22.14
│   │       │   │   │   │   │   └── num-traits v0.2.19 (*)
│   │       │   │   │   │   └── svg_fmt v0.4.5
│   │       │   │   │   ├── half v2.7.1 (*)
│   │       │   │   │   ├── image v0.25.10
│   │       │   │   │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │   ├── byteorder-lite v0.1.0
│   │       │   │   │   │   ├── moxcms v0.8.1
│   │       │   │   │   │   │   ├── num-traits v0.2.19 (*)
│   │       │   │   │   │   │   └── pxfm v0.1.30
│   │       │   │   │   │   ├── num-traits v0.2.19 (*)
│   │       │   │   │   │   └── png v0.18.1
│   │       │   │   │   │       ├── bitflags v2.13.1 (*)
│   │       │   │   │   │       ├── crc32fast v1.5.0
│   │       │   │   │   │       │   └── cfg-if v1.0.4
│   │       │   │   │   │       ├── fdeflate v0.3.7
│   │       │   │   │   │       │   └── simd-adler32 v0.3.10
│   │       │   │   │   │       ├── flate2 v1.1.9
│   │       │   │   │   │       │   ├── crc32fast v1.5.0 (*)
│   │       │   │   │   │       │   └── miniz_oxide v0.8.9
│   │       │   │   │   │       │       ├── adler2 v2.0.1
│   │       │   │   │   │       │       └── simd-adler32 v0.3.10
│   │       │   │   │   │       └── miniz_oxide v0.8.9 (*)
│   │       │   │   │   ├── ktx2 v0.5.0
│   │       │   │   │   │   └── bitflags v2.13.1 (*)
│   │       │   │   │   ├── rectangle-pack v0.4.2
│   │       │   │   │   ├── ruzstd v0.8.3
│   │       │   │   │   │   └── twox-hash v2.1.3
│   │       │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   ├── tracing v0.1.44 (*)
│   │       │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   ├── bevy_mesh v0.19.0 (*)
│   │       │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   ├── bevy_window v0.19.0
│   │       │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   ├── bevy_image v0.19.0 (*)
│   │       │   │   │   ├── bevy_input v0.19.0
│   │       │   │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   │   │   ├── log v0.4.33
│   │       │   │   │   │   ├── smol_str v0.2.2 (*)
│   │       │   │   │   │   └── thiserror v2.0.19 (*)
│   │       │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   ├── log v0.4.33
│   │       │   │   │   └── raw-window-handle v0.6.2
│   │       │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   ├── downcast-rs v2.0.2
│   │       │   │   ├── serde v1.0.229 (*)
│   │       │   │   ├── smallvec v1.15.2
│   │       │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   └── wgpu-types v29.0.4 (*)
│   │       │   ├── bevy_core_pipeline v0.19.0
│   │       │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   ├── bevy_camera v0.19.0 (*)
│   │       │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   ├── bevy_diagnostic v0.19.0 (*)
│   │       │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   ├── bevy_image v0.19.0 (*)
│   │       │   │   ├── bevy_light v0.19.0
│   │       │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   ├── bevy_camera v0.19.0 (*)
│   │       │   │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   ├── bevy_gizmos v0.19.0
│   │       │   │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_camera v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_gizmos_macros v0.19.0 (proc-macro)
│   │       │   │   │   │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   │   ├── bevy_input v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_log v0.19.0
│   │       │   │   │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   │   │   ├── tracing v0.1.44 (*)
│   │       │   │   │   │   │   ├── tracing-log v0.2.0
│   │       │   │   │   │   │   │   ├── log v0.4.33
│   │       │   │   │   │   │   │   ├── once_cell v1.21.4
│   │       │   │   │   │   │   │   └── tracing-core v0.1.36 (*)
│   │       │   │   │   │   │   └── tracing-subscriber v0.3.23
│   │       │   │   │   │   │       ├── matchers v0.2.0
│   │       │   │   │   │   │       │   └── regex-automata v0.4.18
│   │       │   │   │   │   │       │       ├── aho-corasick v1.1.5
│   │       │   │   │   │   │       │       │   └── memchr v2.8.3
│   │       │   │   │   │   │       │       ├── memchr v2.8.3
│   │       │   │   │   │   │       │       └── regex-syntax v0.8.11
│   │       │   │   │   │   │       ├── nu-ansi-term v0.50.3
│   │       │   │   │   │   │       ├── once_cell v1.21.4
│   │       │   │   │   │   │       ├── regex-automata v0.4.18 (*)
│   │       │   │   │   │   │       ├── sharded-slab v0.1.7
│   │       │   │   │   │   │       │   └── lazy_static v1.5.0
│   │       │   │   │   │   │       ├── smallvec v1.15.2
│   │       │   │   │   │   │       ├── thread_local v1.1.10 (*)
│   │       │   │   │   │   │       ├── tracing v0.1.44 (*)
│   │       │   │   │   │   │       ├── tracing-core v0.1.36 (*)
│   │       │   │   │   │   │       └── tracing-log v0.2.0 (*)
│   │       │   │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_mesh v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_time v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   │   └── bevy_window v0.19.0 (*)
│   │       │   │   │   ├── bevy_image v0.19.0 (*)
│   │       │   │   │   ├── bevy_log v0.19.0 (*)
│   │       │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   ├── bevy_mesh v0.19.0 (*)
│   │       │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   ├── half v2.7.1 (*)
│   │       │   │   │   ├── smallvec v1.15.2
│   │       │   │   │   ├── tracing v0.1.44 (*)
│   │       │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   ├── bevy_log v0.19.0 (*)
│   │       │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   ├── bevy_render v0.19.0
│   │       │   │   │   ├── async-channel v2.5.0 (*)
│   │       │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   ├── bevy_camera v0.19.0 (*)
│   │       │   │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   │   ├── bevy_diagnostic v0.19.0 (*)
│   │       │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   ├── bevy_encase_derive v0.19.0 (proc-macro) (*)
│   │       │   │   │   ├── bevy_image v0.19.0 (*)
│   │       │   │   │   ├── bevy_log v0.19.0 (*)
│   │       │   │   │   ├── bevy_material v0.19.0
│   │       │   │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_material_macros v0.19.0 (proc-macro)
│   │       │   │   │   │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   │   ├── bevy_mesh v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_shader v0.19.0
│   │       │   │   │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   │   │   ├── naga v29.0.4
│   │       │   │   │   │   │   │   ├── arrayvec v0.7.8
│   │       │   │   │   │   │   │   ├── bit-set v0.9.1
│   │       │   │   │   │   │   │   │   └── bit-vec v0.9.1
│   │       │   │   │   │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   │   │   │   ├── cfg-if v1.0.4
│   │       │   │   │   │   │   │   ├── codespan-reporting v0.13.1
│   │       │   │   │   │   │   │   │   ├── termcolor v1.4.1
│   │       │   │   │   │   │   │   │   └── unicode-width v0.2.2
│   │       │   │   │   │   │   │   ├── half v2.7.1 (*)
│   │       │   │   │   │   │   │   ├── hashbrown v0.16.1 (*)
│   │       │   │   │   │   │   │   ├── hexf-parse v0.2.1
│   │       │   │   │   │   │   │   ├── indexmap v2.14.0 (*)
│   │       │   │   │   │   │   │   ├── libm v0.2.16
│   │       │   │   │   │   │   │   ├── log v0.4.33
│   │       │   │   │   │   │   │   ├── num-traits v0.2.19 (*)
│   │       │   │   │   │   │   │   ├── once_cell v1.21.4
│   │       │   │   │   │   │   │   ├── rustc-hash v1.1.0
│   │       │   │   │   │   │   │   ├── spirv v0.4.0+sdk-1.4.341.0
│   │       │   │   │   │   │   │   │   └── bitflags v2.13.1 (*)
│   │       │   │   │   │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   │   │   │   └── unicode-ident v1.0.24
│   │       │   │   │   │   │   │   [build-dependencies]
│   │       │   │   │   │   │   │   └── cfg_aliases v0.2.2
│   │       │   │   │   │   │   ├── naga_oil v0.22.0
│   │       │   │   │   │   │   │   ├── codespan-reporting v0.12.0
│   │       │   │   │   │   │   │   │   ├── termcolor v1.4.1
│   │       │   │   │   │   │   │   │   └── unicode-width v0.2.2
│   │       │   │   │   │   │   │   ├── data-encoding v2.11.1
│   │       │   │   │   │   │   │   ├── indexmap v2.14.0 (*)
│   │       │   │   │   │   │   │   ├── naga v29.0.4 (*)
│   │       │   │   │   │   │   │   ├── regex v1.13.1
│   │       │   │   │   │   │   │   │   ├── aho-corasick v1.1.5 (*)
│   │       │   │   │   │   │   │   │   ├── memchr v2.8.3
│   │       │   │   │   │   │   │   │   ├── regex-automata v0.4.18 (*)
│   │       │   │   │   │   │   │   │   └── regex-syntax v0.8.11
│   │       │   │   │   │   │   │   ├── rustc-hash v1.1.0
│   │       │   │   │   │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   │   │   │   ├── tracing v0.1.44 (*)
│   │       │   │   │   │   │   │   └── unicode-ident v1.0.24
│   │       │   │   │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   │   │   ├── tracing v0.1.44 (*)
│   │       │   │   │   │   │   ├── wgpu-naga-bridge v29.0.4
│   │       │   │   │   │   │   │   ├── naga v29.0.4 (*)
│   │       │   │   │   │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   │   ├── encase v0.12.0 (*)
│   │       │   │   │   │   ├── smallvec v1.15.2
│   │       │   │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   │   ├── tracing v0.1.44 (*)
│   │       │   │   │   │   ├── variadics_please v1.1.0 (proc-macro) (*)
│   │       │   │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   │   ├── bevy_material_macros v0.19.0 (proc-macro) (*)
│   │       │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   ├── bevy_mesh v0.19.0 (*)
│   │       │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   ├── bevy_render_macros v0.19.0 (proc-macro)
│   │       │   │   │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   ├── bevy_shader v0.19.0 (*)
│   │       │   │   │   ├── bevy_tasks v0.19.0 (*)
│   │       │   │   │   ├── bevy_time v0.19.0 (*)
│   │       │   │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   ├── bevy_window v0.19.0 (*)
│   │       │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   │   ├── downcast-rs v2.0.2
│   │       │   │   │   ├── encase v0.12.0 (*)
│   │       │   │   │   ├── glam v0.32.1 (*)
│   │       │   │   │   ├── image v0.25.10 (*)
│   │       │   │   │   ├── indexmap v2.14.0 (*)
│   │       │   │   │   ├── itertools v0.14.0 (*)
│   │       │   │   │   ├── naga v29.0.4 (*)
│   │       │   │   │   ├── nonmax v0.5.5
│   │       │   │   │   ├── offset-allocator v0.2.0
│   │       │   │   │   │   ├── log v0.4.33
│   │       │   │   │   │   └── nonmax v0.5.5
│   │       │   │   │   ├── smallvec v1.15.2
│   │       │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   ├── variadics_please v1.1.0 (proc-macro) (*)
│   │       │   │   │   ├── weak-table v0.3.2
│   │       │   │   │   ├── wgpu v29.0.4
│   │       │   │   │   │   ├── arrayvec v0.7.8
│   │       │   │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │   ├── cfg-if v1.0.4
│   │       │   │   │   │   ├── document-features v0.2.12 (proc-macro)
│   │       │   │   │   │   │   └── litrs v1.0.0
│   │       │   │   │   │   ├── hashbrown v0.16.1 (*)
│   │       │   │   │   │   ├── log v0.4.33
│   │       │   │   │   │   ├── naga v29.0.4 (*)
│   │       │   │   │   │   ├── profiling v1.0.18
│   │       │   │   │   │   ├── raw-window-handle v0.6.2
│   │       │   │   │   │   ├── smallvec v1.15.2
│   │       │   │   │   │   ├── static_assertions v1.1.0
│   │       │   │   │   │   ├── wgpu-core v29.0.4
│   │       │   │   │   │   │   ├── arrayvec v0.7.8
│   │       │   │   │   │   │   ├── bit-set v0.9.1 (*)
│   │       │   │   │   │   │   ├── bit-vec v0.9.1
│   │       │   │   │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │   │   ├── document-features v0.2.12 (proc-macro) (*)
│   │       │   │   │   │   │   ├── hashbrown v0.16.1 (*)
│   │       │   │   │   │   │   ├── indexmap v2.14.0 (*)
│   │       │   │   │   │   │   ├── log v0.4.33
│   │       │   │   │   │   │   ├── naga v29.0.4 (*)
│   │       │   │   │   │   │   ├── once_cell v1.21.4
│   │       │   │   │   │   │   ├── parking_lot v0.12.5
│   │       │   │   │   │   │   │   ├── lock_api v0.4.14
│   │       │   │   │   │   │   │   │   └── scopeguard v1.2.0
│   │       │   │   │   │   │   │   └── parking_lot_core v0.9.12
│   │       │   │   │   │   │   │       ├── cfg-if v1.0.4
│   │       │   │   │   │   │   │       ├── libc v0.2.189
│   │       │   │   │   │   │   │       └── smallvec v1.15.2
│   │       │   │   │   │   │   ├── profiling v1.0.18
│   │       │   │   │   │   │   ├── raw-window-handle v0.6.2
│   │       │   │   │   │   │   ├── rustc-hash v1.1.0
│   │       │   │   │   │   │   ├── smallvec v1.15.2
│   │       │   │   │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   │   │   ├── wgpu-core-deps-windows-linux-android v29.0.4
│   │       │   │   │   │   │   │   └── wgpu-hal v29.0.4
│   │       │   │   │   │   │   │       ├── arrayvec v0.7.8
│   │       │   │   │   │   │   │       ├── ash v0.38.0+1.3.281
│   │       │   │   │   │   │   │       │   └── libloading v0.8.9
│   │       │   │   │   │   │   │       │       └── cfg-if v1.0.4
│   │       │   │   │   │   │   │       ├── bitflags v2.13.1 (*)
│   │       │   │   │   │   │   │       ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │   │   │       ├── cfg-if v1.0.4
│   │       │   │   │   │   │   │       ├── gpu-allocator v0.28.0
│   │       │   │   │   │   │   │       │   ├── ash v0.38.0+1.3.281 (*)
│   │       │   │   │   │   │   │       │   ├── hashbrown v0.16.1 (*)
│   │       │   │   │   │   │   │       │   ├── log v0.4.33
│   │       │   │   │   │   │   │       │   ├── presser v0.3.1
│   │       │   │   │   │   │   │       │   └── thiserror v2.0.19 (*)
│   │       │   │   │   │   │   │       ├── gpu-descriptor v0.3.2
│   │       │   │   │   │   │   │       │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   │   │   │       │   ├── gpu-descriptor-types v0.2.0
│   │       │   │   │   │   │   │       │   │   └── bitflags v2.13.1 (*)
│   │       │   │   │   │   │   │       │   └── hashbrown v0.15.5 (*)
│   │       │   │   │   │   │   │       ├── hashbrown v0.16.1 (*)
│   │       │   │   │   │   │   │       ├── libc v0.2.189
│   │       │   │   │   │   │   │       ├── libloading v0.8.9 (*)
│   │       │   │   │   │   │   │       ├── log v0.4.33
│   │       │   │   │   │   │   │       ├── naga v29.0.4 (*)
│   │       │   │   │   │   │   │       ├── ordered-float v5.3.0
│   │       │   │   │   │   │   │       │   └── num-traits v0.2.19 (*)
│   │       │   │   │   │   │   │       ├── parking_lot v0.12.5 (*)
│   │       │   │   │   │   │   │       ├── profiling v1.0.18
│   │       │   │   │   │   │   │       ├── raw-window-handle v0.6.2
│   │       │   │   │   │   │   │       ├── renderdoc-sys v1.1.0
│   │       │   │   │   │   │   │       ├── smallvec v1.15.2
│   │       │   │   │   │   │   │       ├── thiserror v2.0.19 (*)
│   │       │   │   │   │   │   │       ├── wgpu-naga-bridge v29.0.4 (*)
│   │       │   │   │   │   │   │       └── wgpu-types v29.0.4 (*)
│   │       │   │   │   │   │   │       [build-dependencies]
│   │       │   │   │   │   │   │       └── cfg_aliases v0.2.2
│   │       │   │   │   │   │   ├── wgpu-hal v29.0.4 (*)
│   │       │   │   │   │   │   ├── wgpu-naga-bridge v29.0.4 (*)
│   │       │   │   │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   │   │   │   [build-dependencies]
│   │       │   │   │   │   │   └── cfg_aliases v0.2.2
│   │       │   │   │   │   ├── wgpu-hal v29.0.4 (*)
│   │       │   │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   │   │   [build-dependencies]
│   │       │   │   │   │   └── cfg_aliases v0.2.2
│   │       │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   ├── bevy_shader v0.19.0 (*)
│   │       │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   ├── bevy_window v0.19.0 (*)
│   │       │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   ├── indexmap v2.14.0 (*)
│   │       │   │   └── nonmax v0.5.5
│   │       │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   ├── bevy_diagnostic v0.19.0 (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_image v0.19.0 (*)
│   │       │   ├── bevy_math v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_render v0.19.0 (*)
│   │       │   ├── bevy_shader v0.19.0 (*)
│   │       │   ├── bevy_utils v0.19.0 (*)
│   │       │   └── tracing v0.1.44 (*)
│   │       ├── bevy_app v0.19.0 (*)
│   │       ├── bevy_asset v0.19.0 (*)
│   │       ├── bevy_audio v0.19.0
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_asset v0.19.0 (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_math v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_transform v0.19.0 (*)
│   │       │   ├── rodio v0.22.2
│   │       │   │   ├── cpal v0.17.3
│   │       │   │   │   ├── alsa v0.11.0
│   │       │   │   │   │   ├── alsa-sys v0.4.0
│   │       │   │   │   │   │   └── libc v0.2.189
│   │       │   │   │   │   │   [build-dependencies]
│   │       │   │   │   │   │   └── pkg-config v0.3.33
│   │       │   │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   │   ├── cfg-if v1.0.4
│   │       │   │   │   │   └── libc v0.2.189
│   │       │   │   │   ├── dasp_sample v0.11.0
│   │       │   │   │   └── libc v0.2.189
│   │       │   │   ├── dasp_sample v0.11.0
│   │       │   │   ├── lewton v0.10.2
│   │       │   │   │   ├── byteorder v1.5.0
│   │       │   │   │   ├── ogg v0.8.0
│   │       │   │   │   │   └── byteorder v1.5.0
│   │       │   │   │   └── tinyvec v1.12.0
│   │       │   │   │       └── tinyvec_macros v0.1.1
│   │       │   │   ├── num-rational v0.4.2
│   │       │   │   │   ├── num-bigint v0.4.8
│   │       │   │   │   │   ├── num-integer v0.1.46
│   │       │   │   │   │   │   └── num-traits v0.2.19 (*)
│   │       │   │   │   │   └── num-traits v0.2.19 (*)
│   │       │   │   │   ├── num-integer v0.1.46 (*)
│   │       │   │   │   └── num-traits v0.2.19 (*)
│   │       │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   └── tracing v0.1.44 (*)
│   │       │   └── tracing v0.1.44 (*)
│   │       ├── bevy_camera v0.19.0 (*)
│   │       ├── bevy_clipboard v0.19.0
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_log v0.19.0 (*)
│   │       │   └── bevy_platform v0.19.0 (*)
│   │       ├── bevy_color v0.19.0 (*)
│   │       ├── bevy_core_pipeline v0.19.0 (*)
│   │       ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       ├── bevy_diagnostic v0.19.0 (*)
│   │       ├── bevy_ecs v0.19.0 (*)
│   │       ├── bevy_gilrs v0.19.0
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_input v0.19.0 (*)
│   │       │   ├── bevy_platform v0.19.0 (*)
│   │       │   ├── bevy_time v0.19.0 (*)
│   │       │   ├── gilrs v0.11.2
│   │       │   │   ├── fnv v1.0.7
│   │       │   │   ├── gilrs-core v0.6.8
│   │       │   │   │   ├── inotify v0.11.4
│   │       │   │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   │   ├── inotify-sys v0.1.8
│   │       │   │   │   │   │   └── libc v0.2.189
│   │       │   │   │   │   └── libc v0.2.189
│   │       │   │   │   ├── libc v0.2.189
│   │       │   │   │   ├── libudev-sys v0.1.4
│   │       │   │   │   │   └── libc v0.2.189
│   │       │   │   │   │   [build-dependencies]
│   │       │   │   │   │   └── pkg-config v0.3.33
│   │       │   │   │   ├── log v0.4.33
│   │       │   │   │   ├── nix v0.31.3 (*)
│   │       │   │   │   ├── uuid v1.24.0 (*)
│   │       │   │   │   └── vec_map v0.8.2
│   │       │   │   ├── log v0.4.33
│   │       │   │   ├── uuid v1.24.0 (*)
│   │       │   │   └── vec_map v0.8.2
│   │       │   ├── thiserror v2.0.19 (*)
│   │       │   └── tracing v0.1.44 (*)
│   │       ├── bevy_gizmos v0.19.0 (*)
│   │       ├── bevy_gizmos_render v0.19.0
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_asset v0.19.0 (*)
│   │       │   ├── bevy_camera v0.19.0 (*)
│   │       │   ├── bevy_color v0.19.0 (*)
│   │       │   ├── bevy_core_pipeline v0.19.0 (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_gizmos v0.19.0 (*)
│   │       │   ├── bevy_image v0.19.0 (*)
│   │       │   ├── bevy_log v0.19.0 (*)
│   │       │   ├── bevy_material v0.19.0 (*)
│   │       │   ├── bevy_math v0.19.0 (*)
│   │       │   ├── bevy_mesh v0.19.0 (*)
│   │       │   ├── bevy_pbr v0.19.0
│   │       │   │   ├── arrayvec v0.7.8
│   │       │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   ├── bevy_camera v0.19.0 (*)
│   │       │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   ├── bevy_core_pipeline v0.19.0 (*)
│   │       │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   ├── bevy_diagnostic v0.19.0 (*)
│   │       │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   ├── bevy_gltf v0.19.0
│   │       │   │   │   ├── async-lock v3.4.2 (*)
│   │       │   │   │   ├── base64 v0.22.1
│   │       │   │   │   ├── bevy_animation v0.19.0 (*)
│   │       │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   ├── bevy_camera v0.19.0 (*)
│   │       │   │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   ├── bevy_image v0.19.0 (*)
│   │       │   │   │   ├── bevy_light v0.19.0 (*)
│   │       │   │   │   ├── bevy_material v0.19.0 (*)
│   │       │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   ├── bevy_mesh v0.19.0 (*)
│   │       │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   ├── bevy_tasks v0.19.0 (*)
│   │       │   │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   │   ├── bevy_world_serialization v0.19.0
│   │       │   │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_camera v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   │   │   ├── ron v0.12.2 (*)
│   │       │   │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   │   └── uuid v1.24.0 (*)
│   │       │   │   │   ├── fixedbitset v0.5.7
│   │       │   │   │   ├── gltf v1.4.1
│   │       │   │   │   │   ├── byteorder v1.5.0
│   │       │   │   │   │   ├── gltf-json v1.4.1
│   │       │   │   │   │   │   ├── gltf-derive v1.4.1 (proc-macro)
│   │       │   │   │   │   │   │   ├── inflections v1.1.1
│   │       │   │   │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   │   └── syn v2.0.119 (*)
│   │       │   │   │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   │   │   ├── serde_derive v1.0.229 (proc-macro) (*)
│   │       │   │   │   │   │   └── serde_json v1.0.151
│   │       │   │   │   │   │       ├── itoa v1.0.18
│   │       │   │   │   │   │       ├── memchr v2.8.3
│   │       │   │   │   │   │       ├── serde_core v1.0.229
│   │       │   │   │   │   │       └── zmij v1.0.23
│   │       │   │   │   │   ├── lazy_static v1.5.0
│   │       │   │   │   │   └── serde_json v1.0.151 (*)
│   │       │   │   │   ├── itertools v0.14.0 (*)
│   │       │   │   │   ├── percent-encoding v2.3.2
│   │       │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   ├── serde_json v1.0.151 (*)
│   │       │   │   │   ├── smallvec v1.15.2
│   │       │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   ├── tracing v0.1.44 (*)
│   │       │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   ├── bevy_image v0.19.0 (*)
│   │       │   │   ├── bevy_light v0.19.0 (*)
│   │       │   │   ├── bevy_log v0.19.0 (*)
│   │       │   │   ├── bevy_material v0.19.0 (*)
│   │       │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   ├── bevy_mesh v0.19.0 (*)
│   │       │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   ├── bevy_render v0.19.0 (*)
│   │       │   │   ├── bevy_shader v0.19.0 (*)
│   │       │   │   ├── bevy_tasks v0.19.0 (*)
│   │       │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   ├── fixedbitset v0.5.7
│   │       │   │   ├── indexmap v2.14.0 (*)
│   │       │   │   ├── nonmax v0.5.5
│   │       │   │   ├── offset-allocator v0.2.0 (*)
│   │       │   │   ├── smallvec v1.15.2
│   │       │   │   ├── static_assertions v1.1.0
│   │       │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   ├── tracing v0.1.44 (*)
│   │       │   │   └── wgpu-types v29.0.4 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_render v0.19.0 (*)
│   │       │   ├── bevy_shader v0.19.0 (*)
│   │       │   ├── bevy_sprite_render v0.19.0
│   │       │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   ├── bevy_camera v0.19.0 (*)
│   │       │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   ├── bevy_core_pipeline v0.19.0 (*)
│   │       │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   ├── bevy_image v0.19.0 (*)
│   │       │   │   ├── bevy_material v0.19.0 (*)
│   │       │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   ├── bevy_mesh v0.19.0 (*)
│   │       │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   ├── bevy_render v0.19.0 (*)
│   │       │   │   ├── bevy_shader v0.19.0 (*)
│   │       │   │   ├── bevy_sprite v0.19.0
│   │       │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   ├── bevy_camera v0.19.0 (*)
│   │       │   │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   ├── bevy_image v0.19.0 (*)
│   │       │   │   │   ├── bevy_log v0.19.0 (*)
│   │       │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   ├── bevy_mesh v0.19.0 (*)
│   │       │   │   │   ├── bevy_picking v0.19.0
│   │       │   │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_camera v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_input v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_mesh v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_time v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_window v0.19.0 (*)
│   │       │   │   │   │   ├── crossbeam-channel v0.5.16 (*)
│   │       │   │   │   │   ├── tracing v0.1.44 (*)
│   │       │   │   │   │   └── uuid v1.24.0 (*)
│   │       │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   ├── bevy_text v0.19.0
│   │       │   │   │   │   ├── bevy_app v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_asset v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_clipboard v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_color v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   │   │   │   ├── bevy_ecs v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_image v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_log v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_math v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_platform v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_reflect v0.19.0 (*)
│   │       │   │   │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   │   │   ├── parley v0.9.0
│   │       │   │   │   │   │   ├── fontique v0.9.0
│   │       │   │   │   │   │   │   ├── hashbrown v0.17.1 (*)
│   │       │   │   │   │   │   │   ├── linebender_resource_handle v0.1.1
│   │       │   │   │   │   │   │   ├── memmap2 v0.9.11
│   │       │   │   │   │   │   │   │   └── libc v0.2.189
│   │       │   │   │   │   │   │   ├── parlance v0.1.0
│   │       │   │   │   │   │   │   ├── read-fonts v0.39.2
│   │       │   │   │   │   │   │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │   │   │   │   └── font-types v0.11.3
│   │       │   │   │   │   │   │   │       └── bytemuck v1.25.2 (*)
│   │       │   │   │   │   │   │   └── smallvec v1.15.2
│   │       │   │   │   │   │   ├── harfrust v0.6.2
│   │       │   │   │   │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   │   │   │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │   │   │   ├── core_maths v0.1.1
│   │       │   │   │   │   │   │   │   └── libm v0.2.16
│   │       │   │   │   │   │   │   ├── read-fonts v0.39.2 (*)
│   │       │   │   │   │   │   │   └── smallvec v1.15.2
│   │       │   │   │   │   │   ├── hashbrown v0.17.1 (*)
│   │       │   │   │   │   │   ├── icu_normalizer v2.2.0
│   │       │   │   │   │   │   │   ├── icu_collections v2.2.0
│   │       │   │   │   │   │   │   │   ├── displaydoc v0.2.7 (proc-macro)
│   │       │   │   │   │   │   │   │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   │   │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   │   │   │   └── syn v3.0.3 (*)
│   │       │   │   │   │   │   │   │   ├── potential_utf v0.1.5
│   │       │   │   │   │   │   │   │   │   ├── writeable v0.6.3
│   │       │   │   │   │   │   │   │   │   └── zerovec v0.11.6
│   │       │   │   │   │   │   │   │   │       ├── yoke v0.8.3
│   │       │   │   │   │   │   │   │   │       │   ├── stable_deref_trait v1.2.1
│   │       │   │   │   │   │   │   │   │       │   ├── yoke-derive v0.8.2 (proc-macro)
│   │       │   │   │   │   │   │   │   │       │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   │   │   │       │   │   ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   │   │   │       │   │   ├── syn v2.0.119 (*)
│   │       │   │   │   │   │   │   │   │       │   │   └── synstructure v0.13.2
│   │       │   │   │   │   │   │   │   │       │   │       ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   │   │   │       │   │       ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   │   │   │       │   │       └── syn v2.0.119 (*)
│   │       │   │   │   │   │   │   │   │       │   └── zerofrom v0.1.8
│   │       │   │   │   │   │   │   │   │       │       └── zerofrom-derive v0.1.7 (proc-macro)
│   │       │   │   │   │   │   │   │   │       │           ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   │   │   │       │           ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   │   │   │       │           ├── syn v2.0.119 (*)
│   │       │   │   │   │   │   │   │   │       │           └── synstructure v0.13.2 (*)
│   │       │   │   │   │   │   │   │   │       ├── zerofrom v0.1.8 (*)
│   │       │   │   │   │   │   │   │   │       └── zerovec-derive v0.11.3 (proc-macro)
│   │       │   │   │   │   │   │   │   │           ├── proc-macro2 v1.0.107 (*)
│   │       │   │   │   │   │   │   │   │           ├── quote v1.0.47 (*)
│   │       │   │   │   │   │   │   │   │           └── syn v2.0.119 (*)
│   │       │   │   │   │   │   │   │   ├── utf8_iter v1.0.4
│   │       │   │   │   │   │   │   │   ├── yoke v0.8.3 (*)
│   │       │   │   │   │   │   │   │   ├── zerofrom v0.1.8 (*)
│   │       │   │   │   │   │   │   │   └── zerovec v0.11.6 (*)
│   │       │   │   │   │   │   │   ├── icu_normalizer_data v2.2.0
│   │       │   │   │   │   │   │   ├── icu_provider v2.2.0
│   │       │   │   │   │   │   │   │   ├── displaydoc v0.2.7 (proc-macro) (*)
│   │       │   │   │   │   │   │   │   ├── icu_locale_core v2.2.0
│   │       │   │   │   │   │   │   │   │   ├── displaydoc v0.2.7 (proc-macro) (*)
│   │       │   │   │   │   │   │   │   │   ├── litemap v0.8.2
│   │       │   │   │   │   │   │   │   │   ├── tinystr v0.8.3
│   │       │   │   │   │   │   │   │   │   │   ├── displaydoc v0.2.7 (proc-macro) (*)
│   │       │   │   │   │   │   │   │   │   │   └── zerovec v0.11.6 (*)
│   │       │   │   │   │   │   │   │   │   ├── writeable v0.6.3
│   │       │   │   │   │   │   │   │   │   └── zerovec v0.11.6 (*)
│   │       │   │   │   │   │   │   │   ├── stable_deref_trait v1.2.1
│   │       │   │   │   │   │   │   │   ├── writeable v0.6.3
│   │       │   │   │   │   │   │   │   ├── yoke v0.8.3 (*)
│   │       │   │   │   │   │   │   │   ├── zerofrom v0.1.8 (*)
│   │       │   │   │   │   │   │   │   ├── zerotrie v0.2.4
│   │       │   │   │   │   │   │   │   │   ├── displaydoc v0.2.7 (proc-macro) (*)
│   │       │   │   │   │   │   │   │   │   ├── yoke v0.8.3 (*)
│   │       │   │   │   │   │   │   │   │   └── zerofrom v0.1.8 (*)
│   │       │   │   │   │   │   │   │   └── zerovec v0.11.6 (*)
│   │       │   │   │   │   │   │   ├── smallvec v1.15.2
│   │       │   │   │   │   │   │   └── zerovec v0.11.6 (*)
│   │       │   │   │   │   │   ├── icu_properties v2.2.0
│   │       │   │   │   │   │   │   ├── icu_collections v2.2.0 (*)
│   │       │   │   │   │   │   │   ├── icu_locale_core v2.2.0 (*)
│   │       │   │   │   │   │   │   ├── icu_properties_data v2.2.0
│   │       │   │   │   │   │   │   ├── icu_provider v2.2.0 (*)
│   │       │   │   │   │   │   │   ├── zerotrie v0.2.4 (*)
│   │       │   │   │   │   │   │   └── zerovec v0.11.6 (*)
│   │       │   │   │   │   │   ├── icu_segmenter v2.2.0
│   │       │   │   │   │   │   │   ├── icu_collections v2.2.0 (*)
│   │       │   │   │   │   │   │   ├── icu_locale v2.2.0
│   │       │   │   │   │   │   │   │   ├── icu_collections v2.2.0 (*)
│   │       │   │   │   │   │   │   │   ├── icu_locale_core v2.2.0 (*)
│   │       │   │   │   │   │   │   │   ├── icu_locale_data v2.2.0
│   │       │   │   │   │   │   │   │   ├── icu_provider v2.2.0 (*)
│   │       │   │   │   │   │   │   │   ├── potential_utf v0.1.5 (*)
│   │       │   │   │   │   │   │   │   ├── tinystr v0.8.3 (*)
│   │       │   │   │   │   │   │   │   └── zerovec v0.11.6 (*)
│   │       │   │   │   │   │   │   ├── icu_provider v2.2.0 (*)
│   │       │   │   │   │   │   │   ├── icu_segmenter_data v2.2.0
│   │       │   │   │   │   │   │   ├── potential_utf v0.1.5 (*)
│   │       │   │   │   │   │   │   ├── utf8_iter v1.0.4
│   │       │   │   │   │   │   │   └── zerovec v0.11.6 (*)
│   │       │   │   │   │   │   ├── linebender_resource_handle v0.1.1
│   │       │   │   │   │   │   ├── parlance v0.1.0
│   │       │   │   │   │   │   ├── parley_data v0.9.0
│   │       │   │   │   │   │   │   └── icu_properties v2.2.0 (*)
│   │       │   │   │   │   │   └── skrifa v0.42.1
│   │       │   │   │   │   │       ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │   │       └── read-fonts v0.39.2 (*)
│   │       │   │   │   │   ├── serde v1.0.229 (*)
│   │       │   │   │   │   ├── smallvec v1.15.2
│   │       │   │   │   │   ├── smol_str v0.2.2 (*)
│   │       │   │   │   │   ├── swash v0.2.10
│   │       │   │   │   │   │   ├── skrifa v0.44.0
│   │       │   │   │   │   │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │   │   │   └── read-fonts v0.41.0
│   │       │   │   │   │   │   │       ├── bytemuck v1.25.2 (*)
│   │       │   │   │   │   │   │       ├── font-types v0.12.2
│   │       │   │   │   │   │   │       │   └── bytemuck v1.25.2 (*)
│   │       │   │   │   │   │   │       └── once_cell v1.21.4
│   │       │   │   │   │   │   ├── yazi v0.2.1
│   │       │   │   │   │   │   └── zeno v0.3.3
│   │       │   │   │   │   ├── sys-locale v0.3.2
│   │       │   │   │   │   ├── thiserror v2.0.19 (*)
│   │       │   │   │   │   ├── tracing v0.1.44 (*)
│   │       │   │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   │   ├── bevy_window v0.19.0 (*)
│   │       │   │   │   ├── radsort v0.1.1
│   │       │   │   │   ├── tracing v0.1.44 (*)
│   │       │   │   │   └── wgpu-types v29.0.4 (*)
│   │       │   │   ├── bevy_text v0.19.0 (*)
│   │       │   │   ├── bevy_transform v0.19.0 (*)
│   │       │   │   ├── bevy_utils v0.19.0 (*)
│   │       │   │   ├── bitflags v2.13.1 (*)
│   │       │   │   ├── bytemuck v1.25.2 (*)
│   │       │   │   ├── derive_more v2.1.1 (*)
│   │       │   │   ├── fixedbitset v0.5.7
│   │       │   │   ├── nonmax v0.5.5
│   │       │   │   └── tracing v0.1.44 (*)
│   │       │   ├── bevy_transform v0.19.0 (*)
│   │       │   ├── bevy_utils v0.19.0 (*)
│   │       │   ├── bytemuck v1.25.2 (*)
│   │       │   └── tracing v0.1.44 (*)
│   │       ├── bevy_gltf v0.19.0 (*)
│   │       ├── bevy_image v0.19.0 (*)
│   │       ├── bevy_input v0.19.0 (*)
│   │       ├── bevy_input_focus v0.19.0
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_input v0.19.0 (*)
│   │       │   ├── bevy_math v0.19.0 (*)
│   │       │   ├── bevy_picking v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_window v0.19.0 (*)
│   │       │   ├── log v0.4.33
│   │       │   └── thiserror v2.0.19 (*)
│   │       ├── bevy_light v0.19.0 (*)
│   │       ├── bevy_log v0.19.0 (*)
│   │       ├── bevy_material v0.19.0 (*)
│   │       ├── bevy_math v0.19.0 (*)
│   │       ├── bevy_mesh v0.19.0 (*)
│   │       ├── bevy_pbr v0.19.0 (*)
│   │       ├── bevy_picking v0.19.0 (*)
│   │       ├── bevy_platform v0.19.0 (*)
│   │       ├── bevy_post_process v0.19.0
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_asset v0.19.0 (*)
│   │       │   ├── bevy_camera v0.19.0 (*)
│   │       │   ├── bevy_color v0.19.0 (*)
│   │       │   ├── bevy_core_pipeline v0.19.0 (*)
│   │       │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_image v0.19.0 (*)
│   │       │   ├── bevy_math v0.19.0 (*)
│   │       │   ├── bevy_platform v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_render v0.19.0 (*)
│   │       │   ├── bevy_shader v0.19.0 (*)
│   │       │   ├── bevy_utils v0.19.0 (*)
│   │       │   ├── smallvec v1.15.2
│   │       │   ├── thiserror v2.0.19 (*)
│   │       │   └── tracing v0.1.44 (*)
│   │       ├── bevy_ptr v0.19.0
│   │       ├── bevy_reflect v0.19.0 (*)
│   │       ├── bevy_render v0.19.0 (*)
│   │       ├── bevy_scene v0.19.0
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_asset v0.19.0 (*)
│   │       │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_log v0.19.0 (*)
│   │       │   ├── bevy_platform v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_scene_macros v0.19.0 (proc-macro)
│   │       │   │   ├── bevy_ecs_macro_logic v0.19.0 (*)
│   │       │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   ├── proc-macro2 v1.0.107 (*)
│   │       │   │   ├── quote v1.0.47 (*)
│   │       │   │   └── syn v2.0.119 (*)
│   │       │   ├── bevy_utils v0.19.0 (*)
│   │       │   ├── smallvec v1.15.2
│   │       │   ├── thiserror v2.0.19 (*)
│   │       │   ├── tracing v0.1.44 (*)
│   │       │   └── variadics_please v1.1.0 (proc-macro) (*)
│   │       ├── bevy_shader v0.19.0 (*)
│   │       ├── bevy_sprite v0.19.0 (*)
│   │       ├── bevy_sprite_render v0.19.0 (*)
│   │       ├── bevy_state v0.19.0
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_platform v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_state_macros v0.19.0 (proc-macro)
│   │       │   │   ├── bevy_macro_utils v0.19.0 (*)
│   │       │   │   ├── quote v1.0.47 (*)
│   │       │   │   └── syn v2.0.119 (*)
│   │       │   ├── bevy_utils v0.19.0 (*)
│   │       │   ├── log v0.4.33
│   │       │   └── variadics_please v1.1.0 (proc-macro) (*)
│   │       ├── bevy_tasks v0.19.0 (*)
│   │       ├── bevy_text v0.19.0 (*)
│   │       ├── bevy_time v0.19.0 (*)
│   │       ├── bevy_transform v0.19.0 (*)
│   │       ├── bevy_ui v0.19.0
│   │       │   ├── accesskit v0.24.1 (*)
│   │       │   ├── bevy_a11y v0.19.0 (*)
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_asset v0.19.0 (*)
│   │       │   ├── bevy_camera v0.19.0 (*)
│   │       │   ├── bevy_color v0.19.0 (*)
│   │       │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_image v0.19.0 (*)
│   │       │   ├── bevy_input v0.19.0 (*)
│   │       │   ├── bevy_input_focus v0.19.0 (*)
│   │       │   ├── bevy_log v0.19.0 (*)
│   │       │   ├── bevy_math v0.19.0 (*)
│   │       │   ├── bevy_picking v0.19.0 (*)
│   │       │   ├── bevy_platform v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_sprite v0.19.0 (*)
│   │       │   ├── bevy_text v0.19.0 (*)
│   │       │   ├── bevy_time v0.19.0 (*)
│   │       │   ├── bevy_transform v0.19.0 (*)
│   │       │   ├── bevy_utils v0.19.0 (*)
│   │       │   ├── bevy_window v0.19.0 (*)
│   │       │   ├── derive_more v2.1.1 (*)
│   │       │   ├── parley v0.9.0 (*)
│   │       │   ├── smallvec v1.15.2
│   │       │   ├── swash v0.2.10 (*)
│   │       │   ├── taffy v0.10.1
│   │       │   │   ├── arrayvec v0.7.8
│   │       │   │   ├── grid v1.0.1
│   │       │   │   └── slotmap v1.1.1 (*)
│   │       │   ├── thiserror v2.0.19 (*)
│   │       │   ├── tracing v0.1.44 (*)
│   │       │   └── uuid v1.24.0 (*)
│   │       ├── bevy_ui_render v0.19.0
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_asset v0.19.0 (*)
│   │       │   ├── bevy_camera v0.19.0 (*)
│   │       │   ├── bevy_color v0.19.0 (*)
│   │       │   ├── bevy_core_pipeline v0.19.0 (*)
│   │       │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_image v0.19.0 (*)
│   │       │   ├── bevy_input_focus v0.19.0 (*)
│   │       │   ├── bevy_math v0.19.0 (*)
│   │       │   ├── bevy_mesh v0.19.0 (*)
│   │       │   ├── bevy_platform v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_render v0.19.0 (*)
│   │       │   ├── bevy_shader v0.19.0 (*)
│   │       │   ├── bevy_sprite v0.19.0 (*)
│   │       │   ├── bevy_sprite_render v0.19.0 (*)
│   │       │   ├── bevy_text v0.19.0 (*)
│   │       │   ├── bevy_transform v0.19.0 (*)
│   │       │   ├── bevy_ui v0.19.0 (*)
│   │       │   ├── bevy_utils v0.19.0 (*)
│   │       │   ├── bytemuck v1.25.2 (*)
│   │       │   ├── derive_more v2.1.1 (*)
│   │       │   ├── indexmap v2.14.0 (*)
│   │       │   └── tracing v0.1.44 (*)
│   │       ├── bevy_ui_widgets v0.19.0
│   │       │   ├── accesskit v0.24.1 (*)
│   │       │   ├── bevy_a11y v0.19.0 (*)
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_camera v0.19.0 (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_input v0.19.0 (*)
│   │       │   ├── bevy_input_focus v0.19.0 (*)
│   │       │   ├── bevy_log v0.19.0 (*)
│   │       │   ├── bevy_math v0.19.0 (*)
│   │       │   ├── bevy_picking v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_text v0.19.0 (*)
│   │       │   ├── bevy_ui v0.19.0 (*)
│   │       │   ├── bevy_window v0.19.0 (*)
│   │       │   ├── parley v0.9.0 (*)
│   │       │   └── smol_str v0.2.2 (*)
│   │       ├── bevy_utils v0.19.0 (*)
│   │       ├── bevy_window v0.19.0 (*)
│   │       ├── bevy_winit v0.19.0
│   │       │   ├── accesskit v0.24.1 (*)
│   │       │   ├── accesskit_winit v0.32.2
│   │       │   │   ├── accesskit v0.24.1 (*)
│   │       │   │   ├── raw-window-handle v0.6.2
│   │       │   │   └── winit v0.30.13
│   │       │   │       ├── ahash v0.8.12
│   │       │   │       │   ├── cfg-if v1.0.4
│   │       │   │       │   ├── getrandom v0.3.4
│   │       │   │       │   │   ├── cfg-if v1.0.4
│   │       │   │       │   │   └── libc v0.2.189
│   │       │   │       │   ├── once_cell v1.21.4
│   │       │   │       │   └── zerocopy v0.8.56 (*)
│   │       │   │       │   [build-dependencies]
│   │       │   │       │   └── version_check v0.9.5
│   │       │   │       ├── bitflags v2.13.1 (*)
│   │       │   │       ├── bytemuck v1.25.2 (*)
│   │       │   │       ├── calloop v0.13.0
│   │       │   │       │   ├── bitflags v2.13.1 (*)
│   │       │   │       │   ├── log v0.4.33
│   │       │   │       │   ├── polling v3.11.0 (*)
│   │       │   │       │   ├── rustix v0.38.44
│   │       │   │       │   │   ├── bitflags v2.13.1 (*)
│   │       │   │       │   │   └── linux-raw-sys v0.4.15
│   │       │   │       │   ├── slab v0.4.12
│   │       │   │       │   └── thiserror v1.0.69
│   │       │   │       │       └── thiserror-impl v1.0.69 (proc-macro)
│   │       │   │       │           ├── proc-macro2 v1.0.107 (*)
│   │       │   │       │           ├── quote v1.0.47 (*)
│   │       │   │       │           └── syn v2.0.119 (*)
│   │       │   │       ├── cursor-icon v1.2.0
│   │       │   │       ├── dpi v0.1.2
│   │       │   │       ├── libc v0.2.189
│   │       │   │       ├── memmap2 v0.9.11 (*)
│   │       │   │       ├── percent-encoding v2.3.2
│   │       │   │       ├── raw-window-handle v0.6.2
│   │       │   │       ├── rustix v0.38.44 (*)
│   │       │   │       ├── sctk-adwaita v0.10.1
│   │       │   │       │   ├── ab_glyph v0.2.32
│   │       │   │       │   │   ├── ab_glyph_rasterizer v0.1.10
│   │       │   │       │   │   └── owned_ttf_parser v0.25.1
│   │       │   │       │   │       └── ttf-parser v0.25.1
│   │       │   │       │   ├── log v0.4.33
│   │       │   │       │   ├── memmap2 v0.9.11 (*)
│   │       │   │       │   ├── smithay-client-toolkit v0.19.2
│   │       │   │       │   │   ├── bitflags v2.13.1 (*)
│   │       │   │       │   │   ├── calloop v0.13.0 (*)
│   │       │   │       │   │   ├── calloop-wayland-source v0.3.0
│   │       │   │       │   │   │   ├── calloop v0.13.0 (*)
│   │       │   │       │   │   │   ├── rustix v0.38.44 (*)
│   │       │   │       │   │   │   ├── wayland-backend v0.3.16
│   │       │   │       │   │   │   │   ├── downcast-rs v1.2.1
│   │       │   │       │   │   │   │   ├── rustix v1.1.4 (*)
│   │       │   │       │   │   │   │   ├── scoped-tls v1.0.1
│   │       │   │       │   │   │   │   ├── smallvec v1.15.2
│   │       │   │       │   │   │   │   └── wayland-sys v0.31.11
│   │       │   │       │   │   │   │       ├── dlib v0.5.3
│   │       │   │       │   │   │   │       │   └── libloading v0.8.9 (*)
│   │       │   │       │   │   │   │       └── log v0.4.33
│   │       │   │       │   │   │   │       [build-dependencies]
│   │       │   │       │   │   │   │       └── pkg-config v0.3.33
│   │       │   │       │   │   │   │   [build-dependencies]
│   │       │   │       │   │   │   │   └── cc v1.4.0 (*)
│   │       │   │       │   │   │   └── wayland-client v0.31.15
│   │       │   │       │   │   │       ├── bitflags v2.13.1 (*)
│   │       │   │       │   │   │       ├── rustix v1.1.4 (*)
│   │       │   │       │   │   │       ├── wayland-backend v0.3.16 (*)
│   │       │   │       │   │   │       └── wayland-scanner v0.31.11 (proc-macro)
│   │       │   │       │   │   │           ├── proc-macro2 v1.0.107 (*)
│   │       │   │       │   │   │           ├── quick-xml v0.41.0
│   │       │   │       │   │   │           │   └── memchr v2.8.3
│   │       │   │       │   │   │           └── quote v1.0.47 (*)
│   │       │   │       │   │   ├── cursor-icon v1.2.0
│   │       │   │       │   │   ├── libc v0.2.189
│   │       │   │       │   │   ├── log v0.4.33
│   │       │   │       │   │   ├── memmap2 v0.9.11 (*)
│   │       │   │       │   │   ├── rustix v0.38.44 (*)
│   │       │   │       │   │   ├── thiserror v1.0.69 (*)
│   │       │   │       │   │   ├── wayland-backend v0.3.16 (*)
│   │       │   │       │   │   ├── wayland-client v0.31.15 (*)
│   │       │   │       │   │   ├── wayland-csd-frame v0.3.0
│   │       │   │       │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │       │   │   │   ├── cursor-icon v1.2.0
│   │       │   │       │   │   │   └── wayland-backend v0.3.16 (*)
│   │       │   │       │   │   ├── wayland-cursor v0.31.14
│   │       │   │       │   │   │   ├── rustix v1.1.4 (*)
│   │       │   │       │   │   │   ├── wayland-client v0.31.15 (*)
│   │       │   │       │   │   │   └── xcursor v0.3.11
│   │       │   │       │   │   ├── wayland-protocols v0.32.13
│   │       │   │       │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │       │   │   │   ├── wayland-backend v0.3.16 (*)
│   │       │   │       │   │   │   ├── wayland-client v0.31.15 (*)
│   │       │   │       │   │   │   └── wayland-scanner v0.31.11 (proc-macro) (*)
│   │       │   │       │   │   ├── wayland-protocols-wlr v0.3.12
│   │       │   │       │   │   │   ├── bitflags v2.13.1 (*)
│   │       │   │       │   │   │   ├── wayland-backend v0.3.16 (*)
│   │       │   │       │   │   │   ├── wayland-client v0.31.15 (*)
│   │       │   │       │   │   │   ├── wayland-protocols v0.32.13 (*)
│   │       │   │       │   │   │   └── wayland-scanner v0.31.11 (proc-macro) (*)
│   │       │   │       │   │   ├── wayland-scanner v0.31.11 (proc-macro) (*)
│   │       │   │       │   │   └── xkeysym v0.2.1
│   │       │   │       │   └── tiny-skia v0.11.4
│   │       │   │       │       ├── arrayref v0.3.9
│   │       │   │       │       ├── arrayvec v0.7.8
│   │       │   │       │       ├── bytemuck v1.25.2 (*)
│   │       │   │       │       ├── cfg-if v1.0.4
│   │       │   │       │       ├── log v0.4.33
│   │       │   │       │       └── tiny-skia-path v0.11.4
│   │       │   │       │           ├── arrayref v0.3.9
│   │       │   │       │           ├── bytemuck v1.25.2 (*)
│   │       │   │       │           └── strict-num v0.1.1
│   │       │   │       ├── smithay-client-toolkit v0.19.2 (*)
│   │       │   │       ├── smol_str v0.2.2 (*)
│   │       │   │       ├── tracing v0.1.44 (*)
│   │       │   │       ├── wayland-backend v0.3.16 (*)
│   │       │   │       ├── wayland-client v0.31.15 (*)
│   │       │   │       ├── wayland-protocols v0.32.13 (*)
│   │       │   │       ├── wayland-protocols-plasma v0.3.12
│   │       │   │       │   ├── bitflags v2.13.1 (*)
│   │       │   │       │   ├── wayland-backend v0.3.16 (*)
│   │       │   │       │   ├── wayland-client v0.31.15 (*)
│   │       │   │       │   ├── wayland-protocols v0.32.13 (*)
│   │       │   │       │   └── wayland-scanner v0.31.11 (proc-macro) (*)
│   │       │   │       ├── x11-dl v2.21.0
│   │       │   │       │   ├── libc v0.2.189
│   │       │   │       │   └── once_cell v1.21.4
│   │       │   │       │   [build-dependencies]
│   │       │   │       │   └── pkg-config v0.3.33
│   │       │   │       ├── x11rb v0.13.2
│   │       │   │       │   ├── as-raw-xcb-connection v1.0.1
│   │       │   │       │   ├── gethostname v1.1.0
│   │       │   │       │   │   └── rustix v1.1.4 (*)
│   │       │   │       │   ├── libc v0.2.189
│   │       │   │       │   ├── libloading v0.8.9 (*)
│   │       │   │       │   ├── once_cell v1.21.4
│   │       │   │       │   ├── rustix v1.1.4 (*)
│   │       │   │       │   └── x11rb-protocol v0.13.2
│   │       │   │       └── xkbcommon-dl v0.4.2
│   │       │   │           ├── bitflags v2.13.1 (*)
│   │       │   │           ├── dlib v0.5.3 (*)
│   │       │   │           ├── log v0.4.33
│   │       │   │           ├── once_cell v1.21.4
│   │       │   │           └── xkeysym v0.2.1
│   │       │   │       [build-dependencies]
│   │       │   │       └── cfg_aliases v0.2.2
│   │       │   ├── approx v0.5.1
│   │       │   │   └── num-traits v0.2.19 (*)
│   │       │   ├── bevy_a11y v0.19.0 (*)
│   │       │   ├── bevy_app v0.19.0 (*)
│   │       │   ├── bevy_asset v0.19.0 (*)
│   │       │   ├── bevy_derive v0.19.0 (proc-macro) (*)
│   │       │   ├── bevy_ecs v0.19.0 (*)
│   │       │   ├── bevy_image v0.19.0 (*)
│   │       │   ├── bevy_input v0.19.0 (*)
│   │       │   ├── bevy_input_focus v0.19.0 (*)
│   │       │   ├── bevy_log v0.19.0 (*)
│   │       │   ├── bevy_math v0.19.0 (*)
│   │       │   ├── bevy_platform v0.19.0 (*)
│   │       │   ├── bevy_reflect v0.19.0 (*)
│   │       │   ├── bevy_tasks v0.19.0 (*)
│   │       │   ├── bevy_window v0.19.0 (*)
│   │       │   ├── bytemuck v1.25.2 (*)
│   │       │   ├── tracing v0.1.44 (*)
│   │       │   ├── wgpu-types v29.0.4 (*)
│   │       │   └── winit v0.30.13 (*)
│   │       └── bevy_world_serialization v0.19.0 (*)
│   └── bevy_internal v0.19.0 (*)
└── chrono v0.4.45
    ├── iana-time-zone v0.1.65
    └── num-traits v0.2.19 (*)
```