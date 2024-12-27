+++
title = "Rust Ergonomics: Default and From"

date = "2022-05-17"

description = "Exploring ways to make Rust more ergonomic through the use of the Default From Traits"

taxonomies.tags = [
    "rust"
]
+++

I've been writing Rust off and on since 2014 and consistently since 2019
when I got into Rust Game Development. Once I started writing more Rust
code I noticed it wasn't just more lines of code, but each part of the
code was more verbose.

Coming from Python where ideas *tend* to be pretty succinct, Rust forced
you to spell everything out in intense detail. Of course you *got*
something for that verbosity -- "if it compiles, it probably works" --
but my hands were getting tired. There has to be a better way!

## Verbose Structs

There is a better way, but let's clarify what the problem *is* exactly.
Take this example of a real struct from the [bevy game engine](https://bevyengine.org/) `PbrBundle`:

``` rust
pub type PbrBundle = MaterialMeshBundle<StandardMaterial>;
```

It's a type-alias to a `MaterialMeshBundle`.
Let's check that out:

``` rust
pub struct MaterialMeshBundle<M> where
    M: SpecializedMaterial,
{
    pub mesh: Handle<Mesh>,
    pub material: Handle<M>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
```

Wow that's a fair number of struct members.

If you wanted to create a `PbrBundle` by hand it would be a *tedious* process.

``` rust
let my_pbr = PbrBundle {
    mesh: get_mesh_handle(),
    material: get_material_handle(),
    transform: Transform {
        translation: Vec3::new(),
        rotation: Quat::new(),
        scale: Vec3::new(),
    },
    global_transform: GlobalTransform {
        translation: Vec3::new(),
        rotation: Quat::new(),
        scale: Vec3::new(),
    },
    visibility: Visibility {
        is_visibile: true,
    },
    computed_visibility: ComputedVisibility {
        is_visibile: true,
    },
}
```

Wow my fingers are already tired.

Now, assuming you don't know the punchline, you're probably thinking:
*Just use a constructor!*

That solves the use-case where the author of the code has a constructor for my use-case, something like this:

``` rust
let my_pbr = PbrBundle::new(mesh, material); // Default Transform and Visibilty
```

But what if I want a *mostly* "default" `PbrBundle`, with say `is_visible = false`?
Or I want to add a Transform with a custom scale but a default translation and rotation?

Basically, what if I am *picky* and want the *flexibility* of struct initialization with the *convenience* of constructor methods?

## Default: Fill in the blanks

This is totally supported thanks to [Rust's "Default" Trait](https://doc.rust-lang.org/std/default/trait.Default.html).

The usage is something like this from the previous example:

``` rust
let my_pbr = PbrBundle {
    mesh: get_mesh_handle(),
    material: get_material_bundle(),
    transform: Transform {
        scale: Vec3::new(2.0, 2.0, 2.0),
        ..Default::default()
    },
    visibility: Visibilty {
        is_visible: true,
    },
    ..Default::default()
}
```

This example shows creating a `PbrBundle` with a custom `mesh`, `material`, and `scale`, but everything else is a "Default" value.

While this flexibility is totally possible with constructors, it would require some creativity, maybe something like this?

``` rust
let pbr_bundle = PbrBundle::new(mesh, material)
    .with_scale(Vec3::new(2.0, 2.0, 2.0))
    .with_visibility(true);
```

This is fine, but it is a **lot** of toil for the author.
They need to add and maintain a method for each element of their nested struct, document those methods, probably write tests, and all to accomplish the goal of a "Fill in the rest for me" API.

One nice part of `Default` is it can be automagically added to any struct whose members implement it via `#[derive(Default)]`.
This means you get that "Fill in the rest for me" interface for free!

## Close but Distinct Types

Another pain-point I found in Rust was converting between similar but distinct types.
Unlike my last language Python, which was *very* forgiving about types (to a fault), Rust requires very precise type expressions.

Let's take this example:

``` rust
// Base Engine Color
#[derive(Default, Debug, PartialEq)]
struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

/// Color for UI elements
#[derive(Default, Debug, PartialEq)]
struct UiColor(Color);

/// Just a demo function, not sure if this is useful...
fn color_rotate(color: Color) -> Color {
    Color {
        red: color.green,
        green: color.blue,
        blue: color.red,
    }
}

// Does not compile!
// E0308: mismatched types expected struct `Color`, found struct `UiColor`
fn main() {
    let a = UiColor(Color {
        red: 0.5,
        ..Default::default()
    });

    let b = color_rotate(a);
}
```

Here we have a `UiColor` struct that wraps our base `Color` struct.
We want to use a method made for `Color` values but we get an error that the compiler is expecting a `Color` but we gave it a `UiColor`!
Come on Rust, just look inside the box!

We can work around this issue like so:

``` rust
fn main() {
    // ...


    let b = UiColor(color_rotate(a.0));
}
```

Which passes the inside of `a` to `color_rotate` and then wraps the return in a new `UiColor` struct.
This works, but it's hard to read and more importantly it requires a keep our API in our head to write *any* code.

## From and `into`: Simple type coercion

The solution is to use the ["From"](https://doc.rust-lang.org/std/convert/trait.From.html) and trait which provides the `into()` method.

Extending the above example, we can implement `From Color -> UiColor` and `From UiColor -> Color` like so:

``` rust
impl From<Color> for UiColor {
    fn from(input: Color) -> UiColor {
        UiColor(input)
    }
}

impl From<UiColor> for Color {
    fn from(input: UiColor) -> Color {
        input.0
    }
}
```

Unfortunately we can't do anything like `#[derive(From<UiColor>)]` (yet?) but implementing these traits is fairly straight forward and *very* powerful.

Here we can see our `main` function is fixed with passing assertions.

``` rust
fn main() {
    let a = UiColor(Color {
        red: 0.5,
        ..Default::default()
    });
    assert_eq!(a, UiColor(Color { red: 0.5, green: 0.0, blue: 0.0 }));

    let b: UiColor = color_rotate(a.into()).into();
    assert_eq!(b, UiColor(Color { red: 0.0, green: 0.0, blue: 0.5 }));
}
```

Rust was not only able to cast our `UiColor` to a `Color` in the call to `color_rotate` but we were able to coerce the result back to a `UiColor` by declaring the type of our `b` variable.

Using `From` and `into()` is great because it allows you to ignore the specifics of the types you're working with while still getting the benefits of a strong type system.
When you use it enough it can feel like parts of your code are "Duckly" typed, like Python and Ruby which have very ergonomic type interactions.

## From Simple to Complex

Since learning about `From` started to abuse it to convert simplified types to more complex ones.

Take for example this UI struct in Bevy:

``` rust
pub struct NodeBundle {
    pub node: Node,
    pub style: Style,
    pub color: UiColor,
    pub image: UiImage,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
}
```

On it's own this isn't bad, but if you write enough UI code it can get tedious.
Both `Node` and `Style` are nested structs that have a lot of complexity -- `Style` is a struct with 21 members! -- so using `Default` won't cut it here.

Instead I made a "dumbed down" version like this:

``` rust
struct SimpleNodeBundle {
    position: SimplePosition,
    color: Color,
    size: Vec2,
}

enum SimplePosition {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}
```

This is maybe *too* simple, but you can add the complexity you need down the line. The important part is that our `Simple` struct is you know... less complex than what it's going to map to.

Now that we have a simple struct that we can use to quickly write out some UI elements.

``` rust
let my_ui_element = SimpleNodeBundle {
    position: SimplePosition::TopRight,
    size: Vec2 { x: 25.0, y: 100.0 },
    color: Color::RED,
}
```

On it's own though this is useless.
Bevy doesn't know what a `SimpleNodeBundle` is, we need to convert this to the "lower level" struct it's replacing.
We need to cast it up to a Bevy `NodeBundle` with an implementation of `From<SimpleNodeBundle> for NodeBundle`:

``` rust
impl From<SimpleNodeBundle> for NodeBundle {
    impl from(input: SimpleNodeBundle) -> NodeBundle {
        NodeBundle {
            color: UiColor(input.color),
            style: Style {
                position: input.position.into(),
                size: input.size.into(),
                ..Default::default()
            }
            ..Default::default()
        }
    }
}

// NodeBundle's position is a Rect<Val> so we convert SimplePosition to Rect<Val>
impl From<SimplePosition> for Rect<Val> {
    impl from(input: SimplePosition) -> Rect<Val> {
        use SimplePosition::*;
        match input {
            BottomLeft  => Rect {
                bottom: percent(0.0),
                left: percent(0.0),
                ..Default::default()
            },
            BottomRight => Rect {
                bottom: percent(0.0),
                right: percent(0.0),
                ..Default::default()
            },
            TopLeft     => Rect {
                top: percent(0.0),
                left: percent(0.0),
                ..Default::default()
            },
            TopRight    => Rect {
                top: percent(0.0),
                right: percent(0.0),
                ..Default::default()
            },
        }
    }
}

// Similarly size is a Size<Val> but we have a Vec2, so we conver to the right type
impl From<Vec2> for Size<Val> {
    impl from(input: Vec2) -> Size<Val> {
        Size {
            width: Val::Px(input.x),
            height: Val::Px(input.y),
        }
    }
}
```

Putting this all together we get (pseudocode) something like this:

``` rust
let my_ui_element = SimpleNodeBundle {
    position: SimplePosition::TopRight,
    size: Vec2 { x: 25.0, y: 100.0 },
    color: Color::RED,
}

some_bevy_ui_method(my_ui_element.into());
```

Skeptical readers might be thinking "Wow that is awful.
This is so much code just to convert one stuct to another slightly simpler struct".
You're right that it's a lot of code, but I promise in practice this is a game changer.
Instead of remembering how to express your ideas to your library of choice every single time, you can express a higher level concept and `.into()` your framework's lower-level structure.
Being able to succinctly express yourself while still getting the flexibility of a strong expressive type system is a killer feature of Rust and the use of these traits.

## Conclusion

Many of Rust's "pros" are also "cons".
Memory safety results in frustratingly negotiating with the compiler.
A strong type system with compile-time complexity results in slow (but improving) compile times.

For the purposes of this post it is Rust's bias toward being explicit.
Unlike other languages which automagically apply crazy changes to your code, Rust rarely *assumes* you want magic sprinkled everywhere.
You can opt-in to that magic and all of the compile-time and runtime penalties that come with it, but it won't be secretly given to you for you to
opt-out of.

Rust can sprinkle magic on your code, but you have to explicitly call `.magic()` -- or in our case `.into()` and `.default()`.
This is a nice middle ground between tedious code and black magic.
When a language has *too much* magic it can result in wild performance implications from seemingly small code changes.
While tedious code is just a pain to write, even if it is transparent about it's runtime performance.
Here Rust is able to be transparent, you can audit every use of `.into()` and assess the runtime penalty, while still feeling magical.
