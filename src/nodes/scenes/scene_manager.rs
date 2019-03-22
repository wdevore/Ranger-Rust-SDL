extern crate sdl2;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use nodes::{
    node::{NodeType, Nodes, RNode},
    node_nil::NodeNil,
};
use rendering::{color::Palette, render_context::Context};
use world::{RCCanvas, WorldProperties};

pub type RSceneManager = Rc<RefCell<SceneManager>>;

pub enum SceneActions {
    NO_ACTION,
    REPLACE,
    REPLACE_TAKE,
    REPLACE_TAKE_UNREGISTER,
}

pub enum IOEvent {
    NONE,
    MOUSE,
    JOYSTICK,
    KEYBOARD,
}

pub struct IOEventData {
    pub event: IOEvent,
    pub coord: (i32, i32),
    pub node: RNode,
}

impl IOEventData {
    pub fn new() -> Self {
        Self {
            event: IOEvent::NONE,
            coord: (0, 0),
            node: NodeNil::new(),
        }
    }

    pub fn new_mouse_event(x: i32, y: i32) -> Self {
        Self {
            event: IOEvent::MOUSE,
            coord: (x, y),
            node: NodeNil::new(),
        }
    }
}

pub struct GlobalSceneData {
    // Mouse-space is synonymous with window/device space.
    mouse: (i32, i32), // (x,y)
    mouse_changed: bool,

    // View-space coordinates
    view: (f64, f64),
    // node-space coordinates
    // node: (f64, f64),
    io_event_targets: Vec<RNode>,
}

impl Drop for GlobalSceneData {
    fn drop(&mut self) {
        println!("Dropping GlobalSceneData");
        self.io_event_targets.clear();
    }
}

impl GlobalSceneData {
    fn new() -> Self {
        Self {
            mouse: (0, 0),
            view: (0.0, 0.0),
            // node: (0.0, 0.0),
            mouse_changed: false,
            io_event_targets: Vec::new(),
        }
    }

    pub fn set_mouse(&mut self, x: i32, y: i32) {
        self.mouse = (x, y);
        self.mouse_changed = true;
    }

    pub fn update_view_coords(&mut self, context: &mut Context) {
        if self.mouse_changed {
            self.view = Nodes::map_device_to_view(self.mouse.0, self.mouse.1, context);
            self.mouse_changed = false;
        }
    }

    pub fn register_for_io_events(&mut self, parent: RNode, child: usize) {
        let fin = Nodes::find_node(child, &parent);
        if !fin.borrow().is_nil() {
            println!("Register '{}' for io events", fin.borrow().name());
            self.io_event_targets.push(fin);
        } else {
            println!("Could not find ({}) to register", child);
        }

        // self.io_event_targets.push(node);
        println!("len: {}", self.io_event_targets.len());
    }

    pub fn unregister_for_io_events(&mut self, parent: RNode, child: usize) {
        // let contains = self.io_event_targets.contains(&node);
        // if contains {
        //     return;
        // }
        let fin = Nodes::find_node(child, &parent);
        if !fin.borrow().is_nil() {
            println!("Register '{}' for io events", fin.borrow().name());
            let nb = fin.borrow();
            self.io_event_targets
                .retain(|n| n.borrow().id() != nb.data().borrow().node.id());
        } else {
            println!("Could not find ({}) to unregister", child);
        }

        // Keep all but the referenced `node`
        println!("un len: {}", self.io_event_targets.len());
    }
}
// SM can animate two scenes at once:
// An incoming scene and an outgoing scene.
// When a scene
pub struct SceneManager {
    world_properties: WorldProperties,

    perform_clear: bool,

    context: Context,

    // Scenes
    scenes: SceneStack,

    // Global Scene data
    global_data: GlobalSceneData,

    // DEBUG
    fps_color: Palette,
    coords_color: Palette,
}

impl Drop for SceneManager {
    fn drop(&mut self) {
        println!("Dropping SceneManager");
    }
}

impl SceneManager {
    pub fn new(canvas: RCCanvas) -> Self {
        Self {
            world_properties: WorldProperties::new(),
            perform_clear: false,
            context: Context::new(canvas),
            scenes: SceneStack::new(),
            global_data: GlobalSceneData::new(),
            fps_color: Palette::WHITE(127),
            coords_color: Palette::LIME(),
        }
    }

    pub fn initialize(&mut self, world_props: &WorldProperties) {
        self.world_properties.set(&world_props);
        self.perform_clear = world_props.perform_clear;

        self.context.initialize(world_props);
    }

    pub fn global_data_mut(&mut self) -> &mut GlobalSceneData {
        &mut self.global_data
    }

    pub fn pre_process(&self) {
        // Typically Scenes/Layers will clear the background themselves so the default
        // is to NOT perform a clear here.
        if self.perform_clear {
            // If vsync is enabled then this takes nearly 1/fps milliseconds.
            // For example, 60fps -> 1/60 = 16.666~ms
            self.context.clear();
        }
    }

