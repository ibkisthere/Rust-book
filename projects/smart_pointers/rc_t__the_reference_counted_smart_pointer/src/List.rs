use std::rc::Rc;


pub enum List {
    Cons(i32,  Rc<List>),
    Nil,
}