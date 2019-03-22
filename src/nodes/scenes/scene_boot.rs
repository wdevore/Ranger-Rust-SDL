use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use nodes::{
    node::{NodeTrait, NodeType, RNode, RONode},
    node_nil::NodeNil,
    node_properties::NodeData,
    scenes::scene_manager::{SceneActions, SceneManager},
};
use world::World;

pub struct SceneBoot {
    replacement: RefCell<Option<RNode>>,

    data: RefCell<NodeData>,

    parent: RONode,
}

impl Drop for SceneBoot {
    fn drop(&mut self) {
        println!("Dropping: '{}'", self.data().borrow().node.name());
    }
}

impl SceneBoot {
    pub fn with_replacement(
        name: &str,
        replace: RNode,
        world: &mut World,
    ) -> Rc<RefCell<SceneBoot>> {
        let mut n = NodeData::new();
        n.node.set_name(name.to_string());
        n.node.set_type(NodeType::Scene);
        n.node.set_id(world.gen_id());

        let sb = Self {
            replacement: RefCell::new(Some(replace)),
            data: RefCell::new(n),
            parent: Rc::new(RefCell::new(None)),
        };

        Rc::new(RefCell::new(sb))
    }
}

impl NodeTrait for SceneBoot {
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

    fn set_parent(&self, parent: RNode) {
        self.parent.borrow_mut().replace(parent);
    }

    // --------------------------------------------------------
    // Life cycle events
    // --------------------------------------------------------
    fn enter(&self, _scene_manager: &SceneManager) {
        println!("enter '{}'", self.to_string());
        // scene_manager.replace_scene(self.replacement.clone());
    }

    // --------------------------------------------------------
    // Transitions
    // --------------------------------------------------------
    fn transition(&self, _scene_manager: &SceneManager) -> SceneActions {
        // println!("transition '{}'", self.to_string());
        // scene_manager.replace_scene(self.replacement.clone());
        SceneActions::REPLACE
    }

    fn take_transition_scene(&self) -> Option<RNode> {
        self.replacement.borrow_mut().take()
    }

    // fn exit(&self, _data: &GlobalSceneData) {
    //     println!("exit '{}'", self.to_string());
    // }

    // --------------------------------------------------------
    // Rendering
    // --------------------------------------------------------
}
