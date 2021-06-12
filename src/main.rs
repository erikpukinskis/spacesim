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
        .add_startup_system(setup.system())
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
