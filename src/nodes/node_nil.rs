use std::any::Any;

use std::cell::RefCell;
use std::rc::Rc;

use nodes::node::{NodeTrait, RNode};
use nodes::node_properties::NodeData;

// --------------------------------------------------------
// Nil/Null node
// --------------------------------------------------------
pub struct NodeNil {
    data: RefCell<NodeData>,
}

impl NodeNil {
    pub fn new() -> RNode {
        let mut n = NodeData::new();
        n.node.set_name("Nil".to_string());

        Rc::new(RefCell::new(Self {
            data: RefCell::new(n),
        }))
    }
}

impl NodeTrait for NodeNil {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn id(&self) -> usize {
        0
    }

    // --------------------------------------------------------
    // Node properties
    // --------------------------------------------------------
    fn data(&self) -> &RefCell<NodeData> {
        &self.data
    }
}
