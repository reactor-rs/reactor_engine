use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};


pub type Object<T> = Rc<RefCell<T>>;
pub type ObjectPar<T> = Arc<Mutex<T>>;

pub trait ObjectMethods<T> {
    fn construct(value: T) -> Self;
}

impl<T> ObjectMethods<T> for Object<T> {
    fn construct(value: T) -> Object<T> {
        Rc::new(RefCell::new(value))
    }
}

impl<T> ObjectMethods<T> for ObjectPar<T> {
    fn construct(value: T) -> ObjectPar<T> {
        Arc::new(Mutex::new(value))
    }
}