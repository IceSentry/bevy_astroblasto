use bevy::prelude::*;

struct Player {}
struct Speed(f32);

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
            translation: Translation(Vec3::new(0.0, 0.0, 1.0)),
            rotation: Rotation(Quat::identity()),
            ..Default::default()
        })
        .with(Player {})
        .with(Speed(200.0));
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &Speed, &mut Translation, &mut Rotation)>,
) {
    for (_player, speed, mut translation, mut rotation) in &mut query.iter() {
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

        translation.0 += time.delta_seconds * direction * speed.0;
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(player_movement_system.system())
        .run();
}
