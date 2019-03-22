use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use ranger::{
    animation::motion::AngularMotion,
    geometry::point::Point,
    nodes::{
        node::{NodeGroup, NodeTrait, NodeType, Nodes, RNode, RONode},
        node_properties::NodeData,
    },
    rendering::{color::Palette, render_context::Context},
    world::World,
};

// A basic leaf node that renders a single triangle mesh.

pub struct TriangleNode {
    data: RefCell<NodeData>,

    // Hierarchy
    parent: RONode,

    angle_motion: RefCell<AngularMotion>,

    // Original model vertices
    vertices: Vec<Point>,
    // Transformed vertices
    bucket: RefCell<Vec<Point>>,
}

impl Drop for TriangleNode {
    fn drop(&mut self) {
        println!("Dropping: '{}'", self.data().borrow().node.name());
    }
}

impl TriangleNode {
    pub fn new(name: &str, parent: Option<RNode>, world: &mut World) -> RNode {
        let mut n = NodeData::new();
        n.node.set_name(name.to_string());
        n.node.set_type(NodeType::Node);
        n.node.set_id(world.gen_id());
        n.node.make_timing_target(true);

        let mut tn = Self {
            data: RefCell::new(n),
            parent: Rc::new(RefCell::new(parent)),
            angle_motion: RefCell::new(AngularMotion::new()),
            vertices: Vec::new(),
            bucket: RefCell::new(Vec::new()),
        };

        TriangleNode::build(&mut tn, world);

        let rc: Rc<RefCell<NodeTrait>> = Rc::new(RefCell::new(tn));

        NodeGroup::attach_parent(&rc);

        rc
    }

    fn build(node: &mut TriangleNode, _world: &mut World) {
        node.vertices.push(Point::from_xy(-0.5, 0.5));
        node.vertices.push(Point::from_xy(0.5, 0.5));
        node.vertices.push(Point::from_xy(0.0, -0.5));

        let mut b = node.bucket.borrow_mut();
        for _ in 0..node.vertices.len() {
            b.push(Point::new());
        }

        // The rotation rate needs to be twice the anchors to counteract/cancel
        // the anchor's rate.
        node.angle_motion.borrow_mut().set_step_value(10.0);
    }
}

impl NodeTrait for TriangleNode {
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
    // Rendering
    // --------------------------------------------------------
    fn draw(&self, context: &mut Context) {
        // Transform this node's vertices using the context
        if self.is_node_dirty() {
            context.transform(&self.vertices, &self.bucket);
            self.set_node_dirty(false);
        }

        context.set_draw_color(&Palette::LIME());
        context.render_triangle(&self.bucket);

        Nodes::draw_aabb(&self.bucket.borrow(), context);
    }

    fn interpolate(&self, interpolation: f64) {
        let value = self.angle_motion.borrow_mut().interpolate(interpolation);

        self.set_rotation_degrees(value);
    }

    // --------------------------------------------------------
    // Transformations
    // --------------------------------------------------------
    fn parent(&self) -> RONode {
        self.parent.clone()
    }

    fn set_parent(&self, parent: RNode) {
        self.parent.borrow_mut().replace(parent);
    }

    // --------------------------------------------------------
    // Timing target
    // --------------------------------------------------------
    fn update(&self, dt: f64) {
        self.angle_motion.borrow_mut().update(dt);
    }
}
