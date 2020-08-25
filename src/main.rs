mod mouse_position_plugin;
mod window_resize_plugin;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    math::{vec2, vec3},
    prelude::*,
    render::pass::ClearColor,
};
use mouse_position_plugin::{MousePos, MousePositionPlugin};
use window_resize_plugin::WindowResizePlugin;

const BULLET_SPEED: f32 = 500.0;
const PLAYER_SPEED: f32 = 400.0;

struct Player {
    speed: f32,
}

struct Bullet {
    velocity: Vec3,
}

struct State {
    shots: Vec<Entity>,
}

enum TextTag {
    FPS,
    ShotCounter,
}

struct ShotHandle(Option<Handle<ColorMaterial>>);

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
    mouse_button_input: Res<Input<MouseButton>>,
    window_desc: Res<WindowDescriptor>,
    mouse_pos: Res<MousePos>,
    shot_handle_res: Res<ShotHandle>,
    mut state: ResMut<State>,
    mut query: Query<(&Player, &Translation, &Rotation)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) && shot_handle_res.0.is_some() {
        for (_player, translation, _rotation) in &mut query.iter() {
            let world_mouse_pos = screen_to_world_coords(
                window_desc.width as f32,
                window_desc.height as f32,
                mouse_pos.pos,
            );
            let direction = (world_mouse_pos - translation.0).normalize();

            let shot_entity = commands
                .spawn(Camera2dComponents::default())
                .spawn(SpriteComponents {
                    material: shot_handle_res.0.unwrap(),
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
                })
                .current_entity();

            if let Some(shot) = shot_entity {
                state.shots.push(shot)
            }
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

fn wrap_position_system(windows: Res<Windows>, mut query: Query<(&mut Translation,)>) {
    for window in windows.iter() {
        let sx = window.width as f32;
        let sy = window.height as f32;

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
}

fn text_update_system(
    diagnostics: Res<Diagnostics>,
    state: Res<State>,
    mut query: Query<(&TextTag, &mut Text)>,
) {
    for (tag, mut text) in &mut query.iter() {
        match tag {
            TextTag::FPS => {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        text.value = format!("FPS: {:.2}", average);
                    }
                }
            }
            TextTag::ShotCounter => text.value = format!("Shot count: {}", state.shots.len()),
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut shot_handle_res: ResMut<ShotHandle>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // load assets
    let player_handle = asset_server.load("assets/player.png").unwrap();

    let shot_handle = asset_server.load("assets/shot.png").unwrap();
    let shot_mat = materials.add(shot_handle.into());
    shot_handle_res.0 = Some(shot_mat);

    let font_handle = asset_server.load("assets/DejaVuSerif.ttf").unwrap();
    let none_color_mat = materials.add(Color::NONE.into());

    commands
        // 2D camera
        .spawn(Camera2dComponents::default())
        // Player
        .spawn(SpriteComponents {
            material: materials.add(player_handle.into()),
            ..Default::default()
        })
        .with(Player {
            speed: PLAYER_SPEED,
        })
        // UI
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            material: none_color_mat,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeComponents {
                    material: none_color_mat,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextComponents {
                            text: Text {
                                value: "FPS:".to_string(),
                                font: font_handle,
                                style: TextStyle {
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                            ..Default::default()
                        })
                        .with(TextTag::FPS);
                })
                .spawn(NodeComponents {
                    material: none_color_mat,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextComponents {
                            text: Text {
                                value: "Shots count:".to_string(),
                                font: font_handle,
                                style: TextStyle {
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                            ..Default::default()
                        })
                        .with(TextTag::ShotCounter);
                });
        });
}

fn main() {
    App::build()
        // resources
        .add_resource(WindowDescriptor {
            title: "bevy_astroblasto".to_string(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(ShotHandle(None))
        .add_resource(State { shots: vec![] })
        // plugins
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MousePositionPlugin)
        // startup
        .add_startup_system(setup.system())
        // systems
        .add_system(player_movement_system.system())
        .add_system(wrap_position_system.system())
        .add_system(fire_shot_system.system())
        .add_system(update_bullet_position_system.system())
        .add_system(text_update_system.system())
        .run();
}
