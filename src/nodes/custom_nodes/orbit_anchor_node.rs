use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use animation::motion::AngularMotion;
use nodes::{
    node::{NodeGroup, NodeTrait, NodeType, RNode, RONode},
    node_properties::NodeData,
};
use world::World;

// An anchor node is a headless node used for various types of "associated"
// transformations.
// It will modify Context.current prior to any children being visited.
//
// The anchor needs only the translation from the parent in order to sync
// its position. It will then add a rotation which is then passed to the
// children.

pub struct OrbitAnchorNode {
    data: RefCell<NodeData>,

    children: Option<RefCell<Vec<RNode>>>,

    // Hierarchy
    parent: RONode,

    angle_motion: RefCell<AngularMotion>,
}

impl Drop for OrbitAnchorNode {
    fn drop(&mut self) {
        println!("Dropping: '{}'", self.data().borrow().node.name());
    }
}

impl OrbitAnchorNode {
    pub fn new(name: &str, parent: Option<RNode>, world: &mut World) -> RNode {
        let mut n = NodeData::new();
        n.node.set_name(name.to_string());
        n.node.set_type(NodeType::Node);
        n.node.set_id(world.gen_id());
        n.node.make_timing_target(true);

        let an = Self {
            data: RefCell::new(n),
            parent: Rc::new(RefCell::new(parent)),
            children: Some(RefCell::new(Vec::new())),
            angle_motion: RefCell::new(AngularMotion::new()),
        };

        OrbitAnchorNode::construct(&an, world);

        let rc: Rc<RefCell<NodeTrait>> = Rc::new(RefCell::new(an));

        NodeGroup::attach_parent(&rc);

        rc
    }

    fn construct(node: &OrbitAnchorNode, _world: &mut World) {
        node.angle_motion.borrow_mut().set_step_value(-5.0);
        // node.set_rotation_degrees(-15.0);
    }
}

impl NodeTrait for OrbitAnchorNode {
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
    // Transformations
    // --------------------------------------------------------
    fn parent(&self) -> RONode {
        self.parent.clone()
    }

    // --------------------------------------------------------
    // Timing target
    // --------------------------------------------------------
    fn update(&self, dt: f64) {
        self.angle_motion.borrow_mut().update(dt);
    }

    // --------------------------------------------------------
    // Rendering: visiting and drawing
    // --------------------------------------------------------
    fn interpolate(&self, interpolation: f64) {
        // println!(
        //     "A mo: {}, pol: {}",
        //     self.angle_motion.borrow().to_string(),
        //     interpolation
        // );
        let value = self.angle_motion.borrow_mut().interpolate(interpolation);
        // println!(
        //     "B value: {}, mo: {}, pol: {}",
        //     value,
        //     self.angle_motion.borrow().to_string(),
        //     interpolation
        // );
        self.set_rotation_degrees(value);
    }

    // fn filter_required(&self) -> bool {
    //     true
    // }

    // --------------------------------------------------------
    // Grouping
    // --------------------------------------------------------
    fn get_children(&self) -> &Option<RefCell<Vec<RNode>>> {
        &self.children
    }
}
