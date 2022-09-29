use std::collections::HashMap;

use bevy::{prelude::*, math::{vec2, vec3}, render::view::RenderLayers, core_pipeline::clear_color::ClearColorConfig};
use bevy::window::{WindowId, CreateWindow, WindowPosition};
use bevy::render::camera::RenderTarget;

fn main() {
    println!("F or 1 - choose Foreground Camera as the target");
    println!("S or 2 - choose Scene Camera as the target");
    println!("B or 3 - choose Background Camera as the target");
    println!("A or 4 - change is_active of the target");
    println!("P or 5 - change priority of the target");
    println!("L or 6 - change layer of the target");
    println!("M or 7 - multi window mode");

    App::new()
        .insert_resource(WindowDescriptor {
            width: 1280.0 / 3.0,
            height: 960.0 / 3.0,
            ..Default::default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(input)
        .add_system(multi_window)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let foreground_layer = RenderLayers::layer(1);
    let scene_layer = RenderLayers::layer(2);
    let background_layer = RenderLayers::layer(3);

    let background_bandle = (
        background_layer,
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            background_color: Color::BLUE.into(),
            ..Default::default()
        },
    );

    let foreground_bandle = (
        foreground_layer,
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                ..Default::default()
            },
            background_color: Color::RED.into(),
            ..Default::default()
        },
    );

    let scene_object_bandle = (
        scene_layer,
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(1.0).into()),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_rotation(Quat::from_axis_angle(vec3(1.0, 1.0, 1.0).normalize(), 30.0)),
            ..Default::default()
        },
    );
    
    commands.spawn(foreground_bandle);
    commands.spawn(scene_object_bandle);
    commands.spawn(background_bandle);

    commands.spawn((
        scene_layer,
        SceneCamera,
        ConfigureTarget::SceneCamera,
        Camera3dBundle {
            camera: Camera {
                priority: 2,
                is_active: true,
                ..Default::default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        UiCameraConfig { show_ui: false, },
    ));

    commands.spawn((
        background_layer,
        BackgroundCamera,
        ConfigureTarget::BackgroundCamera,
        Camera2dBundle {
            camera: Camera {
                priority: 1,
                is_active: true,
                ..Default::default()
            },
            camera_2d: Camera2d { clear_color: ClearColorConfig::None },
            ..Default::default()
        },
    ));

    commands.spawn((
        foreground_layer,
        ForegroundCamera,
        ConfigureTarget::ForegroundCamera,
        Camera2dBundle {
            camera: Camera {
                priority: 3,
                is_active: true,
                ..Default::default()
            },
            camera_2d: Camera2d { clear_color: ClearColorConfig::None },
            ..Default::default()
        },
    ));
}

#[derive(Component)]
struct ForegroundCamera;
#[derive(Component)]
struct BackgroundCamera;
#[derive(Component)]
struct SceneCamera;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
enum ConfigureTarget {
    ForegroundCamera,
    BackgroundCamera,
    SceneCamera,
    #[default]
    None,
}

fn input(
    mut commands: Commands,
    mut target: Local<ConfigureTarget>,
    keys: Res<Input<KeyCode>>,
    mut cameras: Query<(&mut Camera, &RenderLayers, &ConfigureTarget)>,
    foreground_camera: Query<Entity, With<ForegroundCamera>>,
    background_camera: Query<Entity, With<BackgroundCamera>>,
    scene_camera: Query<Entity, With<SceneCamera>>,
) {
    if keys.just_pressed(KeyCode::F) || keys.just_pressed(KeyCode::Key1) {
        *target = ConfigureTarget::ForegroundCamera;
    }

    if keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Key2) {
        *target = ConfigureTarget::SceneCamera;
    }

    if keys.just_pressed(KeyCode::B) || keys.just_pressed(KeyCode::Key3) {
        *target = ConfigureTarget::BackgroundCamera;
    }

    let entity = match *target {
        ConfigureTarget::ForegroundCamera => foreground_camera.single(),
        ConfigureTarget::BackgroundCamera => background_camera.single(),
        ConfigureTarget::SceneCamera => scene_camera.single(),
        ConfigureTarget::None => return,
    };

    let (mut camera, layers, _) = cameras.get_mut(entity).unwrap();

    if keys.just_pressed(KeyCode::A) || keys.just_pressed(KeyCode::Key4) {
        camera.is_active = !camera.is_active;
        println!("{:?}: is_active changed to {}", *target, camera.is_active);
    }

    if keys.just_pressed(KeyCode::P) || keys.just_pressed(KeyCode::Key5) {
        camera.priority = (camera.priority + 3) % 9;
        println!("{:?}: priority changed to {}", *target, (&["high", "mid", "low"])[camera.priority as usize / 3]);
    }

    if keys.just_pressed(KeyCode::L) || keys.just_pressed(KeyCode::Key6) {
        let layer = layers.iter().nth(0).unwrap();
        commands.entity(entity).insert(RenderLayers::layer((layer + 1) % 6));
        println!("{:?}: layer changed to {}", *target, layer);
    }

    if keys.get_just_pressed().next().is_some() {
        let mut cameras = cameras.into_iter().collect::<Vec<_>>();
        cameras.sort_by_key(|(camera, _, _)| camera.priority);
        let order = cameras.into_iter()
            .filter(|(camera, _, _)| camera.is_active)
            .map(|(_, layer, target)| format!("{:?}({})", target, layer.iter().nth(0).unwrap()))
            .collect::<Vec<_>>()
            .join(" -> ");
        println!("current order: {}", order);
    }
}

fn multi_window(
    mut is_multi_window: Local<bool>,
    mut ids: Local<HashMap<ConfigureTarget, WindowId>>,
    keys: Res<Input<KeyCode>>,
    mut cameras: Query<(&mut Camera, &ConfigureTarget)>,
    mut create_window_events: EventWriter<CreateWindow>,
    mut windows: ResMut<Windows>,
    desc: Res<WindowDescriptor>,
) {
    if !keys.just_pressed(KeyCode::M) && !keys.just_pressed(KeyCode::Key7) {
        return;
    }

    if *is_multi_window {
        // multi -> single
        for (mut camera, target) in &mut cameras {
            windows.get_mut(ids.remove(target).unwrap()).unwrap().close();
            camera.target = RenderTarget::Window(WindowId::primary());
        }
    } else {
        // single -> multi
        for (i, (mut camera, target)) in cameras.iter_mut().enumerate() {
            let window_id = WindowId::new();
            create_window_events.send(CreateWindow {
                id: window_id,
                descriptor: WindowDescriptor {
                    title: format!("{:?}", target),
                    position: WindowPosition::At(vec2((desc.width + 20.0) * i as f32, 500.0)),
                    ..desc.clone()
                },
            });
            camera.target = RenderTarget::Window(window_id);
            ids.insert(target.clone(), window_id);
        }
    }
    *is_multi_window = !*is_multi_window;
}