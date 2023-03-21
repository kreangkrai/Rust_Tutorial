//Basic Lifetimes
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Point<'a, T>{
    x:&'a T,
    y:&'a T,
}
impl <'a,T> Point<'a,T> {
    fn new(x:&'a T,y:&'a T)->Self{
        Point { x: x, y: y }
    }
}
impl <'a,T> Deref for Point<'a,T> {
    type Target = Point<'a,T>;

    fn deref(&self) -> &Self::Target {
        &self
    }
}
impl <'a,T> DerefMut for Point<'a,T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}
fn main() {
    let p1 = Point::new(&1,&2);
    let x = &p1.x;
    let y = &p1.y;
    let mut p3 = Point::new(x, y);
    println!("OLD {:?}",*p3);

    p3 = Point::new(&&3,&&4);
    println!("NEW {:?}",*p3);
    
}
