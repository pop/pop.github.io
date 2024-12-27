+++
title = "Let's Build a Game with Rust"

date = "2020-11-13"

description = "A talk I gave at Seattle GNU Linux Conference 2020"

taxonomies.tags = [
    "rust", "amethyst", "games"
]
+++

{% note() %}
This is the outline for a talk I gave at the Seattle GNU Linux
conference (SeaGL) 2020. Once the video is posted I will link it here.
{% end %}


{% note() %}
The code for this post can be found at https://github.com/pop/lets-make-games-with-rust.
{% end %}

Like many of you, I really like games.
I enjoy playing games, talking about games, and a few times I've even tried making games.
I enjoy making games for a few reasons:

- Games are an interesting technical challenge.
- They are a flexible artistic outlet.
- I don't make games in my day-job (yay hobbies).

I'm also interested in this programming language called Rust!
You might have heard of it.
Rust is a maturing systems programming language which aims to be performant, reliable, and productive.

- Performant in that it often goes head to head with C and C++ in benchmarks.
- Reliable in that it refuses to compile memory unsafe code.
- Productive because it includes "zero cost abstractions" (link `filter` and `map`) and awesome tooling.

The community is pretty good too!

This post is about bringing those two interests together.
Let's learn how to build a videogame with the Rust programming language.

{% warning() %}
This post is for folks who have a passing familiarity with Rust.
If Rust is new to you, you get a little confused.

You're a smart cookie though, I'm sure you'll keep up.
{% end %}

## Making Games: Entities, Components, Systems ⚙️

Games are an incredibly fun and flexible type of project to work on.

At it's core, all games need a "game loop":

1. Process input
2. Transforms state
3. Display new state
4. Repeat

Outside of that, the possibilities are endless.
But while the *possibilities* are endless, there are a few *patterns* that lots of folks seem to gravitate toward.

You could write a whole book on game programming patterns (and somebody has, links at the bottom).
Today we're going to talk about one popular pattern: **ECS: Entity Component System**.
Here's what that looks like:

### Components

Pieces of data mixed, matched, and queried on.

Example: Some components needed to simulate physics might be "Mass", "Movable", and "Friction"

### Entities

> A Unique ID associated with a collection of Components.

Example: Potted plant you can break have the following components:

- `Sprite("/path/to/plant.png")`
- `Mass(6.8)`
- `Movable(True)`
- `Location((2, 5, 2))`

Each of these components are reusable. Instead of creating a "PottedPlant" class in code, we can define each entity in a config file like this:

```
potted-plant.txt:
Sprite /path/to/plant.png
Mass 6.8
Movable True
Location 2 5 2
```

This makes our engine much more reusable and separates our content from our logic.

But how do we use these components?

### Systems

Functions that operate on entities with specific components.

A System queries for all entities with a specific subset of components and does some transformation on it.

Example: a system that applies fire damage to an entity might look like this:

```
entities_on_fire = entities.query(on_fire=True, health > 0)
for entity in entities_on_fire:
    entity.health -= 5
```

This is nice compared with a classes-based approach where we would need to manage inheritance to manually make sure everything interacted
correctly.
Here we define systems based on what features an entity has.
The resulting systems and components interact with eachother in interesting and potentially unexpected ways.

ECS is a popular pattern for creating interactive games and simulations.
Engines like Unity have some ECS patterns built in, and almost every big game engine uses ECS in some way.

Of course ECS isn't a silver bullet, but for this blog post it's good enough.

## Tools of the Trade: C and C++ ⚒️

C and C++ are very popular languages in games programming.
They are defacto in the industry and many large engines, both internal and licensed engines, are written in C++.

I'm not here to bash on C and C++, but it can be useful to point out why you would bother using Rust if the norm is C++.

The usual arguments in favor of Rust go a little something like this:

- Rust is memory safe; in Rust it is very difficult to reference freed memory, mutate memory in two threads, and dereference a null pointer.
- Rust is expressive; a lot of functional-programming language features exist in Rust without the usual run-time cost of those languages.
- Rust doesn't have any of the C/C++ legacy baggage but *can* inter-operate with C/C++ codebases.
- Rust has a kick-ass community and an ecosystem of battle-tested and safe code.

So why does this all matter for games? I think of it it like this: Any project when it gets sufficiently complex benefits from Rust.
Rust, by preventing a whole class of memory bugs, makes it easier to maintain a complex codebase over time.
It might not be life or death, or as mission critical as security software, but completely avoiding null-pointer bugs, at essentially no performance cost, sounds like a huge weight off my shoulders.

