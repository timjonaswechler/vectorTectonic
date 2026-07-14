//! PROTOTYPE ONLY: scripted visual bootstrap, not production tectonics.

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    asset::RenderAssetUsages, input::mouse::AccumulatedMouseMotion, mesh::Indices, prelude::*,
    render::render_resource::PrimitiveTopology,
};

const GLOBE_RADIUS: f32 = 2.0;
const END_MYR: f32 = 110.0;
const BREAKUP_MYR: f32 = 35.0;
const MYR_PER_SECOND: f32 = 4.0;

#[derive(Resource)]
struct Playback {
    myr: f32,
    paused: bool,
}

#[derive(Component)]
struct Hud;

#[derive(Component, Clone, Copy)]
struct PlateSurface {
    side: Side,
}

#[derive(Component)]
struct OceanTile {
    activation: f32,
    side: Side,
    resolved: bool,
    material: Handle<StandardMaterial>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PROTOTYP — Bootstrap zur Plattenabdeckung".into(),
                resolution: (1280, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Playback {
            myr: 0.0,
            paused: false,
        })
        .insert_resource(ClearColor(Color::srgb(0.015, 0.02, 0.035)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                playback_controls,
                orbit_camera,
                animate_plates,
                update_ocean_tiles,
                draw_features,
                update_hud,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 6.8).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 7_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, -0.6, 0.0)),
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 350.0,
        affects_lightmapped_meshes: true,
    });

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(GLOBE_RADIUS).mesh().uv(72, 36))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.12, 0.16),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    spawn_ocean_tiles(&mut commands, &mut meshes, &mut materials);
    spawn_continent(&mut commands, &mut meshes, &mut materials);
    spawn_hud(&mut commands);
}

fn spawn_ocean_tiles(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    for lat_band in 0..6 {
        let lat0 = -89.5 + lat_band as f32 * (179.0 / 6.0);
        let lat1 = -89.5 + (lat_band + 1) as f32 * (179.0 / 6.0);
        for lon_band in 0..12 {
            let lon0 = -180.0 + lon_band as f32 * 30.0;
            let lon1 = lon0 + 30.0;
            let side = if lon_band < 6 {
                Side::Left
            } else {
                Side::Right
            };
            // Resolution grows visibly away from the new ridge instead of scanning the globe.
            let activation = ((lon0 + lon1) * 0.5).abs() / 165.0;
            let material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.12, 0.17, 0.21),
                unlit: true,
                cull_mode: None,
                ..default()
            });
            commands.spawn((
                Mesh3d(meshes.add(spherical_quad(lat0, lat1, lon0, lon1, GLOBE_RADIUS + 0.006))),
                MeshMaterial3d(material.clone()),
                OceanTile {
                    activation,
                    side,
                    resolved: false,
                    material,
                },
            ));
        }
    }
}

fn spawn_continent(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let left = [
        (-42.0, -18.0),
        (-32.0, -48.0),
        (-8.0, -62.0),
        (18.0, -50.0),
        (34.0, -22.0),
        (24.0, 0.0),
        (-12.0, 0.0),
    ];
    let right = [
        (-12.0, 0.0),
        (24.0, 0.0),
        (38.0, 20.0),
        (18.0, 55.0),
        (-6.0, 66.0),
        (-32.0, 43.0),
        (-42.0, 16.0),
    ];
    let colors = [Color::srgb(0.78, 0.45, 0.16), Color::srgb(0.88, 0.58, 0.18)];
    for (side, polygon, color) in [
        (Side::Left, left.as_slice(), colors[0]),
        (Side::Right, right.as_slice(), colors[1]),
    ] {
        commands.spawn((
            Mesh3d(meshes.add(spherical_polygon(polygon, GLOBE_RADIUS + 0.035))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                cull_mode: None,
                ..default()
            })),
            PlateSurface { side },
        ));
    }

    let cratons = [
        (-23.0, -35.0, Side::Left),
        (-7.0, -45.0, Side::Left),
        (12.0, -35.0, Side::Left),
        (22.0, -18.0, Side::Left),
        (-18.0, -12.0, Side::Left),
        (-25.0, 18.0, Side::Right),
        (-8.0, 26.0, Side::Right),
        (12.0, 20.0, Side::Right),
        (22.0, 40.0, Side::Right),
        (-12.0, 48.0, Side::Right),
    ];
    for (lat, lon, side) in cratons {
        let ring = small_ring(lat, lon, 5.5);
        commands.spawn((
            Mesh3d(meshes.add(spherical_polygon(&ring, GLOBE_RADIUS + 0.055))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.96, 0.86, 0.48),
                unlit: true,
                cull_mode: None,
                ..default()
            })),
            PlateSurface { side },
        ));
    }
}

