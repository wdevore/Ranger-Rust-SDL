use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use engine::{timing::scheduler::Scheduler, timing::scheduler::TimingPriority};
use geometry::{aabb::AABBox, point::Point};
use math::affine_transform::AffineTransform;
use nodes::{
    node_nil::NodeNil,
    node_properties::NodeData,
    scenes::scene_manager::{GlobalSceneData, IOEventData, SceneActions, SceneManager},
};
use rendering::{
    color::Palette,
    render_context::{Context, RenderStyle},
};

// The node system is similar to Inventor and/or Cocos2D:
// http://www-evasion.imag.fr/~Francois.Faure/doc/inventorMentor/sgi_html/ch09.html

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeType {
    Nil,
    Node,
    Group,
    Scene,
    SceneTransition,
}

pub type RNode = Rc<RefCell<NodeTrait>>;

// RONode exists because a node can be a parent to
// 1 or more children which means the parent can be referenced
// more than once.
// It needs to be a RefCell because the parent could change at
// any point in time.
pub type RONode = Rc<RefCell<Option<RNode>>>;

pub type OChildren = Option<RefCell<Vec<RNode>>>;

pub struct Nodes;

impl Nodes {
    pub fn draw_aabb(vertices: &Vec<Point>, context: &mut Context) {
        let mut aabb = AABBox::new();
        aabb.set_from_vertices(vertices);
        context.set_draw_color(&Palette::RED());
        context.render_aabb_rectangle(&aabb, RenderStyle::OUTLINE);
    }

    // Map device/mouse/pixel/window space to view-space.
    pub fn map_device_to_view(dx: i32, dy: i32, context: &mut Context) -> (f64, f64) {
        let inv = context.get_view_space().inverse();
        let device = Point::from_xy(dx as f64, dy as f64);
        let mut view = Point::new();
        AffineTransform::transform_to_point(&device, &mut view, &inv);
        (view.x, view.y)
    }

    // Note: world is the identity matrix so view is actually used
    pub fn map_device_to_node(dx: i32, dy: i32, node: RNode, context: &mut Context) -> (f64, f64) {
        let dev_map = Nodes::map_device_to_view(dx, dy, context);
        let device = Point::from_tup(dev_map);

        let mut aft = AffineTransform::new();
        node.borrow().node_to_world(&mut aft);
        aft.invert();
        let mut inv = context.get_view_space().inverse();
        inv.multiply(&aft);

        let mut node = Point::new();
        AffineTransform::transform_to_point(&device, &mut node, &inv);
        (node.x, node.y)
    }

    pub fn id_equal_node(id: usize, node: &RNode) -> bool {
        let n = node.borrow();
        if id == n.data().borrow().node.id() {
            return true;
        }

        false
    }

    pub fn find_node(id: usize, node: &RNode) -> RNode {
        if let Some(children) = node.borrow().get_children() {
            let ng = Nodes::sub_find_node(id, children);
            if !ng.borrow().is_nil() {
                return ng.clone();
            }
        } else {
            if Nodes::id_equal_node(id, node) {
                return node.clone();
            }
        }

        NodeNil::new()
    }

    fn sub_find_node(id: usize, children: &RefCell<Vec<RNode>>) -> RNode {
        for child in children.borrow().iter() {
            if let Some(sub_children) = child.borrow().get_children() {
                let ng = Nodes::sub_find_node(id, sub_children);
                if !ng.borrow().is_nil() {
                    return ng.clone();
                }
            } else {
                if Nodes::id_equal_node(id, child) {
                    return child.clone();
                }
            }
        }

        NodeNil::new()
    }

    pub fn register_timing_targets(node: &RNode, sch: &mut Scheduler) {
        let no = node.borrow();
        if no.data().borrow().node.canbe_timing_target() {
            sch.register_timing_target(node.clone());
        }

        if let Some(children) = node.borrow().get_children() {
            Nodes::sub_register_timing_targets(children, sch);
        }
    }

    fn sub_register_timing_targets(children: &RefCell<Vec<RNode>>, sch: &mut Scheduler) {
        for child in children.borrow().iter() {
            if let Some(sub_children) = child.borrow().get_children() {
                let chi = child.borrow();
                if chi.data().borrow().node.canbe_timing_target() {
                    sch.register_timing_target(child.clone());
                }
                Nodes::sub_register_timing_targets(sub_children, sch);
            } else {
                let chi = child.borrow();
                if chi.data().borrow().node.canbe_timing_target() {
                    sch.register_timing_target(child.clone());
                }
            }
        }
    }

    pub fn unregister_timing_targets_by_id(node_id: usize, sch: &mut Scheduler) {
        // Use scheduler to unregister based on id.
        sch.unschedule_timing_target_by_id(node_id);
    }

