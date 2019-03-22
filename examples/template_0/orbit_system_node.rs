use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use ranger::{
    animation::motion::AngularMotion,
    geometry::point::Point,
    nodes::{
        custom_nodes::orbit_anchor_node::OrbitAnchorNode,
        node::{NodeGroup, NodeTrait, NodeType, Nodes, RNode, RONode},
        node_properties::NodeData,
        transform_filter::TransformFilter,
    },
    rendering::{color::Palette, render_context::Context},
    world::World,
};

use template_0::triangle_node::TriangleNode;

// A rectangle that has a triangle child.
// A rotation has an angular velocity measured as radians per frame.
// If there are 2 updates per frame an each update is 5 radians then we
// have 10radians/frame. The interpolation is multplied into
// the angular velocity.

pub struct OrbitSystemNode {
    data: RefCell<NodeData>,

    // Hierarchy
    parent: RONode,

    children: Option<RefCell<Vec<RNode>>>,

    angle_motion: RefCell<AngularMotion>,

    // Original vertices
    vertices: Vec<Point>,
    // Transformed vertices
    bucket: RefCell<Vec<Point>>,

    color: Palette,
}

impl Drop for OrbitSystemNode {
    fn drop(&mut self) {
        println!("Dropping: '{}'", self.data().borrow().node.name());
    }
}

impl OrbitSystemNode {
    pub fn new(name: &str, parent: Option<RNode>, world: &mut World) -> RNode {
        let mut n = NodeData::new();
        n.node.set_name(name.to_string());
        n.node.set_type(NodeType::Node);
        n.node.set_id(world.gen_id());
        n.node.make_timing_target(true);

        let mut tn = Self {
            data: RefCell::new(n),
            parent: Rc::new(RefCell::new(parent)),
            children: Some(RefCell::new(Vec::new())),
            angle_motion: RefCell::new(AngularMotion::new()),
            vertices: Vec::new(),
            bucket: RefCell::new(Vec::new()),
            color: Palette::DEFAULT(),
        };

        OrbitSystemNode::build(&mut tn, world);

        let rc: Rc<RefCell<NodeTrait>> = Rc::new(RefCell::new(tn));

        NodeGroup::attach_parent(&rc);

        OrbitSystemNode::build_heirarchy(&rc, world);

        rc
    }

    fn build_heirarchy(node: &RNode, world: &mut World) {
        let filter = TransformFilter::new("OSN_FilterNode", Some(node.clone()), world);
        {
            // We have to scope this code because OrbitSystemNode::new will
            // want to borrow_mut the filter too.
            let mut bfilt = filter.borrow_mut();
            if let Some(n) = bfilt.as_any_mut().downcast_mut::<TransformFilter>() {
                n.exclude_translation(false);
            }
        }

        let orbit_anchor = OrbitAnchorNode::new("OrbitAnchorNode", Some(filter.clone()), world);

        let filter = TransformFilter::new("TRI_FilterNode", Some(orbit_anchor.clone()), world);
        {
            // We have to scope this code because OrbitSystemNode::new will
            // want to borrow_mut the filter too.
            let mut bfilt = filter.borrow_mut();
            if let Some(n) = bfilt.as_any_mut().downcast_mut::<TransformFilter>() {
                // We want to rotation from the parent but nothing else.
                // Remember, the default behaviour is to exclude everything.
                n.exclude_rotation(false);
            }
        }

        let tri = TriangleNode::new("TriNode", Some(filter), world);
        tri.borrow().set_position(200.0, 0.0);
        tri.borrow().set_scale(50.0);
        // tri.borrow().set_position(2.0, 0.0);
        // tri.borrow().set_scale(1.0);
    }

    fn build(orbit: &mut OrbitSystemNode, _world: &mut World) {
        // Shared edges
        orbit.vertices.push(Point::from_xy(-0.5, 0.5));
        orbit.vertices.push(Point::from_xy(0.5, 0.5));
        orbit.vertices.push(Point::from_xy(0.5, -0.5));
        orbit.vertices.push(Point::from_xy(-0.5, -0.5));

        let mut b = orbit.bucket.borrow_mut();
        for _ in 0..orbit.vertices.len() {
            b.push(Point::new());
        }

        orbit.angle_motion.borrow_mut().set_step_value(2.0);
    }

    pub fn set_color(&mut self, color: Palette) {
        self.color = color;
    }
}

impl NodeTrait for OrbitSystemNode {
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
            // self.data().borrow_mut().node.set_dirty(false);
        }

        context.set_draw_color(&self.color);
        context.render_rectangle(&self.bucket);

        // Draw AABB box for debugging
        Nodes::draw_aabb(&self.bucket.borrow(), context);
    }

    // fn filter_required(&self) -> bool {
    //     true
    // }

    // fn include_translation(&self) -> bool {
    //     true
    // }

    // fn include_rotation(&self) -> bool {
    //     true
    // }

    fn interpolate(&self, interpolation: f64) {
        let value = self.angle_motion.borrow_mut().interpolate(interpolation);
        // println!(
        //     "value: {}, mo: {}, pol: {}",
        //     value,
        //     self.angle_motion.borrow().to_string(),
        //     interpolation
        // );
        self.set_rotation_degrees(value);
    }

    // --------------------------------------------------------
    // Transformations
    // --------------------------------------------------------
    // fn ripple_transform_dirty(&self) {
    //     self.set_node_dirty();
    //     // Because this node has children we need to override set_dirty and call ripple.
    //     self.ripple_transform_dirty();
    // }

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

    // --------------------------------------------------------
    // Grouping
    // --------------------------------------------------------
    fn get_children(&self) -> &Option<RefCell<Vec<RNode>>> {
        &self.children
    }
}
