use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use geometry::point::Point;
use nodes::{
    node::{NodeGroup, NodeTrait, NodeType, RNode, RONode},
    node_properties::NodeData,
};
use rendering::{color::Palette, render_context::Context, vector_font::VectorFont};
use world::World;

pub struct VectorTextNode {
    data: RefCell<NodeData>,

    // Hierarchy
    parent: RONode,

    text: RefCell<String>,
    font: VectorFont,

    // Original model vertices
    vertices: RefCell<Vec<Point>>,

    // Transformed vertices
    bucket: RefCell<Vec<Point>>,
}

impl Drop for VectorTextNode {
    fn drop(&mut self) {
        println!("Dropping: '{}'", self.data().borrow().node.name());
    }
}

impl VectorTextNode {
    pub fn new(name: &str, parent: Option<RNode>, world: &mut World) -> RNode {
        let mut nd = NodeData::new();
        nd.node.set_name(name.to_string());
        nd.node.set_type(NodeType::Node);
        nd.node.set_id(world.gen_id());

        let n = Self {
            data: RefCell::new(nd),
            parent: Rc::new(RefCell::new(parent)),
            vertices: RefCell::new(Vec::new()),
            bucket: RefCell::new(Vec::new()),
            text: RefCell::new(String::from("")),
            font: VectorFont::new(),
        };

        let rc: Rc<RefCell<NodeTrait>> = Rc::new(RefCell::new(n));

        NodeGroup::attach_parent(&rc);

        rc
    }

    pub fn set_text(&self, text: &String) {
        self.text.borrow_mut().replace_range(.., text);

        // Use glyph properties to adjust char location.
        let hoff = self.font.get_horz_offset();
        let mut xpos = 0.0;
        let mut p = Point::new();

        // Rebuild vertex buffer to match text.
        let txt = self.text.borrow();
        let mut v = self.vertices.borrow_mut();
        let mut b = self.bucket.borrow_mut();

        for c in txt.chars() {
            let lines = self.font.get_glyph(c).get_lines();

            for l in lines.iter() {
                p.set_xy(l.x + xpos, l.y);
                v.push(p);
                b.push(Point::new());
            }

            xpos += hoff;
        }

        self.set_node_dirty(true);
    }
}

impl NodeTrait for VectorTextNode {
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
        if self.is_node_dirty() {
            let verts = self.vertices.borrow();
            context.transform(&verts, &self.bucket);
            self.set_node_dirty(false);
        }

        context.set_draw_color(&Palette::WHITE(127));

        context.render_lines(&self.bucket);
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
