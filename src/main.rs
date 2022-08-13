#![allow(unused)]

use bevy::prelude::*;
use bevy::render::render_resource::Texture;

const WINDOW_TITLE: &str = "titre de la fenetre";
struct WinSize {
    width: f32,
    height: f32
}

const WIN_SIZE: WinSize = WinSize { 
    width: 600.0, 
    height: 800.0 
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0., 0.2)))
        .insert_resource(WindowDescriptor {
            title: WINDOW_TITLE.to_string(),
            width: WIN_SIZE.width,
            height: WIN_SIZE.height,
            ..Default::default()
        })
        //
        .add_startup_system(setup_system)
        //
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_descriptor: Res<WindowDescriptor>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());

    // spwan a sprite rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(
                window_descriptor.width,,
                window_descriptor.height
            )),
            ..Default::default()
        },
        transform: Transform {
            ..Default::default()
        },
        ..Default::default()
    });
}