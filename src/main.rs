mod mouse_position_plugin;

use bevy::{prelude::*, render::pass::ClearColor};
use mouse_position_plugin::{MousePos, MousePositionPlugin};

struct Player {
    speed: f32,
}

fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: Vec3) -> Vec2 {
    let point = Vec2::new(point.x(), point.y());
    let x = point.x() + screen_width / 2.0;
    let y = point.y() + screen_height / 2.0;
    Vec2::new(x, y)
}

fn look_at(from: Vec2, target: Vec2) -> Quat {
    let dir = target - from;
    let angle = dir.y().atan2(dir.x()) - std::f32::consts::FRAC_PI_2;
    Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle)
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
        .with(Player { speed: 500.0 });
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
        .run();
}
