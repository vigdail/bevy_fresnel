use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "5fbd7ab7-50bb-439c-a688-d193177ed629"]
struct FresnelMaterial {}

impl Material for FresnelMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fresnel_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<FresnelMaterial>::default())
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FresnelMaterial>>,
) {
    let sphere = meshes.add(
        shape::Icosphere {
            radius: 0.5,
            subdivisions: 5,
        }
        .into(),
    );
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: sphere,
        material: materials.add(FresnelMaterial {}.into()),
        ..Default::default()
    });

    let plane = meshes.add(shape::Plane { size: 10.0 }.into());
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: plane,
        material: materials.add(FresnelMaterial {}.into()),
        ..Default::default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
