extern crate sdl2;

use self::sdl2::{event::Event, keyboard::Keycode, Sdl};
use std::time::{Duration, Instant};

use engine::timing::scheduler::Scheduler;
use nodes::{
    node::RNode,
    scenes::scene_manager::{IOEventData, SceneManager},
};

const SECOND: u32 = 1000000000; // billion ns in a second

// If Sleep is disabled or removed from the code, or VSync is enabled
// then the below FPS variable has no effect.
const FRAMES_PER_SECOND: usize = 120; // Minimum fps to achieve

const UPDATES_PER_SECOND: usize = 30; // Maximum updates per second

// 1 frame period is equal to a fraction. For example, if
// FRAMES_PER_SECOND = 60.0 then frame period is 0.01666666667s of a second
// or in milliseconds it is 1000.0/60.0 = 16.66666667ms per frame.
// 1ms = 1000us = 1000000ns
const FRAME_PERIOD: f64 = 1_000_000_000.0 / FRAMES_PER_SECOND as f64; // in nanoseconds
const UPDATE_PERIOD: f64 = 1_000_000_000.0 / UPDATES_PER_SECOND as f64; // in nanoseconds

pub struct Core {
    // -------------------------------------------------------
    // Debug and diagnostics
    // -------------------------------------------------------
    // step_enabled: bool,
    fps: usize,
    ups: usize,
    frame_period: Duration,
    update_period: Duration,
    avg_blit_time: f64,
    avg_ren_time: f64,
    // avg_sleep_time: f64,
    avg_up_time: f64,
    // -------------------------------------------------------
}

impl Core {
    pub fn new() -> Self {
        Self {
            // -------------------------------------------------------
            // Debug and diagnostics
            // -------------------------------------------------------
            // step_enabled: false,
            fps: 0,
            ups: 0,
            frame_period: Duration::new(0, FRAME_PERIOD.round() as u32),
            update_period: Duration::new(0, UPDATE_PERIOD.round() as u32),
            // frame_count: 0,
            // avg_process_time: 0.0,
            avg_blit_time: 0.0,
            avg_ren_time: 0.0,
            // avg_sleep_time: 0.0,
            avg_up_time: 0.0,
        }
    }

    pub fn initialize(&mut self) {}

