#![allow(unused)]

use bevy::prelude::*;
use crate:: { 
    my_window::*,
};

mod my_window;

fn main() {
    App::new()
        // 
        .add_plugin(MyWindowPlugin)
        // 
        .add_plugins(DefaultPlugins)
        .run();
}
