use std::cell::RefCell;
use crate::models::item::Item;

#[derive(Clone)]
pub struct AppState {
    pub count: Cell<usize>,
    pub items: RefCell<Vec<Item>>, // Thread-safe mutable storage
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            count: Cell::new(0),
            items: RefCell::new(Vec::new()),
        }
    }
}