    pub fn visit(&mut self, interpolation: f64) -> bool {
        // Check for scenes
        if self.scenes.is_empty() {
            println!("SceneManager: no more scenes to visit.");
            return false;
        }

        if !self.scenes.next_scene_nil() {
            self.set_next_scene();
        }

        // This will save view-space matrix
        self.context.save();

        // If mouse coords changed then update view coords.
        self.global_data.update_view_coords(&mut self.context);

        {
            let rfc = self.scenes.running_scene().borrow();
            let r = rfc.borrow();
            let action = r.transition(self);
            match action {
                SceneActions::REPLACE => {
                    if let Some(repl) = r.take_transition_scene() {
                        self.scenes.replace(repl);
                    }
                }
                _ => (),
            }

            rfc.borrow().visit(&mut self.context, interpolation);
        }

        // Process view after visiting Nodes.
        self.context.restore();

        true // continue to draw.
    }

    pub fn post_process(&self) {
        self.context.post();
    }

    pub fn set_next_scene(&mut self) {
        // Is the next scene a transition
        if !self.scenes.next_scene_is_transition() {
            if !self.scenes.running_scene_nil() {
                let rfc = self.scenes.running_scene().borrow();
                let r = rfc.borrow();

                // It is not a transition so it must be a regular scene which means it
                // needs to start transitioning off the stage.
                // Signal the scene to start exiting the stage via a transition.
                r.start_exit_transition(&mut self.global_data);
                // Transition is complete signal the scene to complete its exit.
                r.exit(&mut self.global_data);

                if self.scenes.signal_flush() {
                    // Some scenes may need to release resources.
                    r.flush(self.scenes.signal_flush());
                }
            }
        };

        // Make the running scene the next active scene.
        // self.running_scene = self.next_scene.clone();
        self.scenes.make_running_scene();

        println!("---- Next scene set to Nil ----: ");
        self.scenes.make_next_scene_nil();

        let rfc = self.scenes.running_scene().borrow();
        let r = rfc.borrow();
        print!("---- Running scene ----: ");
        println!("{}", r.to_string());

        // Are we transitioning from one Scene to the next.
        if !self.scenes.running_scene_is_transition() {
            // This is a regular scene.
            // Signal the scene that it should enter the stage.
            let rfc = self.scenes.running_scene().borrow();
            let r = rfc.borrow();
            if !r.is_nil() {
                r.enter(self);
                r.end_enter_transition();
            }
        }
    }

    pub fn pop_scene(&self) {
        self.scenes.pop();
    }

    pub fn push_scene(&self, scene: RNode) {
        self.scenes.push(scene);
    }

    /// Replaces the top scene with provided scene.
    pub fn replace_scene(&self, scene: RNode) {
        let rfc = self.scenes.running_scene().borrow();
        // let r = rfc.borrow();
        if rfc.borrow().is_nil() {
            panic!("SceneManager::replace_scene -- no running scene.");
        }

        self.scenes.replace(scene);
    }

    // --------------------------------------------------------------------------
    // IO events
    // --------------------------------------------------------------------------
    pub fn io_event(&mut self, io_event: IOEventData) {
        match io_event.event {
            IOEvent::MOUSE => {
                self.global_data
                    .set_mouse(io_event.coord.0, io_event.coord.1);

                let rfc = self.scenes.running_scene().borrow();
                let mut r = rfc.borrow_mut();
                if !r.is_nil() {
                    r.io_event(&io_event);
                }
            }
            _ => (),
        }
    }

    // pub fn register_for_io_events(&mut self, node_id: usize) {
    //     // self.io_event_targets.push(node_id);
    //     // Find node in running scene

    //     let rfc = self.scenes.running_scene().borrow();
    //     let r = rfc.borrow();

    //     if !r.is_nil() {
    //         let r = Nodes::find_node(node_id, &rfc);
    //         if !r.borrow().is_nil() {
    //             println!("#### found");
    //             self.global_data.register_for_io_events(r);
    //         } else {
    //             println!("##### NOT FOUND");
    //         }
    //         // Search
    //     }
    // }

    // --------------------------------------------------------------------------
    // Debug stuff
    // --------------------------------------------------------------------------
    pub fn render_stats(
        &mut self,
        fps: f64,
        ups: f64,
        avg_ren_time: f64,
        avg_up_time: f64,
        avg_blit_time: f64,
    ) {
        // Draws to device space (aka window space)
        self.context.set_draw_color(&self.fps_color);
        self.context.text(
            5,
            (self.world_properties.window_height - 24) as i32,
            &format!(
                "Fps:{}, Ups:{:5.1}, ren:{:3.2}, upd: {:3.2} Blt:{:5.2}ms",
                fps, ups, avg_ren_time, avg_up_time, avg_blit_time
            ),
            2,
            1,
        );
    }

