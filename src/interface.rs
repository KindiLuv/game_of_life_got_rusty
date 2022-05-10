use bevy::prelude::*;

const BUTTON: Color = color::rgb(0,0,0);
const HOVERED_BUTOON : Color = Color::rgb(0,0,0.4);
const CLICKED_BUTTON : Color = Color::rgb(0,0.4,0);


// BUTTON INIT
pub struct GameExitEvent;

pub struct StartSimEvent;

pub struct StopSimEvent;

#[derive(component)]
struct ClassicButton(ButtonType);

#[derive(component)]
enum ButtonType {
    Start,
    Stop,
    Exit
}


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{

    fn build(&self, app: &mut App) {
        app
        .add_event::<GameExitEvent>()
        .add_event::<StartSimEvent>()
        .add_event::<StopSimEvent>()
        .add_startup_system(setup);
    }
}

/**
 * Instantiate the camera
 */
fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn_bundle(UiCameraBundle::default());
}

fn build_button(asset_server: &Res<AssetServer>) -> ButtonBundle {
    Buttonbundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::new(50.0))
        }
    }
}