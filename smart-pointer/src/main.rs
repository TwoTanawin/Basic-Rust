use std::{cell::RefCell, rc::Rc};

fn main() {
    let chest: Box<i32> = Box::new(10);
    let sheared_chest: Rc<RefCell<Box<i32>>> = Rc::new(RefCell::new(chest));

    **sheared_chest.borrow_mut() += 10;
    **sheared_chest.borrow_mut() += 5;

    println!("Gold {}", sheared_chest.borrow());
}
