use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Untitled Space Sim!".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.01, 0.02)))
        .init_resource::<Ship>()
        .add_startup_system(setup.system())
        .add_system(handle_key_press.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // Plane
        .spawn()
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 4.0,
                subdivisions: 8,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("fff8c1").unwrap(),
                unlit: true,
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
            ..Default::default()
        });
    commands
        // Camera
        .spawn()
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        });
    commands
        // Light
        .spawn()
        .insert_bundle(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

#[derive(Default)]
struct Ship {
    entity: Option<Entity>,
    i: usize,
    j: usize,
}

fn handle_key_press(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut ship: ResMut<Ship>,
    mut transforms: Query<&mut Transform>,
) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        info!("pitch down");
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        info!("pitch up");
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        info!("roll right");
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        info!("roll left");
    }

    // let rotation = -std::f32::consts::FRAC_PI_2;

    // *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
    //     translation: Vec3::new(
    //         game.player.i as f32,
    //         game.board[game.player.j][game.player.i].height,
    //         game.player.j as f32,
    //     ),
    //     rotation: Quat::from_rotation_y(rotation),
    //     ..Default::default()
    // };
}
