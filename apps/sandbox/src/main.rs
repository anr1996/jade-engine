//! Demo binary exercising the [`engine_core`] interfaces.

use engine_core::{App, Local, Plugin, Stage};

fn startup() {
    println!("startup");
}

fn tick(mut counter: Local<u32>) {
    *counter += 1;
    println!("tick {}", *counter);
}

struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Stage::Startup, startup)
            .add_systems(Stage::Update, tick);
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugin(&DemoPlugin);

    app.update();
    app.update();
    app.update();
}
