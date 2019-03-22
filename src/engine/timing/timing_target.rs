use engine::timing::scheduler::TimingPriority;

pub trait TimingTarget {
    fn update(&self, dt: f64);
    fn pause(&mut self, paused: bool);
    fn paused(&self) -> bool;
    fn tt_eq(&self, other: &TimingTarget) -> bool;
    fn id(&self) -> u32;
}

impl PartialEq for TimingTarget {
    fn eq(&self, other: &TimingTarget) -> bool {
        self.tt_eq(other)
    }
}

pub struct TimingProperties {
    priority: TimingPriority,
    paused: bool,
}

impl TimingProperties {
    pub fn new() -> Self {
        Self {
            priority: TimingPriority::Normal,
            paused: false,
        }
    }

    pub fn pause(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn set_priority(&mut self, priority: TimingPriority) {
        self.priority = priority;
    }
}
