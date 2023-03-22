//Basic Rc + RefCell
use std::{cell::RefCell, rc::Rc};
#[derive(Debug,Clone,Copy)]
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
    // let mut point = Point::new(1,2);
    // println!("Before : {:?}",point);

    // let mut borrow_point_1 = &mut point;
    // borrow_point_1 = &mut Point::new(3,4);
    // println!("After  : {:?}",point);

    // let mut borrow_point_2 = &mut point;
    // borrow_point_2 = &mut Point::new(3,4);
    // println!("After  : {:?}",point);

    let point = Rc::new(RefCell::new(Point::new(1,2)));
    println!("Before : {:?}",point.clone());
    let point_1 = Rc::clone(&point);
    let point_2 = Rc::clone(&point);

    let borrow_point_1 = point_1.try_borrow_mut();
    if let Some(mut s) = borrow_point_1.ok(){
        *s = Point::new(3,4);
    }else{
        println!("Error");
    }
    
    println!("After1 : {:?}",point.clone());
    
    let borrow_point_2 = point_2.try_borrow_mut();
    if let Some(mut s) = borrow_point_2.ok(){
        *s = Point::new(5,6);
    }else{
        println!("Error");
    }
    println!("After2 : {:?}",point.clone());
}