    // This loop is losely based on:
    // http://gameprogrammingpatterns.com/game-loop.html
    pub fn core_loop(
        &mut self,
        context: &Sdl,
        scene_manager: &mut SceneManager,
        scheduler: &mut Scheduler,
    ) -> Result<String, String> {
        let ns_per_update = self.update_period.subsec_nanos();
        let frame_dt = ns_per_update as f64 / 1000000.0;
        println!("ns_per_update: {}", ns_per_update);

        let mut lag = 0;
        let mut second_acm = 0;
        let mut fps = 0;
        let mut ups = 0;
        // #[allow(unused_assignments)]
        // let mut sleep = Duration::new(0, 0);
        // let mut sleep_accum = 0u32;
        let mut blit_accum = 0u32;
        let mut proc_accum = 0u32;
        let mut up_accum = 0u32;

        let mut previous_t = Instant::now();

        let mut keycode = Keycode::Clear;

        let mut event_pump = match context.event_pump() {
            Ok(pump) => pump,
            Err(err) => return Err(err),
        };

        'fast: loop {
            let current_t = Instant::now();

            // ##############################################################
            // Input
            // ##############################################################
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        .. // don't-care about other fields
                    } => break 'fast,
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        repeat: false,
                        ..
                    } => {
                        // Do something
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        keycode = Keycode::Right;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        keycode = Keycode::Left;
                    }
                    Event::KeyUp {
                        ..
                    } => {
                        keycode = Keycode::Clear;
                    }
                    Event::MouseMotion {
                        x,
                        y,
                        ..
                    } => {
                        // println!("mouse {},{}", x, y);
                        scene_manager.io_event(IOEventData::new_mouse_event(x,y));
                    }
                    _ => {}
                }
            }

            if keycode != Keycode::Clear {
                println!("key: {}", keycode);
            }

            // println!("================================================");

            // ##############################################################
            // Update
            // ##############################################################
            let elapsed_t = current_t - previous_t;
            previous_t = current_t;
            lag += elapsed_t.subsec_nanos();

            let u = Instant::now();

            'up: loop {
                if lag >= ns_per_update {
                    scheduler.update(frame_dt);
                    lag -= ns_per_update;
                    ups += 1;
                } else {
                    ups = 0;
                    break 'up;
                }
            }

            // ::std::thread::sleep(Duration::from_millis(15)); // force/test pipeline overload

            let un = Instant::now().duration_since(u);
            up_accum += un.subsec_nanos();

            // ##############################################################
            // Render
            // ##############################################################
            let interpolation = (lag as f64) / (ns_per_update as f64);
            let pn: Duration;
            {
                // let mut scm = scene_manager.borrow_mut();

                // If vsync is enabled then this takes nearly 1/fps milliseconds.
                // In other words it is waiting for the refresh vertical sync.
                scene_manager.pre_process();

                let p = Instant::now();

                if !scene_manager.visit(interpolation) {
                    // There are no more scenes to draw
                    break 'fast;
                }

                scene_manager.render_stats(
                    self.fps as f64,
                    self.ups as f64,
                    self.avg_ren_time,
                    self.avg_up_time,
                    self.avg_blit_time,
                );

                scene_manager.render_coordinates();

                pn = Instant::now().duration_since(p);
                // println!("render time: {}", pn.subsec_micros());
                proc_accum += pn.subsec_nanos();
            }

            // ##############################################################
            // Blit
            // ##############################################################
            // SDL appears to only take about 0.3ms to blit.
            let b = Instant::now();
            scene_manager.post_process();
            let bn = Instant::now().duration_since(b);

            // print!("[({}), {}]", blit_accum, bn.subsec_nanos());
            // print!("[{}]", bn.subsec_nanos() as f64 / 1000000.0);
            blit_accum += bn.subsec_nanos();

            // ##############################################################
            // Sleep
            // ##############################################################
            // How much time was taken for the above steps
            let work = un + bn + pn;

            // Was the work done in this frame less than the alotted period
            if work < self.frame_period {
                // Sleep is the remainder.
                // sleep = self.frame_period - work;
                // std::thread::sleep(sleep);

                // Simply sleep for a tiny amount to allow main thread processing.
                // std::thread::sleep(Duration::from_micros(500));
                // std::thread::sleep(Duration::from_millis(1));
                // std::thread::yield_now();
            } else {
                //sleep = Duration::new(0, 0);
            }

            // The total elapsed time spent for the frame is the work plus any
            // sleeping.
            // let elapsed = work; // + sleep;

            // let frame_time = elapsed.subsec_nanos();
            // Because we are counting using what amounts to nothing more
            // than the frame period then the second counter will be slightly off
            // by a frame period.
            second_acm += elapsed_t.subsec_nanos(); //frame_time;
            fps += 1;

            if second_acm >= SECOND {
                // println!("display update {}, {}", ns_per_update, SECOND);
                self.fps = fps;
                // (1 / (((ups+1) * ns_per_update) / 1000000)) * 1000 = fps
                self.ups =
                    ((1.0 / (((ups + 1) * ns_per_update) as f32 / 1000000.0)) * 1000.0) as usize;
                self.avg_blit_time = ((blit_accum as f64) / (fps as f64)) / 1000000.0;
                self.avg_ren_time = ((proc_accum as f64) / (fps as f64)) / 1000000.0;
                // self.avg_sleep_time = ((sleep_accum as f64) / (fps as f64)) / 1000000.0;
                self.avg_up_time = ((up_accum as f64) / (fps as f64)) / 1000000.0;

                // println!(
                //     "elap: {:8.5}, fps: {} ({:8.5}), up: {}, [slp: {:8.5}, ren: {:8.5}, aup: {:8.5}, blit: {:8.5}] = {:8.5}",
                //     frame_acm as f64 / 1000000.0,
                //     self.fps,
                //     1000.0 / total,
                //     self.ups,
                //     self.avg_sleep_time,
                //     self.avg_ren_time,
                //     self.avg_up_time,
                //     self.avg_blit_time,
                //     total ,
                // );

                second_acm = 0;
                // sleep_accum = 0;
                proc_accum = 0;
                blit_accum = 0;
                up_accum = 0;

                fps = 0;
                ups = 0;
            }

            // break 'fast; // Debug
        }

        Ok(String::from("Exited Game loop"))
    }

    /// The core takes ownership.
    // pub fn add_scene<S: Scene + 'static>(&mut self, mut scene: S) {
    pub fn push_scene(&mut self, scene: RNode, scene_manager: &SceneManager) {
        // The core will take ownership for the duration of the game.
        scene_manager.push_scene(scene);
    }
}
