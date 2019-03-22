use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use ranger::{
    // animation::motion::AngularMotion,
    geometry::point::Point,
    nodes::{
        node::{NodeGroup, NodeTrait, NodeType, Nodes, RNode, RONode},
        node_properties::NodeData,
        scenes::scene_manager::{GlobalSceneData, IOEvent, IOEventData, SceneManager},
    },
    rendering::{color::Palette, render_context::Context},
    world::World,
};

// A rectangle that has a triangle child.
// A rotation has an angular velocity measured as radians per frame.
// If there are 2 updates per frame an each update is 5 radians then we
// have 10radians/frame. The interpolation is multplied into
// the angular velocity.

pub struct RectangleNode {
    data: RefCell<NodeData>,

    // Hierarchy
    parent: RONode,

    // Original vertices
    vertices: Vec<Point>,
    // Transformed vertices
    bucket: RefCell<Vec<Point>>,

    color: Palette,
}

impl Drop for RectangleNode {
    fn drop(&mut self) {
        println!("Dropping: '{}'", self.data().borrow().node.name());
    }
}

impl RectangleNode {
    pub fn new(name: &str, parent: Option<RNode>, world: &mut World) -> RNode {
        let mut n = NodeData::new();
        n.node.set_name(name.to_string());
        n.node.set_type(NodeType::Node);
        n.node.set_id(world.gen_id());
        n.node.make_timing_target(true);

        let mut tn = Self {
            data: RefCell::new(n),
            parent: Rc::new(RefCell::new(parent)),
            vertices: Vec::new(),
            bucket: RefCell::new(Vec::new()),
            color: Palette::DEFAULT(),
        };

        RectangleNode::build(&mut tn, world);

        let rc: Rc<RefCell<NodeTrait>> = Rc::new(RefCell::new(tn));

        NodeGroup::attach_parent(&rc);

        rc
    }

    fn build(rectangle: &mut RectangleNode, _world: &mut World) {
        // Shared edges
        rectangle.vertices.push(Point::from_xy(-0.5, 0.5));
        rectangle.vertices.push(Point::from_xy(0.5, 0.5));
        rectangle.vertices.push(Point::from_xy(0.5, -0.5));
        rectangle.vertices.push(Point::from_xy(-0.5, -0.5));

        // Non-shared edges
        // self.vertices.push(Point::from_xy(-0.5, 0.5));
        // self.vertices.push(Point::from_xy(0.5, -0.5));
        // self.vertices.push(Point::from_xy(-0.5, -0.5));
        // self.vertices.push(Point::from_xy(-0.5, 0.5));
        // self.vertices.push(Point::from_xy(0.5, 0.5));
        // self.vertices.push(Point::from_xy(0.5, -0.5));

        let mut b = rectangle.bucket.borrow_mut();
        for _ in 0..rectangle.vertices.len() {
            b.push(Point::new());
        }
    }

    pub fn set_color(&mut self, color: Palette) {
        self.color = color;
    }
}

impl NodeTrait for RectangleNode {
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

        context.set_draw_color(&self.color);
        context.render_rectangle(&self.bucket);

        // Draw AABB box for debugging
        Nodes::draw_aabb(&self.bucket.borrow(), context);
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
    // IO Events
    // --------------------------------------------------------
    fn io_event(&mut self, io_event: &IOEventData) {
        match io_event.event {
            IOEvent::MOUSE => {
                if let Some(_children) = self.get_children() {}
                println!(
                    "{}: {}, {}",
                    self.name(),
                    io_event.coord.0,
                    io_event.coord.1
                );
            }
            _ => (),
        }
    }

    // --------------------------------------------------------
    // Life cycle events
    // --------------------------------------------------------
    fn enter(&self, scene_manager: &SceneManager) {
        println!("enter '{}'", self.to_string());
        // scene_manager.register_for_io_events(self.data().borrow().node.id());
        // data.register_for_io_events(self.parent(), self.data().borrow().node.id());
    }

    fn exit(&self, _data: &mut GlobalSceneData) {
        println!("exit '{}'", self.to_string());
        // data.unregister_for_io_events(self.parent(), self.data().borrow().node.id());
    }
}
