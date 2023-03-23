// Create Macro : sum vector , sum struct , sum numeric 
use std::ops::Add;
#[derive(Debug)]
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
macro_rules! sum {
    ($a:expr) => {
        $a
    };
    ($b:expr,$($rest:expr),+) =>{
        $b + sum!($($rest),+)
    };
}
macro_rules! sum_vec1 {
    ($a:expr) => {
        $a.iter().fold(0,|a,b| a + *b )
    };
    ($b:expr,$($rest:expr),+) =>{
        $b.iter().fold(0,|a,b| a + *b) + sum_vec1!($($rest),+)
    };
}
macro_rules! sum_vec2 {
    ($a:expr) => {
        $a.iter()
    };
    ($b:expr,$($rest:expr),+) =>{
        $b.iter().zip(sum_vec2!($($rest),+)).map(|(a,b)| {
            a + b
        }).collect::<Vec<_>>()
    };
}
fn main(){
    let sum = sum![1,2,3];
    println!("{:?}",sum);

    let sum_point = sum!(Point::new(1,2),Point::new(3,4),Point::new(5,6));
    println!("{:?}",sum_point);

    let sum_vec1 = sum_vec1!(vec![1,2,3,4,5],vec![1,2],vec![3,4]);
    println!("{:?}", sum_vec1);

    let sum_vec2:Vec<i32> = sum_vec2![vec![1,2],vec![3,4],vec![5,6]];
    println!("{:?}", sum_vec2);
}
