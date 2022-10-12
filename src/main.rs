use bevy::{prelude::*, window::PresentMode};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_linear(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Test!".to_string(),
            width: 800.0,
            height: 600.0,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    // add or modify things
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}