    pub fn unregister_timing_targets(node: &RNode, sch: &mut Scheduler) {
        let no = node.borrow();
        if no.data().borrow().node.canbe_timing_target() {
            sch.unschedule_timing_target(node.clone());
        }

        if let Some(children) = node.borrow().get_children() {
            Nodes::sub_unregister_timing_targets(children, sch);
        }
    }

    fn sub_unregister_timing_targets(children: &RefCell<Vec<RNode>>, sch: &mut Scheduler) {
        for child in children.borrow().iter() {
            if let Some(sub_children) = child.borrow().get_children() {
                let chi = child.borrow();
                if chi.data().borrow().node.canbe_timing_target() {
                    sch.unschedule_timing_target(child.clone());
                }
                Nodes::sub_unregister_timing_targets(sub_children, sch);
            } else {
                let chi = child.borrow();
                if chi.data().borrow().node.canbe_timing_target() {
                    sch.unschedule_timing_target(child.clone());
                }
            }
        }
    }

    pub fn print_tree(tree: &RNode) {
        println!("---------- Tree ---------------");
        Nodes::print_branch(0, tree.borrow().name());
        if let Some(children) = tree.borrow().get_children() {
            Nodes::print_sub_tree(children, 1);
        }
        println!("-------------------------------");
    }

    fn print_sub_tree(children: &RefCell<Vec<RNode>>, level: usize) {
        for child in children.borrow().iter() {
            if let Some(sub_children) = child.borrow().get_children() {
                Nodes::print_branch(level, child.borrow().name());
                Nodes::print_sub_tree(sub_children, level + 1);
            } else {
                Nodes::print_branch(level, child.borrow().name());
            }
        }
    }

    fn print_branch(level: usize, name: String) {
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", name);
    }
}

pub struct NodeGroup;

impl NodeGroup {
    // Take given node, which should have a valid parent assigned,
    // and attach it as a child of the assigned parent.
    pub fn attach_parent(node: &RNode) {
        let bnode = node.borrow();
        let pbnode = bnode.parent();
        let bbnode = pbnode.borrow();
        if let Some(parent) = bbnode.as_ref() {
            // println!(
            //     "NodeGroup: Making '{}' child of: '{}'",
            //     bnode.name(),
            //     parent.borrow().name()
            // );
            parent.borrow().add_child(node.clone());
        }
    }
}

pub trait NodeTrait {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // --------------------------------------------------------
    // PartialEq and container delegates
    // --------------------------------------------------------
    // Typically Scheduler uses this indirectly via PartialEq checks.
    fn tt_eq(&self, other: &NodeTrait) -> bool {
        self.id() == other.id()
    }

    // --------------------------------------------------------
    // Node properties
    // --------------------------------------------------------
    fn data(&self) -> &RefCell<NodeData>;

    fn id(&self) -> usize {
        self.data().borrow().node.id()
    }
    fn set_id(&self, id: usize) {
        self.data().borrow_mut().node.set_id(id);
    }

    fn name(&self) -> String {
        self.data().borrow().node.name().to_string()
    }

    fn set_name(&self, _name: String) {}

    fn get_node_type(&self) -> NodeType {
        self.data().borrow().node.node_type()
    }

    fn is_node_type(&self, n_type: NodeType) -> bool {
        self.data().borrow().node.node_type() == n_type
    }

    fn is_nil(&self) -> bool {
        self.get_node_type() == NodeType::Nil
    }

    fn is_visible(&self) -> bool {
        self.data().borrow().node.visible()
    }

    // --------------------------------------------------------
    // Rendering: visiting, modification and drawing
    // --------------------------------------------------------
    // visit() may modify Context's current transform prior to
    // draw() being called.
    // `interpolation` should only be applied to deltas, for example,
    // velocities/direction-vectors or angular velocities.
    // Update() will modify velocities which the interpolation would act against.
    fn visit(&self, context: &mut Context, interpolation: f64) {
        // println!(
        //     "visiting --------------------- {}, stack: {}",
        //     self.name(),
        //     context.top_index()
        // );
        if !self.is_visible() {
            return;
        }

        // println!("Stack as saved at: ({})", context.top_index());
        context.save();
        // context.print_stack(10);

        // Because position and angles are dependent
        // on lerping we perform interpolation first.
        self.interpolate(interpolation);

        // We need to scope the data() here because the draw() method will
        // also want to borrow data().
        {
            let mut data = self.data().borrow_mut();
            let aft: &AffineTransform;
            if data.node.is_dirty() {
                aft = data.transform.calc_transform();
            } else {
                aft = data.transform.get_transform();
            }

            context.apply(aft);
            // println!("context.applied : {:?}", context.current());
            // context.print_stack(10);
        }

        if let Some(children) = self.get_children() {
            // println!("Drawing parent '{}'", self.name());
            self.draw(context); // Draw parent

            // Visit any children contained by this node.
            for child in children.borrow().iter() {
                // println!(
                //     "visiting child '{}' of '{}'",
                //     child.borrow().name(),
                //     self.name()
                // );
                child.borrow().visit(context, interpolation);
                // println!("Done visiting child '{}'", child.borrow().name());
            }
        } else {
            // Just draw node
            // println!("Drawing leaf '{}'", self.name());
            self.draw(context);
        }

        context.restore();
        // context.print_stack(10);

        // Do any post rendering. Note this is really for debugging purposes only.
        // self.device_visit(context);
    }

