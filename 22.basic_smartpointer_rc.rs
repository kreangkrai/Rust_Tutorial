//Basic Smart Pointer : Rc<T> With Rust programing Language

use std::rc::Rc;
use std::ops::Add;
use crate::Coordinates::{Cons,Nil};
#[derive(Debug,Clone, Copy)]
struct Point<T>{
    x:T,
    y:T,
}
impl <T> Point<T> {
    fn new(x:T,y:T)->Self{
        Point { x: x, y: y }
    }
}
impl <T:Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Self) -> Self::Output {
        Point{
            x:self.x + other.x,
            y:self.y + other.y,
        }
    }
}
#[derive(Debug,Clone)]
enum Coordinates{
    Cons(Point<i32>,Rc<Coordinates>),
    Nil,
}
impl Coordinates {
    fn new(point:Point<i32>)->Self{
        Cons(point, Rc::new(Nil))
    }
    fn append(self,point:Point<i32>)->Coordinates{
        Cons(point, Rc::new(self))
    }
    fn print_coors(&self){
        match self {
            Cons(start, ref end)=>{
                end.print_coors();
                println!("{:?}",start);
            },
            Nil=>{}
        }
    }
    fn sum_coors(&self)->Point<i32>{
        match self {
            Cons(ref start, ref end)=> *start + end.sum_coors(),
            Nil=> Point { x: 0, y: 0 }
        }
    }
}
fn main(){
   let mut coors = Coordinates::new(Point::new(3, 4));
   coors = coors.append(Point::new(4, 5));
   coors = coors.append(Point::new(5, 6));
   coors = coors.append(Point::new(2, 3));
   coors.print_coors();
   println!("Sum of Coordinates : {:?}",coors.sum_coors());
}
