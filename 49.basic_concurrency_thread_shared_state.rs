//Basic Concurrency : Thread , Safe Shared Mutable State
use std::ops::Add;
use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex};
#[derive(Debug,Clone,Copy)]
struct Point<T>{
    x:T,
    y:T,
}
impl <T> Point<T> {
    fn new(x:T,y:T)-> Self{
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

fn main() {

    let points:Vec<Point<i32>> = vec![];
    let data = Arc::new(Mutex::new(points));
    let handler:Vec<_> = (0..1000).map(|i|{
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data.push(Point::new(i,i+1) + Point::new(1, 1));
        })
     }).collect();
    for h in handler{
        h.join().unwrap();
    }
    thread::sleep(Duration::from_millis(10));
    //println!("{:?}",data);
    let ok = Arc::try_unwrap(data.clone()).is_err();
    if ok {
        for d in Arc::try_unwrap(data).into_iter(){
            for p in d.into_inner().into_iter(){
                for (i,val) in p.into_iter().enumerate(){
                    println!("{} {:?}",i,val);
                }
            }
                       
        }   
    }
}
