use bevy::prelude::*;
use bevy::core::FixedTimestep;
use crate::GRID_SIZE;
use crate::SPRITE_SIZE;

const CAM_SPEED: f32 = 15.0;
const CAM_ZOOM: f32 = 1.0;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.033))
                    .with_system(camera_move)
                    .with_system(camera_zoom)
                    .with_system(camera_center)
            );
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
struct Movement {
    plane_speed : Vec3,
    zoom_speed : f32,
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Movement {
            plane_speed: Vec3::new(0.0, 0.0, 0.0),
            zoom_speed: 0.0,
        });
}

fn camera_center(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>
)
{
    if keyboard_input.pressed(KeyCode::T) {
        let grid_size = GRID_SIZE as f32;
        let center = Vec3::new(grid_size/2.0 * SPRITE_SIZE, grid_size/2.0 * SPRITE_SIZE, 0.0);
    let mut transform = camera
        .iter_mut()
        .next()
        .expect("No transform on main camera?!");
    transform.translation = center;
    }
}

fn camera_move(
    mut camera: Query<(&mut Transform, &mut Movement), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let mut move_direction = Vec3::new(0.0, 0.0, 0.0);
    if keyboard_input.pressed(KeyCode::Z) {
        move_direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        move_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_direction.x += 1.0;
    }
    let move_direction = move_direction.normalize_or_zero();
    let (mut transform, mut movement) = camera
        .iter_mut()
        .next()
        .expect("No transform on main camera?!");
    movement.plane_speed = (movement.plane_speed  + move_direction)
        .clamp(
            Vec3::new(-CAM_SPEED, -CAM_SPEED, -CAM_SPEED),
            Vec3::new(CAM_SPEED, CAM_SPEED, CAM_SPEED)
        );

    if keyboard_input.pressed(KeyCode::Space) {
        movement.plane_speed = Vec3::new(0.0, 0.0, 0.0);
    }

    transform.translation += movement.plane_speed;
}

fn camera_zoom(
    mut camera: Query<(&mut Movement, &mut OrthographicProjection), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let mut zoom_direction = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        zoom_direction = 0.01;
    }
    if keyboard_input.pressed(KeyCode::E) {
        zoom_direction = -0.01;
    }

    let (mut movement, mut orto_proj) = camera
        .iter_mut()
        .next()
        .unwrap();

    if keyboard_input.pressed(KeyCode::Space) {
        movement.zoom_speed = 0.0;
    }
    
    movement.zoom_speed = (movement.zoom_speed + zoom_direction).clamp(-CAM_ZOOM, CAM_ZOOM);
    orto_proj.scale = (orto_proj.scale + movement.zoom_speed).clamp(1.0, 6.0);

    if (orto_proj.scale - 1.0).abs() < 0.00001 || (orto_proj.scale - 6.0).abs() < 0.00001 {
        movement.zoom_speed = 0.0;
    }
}
