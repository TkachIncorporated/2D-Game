use gdnative::prelude::godot_init;
use player::Player;

mod controls;
mod player;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Player>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
