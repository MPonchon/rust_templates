use bevy::prelude::*;
// use crate::components::Mainwindow;

struct WinSize {
  w: f32,
  h: f32
}

const WINDOW_TITLE:  &'static str = "titre de la fenetre";
const WINDOW_SIZE: WinSize  = WinSize { w: 600.0, h: 800.0 };
const BACKGROUND_COLOR: Color = Color::DARK_GREEN;

pub struct MyWindowPlugin;

impl Plugin for MyWindowPlugin {
    fn build(&self, app: &mut App) {
      app
      // .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
      .insert_resource(ClearColor(BACKGROUND_COLOR))
      .insert_resource(
          WindowDescriptor {
            title: WINDOW_TITLE.to_string(),
            width: WINDOW_SIZE.w,
            height: WINDOW_SIZE.h,
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..Default::default()
          })
      .add_startup_system(window_setup_system)
      .add_system(bevy::window::close_on_esc)
      ;
    }
}

fn window_setup_system(
  mut commands: Commands,
) {
  // camera
  commands.spawn_bundle(Camera2dBundle::default());
}