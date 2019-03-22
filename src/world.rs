extern crate sdl2;

// use std::error::Error;
// use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use self::sdl2::render::WindowCanvas;
use self::sdl2::Sdl;

use engine::core::Core;
use engine::timing::scheduler::Scheduler;
use nodes::{node::RNode, scenes::scene_manager::SceneManager};

// Game developer uses this callback to build their game.
type BuildCallback = fn(&mut World) -> bool;

pub type RCCanvas = Rc<RefCell<WindowCanvas>>;

pub struct WorldProperties {
    pub window_width: usize,
    pub window_height: usize,
    pub view_width: f64,
    pub view_height: f64,
    pub view_centered: bool,
    pub title: String,
    pub config: String,
    pub vysnc_enabled: bool,
    pub perform_clear: bool,
}

impl WorldProperties {
    pub fn new() -> Self {
        Self {
            window_width: 0,
            window_height: 0,
            view_width: 0.0,
            view_height: 0.0,
            view_centered: true,
            title: String::from(""),
            config: String::from("config.json"),
            vysnc_enabled: true,
            perform_clear: true,
        }
    }

    pub fn from(wp: &WorldProperties) -> Self {
        Self {
            window_width: wp.window_width,
            window_height: wp.window_height,
            view_width: wp.view_width,
            view_height: wp.view_height,
            view_centered: wp.view_centered,
            title: String::from(wp.title.to_owned()),
            config: String::from(wp.config.to_owned()),
            vysnc_enabled: wp.vysnc_enabled,
            perform_clear: wp.perform_clear,
        }
    }

    pub fn set(&mut self, wp: &WorldProperties) {
        self.window_width = wp.window_width;
        self.window_height = wp.window_height;
        self.view_width = wp.view_width;
        self.view_height = wp.view_height;
        self.view_centered = wp.view_centered;
        self.title = String::from(wp.title.to_owned());
        self.config = String::from(wp.config.to_owned());
        self.vysnc_enabled = wp.vysnc_enabled;
        self.perform_clear = wp.perform_clear;
    }
}

/// Ranger is the main object hosting your game. You construct [Scene]s and give them to Ranger
/// for execution. When the last Scene exits the game comes to an end.
// #[derive(Clone)]
pub struct World {
    properties: WorldProperties,

    core: Core,

    scene_manager: SceneManager,
    scheduler: Scheduler,

    context: Sdl,
    config: String,

    id: usize,
}

impl World {
    /// Create a Ranger game `Engine`.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of gui window
    /// * `height` - Height of gui window
    /// * `title` - Window's title bar text
    /// * `config` - JSON configuration file. For example, contains the window background color
    pub fn new(
        window_width: u32,
        window_height: u32,
        view_width: f64,
        view_height: f64,
        view_centered: bool,
        title: &str,
        config: &str,
        vysnc_enabled: bool,
    ) -> Result<Self, String> {
        let mut wp = WorldProperties::new();
        wp.window_width = window_width as usize;
        wp.window_height = window_height as usize;
        wp.view_width = view_width;
        wp.view_height = view_height;
        wp.view_centered = view_centered;
        wp.title = String::from(title);

        let sdl_context = match sdl2::init() {
            Ok(context) => context,
            Err(err) => return Err(err),
        };

        let video_subsystem = match sdl_context.video() {
            Ok(system) => system,
            Err(err) => return Err(err),
        };

        let window = match video_subsystem
            .window(title, window_width, window_height)
            .position_centered()
            .build()
        {
            Ok(win) => win,
            Err(build_error) => return Err(build_error.to_string()),
        };

        let canvas = if vysnc_enabled {
            match window.into_canvas().present_vsync().build() {
                Ok(can) => Rc::new(RefCell::new(can)),
                Err(err) => return Err(err.to_string()),
            }
        } else {
            match window.into_canvas().build() {
                Ok(can) => Rc::new(RefCell::new(can)),
                Err(err) => return Err(err.to_string()),
            }
        };

        let mut scene_manager = SceneManager::new(canvas);
        scene_manager.initialize(&wp);

        let mut core = Core::new();
        core.initialize();

        let e = Self {
            properties: wp,
            core: core,
            scene_manager: scene_manager,
            scheduler: Scheduler::new(),
            context: sdl_context,
            config: config.to_string(),
            id: 0,
        };

        Ok(e)
    }

    pub fn gen_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }

    /// Configure using config json
    pub fn configure(&mut self) -> Result<String, String> {
        println!("Using config: {}", self.config);

        // Err(String::from("kaboom"))
        Ok(String::from("Configured"))
    }

    pub fn launch(&mut self, build: BuildCallback) -> Result<String, String> {
        // Create Core. The Core contains the Systems including render Context.
        println!("Initializing Core...");
        // self.core.initialize(self);

        // Perform pre-build of underlying Systems (SceneManager, Scheduler, TweenManager...)
        println!("Constructing and/or initializing Systems...");

        // Now notify the developer to build their game.
        let built = build(self);
        if !built {
            return Err(String::from("Game failed to build."));
        }

        println!("Launching game...");
        self.core
            .core_loop(&self.context, &mut self.scene_manager, &mut self.scheduler)?;

        // Shutdown engine

        Ok(String::from("Exited"))
    }

    // ---------------------------------------------------------------
    // Properties
    // ---------------------------------------------------------------
    pub fn properties(&self) -> &WorldProperties {
        &self.properties
    }

    // pub fn width(&self) -> usize {
    //     self.properties.width
    // }
    // pub fn height(&self) -> usize {
    //     self.properties.height
    // }

    // ---------------------------------------------------------------
    // Scene management
    // ---------------------------------------------------------------
    pub fn get_scene_manager(&mut self) -> &mut SceneManager {
        &mut self.scene_manager
    }

    pub fn replace_scene(&self, replacement: RNode) {
        self.scene_manager.replace_scene(replacement.clone());
    }

    pub fn push_scene(&self, scene: RNode) {
        self.scene_manager.push_scene(scene);
    }

    // ---------------------------------------------------------------
    // Scheduler
    // ---------------------------------------------------------------
    pub fn get_scheduler(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }
}

// #[derive(Debug)]
// pub struct AppError {
//     details: String,
// }

// impl AppError {
//     fn new(msg: &str) -> Self {
//         Self {
//             details: msg.to_string(),
//         }
//     }
// }
// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.details)
//     }
// }

// impl Error for AppError {
//     fn description(&self) -> &str {
//         &self.details
//     }
// }
