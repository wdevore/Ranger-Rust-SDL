// Run this example from the Ranger-Rust-SDL:
// $ cargo run --example template0

extern crate ranger;

use ranger::{nodes::node::Nodes, nodes::scenes::scene_boot::SceneBoot, world::World};

mod template_0;

use template_0::game_scene::GameScene;
use template_0::splash_scene::SplashScene;

const DISPLAY_RATIO: f32 = 16.0 / 9.0;
const WIDTH: u32 = 1024 + 512;
// Larget number causes the view to encompass more of the world
// which makes objects appear smaller.
const VIEW_SCALE: f64 = 1.5;

fn main() {
    // Use the Ranger engine to configure, boot and launch game.
    let window_width = WIDTH;
    let window_height = (WIDTH as f32 / DISPLAY_RATIO) as u32;

    let view_width = window_width as f64 * VIEW_SCALE;
    let view_height = window_height as f64 * VIEW_SCALE;

    println!("Display dimensions: [{} x {}]", window_width, window_height);
    println!("View dimensions: [{} x {}]", view_width, view_height);

    let mut world = match World::new(
        window_width,
        window_height,
        view_width,
        view_height,
        true,
        "Ranger Basic",
        "game.config",
        true,
    ) {
        Ok(eng) => eng,
        Err(err) => {
            panic!("Could not create Engine: {}", err);
        }
    };

    match world.configure() {
        Ok(msg) => {
            if msg != "Configured" {
                panic!("Unknown Configured response: {}", msg);
            }
        }
        Err(err) => {
            panic!("Error during Configured sequence: {}", err);
        }
    }

    match world.launch(build) {
        Ok(msg) => {
            println!("World: {}", msg);
        }
        Err(err) => {
            panic!("Error during launch and/or exit sequence: {}", err);
        }
    }
}

fn build(world: &mut World) -> bool {
    println!("Building...");
    let game_scene = GameScene::new("GameScene", world);

    Nodes::register_timing_targets(&game_scene, world.get_scheduler());

    let splash_scene = SplashScene::with_replacement("SplashScene", game_scene, world);
    {
        let mut splash = splash_scene.borrow_mut();

        if let Some(n) = splash.as_any_mut().downcast_mut::<SplashScene>() {
            n.pause_for_seconds(0.25);
        }
    }

    Nodes::register_timing_targets(&splash_scene, world.get_scheduler());

    let boot_scene = SceneBoot::with_replacement("BootScene", splash_scene, world);

    world.push_scene(boot_scene);

    println!("Build complete.");
    true
}
