# TypeID
Trying to figure out how the heck [bevy][bevy-src] and [futures][futures-src]
do their cool thing with types.

[bevy-src]: https://github.com/bevyengine
[futures-src]: https://github.com/rust-lang/futures-rs

## What do I mean?

From [Bevy's Getting Started Ecs section][bevy-getstarted-ecs] it shows:
```rust
fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(_person: &Person, name: &Name) {
    println!("hello {}!", name.0);
}

fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(greet_people.system())
        .run();
}
```

See how you can give `.spawn()` and kind of type (wrapped in a tuple) and it
calls your `greet_people()` with the correct structures? That's super cool, but
I can't for the life of me figure out how they do it.

[bevy-getstarted-ecs]: https://bevyengine.org/learn/book/getting-started/ecs/

## Notes

We see [`Resoruces::thread_local_data`][1] using a trait called [`ResourceStorage`][2] that requires
an implementation of [`Downcast`][3].

`Resources::thread_local_data` is a `HashMap<TypeId, Box<dyn ResourceStorage>>`. You insert
resoruces by calling methods that ultimately call [`Resources::insert_resource<T: Resource>(T)`][4].
That's not the correct function signature, but it gives all the requries information.
[`Resource`][5] is a trait without any functions or types of its own, it just requires that `Send`,
`Sync`, and `'static` are there.

[1]: https://github.com/bevyengine/bevy/blob/7628f4a64e6f3eacfc4aad3bb6b3d54309722682/crates/bevy_ecs/src/resource/resources.rs#L84
[2]: https://github.com/bevyengine/bevy/blob/7628f4a64e6f3eacfc4aad3bb6b3d54309722682/crates/bevy_ecs/src/resource/resources.rs#L30
[3]: https://github.com/marcianx/downcast-rs
[4]: https://github.com/bevyengine/bevy/blob/7628f4a64e6f3eacfc4aad3bb6b3d54309722682/crates/bevy_ecs/src/resource/resources.rs#L187
[5]: https://github.com/bevyengine/bevy/blob/7628f4a64e6f3eacfc4aad3bb6b3d54309722682/crates/bevy_ecs/src/resource/resources.rs#L13