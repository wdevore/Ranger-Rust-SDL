use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// Until vscode is updated this is required to get ride
// of the false "red" highlighted errors. It isn't needed
// with >1.32.0
extern crate ranger;

use ranger::{
    // engine::timing::scheduler::RScheduler,
    nodes::{
        custom_nodes::{cross_node::CrossNode, vector_text_node::VectorTextNode},
        node::{NodeTrait, NodeType, Nodes, OChildren, RNode, RONode},
        node_properties::NodeData,
        scenes::scene_manager::{GlobalSceneData, SceneManager},
    },
    rendering::color::Palette,
    world::World,
};

use template_0::game_layer::GameLayer;

pub struct GameScene {
    data: RefCell<NodeData>,

    parent: RONode,

    children: OChildren,

    // xaxis_color: Palette,
    // yaxis_color: Palette,
    _title_color: Palette,
}

impl Drop for GameScene {
    fn drop(&mut self) {
        println!("Dropping: '{}'", self.data().borrow().node.name());
        if let Some(childs) = &self.children {
            childs.borrow_mut().clear();
        }
    }
}

impl GameScene {
    pub fn new(name: &str, world: &mut World) -> Rc<RefCell<NodeTrait>> {
        let mut n = NodeData::new();
        n.node.set_name(name.to_string());
        n.node.set_type(NodeType::Scene);
        n.node.set_id(world.gen_id());

        let gs = Self {
            data: RefCell::new(n),
            parent: Rc::new(RefCell::new(None)),
            children: Some(RefCell::new(Vec::new())),
            // xaxis_color: Palette::RED(),
            // yaxis_color: Palette::GREEN(),
            _title_color: Palette::ORANGE(),
        };

        let rc: RNode = Rc::new(RefCell::new(gs));

        GameScene::build_heirarchy(&rc, world);

        rc
    }

    fn build_heirarchy(node: &RNode, world: &mut World) {
        // Hiearchy:
        // GameScene
        //   GameLayer               <-- scaled
        //     OrbitSystemNode       <-- filtered (only pass translate)
        //       OrbitAnschorNode     <-- filtered (only pass translate)
        //         TriangleNode      <-- leaf
        //     YellowRectangle       <-- leaf
        //   WhiteText               <-- leaf
        //   CrossNode               <-- leaf

        let _layer = GameLayer::new("GameLayer", Some(node.clone()), world);
        // layer.borrow().set_position(300.0, 0.0);

        let view_width = world.properties().view_width;
        let view_height = world.properties().view_height;

        let word = VectorTextNode::new("WhiteText", Some(node.clone()), world);
        let bword = word.borrow();
        bword.set_scale(50.0);
        bword.set_position(0.0, 0.0);
        bword.set_rotation_degrees(45.0);

        match bword.as_any().downcast_ref::<VectorTextNode>() {
            Some(vtn) => {
                vtn.set_text(&String::from("RANGER IS A GO!"));
            }
            None => panic!("Downcast failed for VectorTextNode"),
        };

        let cross = CrossNode::new("WhiteCross", Some(node.clone()), world);
        cross.borrow().set_nonuniform_scale(view_width, view_height);

        Nodes::print_tree(&node);
    }
}

impl NodeTrait for GameScene {
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
    // Life cycle events
    // --------------------------------------------------------
    // fn start_exit_transition(&self, _scene_manager: &SceneManager) {
    //     println!("start_exit_transition '{}'", self.to_string());
    //     // scm.signal_state(SceneState::Activate);
    // }

    fn end_enter_transition(&self) {
        println!("end_enter_transition '{}'", self.to_string());
    }

    fn enter(&self, scene_manager: &SceneManager) {
        println!("enter '{}'", self.to_string());

        // Schedule/enable timing
        self.ripple_pause(false);

        if let Some(children) = self.get_children() {
            self.sub_enter(scene_manager, children);
        }
    }

    fn exit(&self, data: &mut GlobalSceneData) {
        println!("exit '{}'", self.to_string());
        // data.register_for_io_events()
        self.ripple_pause(true);

        if let Some(children) = self.get_children() {
            self.sub_exit(data, children);
        }
    }

    // fn exit(&self, _scene_manager: &SceneManager) {
    //     println!("exit '{}'", self.to_string());
    //     self.ripple_pause(true);
    // }

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
    // Rendering
    // --------------------------------------------------------

    // fn device_visit(&self, _context: &mut Context) {
    //     // let tx = 20; //(self.world_props.width / 2 - (9 * 10 * 2)) as i32;
    //     // let ty = 20; //(self.world_props.height / 2 - (4 * 5)) as i32;
    //     // context.set_draw_color(&self.title_color);
    //     // context.text(tx, ty, "Game scene", 5, 3);

    //     // context.set_draw_color(&Palette::YELLOW());
    //     // context.draw_rectangle(100, 100, 300, 300);

    //     // let wp = &self.world_props;
    //     // context.set_draw_color(&self.xaxis_color);
    //     // context.draw_horz_line_color(
    //     //     0,
    //     //     (wp.window_width - 1) as i32,
    //     //     ((wp.window_height - 1) / 2) as i32,
    //     // );

    //     // context.set_draw_color(&self.yaxis_color);
    //     // context.draw_vert_line(
    //     //     0,
    //     //     (wp.window_height - 1) as i32,
    //     //     ((wp.window_width - 1) / 2) as i32,
    //     // );
    // }

    // --------------------------------------------------------
    // Grouping
    // --------------------------------------------------------
    fn get_children(&self) -> &OChildren {
        &self.children
    }
}