    // Drawing here is generally for debugging and in device-space
    // fn device_visit(&self, &mut Context) {} // DEPRECATED

    // visit() calls this method
    fn draw(&self, &mut Context) {
        // println!("{} has no rendering.", self.name());
    }

    fn interpolate(&self, _interpolation: f64) {}

    // --------------------------------------------------------
    // Grouping
    // --------------------------------------------------------
    fn get_children(&self) -> &OChildren {
        &None
    }

    fn add_child(&self, node: RNode) {
        if let Some(children) = self.get_children() {
            children.borrow_mut().push(node.clone());
        }
    }

    // --------------------------------------------------------
    // Transformations
    // --------------------------------------------------------
    fn set_position(&self, x: f64, y: f64) {
        self.data().borrow_mut().transform.set_position(x, y);
        self.ripple_node_dirty(true);
    }

    fn set_rotation_degrees(&self, degrees: f64) {
        self.data()
            .borrow_mut()
            .transform
            .set_rotation_degrees(degrees);
        self.ripple_node_dirty(true);
    }

    fn set_scale(&self, s: f64) {
        self.data().borrow_mut().transform.set_scale(s);
        self.ripple_node_dirty(true);
    }

    fn set_nonuniform_scale(&self, sx: f64, sy: f64) {
        self.data()
            .borrow_mut()
            .transform
            .set_nonuniform_scale(sx, sy);
        self.ripple_node_dirty(true);
    }

    fn parent(&self) -> RONode {
        Rc::new(RefCell::new(None))
    }
    fn set_parent(&self, _parent: RNode) {}

    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // Dirty state
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    fn set_node_dirty(&self, dirty: bool) {
        self.data().borrow_mut().node.set_dirty(dirty);
    }

    fn is_node_dirty(&self) -> bool {
        self.data().borrow().node.is_dirty()
    }

    fn ripple_node_dirty(&self, dirty: bool) {
        if let Some(children) = self.get_children() {
            for child in children.borrow().iter() {
                child.borrow().ripple_node_dirty(dirty);
            }
        }
        self.data().borrow_mut().node.set_dirty(dirty);
    }

    // --------------------------------------------------------
    // IO Events
    // --------------------------------------------------------
    fn io_event(&mut self, io_event: &IOEventData) {
        if let Some(children) = self.get_children() {
            for child in children.borrow().iter() {
                println!("io_event '{}' of '{}'", child.borrow().name(), self.name());
                child.borrow_mut().io_event(io_event);
            }
        }
    }

    // --------------------------------------------------------
    // Mappings
    // --------------------------------------------------------
    // TODO add psuedo_root: RNode
    fn node_to_world(&self, world: &mut AffineTransform) {
        let data = self.data();
        let mut mdata = data.borrow_mut();

        // A composite transform to accumulate the parent transforms.
        let compo_aft = mdata.transform.calc_transform(); // Start with this child

        // Use a copy
        let mut comp = AffineTransform::from_transform(compo_aft);

        // Iterate "upwards" starting with this child's parent.
        let mut ro_parent = self.parent();
        if ro_parent.borrow().is_none() {
            *world = comp;
            return;
        }

        let mut pre = AffineTransform::new();

        'climb: loop {
            match ro_parent.borrow().as_ref() {
                Some(parent) => {
                    let part = parent.borrow();
                    let mut pdata = part.data().borrow_mut();
                    let parent_aft = pdata.transform.calc_transform();

                    // Because we are iterating upwards we need to pre-multiply each
                    // child. Ex: [child] x [parent_aft]
                    //
                    // ----------------------------------------------------------
                    //           [compo] x [parent_aft] = pre
                    //                   |
                    //                   v
                    //                 [compo] x [parent_aft]
                    //                         |
                    //                         v
                    //                      [compo] x [parent_aft...]
                    //
                    // This is a pre-multiply order
                    // [child] x [parent ofchild] x [parent of parent of child]...
                    //
                    // In other words the child is mutiplied "into" the parent.

                    AffineTransform::multiply_mn(&comp, parent_aft, &mut pre);
                    comp = pre;
                }
                _ => break 'climb,
            }

            // if child == psuedo_root {
            //     break;
            // }

            // Move upwards to next parent
            let ron: RONode;
            {
                let o_parent = ro_parent.borrow(); // Option
                let rn = o_parent.as_ref().unwrap(); // RNode
                ron = rn.borrow().parent(); // RONode
            }
            ro_parent = ron;
        }