    pub fn render_coordinates(&mut self) {
        // Draws to device space (aka window space)
        self.context.set_draw_color(&self.coords_color);
        self.context.text(
            10,
            10,
            &format!(
                "M: {}, {}",
                self.global_data.mouse.0, self.global_data.mouse.1
            ),
            2,
            1,
        );

        self.context.text(
            10,
            30,
            &format!(
                "V: {:5.2}, {:5.2}",
                self.global_data.view.0, self.global_data.view.1
            ),
            2,
            1,
        );
    }
}

// --------------------------------------------------------------------------
// Internal scene stack
// --------------------------------------------------------------------------
struct SceneStack {
    scenes: RefCell<Vec<RNode>>,
    // Indicates if a scene should dispose completely once it isn't needed
    // anymore. For example, boot and splash scenes typically have this
    // enabled.
    signal_scene_to_flush: Cell<bool>,
    next_scene: RefCell<RNode>,
    running_scene: RefCell<RNode>,
}

impl Drop for SceneStack {
    fn drop(&mut self) {
        println!(
            "Dropping SceneStack scenes: ({})",
            self.scenes.borrow().len()
        );

        for scene in self.scenes.borrow().iter() {
            scene.borrow().flush(true);
        }

        self.scenes.borrow_mut().clear();
        // use std::mem::drop;
        // drop(&self.signal_scene_to_flush);
        // drop(&self.next_scene);
        // println!("SceneStack scenes: ({})", self.scenes.borrow().len());
    }
}

impl SceneStack {
    fn new() -> Self {
        Self {
            scenes: RefCell::new(Vec::new()),
            next_scene: RefCell::new(NodeNil::new()),
            running_scene: RefCell::new(NodeNil::new()),
            signal_scene_to_flush: Cell::new(false),
        }
    }

    fn is_empty(&self) -> bool {
        self.scenes.borrow().is_empty()
    }

    fn next_scene_nil(&self) -> bool {
        let ns = self.next_scene.borrow();
        let rfc = ns.borrow();
        rfc.is_nil()
    }

    fn running_scene_nil(&self) -> bool {
        let ns = self.running_scene.borrow();
        let rfc = ns.borrow();
        rfc.is_nil()
    }

    fn next_scene_is_transition(&self) -> bool {
        let ns = self.next_scene.borrow();
        let rfc = ns.borrow();
        rfc.is_node_type(NodeType::SceneTransition)
    }

    fn running_scene_is_transition(&self) -> bool {
        let ns = self.running_scene.borrow();
        let rfc = ns.borrow();
        rfc.is_node_type(NodeType::SceneTransition)
    }

    // fn next_scene(&self) -> &RefCell<RNode> {
    // &self.next_scene
    // }

    fn make_next_scene_nil(&self) {
        self.next_scene.replace(NodeNil::new());
    }

    fn running_scene(&self) -> &RefCell<RNode> {
        &self.running_scene
    }

    fn make_running_scene(&self) {
        self.running_scene.replace(self.next_scene.borrow().clone());
    }

    pub fn signal_flush(&self) -> bool {
        self.signal_scene_to_flush.get()
    }

    // pub fn set_signal_flush(&self, flush: bool) {
    //     self.signal_scene_to_flush.set(flush);
    // }

    fn push(&self, scene: RNode) {
        self.signal_scene_to_flush.set(false);

        self.next_scene.replace(scene.clone());

        print!("---- Pushing Scene ----: ");
        let ns = self.next_scene.borrow();
        println!("{}", ns.borrow().to_string());

        self.scenes.borrow_mut().push(scene);
    }

    fn pop(&self) {
        match self.scenes.borrow_mut().pop() {
            Some(scene) => {
                self.next_scene.replace(scene);

                // Allow the current running scene a chance to cleanup.
                self.signal_scene_to_flush.set(true);

                print!("---- Popped Scene ----: ");
                let ns = self.next_scene.borrow();
                println!("{}", ns.borrow().to_string());
            }
            None => {
                // Basically there are no more scenes to execute.
                println!("SceneManager::pop_scene -- no scenes to pop.");
            }
        }
    }

    fn replace(&self, scene: RNode) {
        // let rs = self.running_scene.borrow();
        // if rs.borrow().is_nil() {
        //     panic!("SceneManager::replace_scene -- no running scene.");
        // }

        print!("---- Replacing Scene ----: ");
        {
            let ns = self.next_scene.borrow();
            println!("{}", ns.borrow().to_string());
        }

        self.next_scene.replace(scene.clone());

        print!("---- With Scene ----: ");
        let ns = self.next_scene.borrow();
        println!("{}", ns.borrow().to_string());

        if !self.scenes.borrow().is_empty() {
            println!("Scenes on stack before: ({})", self.scenes.borrow().len());
            if let Some(pscene) = self.scenes.borrow_mut().pop() {
                println!("Popped '{}'", pscene.borrow().to_string());
                // pscene.borrow().flush(true);
            }
            println!("Scenes on stack after: ({})", self.scenes.borrow().len());
        }

        self.scenes.borrow_mut().push(scene);

        self.signal_scene_to_flush.set(true);
    }
}
