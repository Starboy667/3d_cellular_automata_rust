use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    render::{camera::PhysicalCameraParameters, view::NoFrustumCulling},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};
use render::{CustomMaterialPlugin, InstanceData, InstanceMaterialData};
use sim::SimsPlugin;
use sim_ui::SimUIPlugin;

mod color;
mod logic;
mod render;
mod rotating_camera;
mod rule;
mod sim;
mod sim_ui;
mod utils;

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Cellular Automata".to_string(),
                    ..default()
                }),
                ..default()
            }),
            CustomMaterialPlugin,
        ))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(SimsPlugin)
        .add_plugins(SimUIPlugin)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        SpatialBundle::INHERITED_IDENTITY,
        InstanceMaterialData(
            (1..=10)
                .flat_map(|x| (1..=100).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
                .map(|(x, y)| InstanceData {
                    position: Vec3::new(x * 10.0 - 5.0, y * 10.0 - 5.0, 0.0),
                    scale: 1.0,
                    color: LinearRgba::from(Color::hsla(x * 360., y, 0.5, 1.0)).to_f32_array(),
                    emissive: 0.0,
                })
                .collect(),
        ),
        // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
        // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
        // instanced cubes will be culled.
        // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
        // instancing, and that is not taken into account with the built-in frustum culling.
        // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
        // component to avoid incorrect culling.
        NoFrustumCulling,
    ));

    // camera
    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                transform: Transform::from_xyz(200.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            BloomSettings::NATURAL,
        ))
        .insert(PanOrbitCamera::default());
    commands.spawn(PerfUiBundle::default());
}
