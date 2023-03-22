//Basic RefCell

use std::{cell::RefCell};
#[derive(Debug,Clone)]
#[allow(dead_code)]
struct Point<T> {
    x:T,
    y:T,
}
impl <T> Point<T> {
    fn new(x:T,y:T)->Self{
        Point { x: x, y: y }
    }
}
#[allow(unused_assignments,unused_variables)]
fn main() {
    // let mut point = Point::new(1, 2);
    // println!("Brfore : {:?}",point);
    // let mut borrow_point = &mut point;
    // borrow_point = &mut Point::new(3, 4);
    // println!("After  : {:?}",point);

    let point = RefCell::new(Point::new(1,2));
    println!("Before : {:?}",point.clone().into_inner());
    let borrow_point = point.try_borrow_mut();
    
    match borrow_point.ok() {
        Some(mut s) =>{
            *s = Point::new(3, 4);
        },
        None => println!("Error")
    }
    println!("After  : {:?}",point.into_inner());
}
