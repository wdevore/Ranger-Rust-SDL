// This is scratch pad code while I learn Rust.

use std::cell::RefCell;
use std::rc::Rc;

type RefNode = Rc<RefCell<NodeTrait>>;
type RefTransform = Rc<RefCell<TransformProperty>>;

trait NodeTrait {
    fn id(&self) -> usize {
        0
    }
    fn set_id(&mut self, id: usize);

    fn name(&self) -> &String;
    fn set_name(&mut self, name: String);

    fn to_string(&self) -> String;

    fn node(&self) -> &NodeProperties;

    // --------------------------------------------------------
    // Transformations
    // --------------------------------------------------------
    fn transform_p(&mut self) -> &Option<RefTransform> {
        &None
    }

    fn set_position(&mut self, x: f32, y: f32);

    // --------------------------------------------------------
    // Timing target
    // --------------------------------------------------------
    fn target_update(&mut self, dt: f32) {}
}

struct TransformProperty {
    x: f32,
    y: f32,
}

impl TransformProperty {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

struct NodeProperties {
    id: usize,
    name: String,
    transform: Option<RefTransform>,
}

impl NodeProperties {
    fn new() -> Self {
        Self {
            id: 0,
            name: String::from("NoName"),
            transform: None,
        }
    }
}

// ======================================================
struct BootScene {
    node: NodeProperties,
}

impl BootScene {
    fn new() -> RefNode {
        let mut n = NodeProperties::new();
        n.transform = Some(Rc::new(RefCell::new(TransformProperty::new())));
        Rc::new(RefCell::new(Self { node: n }))
    }
}

impl NodeTrait for BootScene {
    fn id(&self) -> usize {
        self.node.id
    }
    fn set_id(&mut self, id: usize) {
        self.node.id = id;
    }

    fn to_string(&self) -> String {
        self.node.name.to_string()
    }

    fn name(&self) -> &String {
        &self.node.name
    }
    fn set_name(&mut self, name: String) {
        self.node.name = name.to_string();
    }

    fn transform_p(&mut self) -> &Option<RefTransform> {
        &self.node.transform
    }

    fn node(&self) -> &NodeProperties {
        &self.node
    }

    fn set_position(&mut self, x: f32, y: f32) {
        if let Some(t) = &self.node.transform {
            t.borrow_mut().set_position(x, y);
        }
    }

    fn target_update(&mut self, dt: f32) {
        println!("tupdate: {}", dt);
    }
}

// ======================================================
struct SplashScene {
    node: NodeProperties,
}

impl SplashScene {
    fn new() -> RefNode {
        Rc::new(RefCell::new(Self { node: NodeProperties::new() }))
    }
}

impl NodeTrait for SplashScene {
    fn id(&self) -> usize {
        self.node.id
    }
    fn set_id(&mut self, id: usize) {
        self.node.id = id;
    }

    fn to_string(&self) -> String {
        self.node.name.to_string()
    }

    fn name(&self) -> &String {
        &self.node.name
    }
    fn set_name(&mut self, name: String) {
        self.node.name = name.to_string();
    }
    fn node(&self) -> &NodeProperties {
        &self.node
    }
    fn set_position(&mut self, x: f32, y: f32) {
        if let Some(t) = &self.node.transform {
            t.borrow_mut().set_position(x, y);
        }
    }
}

// ======================== SceneManager ==============================
struct SceneManager {
    scenes: Vec<RefNode>,
}

impl SceneManager {
    fn new() -> Self {
        Self { scenes: Vec::new() }
    }

    fn push_scene(&mut self, scene: RefNode) {
        self.scenes.push(scene);
    }

    fn to_string(&self) {
        for sc in self.scenes.iter() {
            print!("({}) '{}'", sc.borrow().id(), sc.borrow().to_string());
            let mut scb = sc.borrow_mut();
            match scb.transform_p() {
                Some(tp) => println!(" : {},{}", tp.borrow().x, tp.borrow().y),
                None => println!(""),
            }
        }
    }

