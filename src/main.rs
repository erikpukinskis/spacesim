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
        // .add_startup_system(add_camera_for_star.system())
        // .add_startup_system(add_star.system())
        .add_startup_system(setup.system())
        .add_system(add_grinder_parts.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(handle_key_press.system())
        .run();
}

struct GrinderAssets {
    head: Handle<Mesh>,
    body: Handle<Mesh>,
    handle: Handle<Mesh>,
    cover: Handle<Mesh>,
    nut: Handle<Mesh>,
    washer: Handle<Mesh>,
    rod: Handle<Mesh>,
    trunnion: Handle<Mesh>,
    flanges: Handle<Mesh>,
    disc: Handle<Mesh>,
    spindle: Handle<Mesh>,
    bearing_cap: Handle<Mesh>,
    gear_cover: Handle<Mesh>,
    base: Handle<Mesh>,
    fence: Handle<Mesh>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(GrinderAssets {
        head: asset_server.load("stand-grinder.gltf#Mesh0/Primitive0"),
        body: asset_server.load("stand-grinder.gltf#Mesh1/Primitive0"),
        handle: asset_server.load("stand-grinder.gltf#Mesh2/Primitive0"),
        cover: asset_server.load("stand-grinder.gltf#Mesh3/Primitive0"),
        nut: asset_server.load("stand-grinder.gltf#Mesh4/Primitive0"),
        washer: asset_server.load("stand-grinder.gltf#Mesh5/Primitive0"),
        rod: asset_server.load("stand-grinder.gltf#Mesh6/Primitive0"),
        trunnion: asset_server.load("stand-grinder.gltf#Mesh7/Primitive0"),
        flanges: asset_server.load("stand-grinder.gltf#Mesh8/Primitive0"),
        disc: asset_server.load("stand-grinder.gltf#Mesh9/Primitive0"),
        spindle: asset_server.load("stand-grinder.gltf#Mesh10/Primitive0"),
        bearing_cap: asset_server.load("stand-grinder.gltf#Mesh11/Primitive0"),
        gear_cover: asset_server.load("stand-grinder.gltf#Mesh12/Primitive0"),
        base: asset_server.load("stand-grinder.gltf#Mesh13/Primitive0"),
        fence: asset_server.load("stand-grinder.gltf#Mesh14/Primitive0"),
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..Default::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.3, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn add_grinder_parts(
    mut commands: Commands,
    grinder: Res<GrinderAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.9, 0.9, 0.9),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: grinder.head.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: grinder.handle.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.075, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: grinder.body.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.1, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: grinder.cover.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.3, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: grinder.nut.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.4, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: grinder.washer.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.4, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: grinder.rod.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.4, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: grinder.trunnion.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(-0.1, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: grinder.flanges.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(-0.2, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: grinder.disc.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(-0.2, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: grinder.spindle.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(-0.2, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: grinder.bearing_cap.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(-0.2, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: grinder.gear_cover.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(-0.2, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: grinder.base.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(-0.5, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: grinder.fence.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(-0.7, 0.0, 0.0),
        ..Default::default()
    });
}

fn add_star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn().insert_bundle(PbrBundle {
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
}

fn add_camera_for_star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
