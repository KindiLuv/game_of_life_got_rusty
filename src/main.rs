use bevy::prelude::*;
use interface::MainMenuPlugin;

const GRID_SIZE: i32 = 100;

mod interface;
mod input;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width:1920f32,
            height: 1080f32,
            title: String::from("Game of Life got rusty"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .run()
}