Games are by their nature huge and sprawling codebases.
Many bugs in games are caught by a compiler, but even more errors would be caught by the Rust borrow-checker.
And being able to use some of the nice functional-programming features would be nice too.

Of course Rust is a relatively new language so your mileage may vary.
If I ran a big game studio I don't think I would throw out my C++ code and start fresh with Rust, but I would definitely put some research and development into it for new projects (said the Rust fanboy).

## Rusty Games: Hello Amethyst 💎

Writing games in a safe, expressive, not C/C++ language sounds great; where do I start?

You could write a game from scratch, but there are engines written in Rust you can use today!
Some of these focus on ease of use, some are for 2D games, some focus on compiling for the web.
Most of these engines require you to write Rust, as opposed to using a GUI, but even that is changing.

For a comprehensive list check out <https://www.arewegameyet.rs>

You could write this talk for almost any Rust Game Engine, but my personal favorite is Amethyst, so we'll use that.
Amethyst has a solid API, very active community, and is a good mix of flexible, convenient, and powerful.

Amethyst checks off a few other boxes:

- Implements an ECS runtime. Register Components, create Entities, and run Systems in Amethyst.
- Data driven design. Almost all data in Amethyst can be read in from a Config file.
- Apache + MIT licensed. Free as in speech is always nice.

### Step 0: Join the Cargo Cult

In this step we're going to get Rust setup and create a "hello world" Rust project.

If you haven't already, setup your Rust toolchain and start a Rust project.

1. Install `rustup`, the Rust toolchain manager.
2. Run `rustup toolchain install stable` to install the latest stable Rust.
3. Run `cargo new seagl-game` to create a "hello world" Rust application.
4. Navigate to the new `seagl-game` folder.
   Add this to the end of our project's metadata file, `Cargo.toml`:

```
# Cargo.toml
[dependencies.amethyst]
version = "0.15.1"
features = ["vulkan"]  # "metal" on MacOS
```

5. Run `cargo build` to build and cache our dependencies.
   You should see a **bunch** of output like this:

```
$ cargo build
...
Compiling either v1.6.1
Compiling gimli v0.23.0
Compiling adler v0.2.3
Compiling object v0.22.0
...
```

Now we have a "hello world" Rust project we can start building on.

### Step 1: Draw a Window 📐

Before we run, we need to walk.
And before we walk we crawl.
And before we crawl we draw a window.
This is, of course, a little harder than just asking your computer "Please draw me a window".

First we need to add a bunch of imports to our project:

```rust
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        timing::Time,
        transform::{Transform, TransformBundle},
    },
    derive::SystemDesc,
    ecs::{
        Component, DenseVecStorage, Entities, Join, Read, ReadStorage, System, SystemData,
        WriteStorage,
    },
    input::{InputBundle, InputHandler, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        Camera, ImageFormat, RenderingBundle, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    utils::application_root_dir,
};
```

This is every dependency we will need for the entire project, so if `cargo build|run` complains about unused dependencies, don't worry... we'll get there.

Here we are including a few useful

Then we need to add some boiler-plate to our `main` function:

```rust
// This is necessary to make Rust's type-checker happy
// Our main function technically returns an Amethyst Result
// It can either return an Amethyst error or a unit value
fn main() -> amethyst::Result<()> {
    // Not required, but a logger very useful
    amethyst::start_logger(Default::default());

    // Declare some useful variables used to tell Amethyst where our asset files and config files live
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");

    // Declare a renderer bundle
    // Amethyst adds this collection of 2D Render systems to our game's runtime
    let renderer = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?
                .with_clear([1.00, 0.33, 0.00, 1.0]),
        ).with_plugin(RenderFlat2D::default());

    // Build the game's systems
    let game_data = GameDataBuilder::default()
        .with_bundle(renderer)?;

    // Build our application, which includes our game data, where our assets live, and our starting state
    let mut game = Application::new(assets_dir, SeaglState, game_data)?;

    // Run the game!
    game.run();

    // Nothing bad happened, so return `()`
    Ok(())
}
```

That won't compile because we haven't defined our `SeaglState`.

```
$ cargo run
...
error[E0425]: cannot find value `SeaglState` in this scope
  --> src/main.rs:17:49
   |
30 |     let mut game = Application::new(assets_dir, SeaglState, game_data)?;
   |                                                 ^^^^^^^^^^ not found in this scope
```

Let's add `SeaglState`

```rust
// States can store values, but for now we can use a unit-struct
struct SeaglState;

// We get a working state for free by rubber-stamping the "SimpleState" struct onto our SeaglState
// We will implement our own logic for handling state start-up in the next step
impl SimpleState for SeaglState { }
```

