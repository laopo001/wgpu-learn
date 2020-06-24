use crate::app::App;
use crate::config::Event;
use std::collections::HashMap;

pub trait FnBox {
    fn call_box(&mut self, v: &mut App);
}
impl<F: FnMut(&mut App)> FnBox for F {
    fn call_box(&mut self, v: &mut App) {
        (*self)(v)
    }
}
type Task = Box<dyn FnBox + 'static>;
pub struct EventListener {
    pub event: HashMap<Event, Vec<Task>>,
    pub namespace: String,
}

impl EventListener {
    pub fn new(namespace: &str) -> Self {
        return EventListener {
            namespace: namespace.to_string(),
            event: HashMap::new(),
        };
    }
    pub fn on<F>(&mut self, e: Event, task: F)
    where
        F: FnMut(&mut App) + 'static,
    {
        if let Some(v) = self.event.get_mut(&e) {
            v.push(Box::new(task));
        } else {
            self.event.insert(e, vec![Box::new(task)]);
        }
    }
}
