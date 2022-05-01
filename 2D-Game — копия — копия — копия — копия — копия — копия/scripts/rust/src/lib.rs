use gdnative::init::{godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init};
use player::Player;

mod camera;
mod controls;
mod player;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Player>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
