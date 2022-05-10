use bevy::prelude::*;

const GRID_SIZE: i32 = 100;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width:1024f32,
            height: 720f32,
            title: String::from("Game of Life got rusty"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run()
}
