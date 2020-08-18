mod mouse_position_plugin;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::pass::ClearColor,
};
use mouse_position_plugin::{MousePos, MousePositionPlugin};

const BULLET_SPEED: f32 = 500.0;
const PLAYER_SPEED: f32 = 400.0;

struct Player {
    speed: f32,
}

struct Bullet {
    velocity: Vec3,
}

fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: Vec3) -> Vec2 {
    let point = vec2(point.x(), point.y());
    let x = point.x() + screen_width / 2.0;
    let y = point.y() + screen_height / 2.0;
    vec2(x, y)
}

fn screen_to_world_coords(screen_width: f32, screen_height: f32, point: Vec2) -> Vec3 {
    let x = point.x() - screen_width / 2.0;
    let y = point.y() - screen_height / 2.0;
    vec3(x, y, 0.0)
}

fn look_at(from: Vec2, target: Vec2) -> Quat {
    let dir = target - from;
    // for some reason the sprite is rotated 90 degrees
    let angle = dir.y().atan2(dir.x()) - std::f32::consts::FRAC_PI_2;
    Quat::from_axis_angle(vec3(0.0, 0.0, 1.0), angle)
}

fn look_at_world(from: Vec3, target: Vec3) -> Quat {
    let dir = target - from;
    let angle = dir.y().atan2(dir.x());
    Quat::from_axis_angle(vec3(0.0, 0.0, 1.0), angle)
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    window_desc: Res<WindowDescriptor>,
    mouse_pos: Res<MousePos>,
    mut query: Query<(&Player, &mut Translation, &mut Rotation)>,
) {
    for (player, mut translation, mut rotation) in &mut query.iter() {
        let mut direction = Vec3::zero();
        if keyboard_input.pressed(KeyCode::W) {
            *direction.y_mut() += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            *direction.y_mut() -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            *direction.x_mut() -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            *direction.x_mut() += 1.0;
        }

        if direction.length() != 0.0 {
            direction = direction.normalize();
        }

        translation.0 += time.delta_seconds * direction * player.speed;

        let from = world_to_screen_coords(
            window_desc.width as f32,
            window_desc.height as f32,
            translation.0,
        );
        rotation.0 = look_at(from, mouse_pos.pos);
    }
}

fn fire_shot_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<Input<MouseButton>>,
    window_desc: Res<WindowDescriptor>,
    mouse_pos: Res<MousePos>,
    mut query: Query<(&Player, &Translation, &Rotation)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let shot_handle = asset_server.load("assets/shot.png").unwrap();
        for (_player, translation, _rotation) in &mut query.iter() {
            let world_mouse_pos = screen_to_world_coords(
                window_desc.width as f32,
                window_desc.height as f32,
                mouse_pos.pos,
            );
            let direction = (world_mouse_pos - translation.0).normalize();

            commands
                .spawn(Camera2dComponents::default())
                .spawn(SpriteComponents {
                    material: materials.add(shot_handle.into()),
                    translation: *translation,
                    rotation: Rotation(look_at_world(translation.0, world_mouse_pos)),
                    ..Default::default()
                })
                .with(Bullet {
                    velocity: vec3(
                        BULLET_SPEED * direction.x(),
                        BULLET_SPEED * direction.y(),
                        0.0,
                    ),
                });
        }
    }
}

fn update_bullet_position_system(
    mut _commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Bullet, &mut Translation)>,
) {
    for (_bullet_entity, bullet, mut translation) in &mut query.iter() {
        translation.0 += time.delta_seconds * bullet.velocity;
    }
}

fn wrap_position_system(window_desc: Res<WindowDescriptor>, mut query: Query<(&mut Translation,)>) {
    let sx = window_desc.width as f32;
    let sy = window_desc.height as f32;

    let screen_x_bounds = sx / 2.0;
    let screen_y_bounds = sy / 2.0;

    for translation in &mut query.iter() {
        let mut pos = translation.0;

        if pos.x() > screen_x_bounds {
            *pos.x_mut() -= sx;
        } else if pos.x() < -screen_x_bounds {
            *pos.x_mut() += sx;
        };
        if pos.y() > screen_y_bounds {
            *pos.y_mut() -= sy;
        } else if pos.y() < -screen_y_bounds {
            *pos.y_mut() += sy;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_handle = asset_server.load("assets/player.png").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(player_handle.into()),
            ..Default::default()
        })
        .with(Player {
            speed: PLAYER_SPEED,
        });
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "bevy_astroblasto".to_string(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_default_plugins()
        .add_plugin(MousePositionPlugin)
        .add_startup_system(setup.system())
        .add_system(player_movement_system.system())
        .add_system(wrap_position_system.system())
        .add_system(fire_shot_system.system())
        .add_system(update_bullet_position_system.system())
        .run();
}
