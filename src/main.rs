use bevy::prelude::*;
use turn_td::GamePlugin;

fn main() {
    App::new().add_plugin(GamePlugin).run();
}
