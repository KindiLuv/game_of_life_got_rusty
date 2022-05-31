use bevy::prelude::*;
use interface::MainMenuPlugin;
use game::GamePlugin;
use input::InputPlugin;

const GRID_SIZE: i32 = 250;
const SPRITE_SIZE: f32 = 32.0;

mod interface;
mod game;
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
        .add_plugin(GamePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(MainMenuPlugin)
        .run()
}