use bevy::prelude::*;

const BUTTON: Color = Color::rgb(1.0,1.0,1.0);
const HOVERED_BUTOON : Color = Color::rgb(254.0,254.0,254.0);
const CLICKED_BUTTON : Color = Color::rgb(0.1,0.0,0.1);


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
        .add_startup_system(setup)
        .add_system(button_events_mapper);
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
                            border: Rect::all(Val::Px(3.0)),
                            ..Default::default()
                        },
                        color: Color::rgb(0.4, 0.0, 0.4).into(),
                        ..Default::default()
                    })
                        .with_children( |parent| {
                            parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    align_items: AlignItems::FlexEnd,
                                    border: Rect::all(Val::Px(3.0)),
                                    ..Default::default()
                                },
                                color: Color::rgb(255.0,255.0,255.0).into(),
                                ..Default::default()
                            })
                            .with_children( |parent| {
                                parent
                                    .spawn_bundle(build_button(&asset_server))
                                    .with_children( |parent| {
                                        parent.spawn_bundle(build_text("Go", &asset_server));
                                    })
                                    .insert(ClassicButton(ButtonType::Start));

                                parent
                                    .spawn_bundle(build_button(&asset_server))
                                    .with_children( |parent| {
                                        parent.spawn_bundle(build_text("II", &asset_server));
                                    })
                                    .insert(ClassicButton(ButtonType::Stop));

                                parent
                                    .spawn_bundle(build_button(&asset_server))
                                    .with_children( |parent| {
                                        parent.spawn_bundle(build_text("Exit", &asset_server));
                                    })
                                    .insert(ClassicButton(ButtonType::Exit));
                            });
                        });
            });
}

fn build_button(asset_server: &Res<AssetServer>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(85.0), Val::Px(85.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: BUTTON.into(),
        image: UiImage(asset_server.load("sprites/button.png")),
        ..Default::default()
    }
}

fn build_text(value: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            value,
            TextStyle {
                font: asset_server.load("fonts/SixThousand.ttf"),
                font_size: 35.0,
                color: Color::rgb(0.1, 0.1, 0.1),
            },
            Default::default()
        ),
        ..Default::default()
    }
}

fn button_events_mapper(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ClassicButton),
        (Changed<Interaction>, With<Button>)>,
    mut start_writer: EventWriter<StartSimEvent>,
    mut stop_writer: EventWriter<StopSimEvent>,
    mut exit_writer: EventWriter<GameExitEvent>,
) {
    for (interaction, mut color, classic_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = CLICKED_BUTTON.into();
                match classic_button.0 {
                    ButtonType::Start => {
                        start_writer.send(StartSimEvent);
                    },
                    ButtonType::Stop => {
                        stop_writer.send(StopSimEvent);
                    },
                    ButtonType::Exit => {
                        exit_writer.send(GameExitEvent);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTOON.into();
            }
            Interaction::None => {
                *color = BUTTON.into();
            }
        }
    }
}