        // Copy to output: world
        *world = comp;
    }

    // --------------------------------------------------------
    // Life cycle events
    // --------------------------------------------------------
    fn start_exit_transition(&self, &mut GlobalSceneData) {}
    fn end_enter_transition(&self) {}

    // A leaf node will override this.
    fn enter(&self, _scm: &SceneManager) {}
    fn sub_enter(&self, scm: &SceneManager, children: &RefCell<Vec<RNode>>) {
        for child in children.borrow().iter() {
            if let Some(sub_children) = child.borrow().get_children() {
                child.borrow().enter(scm);
                self.sub_enter(scm, sub_children);
            } else {
                child.borrow().enter(scm);
            }
        }
    }

    // A leaf node will override this.
    fn exit(&self, data: &mut GlobalSceneData) {
        if let Some(children) = self.get_children() {
            self.sub_exit(data, children);
        }
    }
    fn sub_exit(&self, data: &mut GlobalSceneData, children: &RefCell<Vec<RNode>>) {
        for child in children.borrow().iter() {
            if let Some(sub_children) = child.borrow().get_children() {
                child.borrow().exit(data);
                self.sub_exit(data, sub_children);
            } else {
                child.borrow().exit(data);
            }
        }
    }

    fn flush(&self, flush: bool) {
        if let Some(children) = self.get_children() {
            // println!(
            //     "Flushing children of {} ({})",
            //     self.name(),
            //     children.borrow().len()
            // );
            self.sub_flush(flush, children);
        }
    }

    fn sub_flush(&self, flush: bool, children: &RefCell<Vec<RNode>>) {
        for child in children.borrow().iter() {
            if let Some(sub_children) = child.borrow().get_children() {
                // let parent = child.borrow().parent();
                // if let Some(part) = parent.borrow().as_ref() {
                //     data.unregister_for_io_events(*part, child.borrow().data().borrow().node.id());
                // }
                child.borrow().flush(flush);
                self.sub_flush(flush, sub_children);
            } else {
                // let parent = child.borrow().parent();
                // if let Some(part) = parent.borrow().as_ref() {
                //     data.unregister_for_io_events(*part, child.borrow().data().borrow().node.id());
                // }
                child.borrow().flush(flush);
            }
        }

        children.borrow_mut().clear();
    }

    // --------------------------------------------------------
    // Timing target
    // --------------------------------------------------------
    fn update(&self, _dt: f64) {}

    fn pause(&self, paused: bool) {
        self.data().borrow_mut().timing.pause(paused);
    }
    fn paused(&self) -> bool {
        self.data().borrow().timing.paused()
    }

    fn ripple_pause(&self, paused: bool) {
        println!("Ripple pausing: {} for '{}'", paused, self.name());
        if let Some(children) = self.get_children() {
            println!("Pausing children of {}", self.name());
            self.sub_ripple_pause(paused, children);
        }
    }

    fn sub_ripple_pause(&self, paused: bool, children: &RefCell<Vec<RNode>>) {
        // let mut sub_name = String::from("__");
        for child in children.borrow().iter() {
            if let Some(sub_children) = child.borrow().get_children() {
                child.borrow().pause(paused);
                // sub_name = child.borrow().name();
                // println!("recursing sub pause children: {}", sub_name);
                self.sub_ripple_pause(paused, sub_children);
            } else {
                child.borrow().pause(paused);
            }
        }
        // println!("Pause bubbling up from: {}", sub_name);
    }

    fn priority(&self) -> TimingPriority {
        TimingPriority::Normal
    }
    fn set_priority(&self, _priority: TimingPriority) {}

    // --------------------------------------------------------
    // Transitions
    // --------------------------------------------------------
    fn transition(&self, _scene_manager: &SceneManager) -> SceneActions {
        SceneActions::NO_ACTION
    }

    fn take_transition_scene(&self) -> Option<RNode> {
        None
    }

    // --------------------------------------------------------
    // Misc
    // --------------------------------------------------------
    fn to_string(&self) -> String {
        self.name()
    }
}

impl PartialEq for NodeTrait {
    fn eq(&self, other: &NodeTrait) -> bool {
        self.tt_eq(other)
    }
}
