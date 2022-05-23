use bevy::prelude::*;

const BUTTON: Color = Color::rgb(0.0,0.0,0.0);
const HOVERED_BUTOON : Color = Color::rgb(0.0,0.0,0.4);
const CLICKED_BUTTON : Color = Color::rgb(0.0,0.4,0.0);


// BUTTON INIT
pub struct GameExitEvent;

pub struct StartSimEvent;

pub struct StopSimEvent;

#[derive(Component)]
struct ClassicButton(ButtonType);

#[derive(PartialEq, Copy, Clone)]
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

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                            border: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        color: Color::rgb(0.1, 0.1, 0.1).into(),
                        ..Default::default()
                    })
                        .with_children( |parent| {
                            parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    align_items: AlignItems::FlexEnd,
                                    ..Default::default()
                                },
                                color: Color::rgb(0.2,0.2,0.2).into(),
                                ..Default::default()
                            })
                            .with_children( |parent| {
                                parent
                                    .spawn_bundle(build_button(&asset_server))
                                    .with_children( |parent| {
                                        parent.spawn_bundle(build_text("PLAY", &asset_server));
                                    })
                                    .insert(ClassicButton(ButtonType::Start));

                                parent
                                    .spawn_bundle(build_button(&asset_server))
                                    .with_children( |parent| {
                                        parent.spawn_bundle(build_text("STOP", &asset_server));
                                    })
                                    .insert(ClassicButton(ButtonType::Stop));

                                parent
                                    .spawn_bundle(build_button(&asset_server))
                                    .with_children( |parent| {
                                        parent.spawn_bundle(build_text("EXIT", &asset_server));
                                    })
                                    .insert(ClassicButton(ButtonType::Exit));
                            });
                        });
            });
}

fn build_button(asset_server: &Res<AssetServer>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(50.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: BUTTON.into(),
        image: UiImage(asset_server.load("pathoffile")),
        ..Default::default()
    }
}

fn build_text(value: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            value,
            TextStyle {
                font: asset_server.load("fonts/Symtext.ttf"),
                font_size: 30.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default()
        ),
        ..Default::default()
    }
}