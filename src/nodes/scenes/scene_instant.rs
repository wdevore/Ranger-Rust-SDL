// use std::cell::RefCell;
// use std::rc::Rc;

// use engine::timing::scheduler::Scheduler;
// use engine::timing::timing_target::TimingTarget;
// use nodes::node::Node;
// use nodes::node_base::{NodeTrait, NodeType};
// use nodes::node_events::NodeEvents;
// use nodes::scenes::scene::Scene;
// use nodes::scenes::scene_manager::SceneManager;
// use rendering::render_context::Context;

// pub struct SceneInstant {
//     node: Node,
// }

// impl SceneInstant {
//     pub fn new(name: &str) -> Rc<RefCell<SceneInstant>> {
//         let mut n = Node::new(name);
//         n.base.set_type(NodeType::Scene);

//         Rc::new(RefCell::new(Self { node: n }))
//     }
// }

// impl Scene for SceneInstant {}

// impl NodeTrait for SceneInstant {
//     fn initialize(&mut self, id: u32) {
//         self.node.initialize(id);
//         println!("initialize '{}'", self.to_string());
//     }

//     fn is_node_type(&self, n_type: NodeType) -> bool {
//         self.node.is_node_type(n_type)
//     }

//     fn is_nil(&self) -> bool {
//         self.node.is_nil()
//     }

//     fn id(&self) -> u32 {
//         self.node.id()
//     }

//     fn to_string(&self) -> String {
//         self.node.to_string()
//     }
// }

// impl NodeEvents for SceneInstant {
//     fn start_exit_transition(&self, _scm: &mut SceneManager) {}
//     fn end_enter_transition(&self, _scm: &mut SceneManager) {}
//     fn enter(&self, _scene_manager: &mut SceneManager, _scheduler: &mut Scheduler) {}
//     fn exit(&self, _scm: &mut SceneManager) {}

//     fn flush(&self, _flush: bool) {}
//     fn visit(&mut self, _context: &mut Context) {
//         println!("visit '{}'", self.to_string());
//     }
//     fn complete_visit(&mut self) {
//         println!("complete_visit '{}'", self.to_string());
//     }
// }

// impl TimingTarget for SceneInstant {
//     fn update(&self, _dt: f64) {}

//     fn pause(&mut self, paused: bool) {
//         self.node.timing.pause(paused);
//     }

//     fn paused(&self) -> bool {
//         self.node.timing.paused()
//     }

//     fn id(&self) -> u32 {
//         self.node.id()
//     }

//     fn tt_eq(&self, other: &TimingTarget) -> bool {
//         self.node.id() == other.id()
//     }
// }
