pub trait Component {
    fn initialize(&mut self);
    fn destroy(&mut self);
}

pub struct BaseComponent {
    pub initialized: bool,
}
impl BaseComponent {
    pub fn new() -> Self {
        return BaseComponent { initialized: false };
    }
}
impl Component for BaseComponent {
    fn initialize(&mut self) {
        self.initialized = true;
    }
    fn destroy(&mut self) {
        self.initialized = false;
    }
}
