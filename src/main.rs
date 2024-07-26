use bevy::{prelude::*, render::view::NoFrustumCulling};
use iyes_perf_ui::{entries::PerfUiBundle, prelude::*};

mod logic;
mod render;
mod rotating_camera;
mod sim;

use render::{CustomMaterialPlugin, InstanceData, InstanceMaterialData};
use rotating_camera::{RotatingCamera, RotatingCameraPlugin};
use sim::SimsPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CustomMaterialPlugin))
        .add_plugins(RotatingCameraPlugin)
        .add_plugins(SimsPlugin)
        // we want Bevy to measure these values for us:
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        // .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        // .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        // .add_plugins(PerfUiPlugin)
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
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(RotatingCamera::default());
    // commands.spawn(PerfUiCompleteBundle::default());
    // commands.spawn(PerfUiBundle::default());
}
