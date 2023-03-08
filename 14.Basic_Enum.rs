//Basic Enum in Rust Programing Language

use std::rc::Rc;
use crate::List::{Cons,Nil};
#[derive(Debug)]
enum Color{
    Red,
    Green,Blue,
}
#[derive(Debug)]
enum List{
    Cons(Color,Rc<List>),
    Nil,
}
impl List {
    fn new(color:Color)->Self{
        Cons(color, Rc::new(Nil))
    }
    fn append(self,color:Color) -> List{
        Cons(color, Rc::new(self))
    }
    fn count_color(&self)->u32{
        match self {
            Cons(_,ref end ) => 1 + end.count_color(),
            Nil => 0
        }
    }
    fn print_list(&self){
        match self {
            Cons(start, ref end)=>{
                end.print_list();
                print!(" => {:?}({:?})",start,self.count_color());
            },
            Nil => {
                print!("Nil");
            }
        }
    }
}
fn main() {
    let mut list = List::new(Color::Red);
    list = list.append(Color::Green);
    list = list.append(Color::Blue);
    list = list.append(Color::Green);
    list = list.append(Color::Red);
    list.print_list();
}
