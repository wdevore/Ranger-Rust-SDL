// use std::cell::RefCell;
// use std::rc::Rc;

use engine::timing::scheduler::TimingPriority;
use math::affine_transform::AffineTransform;
use nodes::node::NodeType;

// --------------------------------------------------------
// Node property bag
// --------------------------------------------------------
pub struct NodeData {
    pub node: NodeProperties,
    pub transform: TransformProperties,
    pub timing: TimingProperties,
    pub transition: TransitionProperties,
}

impl NodeData {
    pub fn new() -> Self {
        Self {
            node: NodeProperties::new(),
            transform: TransformProperties::new(),
            timing: TimingProperties::new(),
            transition: TransitionProperties::new(),
        }
    }
}

// --------------------------------------------------------
// Base node properties
// --------------------------------------------------------
#[derive(Debug)]
pub struct NodeProperties {
    // Base
    id: usize,
    name: String,
    n_type: NodeType,

    // Rendering
    visible: bool,

    // Timing
    canbe_timing_target: bool,

    // The node "as a whole" dirty state
    dirty: bool,
}

impl NodeProperties {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::from(""),
            n_type: NodeType::Nil,
            visible: true,
            canbe_timing_target: false,
            dirty: true,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name.replace_range(.., &name);
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn node_type(&self) -> NodeType {
        self.n_type
    }

    pub fn set_type(&mut self, n_type: NodeType) {
        self.n_type = n_type;
    }

    pub fn is_nil(&self) -> bool {
        self.n_type == NodeType::Nil
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn canbe_timing_target(&self) -> bool {
        self.canbe_timing_target
    }

    pub fn make_timing_target(&mut self, enabled: bool) {
        self.canbe_timing_target = enabled;
    }

    pub fn to_string(&self) -> String {
        format!("[({}) : '{}']", self.id, self.name)
    }
}

// --------------------------------------------------------
// Timing properties
// --------------------------------------------------------
#[derive(Debug)]
pub struct TimingProperties {
    priority: TimingPriority,
    paused: bool,
}

impl TimingProperties {
    pub fn new() -> Self {
        Self {
            priority: TimingPriority::Normal,
            paused: true,
        }
    }

    pub fn priority(&self) -> TimingPriority {
        self.priority
    }

    pub fn set_priority(&mut self, priority: TimingPriority) {
        self.priority = priority;
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn pause(&mut self, paused: bool) {
        self.paused = paused;
    }
}

// --------------------------------------------------------
// Transition properties
// --------------------------------------------------------
#[derive(Debug)]
pub struct TransitionProperties {
    pause_for: f64,
    pause_for_cnt: f64,
    transition: bool, // true = transition completed.
}

impl TransitionProperties {
    pub fn new() -> Self {
        Self {
            pause_for: 0.0,
            pause_for_cnt: 0.0,
            transition: false,
        }
    }

    pub fn pause_for(&self) -> f64 {
        self.pause_for
    }

    pub fn set_pause_for(&mut self, pause_for: f64) {
        self.pause_for = pause_for;
    }

    pub fn pause_for_cnt(&self) -> f64 {
        self.pause_for_cnt
    }

    pub fn reset_pause(&mut self) {
        self.pause_for_cnt = 0.0;
        self.transition = false;
    }

    pub fn inc_pause_cnt(&mut self, dt: f64) {
        self.pause_for_cnt = self.pause_for_cnt + dt;
    }

    pub fn update(&mut self, dt: f64) {
        self.inc_pause_cnt(dt);
        if self.pause_for_cnt >= self.pause_for {
            self.transition = true;
        }
    }

    pub fn ready(&self) -> bool {
        self.transition
    }
}

// --------------------------------------------------------
// Affine transform properties
// --------------------------------------------------------

// Coord system
//
//      0,0      +X
//      .-------->     <== top
//      |
//      |
//      |
//      v              <== bottom
//      +Y
//
#[derive(Debug)]
pub struct TransformProperties {
    position: (f64, f64), // (x,y)
    rotation: f64,        // radians
    scale: (f64, f64),    // (sx, sy)

    aft: AffineTransform,
    inverse: AffineTransform,
    // dirty: bool,
}

impl TransformProperties {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            rotation: 0.0,
            scale: (1.0, 1.0),
            aft: AffineTransform::new(),
            inverse: AffineTransform::new(),
            // dirty: true,
        }
    }

    pub fn x(&self) -> f64 {
        self.position.0
    }

    pub fn y(&self) -> f64 {
        self.position.1
    }

    // pub fn is_dirty(&self) -> bool {
    //     self.dirty
    // }

    // pub fn set_dirty(&mut self, dirty: bool) {
    //     self.dirty = dirty;
    // }

    pub fn set_position(&mut self, x: f64, y: f64) {
        self.position = (x, y);
        // self.set_dirty(true);
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    pub fn rotation_in_degrees(&self) -> f64 {
        f64::to_degrees(self.rotation)
    }

    /// +angle = Clock-wise rotation
    pub fn set_rotation_degrees(&mut self, degrees: f64) {
        self.rotation = f64::to_radians(degrees);
        // self.set_dirty(true);
    }

    pub fn set_rotation(&mut self, radians: f64) {
        self.rotation = radians;
        // self.set_dirty(true);
    }

    pub fn scale(&self) -> (f64, f64) {
        self.scale
    }

    pub fn uniform_scale(&self) -> f64 {
        assert!(self.scale.0 == self.scale.1);

        self.scale.0
    }

    pub fn scale_x(&self) -> f64 {
        self.scale.0
    }

    pub fn scale_y(&self) -> f64 {
        self.scale.1
    }

    pub fn set_nonuniform_scale(&mut self, sx: f64, sy: f64) {
        self.scale = (sx, sy);
        // self.set_dirty(true);
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = (scale, scale);
        // self.set_dirty(true);
    }

    pub fn get_inverse(&self) -> &AffineTransform {
        &self.inverse
    }

    pub fn get_transform(&self) -> &AffineTransform {
        &self.aft
    }

    pub fn calc_filtered_transform(
        &self,
        exclude_translation: bool,
        exclude_rotation: bool,
        exclude_scale: bool,
        aft: &mut AffineTransform,
    ) {
        if !exclude_translation {
            let pos = self.position;
            aft.make_translate(pos.0, pos.1);
        }

        if !exclude_rotation {
            let rot = self.rotation;
            if rot != 0.0 {
                aft.rotate(rot);
            }
        }

        if !exclude_scale {
            let sca = self.scale;
            if sca.0 != 1.0 || sca.1 != 1.0 {
                aft.scale(sca.0, sca.1);
            }
        }
    }

    pub fn calc_transform(&mut self) -> &AffineTransform {
        // if self.dirty {
        let pos = self.position;
        self.aft.make_translate(pos.0, pos.1);

        let rot = self.rotation;
        if rot != 0.0 {
            self.aft.rotate(rot);
        }

        let sca = self.scale;
        if sca.0 != 1.0 || sca.1 != 1.0 {
            self.aft.scale(sca.0, sca.1);
        }

        AffineTransform::invert_mo(&self.aft, &mut self.inverse);

        //     self.dirty = false;
        // }

        &self.aft
    }
}
