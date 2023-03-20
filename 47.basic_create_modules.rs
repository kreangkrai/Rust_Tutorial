//Basic Create Modules

mod mod_point {
    use std::ops::Add;
    #[derive(Debug)]
    pub struct Point<T> {
        x:T,
        y:T,
    }
    impl <T> Point<T> {
        pub fn new(x:T,y:T)->Self{
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
}
use crate::mod_point::Point;
fn main() {  
    let p1 = Point::new(1,2);
    let p2 = Point::new(3,4);

    let p = p1 + p2;
    println!("{:?}",p);
}
