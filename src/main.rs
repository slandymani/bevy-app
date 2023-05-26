use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
struct Planet {}

#[derive(Component, Default)]
struct Satellite {}

#[derive(Component)]
struct Moon {}

#[derive(Component)]
struct Sun {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_startup_system(setup)
        //.add_system(resize)
        .add_system(move_planet)
        .add_system(move_moon)
        .add_system(move_sun)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut images: ResMut<Assets<Image>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let sphere_handle = asset_server.load("models/sun/sun.png");
    let earth_handle = asset_server.load("models/earth/earth.png");
    let moon_handle = asset_server.load("models/moon/moon.png");
    //let stars_handle : Handle<Image> = asset_server.load("models/stars/stars.png");
    //let background_texture_atlas = TextureAtlas::from_grid(stars_handle, Vec2::new(1000.0, 1000.0), 1, 1, None, None);
    // let material = materials.add(StandardMaterial {
    //     base_color_texture: Some(stars_handle),
    //     ..Default::default()
    // });
    //
    // // Create a quad mesh to render the background
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Quad::new(Vec2::splat(1000.0)).try_into().unwrap()),
    //     material,
    //     ..Default::default()
    // });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        // camera: Camera {
        //     // render before the "main pass" camera
        //     order: -1,
        //     target: RenderTarget::Image(earth_handle.clone()),
        //     ..default()
        // },
        ..default()
    });

    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(
                    Mesh::try_from(shape::Icosphere {
                        radius: 0.2,
                        subdivisions: 20,
                    })
                        .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(earth_handle),
                    ..default()
                }),
                transform: Transform::from_xyz(2., 0., 0.),
                ..default()
            },
            // Add the Name component, and the animation player
            Planet {},
        ))
        .with_children(|p| {
            p.spawn((
                PbrBundle {
                    mesh: meshes.add(
                        Mesh::try_from(shape::Icosphere {
                            radius: 0.1,
                            subdivisions: 20,
                        })
                            .unwrap(),
                    ),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(moon_handle),
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.4, 0., 0.),
                    ..default()
                },
                Moon {},
            ));
        });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: 0.2,
                    subdivisions: 20,
                })
                    .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(sphere_handle),
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        // Add the Name component, and the animation player
        Sun {},
    ));
}

fn _resize(
    keys: Res<Input<KeyCode>>,
    mut satellite: Query<&mut Transform, With<Satellite>>,
    mut planet: Query<&mut Transform, (With<Planet>, Without<Satellite>)>,
) {
    let mut satellite_entity = satellite.single_mut();
    let mut planet_entity = planet.single_mut();

    if keys.just_pressed(KeyCode::A) {
        satellite_entity.scale += Vec3::splat(0.5);
    }

    if keys.just_pressed(KeyCode::S) {
        satellite_entity.scale -= Vec3::splat(0.5);
    }

    if keys.just_pressed(KeyCode::Q) {
        planet_entity.scale += Vec3::splat(0.5);
        satellite_entity.scale -= Vec3::splat(0.5);
    }

    if keys.just_pressed(KeyCode::W) {
        planet_entity.scale -= Vec3::splat(0.5);
        satellite_entity.scale += Vec3::splat(0.5);
    }
}

fn move_planet(mut query: Query<&mut Transform, With<Planet>>, timer: Res<Time>) {
    let mut planet = query.single_mut();
    //moon.rotate_z(2. * PI / 360. * timer.delta_seconds() * 100.);

    planet.rotate_around(
        Vec3::ZERO,
        Quat::from_rotation_y(2. * PI / 900. * timer.delta_seconds() * 100.),
    );
    //let direction = moon.local_x() * Vec3::ONE;
    //moon.translation += direction * 1. * timer.delta_seconds();
}

fn move_moon(mut query: Query<&mut Transform, With<Moon>>, timer: Res<Time>) {
    let mut moon = query.single_mut();
    //moon.rotate_z(2. * PI / 360. * timer.delta_seconds() * 100.);
    //Quat::from
    moon.rotate_around(
        Vec3::ZERO,
        Quat::from_rotation_z(2. * PI / 240. * timer.delta_seconds() * 100.),
    );
    //let direction = moon.local_x() * Vec3::ONE;
    //moon.translation += direction * 1. * timer.delta_seconds();
}

fn move_sun(mut query: Query<&mut Transform, With<Sun>>, timer: Res<Time>) {
    let mut moon = query.single_mut();
    //moon.rotate_z(2. * PI / 360. * timer.delta_seconds() * 100.);

    moon.rotate_around(
        Vec3::ZERO,
        Quat::from_rotation_z(2. * PI / 1800. * timer.delta_seconds() * 100.),
    );
    //let direction = moon.local_x() * Vec3::ONE;
    //moon.translation += direction * 1. * timer.delta_seconds();
}
