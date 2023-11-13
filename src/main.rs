use bevy::{prelude::*, sprite, window::PrimaryWindow};
use rand::prelude::*;


pub const ANT_SIZE: f32 = 64.0 * 0.4;
pub const ANT_SPEED: f32 = 600.0;
pub const ENEMY_COUNT: usize = 8;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(PostStartup, (print_names, wagie_ants, neet_ants, spawn_ant, spawn_enemy))
        .add_systems(Update, (ant_movement, confine_ant_movement))
        .run();
}

#[derive(Component)]
pub struct Ant {
    pub name: String
}

#[derive(Component)]
pub struct Enemy {}


#[derive(Component)]
pub struct Employed {
    pub job: Job
}

#[derive(Debug)]
pub enum Job {
    Worker,
    Male,
    Queen,
    Larvae,
}



pub fn spawn_ant(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0).with_scale(Vec3::new(0.4, 0.4, 1.0)),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Ant {
                name : "Eve".to_string()
            },
        )
    );
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_enemy(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMY_COUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0).with_scale(Vec3::new(0.4, 0.4, 1.0)),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {},
        ));
    }
}


pub fn ant_movement (
    keyboard_input: Res<Input<KeyCode>>,
    mut ant_query: Query<&mut Transform, With<Ant>>,
    time: Res<Time>,
) {
    //using if let OK because .get_single_mut() returns either transform or error if doesnt exist
    if let Ok(mut transform) = ant_query.get_single_mut(){
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * ANT_SPEED * time.delta_seconds();
    }
}

pub fn confine_ant_movement(
    mut ant_query: Query<&mut Transform, With<Ant>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut ant_transform) = ant_query.get_single_mut(){
        let window = window_query.get_single().unwrap();

        let half_ant_size: f32 = ANT_SIZE / 2.0;
        let x_min = 0.0 + half_ant_size;
        let x_max = window.width() - half_ant_size;
        let y_min = 0.0 + half_ant_size;
        let y_max = window.height() - half_ant_size;

        let mut translation = ant_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        ant_transform.translation = translation;
    }
}



// fn setup(
//     mut commands: Commands,
// ) {
//     //the ants are a tuple, of name and job
//     commands.spawn(
//         (Ant {name: "Alex".to_string()},
//         Employed {job: Job::Male})
//     );
//     commands.spawn(
//         (Ant {name: "Kate".to_string()},
//         Employed {job: Job::Queen})
//     );
//     commands.spawn(
//         (Ant {name: "Alexis".to_string()},
//         Employed {job: Job::Worker})
//     );
//     commands.spawn(
//         (Ant {name: "Ashley".to_string()},
//         Employed {job: Job::Worker})
//     );
//     commands.spawn(
//         (Ant {name: "Baby".to_string()},
//         Employed {job: Job::Larvae})
//     );
//     commands.spawn(
//         Ant {name: "RETARD".to_string()}
//     );

//     commands.spawn(Camera2dBundle::default());
// }

pub fn print_names(ant_query: Query<&Ant>) {
    for ant in ant_query.iter(){
        println!("Name: {}", ant.name);
        // println!("Job: {}", ant.job)
    }
}

pub fn wagie_ants(ant_query: Query<(&Ant, &Employed)>) {
    for (ant, employed) in ant_query.iter() {
        println!("{} is a wagie: {:?}", ant.name, employed.job);
    }
}

pub fn neet_ants(ant_query: Query<&Ant, Without<Employed>>) {
    for ant in ant_query.iter() {
        println!("{} is a neet", ant.name);
    }
}

