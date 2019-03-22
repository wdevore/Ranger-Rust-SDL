use std::cell::RefCell;
use std::rc::Rc;

use nodes::node::RNode;

pub type RScheduler = Rc<RefCell<Scheduler>>;

/// UpdateTargets are independent timing targets. That is they are
/// timing events independent of the core timing loop events.
// type UpdateTarget = fn(dt: f64);

/// The `Scheduler` is responsible to updating timing targets.
/// There are two types of targets:
///
/// # Targets:
///
/// * `TimingTarget` - are targets drivin by the `Core` frame timing.
/// * `UpdateTarget` - are targets drivin by an independent timer.
pub struct Scheduler {
    system_targets: Vec<RNode>,
    // non_system_targets: Vec<Rc<RefCell<TimingTarget>>>,
    normal_targets: Vec<RNode>,
}

/// The order of target updates.
#[derive(Debug, Clone, Copy)]
pub enum TimingPriority {
    /// Updated 1st
    System,
    /// Updated 2nd
    // NonSystem,
    /// Updated 3rd
    Normal,
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        println!("Dropping Scheduler");
        self.system_targets.clear();
        self.normal_targets.clear();
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            system_targets: Vec::new(),
            // non_system_targets: Vec::new(),
            normal_targets: Vec::new(),
        }
    }

    pub fn update(&self, dt: f64) {
        // Update TimingTargets first.
        for target in &self.system_targets {
            let t = target.borrow();
            if !t.paused() {
                t.update(dt);
            }
        }

        // for target in &non_system_targets {
        //     let t = target.borrow();
        //     if !t.paused() {
        //         t.update(dt);
        //     }
        // }

        for target in &self.normal_targets {
            let t = target.borrow();
            // println!(
            //     "scheduler normal target: '{}', paused: {}",
            //     t.name(),
            //     t.paused()
            // );
            if !t.paused() {
                t.update(dt);
            }
        }

        // TODO Custom (aka Update targets)
    }

    // ----------------------------------------------------------
    // TimingTargets
    // ----------------------------------------------------------
    pub fn register_timing_target(&mut self, target: RNode) {
        println!(
            "Scheduler: registering timing target:({}) {}",
            target.borrow().id(),
            target.borrow().name()
        );
        match target.borrow().priority() {
            // We check Normals first as they are occur the most.
            TimingPriority::Normal => {
                // Check that the target has not be scheduled.
                let contains = self.normal_targets.contains(&target);
                if contains {
                    return;
                }
                self.normal_targets.push(target.clone());
            }
            TimingPriority::System => {
                let contains = self.system_targets.contains(&target);
                if contains {
                    return;
                }

                self.system_targets.push(target.clone());
            } // TimingPriority::NonSystem => {
              //     println!("TODO");
              // }
        }
    }

    pub fn schedule_timing_target(&mut self, target_id: usize) {
        // We check Normals first as they are occur the most.
        for target in &self.normal_targets {
            let t = target.borrow();
            if t.id() == target_id {
                t.pause(false);
                return;
            }
        }

        for target in &self.system_targets {
            let t = target.borrow();
            if t.id() == target_id {
                t.pause(false);
                return;
            }
        }
    }

    pub fn unschedule_timing_target_by_id(&mut self, target_id: usize) {
        // Check each list
        self.normal_targets
            .retain(|ref node| node.borrow().id() != target_id);
        self.system_targets
            .retain(|ref node| node.borrow().id() != target_id);
    }

    pub fn unschedule_timing_target(&mut self, target: RNode) {
        match target.borrow().priority() {
            // We check Normals first as they are occur the most.
            TimingPriority::Normal => {
                // Check that the target has not be scheduled.
                let contains = self.normal_targets.contains(&target);
                if contains {
                    return;
                }

                // Maintains order
                self.normal_targets
                    .retain(|ref node| node.borrow().id() != target.borrow().id());
            }
            TimingPriority::System => {
                let contains = self.system_targets.contains(&target);
                if contains {
                    return;
                }

                // Maintains order
                self.system_targets
                    .retain(|ref node| node.borrow().id() != target.borrow().id());
            } // TimingPriority::NonSystem => {
              //     println!("TODO");
              // }
        }
    }

    pub fn pause_timing_targets(&mut self, priority: TimingPriority) {
        match priority {
            TimingPriority::System => {
                for target in &self.system_targets {
                    target.borrow_mut().pause(true);
                }
            }
            // TimingPriority::NonSystem => {}
            TimingPriority::Normal => {
                for target in &self.normal_targets {
                    target.borrow_mut().pause(true);
                }
            }
        }
    }

    pub fn resume_timing_targets(&mut self, priority: TimingPriority) {
        match priority {
            TimingPriority::System => {
                for target in &self.system_targets {
                    target.borrow_mut().pause(false);
                }
            }
            // TimingPriority::NonSystem => {}
            TimingPriority::Normal => {
                for target in &self.normal_targets {
                    target.borrow_mut().pause(false);
                }
            }
        }
    }
}