    fn scene(&self, id: usize) -> Option<&RefNode> {
        // self.scenes.get(id) // By ordinal
        // Or by Id
        for sc in self.scenes.iter() {
            if sc.borrow().id() == id {
                return Some(sc);
            }
        }
        
        None
    }
}
// ======================== Scheduler ==============================
struct Scheduler {
    update_targets: Vec<RefNode>,
}

impl Scheduler {
    fn new() -> Self {
        Self { update_targets: Vec::new() }
    }

    fn add_target(&mut self, target: RefNode) {
        self.update_targets.push(target);
    }

    fn to_string(&self) {
        println!("Targets: ({})", self.update_targets.len());
        for sc in self.update_targets.iter() {
            print!("({}) '{}'", sc.borrow().id(), sc.borrow().to_string());
        }
    }

    fn target(&self, id: usize) -> Option<&RefNode> {
        for tar in self.update_targets.iter() {
            if tar.borrow().id() == id {
                return Some(tar);
            }
        }
        
        None
    }

    fn update(&mut self, dt: f32) {
        for tar in self.update_targets.iter() {
            tar.borrow_mut().target_update(dt);
        }
    }
}

// ##############################################################
type Builder = fn(&mut Engine);

struct Engine {
    id: usize,
    scm: SceneManager,
    sch: Scheduler,
}

impl<'a> Engine {
    fn new() -> Self {
        Self {
            id: 0,
            scm: SceneManager::new(),
            sch: Scheduler::new(),
        }
    }

    fn launch(&mut self, build: Builder) {
        build(self);
    }

    fn scene_manager(&'a mut self) -> &'a SceneManager {
        &self.scm
    }

    fn gen_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }

    fn set_id(&mut self, node: RefNode) {
        self.gen_id();
        node.borrow_mut().set_id(self.id);
    }

    // fn scene_manager_mut(&'a mut self) -> &'a mut SceneManager {
    //     &mut self.scm
    // }

    fn push_scene(&mut self, scene: RefNode) {
        self.scm.push_scene(scene);
    }

    fn add_target(&mut self, target: RefNode) {
        self.sch.add_target(target);
    }

    fn step(&mut self, dt: f32) {
        self.sch.update(dt);
    }
}

#[test]
fn scratch_scene_manager() {
    let mut eng = Engine::new();
    eng.launch(build);

    assert!(true);
}

// ############### BUILD #####################################
fn build(eng: &mut Engine) {
    let bs = BootScene::new();
    eng.set_id(bs.clone());
    eng.add_target(bs.clone());

    // {
    //     let mut node = bs.borrow_mut();
    //     node.set_name(String::from("Boot"));
    //     node.set_position(2.2, 5.5);
    // }
    // Or better style
    match bs.borrow_mut() {
        mut node => {
            node.set_name(String::from("Boot"));
            node.set_position(2.2, 5.5);
        }
    }
    // Or cumbersome
    // bs.borrow_mut().set_name(String::from("Boot"));
    // bs.borrow_mut().set_position(2.2, 5.5);

    // if let Some(stp) = bs.borrow_mut().transform_p() {
    //     stp.borrow_mut().set_position(1.0, 3.3);
    // }
    // match bs.borrow_mut().transform_p() {
    //     Some(stp) => {
    //         stp.borrow_mut().set_position(1.0, 3.3);
    //     },
    //     _ => (),
    // }

    let sp = SplashScene::new();
    eng.set_id(sp.clone());
    sp.borrow_mut().set_name(String::from("Splash"));
    sp.borrow_mut().set_position(1.0, 4.4); // doesn't change anything

    eng.push_scene(bs);
    eng.push_scene(sp);


    {
        let scm = eng.scene_manager();

        if let Some(se) = scm.scene(1) {
            se.borrow_mut().set_name(String::from("Boot2"));
        }
        // Or
        // match scm.scene(1) {
        //     Some(se) => {
        //         se.borrow_mut().set_name(String::from("Boot2"));
        //         // println!("sn: {}", se.borrow().to_string());
        //     }
        //     None => println!("2 not found"),
        // }

        println!("----------------------");
        scm.to_string();
    }

    eng.step(1.0);
    eng.step(1.0);
    eng.step(1.0);
}
