use std::any::Any;

use std::cell::RefCell;
use std::rc::Rc;

use geometry::point::Point;
use nodes::{
    node::{NodeGroup, NodeTrait, NodeType, RNode, RONode},
    node_properties::NodeData,
};
use rendering::{color::Palette, render_context::Context};
use world::World;

// A basic leaf node that renders a "+"

pub struct CrossNode {
    data: RefCell<NodeData>,

    // Hierarchy
    parent: RONode,

    // Original model vertices
    vertices: Vec<Point>,
    // Transformed vertices
    bucket: RefCell<Vec<Point>>,
}

impl Drop for CrossNode {
    fn drop(&mut self) {
        println!("Dropping: '{}'", self.data().borrow().node.name());
    }
}

impl CrossNode {
    pub fn new(name: &str, parent: Option<RNode>, world: &mut World) -> RNode {
        let mut n = NodeData::new();
        n.node.set_name(name.to_string());
        n.node.set_type(NodeType::Node);
        n.node.set_id(world.gen_id());

        let mut tn = Self {
            data: RefCell::new(n),
            parent: Rc::new(RefCell::new(parent)),
            vertices: Vec::new(),
            bucket: RefCell::new(Vec::new()),
        };

        CrossNode::construct(&mut tn, world);

        let rc: Rc<RefCell<NodeTrait>> = Rc::new(RefCell::new(tn));

        NodeGroup::attach_parent(&rc);

        rc
    }

    fn construct(node: &mut CrossNode, _world: &mut World) {
        // Horizontal
        node.vertices.push(Point::from_xy(-0.5, 0.0));
        node.vertices.push(Point::from_xy(0.5, 0.0));

        // Vertical
        node.vertices.push(Point::from_xy(0.0, -0.5));
        node.vertices.push(Point::from_xy(0.0, 0.5));

        let mut b = node.bucket.borrow_mut();
        for _ in 0..node.vertices.len() {
            b.push(Point::new());
        }
    }
}

impl NodeTrait for CrossNode {
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

        context.set_draw_color(&Palette::WHITE(255));

        context.render_lines(&self.bucket);

        // let b = self.bucket.borrow();
        // context.render_line(b[0].x, b[0].y, b[1].x, b[1].y);
        // context.render_line(b[2].x, b[2].y, b[3].x, b[3].y);
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
}