fn spawn_hud(commands: &mut Commands) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 19.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: px(18),
            left: px(18),
            padding: UiRect::all(px(14)),
            max_width: px(620),
            ..default()
        },
        BackgroundColor(Color::srgba(0.01, 0.015, 0.025, 0.86)),
        Hud,
    ));
    commands.spawn((
        Text::new("SPACE Pause  |  ←/→ 5 Myr  |  R Neustart  |  Maus ziehen: Globus drehen"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.84, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(16),
            left: px(18),
            ..default()
        },
    ));
}

fn playback_controls(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut playback: ResMut<Playback>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        playback.paused = !playback.paused;
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        playback.myr = 0.0;
        playback.paused = false;
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        playback.myr = (playback.myr - 5.0).max(0.0);
        playback.paused = true;
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        playback.myr = (playback.myr + 5.0).min(END_MYR);
        playback.paused = true;
    }
    if !playback.paused {
        playback.myr = (playback.myr + time.delta_secs() * MYR_PER_SECOND).min(END_MYR);
        if playback.myr >= END_MYR {
            playback.paused = true;
        }
    }
}

fn orbit_camera(
    mut camera: Single<&mut Transform, With<Camera3d>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        return;
    }
    let delta = mouse_motion.delta;
    let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
    let pitch = (pitch + delta.y * 0.004).clamp(-FRAC_PI_2 + 0.05, FRAC_PI_2 - 0.05);
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw + delta.x * 0.005, pitch, 0.0);
    camera.translation = -camera.forward() * 6.8;
}

fn animate_plates(playback: Res<Playback>, mut query: Query<(&PlateSurface, &mut Transform)>) {
    for (plate, mut transform) in &mut query {
        transform.rotation = plate_rotation(playback.myr, plate.side);
    }
}

fn update_ocean_tiles(
    playback: Res<Playback>,
    mut tiles: Query<&mut OceanTile>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let resolved_fraction =
        ((playback.myr - BREAKUP_MYR) / (END_MYR - BREAKUP_MYR)).clamp(0.0, 1.0);
    for mut tile in &mut tiles {
        let should_resolve = resolved_fraction >= tile.activation;
        if tile.resolved == should_resolve {
            continue;
        }
        tile.resolved = should_resolve;
        if let Some(material) = materials.get_mut(&tile.material) {
            material.base_color = if should_resolve {
                match tile.side {
                    Side::Left => Color::srgb(0.08, 0.31, 0.48),
                    Side::Right => Color::srgb(0.09, 0.43, 0.53),
                }
            } else {
                Color::srgb(0.12, 0.17, 0.21)
            };
        }
    }
}

fn draw_features(playback: Res<Playback>, mut gizmos: Gizmos) {
    let seed = plate_rotation(playback.myr.min(BREAKUP_MYR), Side::Left);
    if playback.myr < BREAKUP_MYR {
        draw_lat_line(
            &mut gizmos,
            seed,
            0.0,
            -12.0,
            25.0,
            Color::srgb(1.0, 0.88, 0.2),
        );
    } else {
        draw_lat_line(
            &mut gizmos,
            seed,
            0.0,
            -14.0,
            27.0,
            Color::srgb(0.1, 0.95, 1.0),
        );
    }
    draw_lon_line(
        &mut gizmos,
        seed,
        -58.0,
        -32.0,
        20.0,
        Color::srgb(1.0, 0.2, 0.12),
    );

    if playback.myr >= BREAKUP_MYR {
        let left = plate_rotation(playback.myr, Side::Left);
        let right = plate_rotation(playback.myr, Side::Right);
        let origin = lat_lon(3.0, 0.0, GLOBE_RADIUS + 0.18);
        gizmos.arrow(seed * origin, left * origin, Color::srgb(0.3, 0.7, 1.0));
        gizmos.arrow(seed * origin, right * origin, Color::srgb(0.3, 1.0, 0.8));
    }
}

fn update_hud(playback: Res<Playback>, mut hud: Single<&mut Text, With<Hud>>) {
    let (stage, explanation) = if playback.myr < 5.0 {
        (
            "SEED + SUBDUKTIONSQUALIFIKATION",
            "Ein Superkontinent driftet gegen unbewegten alten Ozean.",
        )
    } else if playback.myr < 17.5 {
        (
            "RIFT — BEGINNEND",
            "Gelbe Riftachse folgt nichtkratonischer Schwäche; Kratone bleiben ganz.",
        )
    } else if playback.myr < BREAKUP_MYR {
        (
            "RIFT — LOKALISIERT",
            "Die Seed-Platte bleibt bis zum vollständigen Break-up eine starre Einheit.",
        )
    } else if playback.myr < END_MYR {
        (
            "BREAK-UP + HINTERGRUNDAUFLÖSUNG",
            "Zwei Platten driften auseinander; Rücken (cyan) erzeugt bekannte Ozeankruste, Graben (rot) verbraucht alten Hintergrund.",
        )
    } else {
        (
            "VOLLSTÄNDIGE EXPLIZITE PLATTENABDECKUNG",
            "Hintergrund = 0 %. Die Initialisierung endet hier unumkehrbar.",
        )
    };
    let unresolved = if playback.myr <= BREAKUP_MYR {
        75.0
    } else {
        75.0 * (1.0 - (playback.myr - BREAKUP_MYR) / (END_MYR - BREAKUP_MYR))
    };
    let plates = if playback.myr < BREAKUP_MYR { 1 } else { 2 };
    hud.0 = format!(
        "PROTOTYP — Bootstrap zur Plattenabdeckung\n\n{stage}\n{explanation}\n\nZeit: {:>5.1} Myr    explizite Platten: {plates}\nKontinent: 25.0 %    bekannte Ozeankruste: {:>4.1} %\nAlter Hintergrund: {:>4.1} %\n\nKratone: 10/10 ganz    Flächenbudget: 100 %    {}",
        playback.myr,
        75.0 - unresolved,
        unresolved.max(0.0),
        if playback.paused { "PAUSE" } else { "LÄUFT" }
    );
}

