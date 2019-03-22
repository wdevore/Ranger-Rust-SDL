use std::any::Any;

use std::cell::RefCell;
use std::rc::Rc;

use math::affine_transform::AffineTransform;
use nodes::{
    node::{NodeGroup, NodeTrait, NodeType, RNode, RONode},
    node_properties::NodeData,
};
use rendering::render_context::Context;
use world::World;

pub struct TransformFilter {
    data: RefCell<NodeData>,

    children: Option<RefCell<Vec<RNode>>>,

    // Hierarchy
    parent: RONode,

    // Filters
    exclude_translation: bool,
    exclude_rotation: bool,
    exclude_scale: bool,
}

impl Drop for TransformFilter {
    fn drop(&mut self) {
        println!("Dropping: '{}'", self.data().borrow().node.name());
    }
}

impl TransformFilter {
    pub fn new(name: &str, parent: Option<RNode>, world: &mut World) -> RNode {
        let mut n = NodeData::new();
        n.node.set_name(name.to_string());
        n.node.set_type(NodeType::Node);
        n.node.set_id(world.gen_id());

        let an = Self {
            data: RefCell::new(n),
            parent: Rc::new(RefCell::new(parent)),
            children: Some(RefCell::new(Vec::new())),
            // By default most nodes will want to "inherit" the parent translation
            // thus we don't want to exclude it.
            exclude_translation: false,
            // Most nodes will NOT want any rotation or scale from the parent
            // thus we wan't to exclude them.
            exclude_rotation: true,
            exclude_scale: true,
        };

        let rc: Rc<RefCell<NodeTrait>> = Rc::new(RefCell::new(an));

        TransformFilter::construct(&rc, world);

        rc
    }

    fn construct(node: &RNode, _world: &mut World) {
        NodeGroup::attach_parent(node);
    }

    pub fn exclude_translation(&mut self, exclude: bool) {
        self.exclude_translation = exclude;
    }

    pub fn exclude_rotation(&mut self, exclude: bool) {
        self.exclude_rotation = exclude;
    }

    pub fn exclude_scale(&mut self, exclude: bool) {
        self.exclude_scale = exclude;
    }
}

impl NodeTrait for TransformFilter {
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
    // Rendering: visiting and drawing
    // --------------------------------------------------------
    fn visit(&self, context: &mut Context, interpolation: f64) {
        context.save();

        if let Some(children) = self.get_children() {
            // Visit any children contained by this node.
            for child in children.borrow().iter() {
                context.save();

                // TODO Figure out a way to cache `inv` and `components`
                let ron_parent = self.parent();
                let ro_parent = ron_parent.borrow();

                // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                // Filtering
                // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                match ro_parent.as_ref() {
                    Some(ref_parent) => {
                        let parent = ref_parent.borrow();
                        let data = parent.data().borrow();
                        let inv = data.transform.get_inverse();
                        // println!("Filter on for: {}", self.name());
                        // println!("filter: transform: {:?}", data.transform.get_transform());
                        // println!("filter: applying inv of: {}", parent.name());
                        // println!("filter: inverse: {:?}", inv);

                        // Remove any transform from the parent by applying inverse of parent.
                        context.apply(&inv);

                        let mut components = AffineTransform::new();

                        // Now re-introduce just specific components from the parent
                        data.transform.calc_filtered_transform(
                            self.exclude_translation,
                            self.exclude_rotation,
                            self.exclude_scale,
                            &mut components,
                        );

                        // println!("filter: components: {:?}", components);
                        context.apply(&components);
                    }
                    _ => {
                        dbg!("Parent NOT FOUND");
                        return;
                    }
                }

                child.borrow().visit(context, interpolation);

                context.restore();
            }
        }

        context.restore();
    }

    // --------------------------------------------------------
    // Grouping
    // --------------------------------------------------------
    fn get_children(&self) -> &Option<RefCell<Vec<RNode>>> {
        &self.children
    }
}
