use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pixel/rock.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Right,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pixel/paper.png"),
            transform: Transform::from_xyz(-100., 100., 0.),
            ..default()
        },
        Direction::Left,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pixel/scissors.png"),
            transform: Transform::from_xyz(-50., 50., 0.),
            ..default()
        },
        Direction::Left,
    ));
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Right => transform.translation.x += 30. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 30. * time.delta_seconds(),
        }

        if transform.translation.x > 200. {
            *logo = Direction::Left;
        } else if transform.translation.x < -200. {
            *logo = Direction::Right;
        }
    }
}
