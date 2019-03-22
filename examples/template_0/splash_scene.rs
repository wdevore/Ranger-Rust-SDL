use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

extern crate ranger;

use ranger::{
    nodes::{
        node::{NodeTrait, NodeType, RNode, RONode},
        node_nil::NodeNil,
        node_properties::NodeData,
        scenes::scene_manager::{SceneActions, SceneManager},
    },
    rendering::color::Palette,
    rendering::render_context::Context,
    world::World,
};

pub struct SplashScene {
    replacement: RefCell<Option<RNode>>,
    // replacement: RNode,
    data: RefCell<NodeData>,

    parent: RONode,
}

impl Drop for SplashScene {
    fn drop(&mut self) {
        println!("Dropping: '{}'", self.data().borrow().node.name());
    }
}

impl SplashScene {
    pub fn with_replacement(
        name: &str,
        replace: RNode,
        world: &mut World,
    ) -> Rc<RefCell<NodeTrait>> {
        let mut n = NodeData::new();
        n.node.set_name(name.to_string());
        n.node.set_type(NodeType::Scene);
        n.node.set_id(world.gen_id());
        n.node.make_timing_target(true);

        let ss = Self {
            replacement: RefCell::new(Some(replace)),
            data: RefCell::new(n),
            parent: Rc::new(RefCell::new(None)),
        };

        Rc::new(RefCell::new(ss))
    }

    pub fn pause_for_seconds(&mut self, pause_for: f64) {
        self.data()
            .borrow_mut()
            .transition
            .set_pause_for(pause_for * 1000.0);
    }
}

impl NodeTrait for SplashScene {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    // --------------------------------------------------------
    // Node properties
    // --------------------------------------------------------
    fn data(&self) -> &RefCell<NodeData> {
        &self.data
    }

    // --------------------------------------------------------
    // Timing target
    // --------------------------------------------------------

    fn update(&self, dt: f64) {
        self.data().borrow_mut().transition.update(dt);

        // println!("update '{}', {}", dt, self.to_string());
    }

    // --------------------------------------------------------
    // Transitions
    // --------------------------------------------------------
    fn transition(&self, _scene_manager: &SceneManager) -> SceneActions {
        // println!("transition '{}'", self.to_string());
        if self.data().borrow().transition.ready() {
            // scene_manager.replace_scene(self.replacement.clone());
            return SceneActions::REPLACE;
        }

        SceneActions::NO_ACTION
    }

    // --------------------------------------------------------
    // Life cycle events
    // --------------------------------------------------------
    fn enter(&self, _scene_manager: &SceneManager) {
        println!("enter '{}'", self.to_string());
        self.data().borrow_mut().transition.reset_pause();

        // Schedule/enable timing
        self.pause(false);
    }

    // --------------------------------------------------------
    // Transformations
    // --------------------------------------------------------
    fn parent(&self) -> RONode {
        self.parent.clone()
    }

    fn take_transition_scene(&self) -> Option<RNode> {
        self.replacement.borrow_mut().take()
    }

    fn set_parent(&self, parent: RNode) {
        self.parent.borrow_mut().replace(parent);
    }

    // --------------------------------------------------------
    // Render events
    // --------------------------------------------------------
    fn visit(&self, context: &mut Context, _interpolation: f64) {
        context.set_draw_color(&Palette::from_hex_rgb(0xffffff));
        context.text(5, 5, "Splash scene", 5, 2);
        // println!("visit '{}'", self.to_string());
    }
}
