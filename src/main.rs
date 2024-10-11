//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_sprite, direct_sprite))
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn direct_sprite(mut query: Query<(&mut Sprite, &mut Transform)>, keys: Res<ButtonInput<KeyCode>>) {
    for (mut sprite, mut transform) in &mut query {
        if keys.pressed(KeyCode::ArrowRight) {
            sprite.flip_x = false;
            transform.translation.x += 10.;
        }
        if keys.pressed(KeyCode::ArrowLeft) {
            sprite.flip_x = true;
            transform.translation.x -= 10.;
        }
        if keys.just_pressed(KeyCode::Space) {
            transform.translation.y += 50.;
        }
        if keys.just_released(KeyCode::Space) {
            transform.translation.y -= 50.;
        }
    }
}

// const X_EXTENT: f32 = 600.0;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture = asset_server.load("gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(24.0), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(6.0)),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
    // let shapes = [
    //     Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
    //     Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
    //     Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
    //     Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
    //     Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
    //     Mesh2dHandle(meshes.add(Triangle2d::new(
    //         Vec2::Y * 50.0,
    //         Vec2::new(-50.0, -50.0),
    //         Vec2::new(50.0, -50.0),
    //     ))),
    // ];
    // let num_shapes = shapes.len();
    //
    // for (i, shape) in shapes.into_iter().enumerate() {
    //     // Distribute colors evenly across the rainbow.
    //     let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
    //
    //     commands.spawn(MaterialMesh2dBundle {
    //         mesh: shape,
    //         material: materials.add(color),
    //         transform: Transform::from_xyz(
    //             // Distribute shapes from -X_EXTENT to +X_EXTENT.
    //             -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
    //             0.0,
    //             0.0,
    //         ),
    //         ..default()
    //     });
    // }
}
//
// use bevy::prelude::*;
//
// #[derive(Component)]
// struct Person;
//
// #[derive(Component)]
// struct Name(String);
//
// fn add_people(mut commands: Commands) {
//     commands.spawn((Person, Name("Leaina Proctor".to_string())));
//     commands.spawn((Person, Name("Austin Rooks".to_string())));
// }
//
// #[derive(Resource)]
// struct GreetTimer(Timer);
//
// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in &query {
//             println!("hello {}!", name.0);
//         }
//     }
// }
//
// fn update_people(mut query: Query<&mut Name, With<Person>>) {
//     for mut name in &mut query {
//         if name.0 == "Austin Rooks" {
//             name.0 = "Sir Austin Rooks".to_string();
//             break;
//         }
//     }
// }
//
// pub struct HelloPlugin;
//
// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
//             .add_systems(Startup, add_people)
//             .add_systems(Update, (update_people, greet_people).chain());
//     }
// }
//
// fn main() {
//     App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
// }