We will add some methods to `SeaglState` later, but for now this makes Rust and Amethyst happy enough to compile.

But if we run our code we get a wonderfully cryptic error message:

```
Compiling seagl-talk v0.1.0 (/home/pop/seagl-talk)
 Finished dev [unoptimized + debuginfo] target(s) in 24.81s
  Running `target/debug/seagl-talk`
Error: Error { inner: Inner { source: None, backtrace: None, error: File(Os { code: 2, kind: NotFound, message: "No such file or directory" }) } }
```

We get errors like this when we have an unhandled exception in our code.
In our `main` function, that is any place where we call a function with a `?`, e.g., `foo(...)?;`.

TLDT (Too Long Didn't Troubleshoot) this is because we haven't created our display config file!

Add a new file `display.ron` in a new folder called `config/`:

```rust
// config/display.ron
(
    title: "SeaGL!",
    dimensions: Some((500, 500)),
)
```

Now when we `cargo run` we should get a wonderful orange window:

![It worked! We drew a window.](/images/seagl-2020/blank-window.png)

### Step 2: Draw a SeaGL 🕊️

Alas, we have a window but no game! Let's draw our first character to the screen.

{% note() %}
Did you know that SeaGL's mascot is named Patch?
https://seagl.org/news/2020/09/10/naming-contest.html
{% end %}

First we'll create a Component for our Seagl.

```rust
#[derive(Default)]
pub struct Seagl;

impl Component for Seagl {
    type Storage = DenseVecStorage<Self>;
}
```

Next we'll create a Seagl entity.

```rust
impl SimpleState for SeaglState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(50.0, 50.0, 0.0);
        let seagl = Seagl::default();
        data.world
            .create_entity()
            .with(seagl)
            .with(transform)
            .build();
    }
}
```

This is a good start, but our Seagl is a spriteless ghost!

{% warning() %}
Seagull ghosts are terrifying.
Add a sprite!
{% end %}


First we need to load the spritesheet into memory.
Add this in our *on_start* function above where we added the seagl:

```rust
let sprite_sheet_handle = {
    let loader = data.world.read_resource::<Loader>();
    let texture_storage = data.world.read_resource::<AssetStorage<Texture>>();
    let texture_handle = loader.load(
        "texture/spritesheet.png",
        ImageFormat::default(),
        (),
        &texture_storage,
    );

    let sprite_sheet_store = data.world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
};
```

Then modify our Seagl entity like so:

```diff
++ main.rs
@@ impl SimpleState for SeaglState
@@ fn on_start(...)
  let mut transform = Transform::default();
  transform.set_translation_xyz(50.0, 50.0, 0.0);
+ let sprite = SpriteRender::new(sprite_sheet_handle.clone(), 0);
  let seagl = Seagl::default();
  data.world
      .create_entity()
      .with(seagl)
+     .with(sprite)
      .with(transform)
      .build();
```

Let's see.
We created a Seagl entity. Let's try running this thing:

```
$ cargo run
...
thread 'main' panicked at 'Tried to fetch resource of type `MaskedStorage<Seagl>`[^1] from the `World`, but the resource does not exist.

You may ensure the resource exists through one of the following methods:

* Inserting it when the world is created: `world.insert(..)`.
* If the resource implements `Default`, include it in a system's `SystemData`, and ensure the system is registered in the dispatcher.
* If the resource does not implement `Default`, insert it in the world during `System::setup`.
```

Hmm.
It seems like our `Seagl` Component isn't registered with Amethyst.
This happens implicitly when we add a system that uses our component, but until we write a System, we'll have to explicitly register our Component with Amethyst.

Add this toward the top of our `on_start` method:

```rust
data.world.register::<Seagl>();
```

Let's try running again:

```
$ cargo run
...
Error { inner: Inner { source: Some(Error { inner: Inner { source: None, backtrace: None,
error: Os { code: 2, kind: NotFound, message: "No such file or directory" } } }), backtrace: None,
error: StringError("Failed to fetch metadata for \"/home/pop/seagl-talk/assets/texture/spritesheet.ron\"") } }
```

Ah, a different runtime error.
This time we forgot to add our spritesheet image and spritesheet config file.
Lets add those.

Add this code to a file in `assets/texture/spritesheet.ron`:

```rust
// assets/texture/spritesheet.ron
List((
    texture_width: 32,
    texture_height: 16,
    sprites: [
        ( // Seagl
            x: 0,
            y: 0,
            width: 16,
            height: 16,
        ),
        ( // Burger
            x: 16,
            y: 0,
            width: 10,
            height: 8,
        ),
    ],
))
```

And save this image to `assets/texture/spritesheet.png`:

![Seagl and Burger. 32x16. Pixel on LCD.](/images/seagl-2020/spritesheet.png)

Now if we run `cargo run` we get the same blank orange window.
This happened because we forgot to add a Camera to the scene!

Add this to the end of our `on_start` function:

```rust
let mut transform = Transform::default();
transform.set_translation_xyz(50.0, 50.0, 1.0);
data.world
    .create_entity()
    .with(Camera::standard_2d(100.0, 100.0))
    .with(transform)
    .build();
```

![That's a nice looking Seagl there...](/images/seagl-2020/window-with-seagl.png)

{% note() %}
It's so beautifull...
{% end %}

### Step 3: Move Around 🏇

Thinking back to our ECS discussion, we have two of the three ingredients: an Entity, some Components, but no Systems!

First, we need to create a System struct and implement `System` on it.

Our System's run function looks like this in psuedocode:

```txt
for every seagl that can move:
    If the user input was to move horizontal:
        Move the seagl horizontally
    If the user input was to move vertical:
        Move the seagl vertically
```

This doesn't look *exactly* the same in Rust, but it's pretty close.

```rust
[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Seagl>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, seagls, time, input): Self::SystemData) {
        let speed: f32 = 50.0;
        for (_seagl, transform) in (&seagls, &mut transforms).join() {
            if let Some(horizontal) = input.axis_value("horizontal") {
                transform.prepend_translation_x(
                    horizontal * time.delta_seconds() * speed  as f32
                );

            };
            if let Some(vertical) = input.axis_value("vertical") {
                transform.prepend_translation_y(
                    vertical * time.delta_seconds() * speed as f32
                );
            };
        }
    }
}
```

We declare a `SystemData` type which is a tuple of components.
The `Transform` component will be modified, so we require it as `mut`, but everything else is `Read` for stuff that Amethyst provides and `ReadStorage` for things we created.

We loop over every entity with the `Seagl` and `Transform` components, then we match against any user input:

- If we had "vertical" input, move the entity on the x axis.
- If we had "horizontal" input, move the entity on the y axis.
- We don't need to explicitly say "move left"/"move right" because the horizontal/vertical inputs can be positive or negative.

Next we need to register this system with out game.
Because we are using Inputs we also need to register the inputs bundle with the game.

```diff
+++ main.rs
@@ fn main() -> amethyst::Result<()>
     )
     .with_plugin(RenderFlat2D::default());

+    let bindings_path = app_root.join("config").join("bindings.ron");
+    let inputs = InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?;
+
     let game_data = GameDataBuilder::default()
         .with_bundle(transform)?
         .with_bundle(renderer)?
+        .with_bundle(inputs)?
+        .with(MoveSystem, "move_system", &["input_system"]);

     let mut game = Application::new(assets_dir, SeaglState, game_data)?;
```

We have a dependency on the `input_system`, so Amethyst will ensure that system runs before `move_system`.

Next we need to create a config file for our movement bindings.
Instead of hard-coding "Up arrow means move up, down arrow means down" we put that in config files:

```rust
// config/bindings.ron
(
    axes: {
        "horizontal": Emulated(pos: Key(Right), neg: Key(Left)),
        "vertical": Emulated(pos: Key(Up), neg: Key(Down)),
    },
    actions: {},
)
```

![It moves!](/images/seagl-2020/SeaGL-move.gif" class="align-center)

This is a good start, but you'll notice the Seagl doesn't turn left and right, this *totally* breaks my suspension of disbelief so we're gonna need to fix that in our `run` method:

```diff
diff --git a/src/main.rs b/src/main.rs
@@ impl<'s> System<'s> for MoveSystem
@@ run(...)
  if let Some(vertical) = input.axis_value("vertical") {
      transform.prepend_translation_x(
        horizontal * time.delta_seconds() * speed  as f32
      );
+     if horizontal > 0.0 {
+       transform.set_rotation_y_axis(std::f32::consts::PI);
+     }
+     if horizontal < 0.0 {
+       transform.set_rotation_y_axis(0.0);
+     }
+
  };
  if let Some(vertical) = input.axis_value("vertical") {
      transform.prepend_translation_y(
```

In our "horizontal" check we added:

- If the input was greater than 0, flip our sprite on the Y axis.
- If the input was less than 0, reset our sprite on the Y axis.

This makes it look like our Seagl is facing the direction they're moving which should help boost our Metacritic score when we publish this at the end of the blogpost.

{% note() %}
We rotate by PI because our 2D sprite is in the 3D world and we're rotating it in radians.

Do you ever feel like a 2D sprite in a 3D world? I know I do...
{% end %}

![It moves!](/images/seagl-2020/SeaGL-move-look.gif)

### Step 4: Eat some food! 🍔

I'm sure we could all get *minutes* of fun out of moving our seagl around the screen, but this game could really use something else... Something tastier.

Let's add burgers.

This will require us to do everything we just did, again:

- Add a Food Compnent.
- Create a Burger entity with the food component.
- Add an Eat system.
- Register our Eat system with the game.

First we need to add a food Component.

Add this component anywhere that feels right:

```rust
#[derive(Default)]
pub struct Food;

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}
```

It's structurally identical to our Seagl, but with a different `struct` it's a totally different component.

With a Food component we can add our Burger entity.
Add this code to our `on_setup` function at the end:

```rust
let burger_sprite = SpriteRender::new(sprite_sheet_handle.clone(), 1);
let mut transform = Transform::default();
transform.set_translation_xyz(75.0, 75.0, -1.0);
data.world
    .create_entity()
    .with(Food::default())
    .with(burger_sprite)
    .with(transform)
    .build();
```

We create an entity spawning it at the point (75, 75, -1).

{% note() %}
We spawn the burger at `z=-1` to ensure the Seagl sprite is closer to
the camera and thus is drawn on top of the burger.

Have you ever seen a Seagull *behind* a burger? That's ridiculous.
{% end %}

{% note() %}
A few exercises left to the reader:

1. How would you spawn multiple burgers?
2. How would you re-spawn burgers when one is eaten?
3. How would you keep track of how many burgers were eaten?
4. How would you display the number of burgers eaten?

I've only covered enough in this post to answer the first two.
{% end %}

And finally an "eat" system.

This system's pseudocode looks like this:

```txt
For each seagl with a location:
    For each Food with a location:
        If the Seagl overlaps with the Food:
            Destory that food
```

This is a bit of a hack.
If this were a real game we would keep track of how many burgers the Seagl ate, but for this demo, we'll be lazy:

```rust
pub struct EatSystem;

impl<'s> System<'s> for EatSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Seagl>,
        ReadStorage<'s, Food>,
        Entities<'s>,
    );

    fn run(&mut self, (transforms, seagls, foods, entities): Self::SystemData) {
        for (_seagl, seagl_pos) in (&seagls, &transforms).join() {
            for (_food, food_pos, entity) in (&foods, &transforms, &entities).join() {
                // https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection
                if (seagl_pos.translation().x < food_pos.translation().x + 5.0) &&
                   (seagl_pos.translation().x + 8.0 > food_pos.translation().x) &&
                   (seagl_pos.translation().y < food_pos.translation().y + 4.0) &&
                   (seagl_pos.translation().y + 8.0 > food_pos.translation().y)
                {
                    entities.delete(entity).unwrap();
                }
            }
        }
    }
}
```

And last but not least, we need to register this system with our game:

```diff
+++ main.rs
@@ fn main() -> amethyst::Result<()>
     let game_data = GameDataBuilder::default()
         .with_bundle(transform)?
         .with_bundle(renderer)?
         .with_bundle(inputs)?
         .with(MoveSystem, "move_system", &["input_system"])
+        .with(EatSystem, "eat_system", &["move_system"]);
```

<img src="/images/seagl-2020/SeaGL-move-look-burger.gif"
class="align-center" alt="It moves!" />

## Conclusions

We did it. We made a lil' game. It had a Seagl and a burger. And we had
fun making it.

I wouldn't say it's *easy* to make games in Rust, but we are *very far*
from having to write games from scratch.

If this post piqued your interest I hope you check out <https://arewegameyet.rs> to learn more about the Rust Games ecosystem, and <https://amethyst.rs> to learn more about this budding Game Engine.

## Links

- SeaGL conference website: <https://seagl.org/> (You should go if you're in the Pacific Northwest)
- The code for this post is avaliable at <https://github.com/pop/lets-make-games-with-rust>. I even tagged each step so you can see exactly what we added!
- Rust Language: <https://www.rust-lang.org/>
- Are We Game Yet?: <https://arewegameyet.rs/>
- Amethyst Game Engine website: <https://amethyst.rs/>
- Amethyst Game Engine book has a great introduction and overview: <https://book.amethyst.rs/stable/>
- Bevy Game Engine is an interesting iteration on Game Engines in Rust: <https://bevyengine.org/>
- My Source that C++ is the defacto language in the games industry: <https://youtu.be/rX0ItVEVjHc>
- Game Programming Patterns is an awesome book with a free & legal copy online: <https://gameprogrammingpatterns.com/>
- `rustup` homepage for installation instructions: <https://rustup.rs/>
