extern crate sdl2;

use self::sdl2::render::WindowCanvas;

use engine::{core::Core, timing::scheduler::Scheduler};
use nodes::{node::RNode, scenes::scene_manager::SceneManager};
use rendering::{color::Palette, fx_triangle::FXTriangle, render_context::Context};

pub struct ConfigData {
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

impl ConfigData {
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
}

pub struct SystemData {
    data: ConfigData,

    scene_manager: SceneManager,
    scheduler: Scheduler,

    // canvas: WindowCanvas,
    context: Context,

    rasterizer: FXTriangle,

    id: usize,
}

impl SystemData {
    pub fn new(canvas: WindowCanvas) -> Self {
        let mut wd = Self {
            data: ConfigData::new(),
            scene_manager: SceneManager::new(),
            scheduler: Scheduler::new(),
            context: Context::new(canvas),
            rasterizer: FXTriangle::new(),

            id: 0,
        };

        wd.scene_manager.initialize(&wd.data);

        wd
    }

    pub fn initialize(&mut self) {}

    pub fn data(&self) -> &ConfigData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut ConfigData {
        &mut self.data
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn scene_manager(&self) -> &SceneManager {
        &self.scene_manager
    }

    pub fn scene_manager_mut(&mut self) -> &mut SceneManager {
        &mut self.scene_manager
    }

    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }

    pub fn rasterizer(&self) -> &FXTriangle {
        &self.rasterizer
    }

    pub fn rasterizer_mut(&mut self) -> &mut FXTriangle {
        &mut self.rasterizer
    }

    pub fn gen_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }

    // pub fn pre_process(&mut self) {
    //     self.scene_manager.pre_process(&mut self.context);
    // }
}