fn plate_rotation(myr: f32, side: Side) -> Quat {
    let seed_myr = myr.min(BREAKUP_MYR);
    let seed_axis = Vec3::new(0.18, 1.0, 0.28).normalize();
    let seed = Quat::from_axis_angle(seed_axis, seed_myr.to_radians() * 0.24);
    if myr <= BREAKUP_MYR {
        return seed;
    }
    let drift = ((myr - BREAKUP_MYR) * 0.34).to_radians();
    let axis = match side {
        Side::Left => Vec3::new(0.2, 1.0, -0.35).normalize(),
        Side::Right => Vec3::new(-0.2, 1.0, 0.35).normalize(),
    };
    let sign = if side == Side::Left { -1.0 } else { 1.0 };
    Quat::from_axis_angle(axis, sign * drift) * seed
}

fn spherical_quad(lat0: f32, lat1: f32, lon0: f32, lon1: f32, radius: f32) -> Mesh {
    let points = [
        lat_lon(lat0, lon0, radius),
        lat_lon(lat0, lon1, radius),
        lat_lon(lat1, lon1, radius),
        lat_lon(lat1, lon0, radius),
    ];
    mesh_from(points.to_vec(), vec![0, 1, 2, 0, 2, 3])
}

fn spherical_polygon(points: &[(f32, f32)], radius: f32) -> Mesh {
    let center_lat = points.iter().map(|p| p.0).sum::<f32>() / points.len() as f32;
    let center_lon = points.iter().map(|p| p.1).sum::<f32>() / points.len() as f32;
    let mut vertices = vec![lat_lon(center_lat, center_lon, radius)];
    vertices.extend(points.iter().map(|&(lat, lon)| lat_lon(lat, lon, radius)));
    let mut indices = Vec::new();
    for i in 0..points.len() {
        indices.extend([0, (i + 1) as u32, ((i + 1) % points.len() + 1) as u32]);
    }
    mesh_from(vertices, indices)
}

fn mesh_from(vertices: Vec<Vec3>, indices: Vec<u32>) -> Mesh {
    let positions: Vec<[f32; 3]> = vertices.iter().map(|v| v.to_array()).collect();
    let normals: Vec<[f32; 3]> = vertices.iter().map(|v| v.normalize().to_array()).collect();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}

fn small_ring(lat: f32, lon: f32, radius_deg: f32) -> Vec<(f32, f32)> {
    (0..7)
        .map(|i| {
            let angle = i as f32 / 7.0 * 2.0 * PI;
            (
                lat + angle.sin() * radius_deg,
                lon + angle.cos() * radius_deg,
            )
        })
        .collect()
}

fn lat_lon(lat_deg: f32, lon_deg: f32, radius: f32) -> Vec3 {
    let lat = lat_deg.to_radians();
    let lon = lon_deg.to_radians();
    Vec3::new(lat.cos() * lon.cos(), lat.sin(), lat.cos() * lon.sin()) * radius
}

fn draw_lat_line(
    gizmos: &mut Gizmos,
    rotation: Quat,
    lon: f32,
    lat0: f32,
    lat1: f32,
    color: Color,
) {
    gizmos.linestrip(
        (0..24).map(|i| {
            let lat = lat0 + (lat1 - lat0) * i as f32 / 23.0;
            rotation * lat_lon(lat, lon, GLOBE_RADIUS + 0.09)
        }),
        color,
    );
}

fn draw_lon_line(
    gizmos: &mut Gizmos,
    rotation: Quat,
    lon0: f32,
    lon1: f32,
    lat: f32,
    color: Color,
) {
    gizmos.linestrip(
        (0..24).map(|i| {
            let lon = lon0 + (lon1 - lon0) * i as f32 / 23.0;
            rotation * lat_lon(lat, lon, GLOBE_RADIUS + 0.09)
        }),
        color,
    );
}
