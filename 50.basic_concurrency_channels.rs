//Basic Concurrency : Channels
//Valid Straight Line 5 Coordinates
use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex, mpsc};
use rand::Rng;
#[derive(Debug,Clone,Copy)]
pub struct Point{
    pub x:i32,
    pub y:i32,
}
impl Point {
    fn new(x:i32,y:i32)-> Self{
        Point { x: x, y: y }
    }
}
struct StraightLine {
    points:Vec<Point>,
}
impl StraightLine {
    fn new(points:Vec<Point>)->Self{
        StraightLine { points: points }
    }
    fn convert(&self) -> Vec<Vec<i32>> {
        let mut coordinates: Vec<Vec<i32>> = Vec::new();
        for p in self.points.clone().into_iter() {
            coordinates.push(vec![p.x,p.y]);
        }
        return coordinates;
    }
    fn check_straight_line(&self) -> bool {
        let coordinates = self.convert();
        let i = coordinates[0][0];
        let j = coordinates[0][1];
        if let true = coordinates.iter().all(|x| x[0] == i) {
            return true;
        }
        if let true = coordinates.iter().all(|x| x[1] == j) {
            return true;
        }
        let mut y = coordinates[1][1] - coordinates[0][1];
        let mut x = coordinates[1][0] - coordinates[0][0];
        if x == 0 {
            return false;
        }
        let m: f32 = y as f32 / x as f32;

        for i in 1..coordinates.len() - 1 {
            y = coordinates[i + 1][1] - coordinates[i][1];
            x = coordinates[i + 1][0] - coordinates[i][0];
            if x == 0 {
                return false;
            }
            if m != y as f32 / x as f32 {
                return false;
            }
        }
        true
    }
}
fn main() {
    let (tx, rx) = mpsc::channel();
    let points:Vec<Point> = vec![];
    let data = Arc::new(Mutex::new(points));
    let handler:Vec<_> = (0..1000).map(|_|{
        let (data,tx) = (Arc::clone(&data),tx.clone());
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut data = data.lock().unwrap();
            data.push(Point::new(rng.gen_range(0..6),rng.gen_range(0..6)));
            let size = data.len();
            if size > 5 {
                tx.send(data[data.len()-6..data.len()-1].to_vec()).unwrap();
            }
        })
     }).collect();
    for h in handler{
        h.join().unwrap();
    }

    for _ in 0..995 {
        if let Some(r) = rx.recv().ok(){
            let check_straight_line = StraightLine::new(r.clone());
            let ok = check_straight_line.check_straight_line();
            if ok {
                println!("{:?}",r);
            }
        }else{
            break;
        }
    }
    thread::sleep(Duration::from_millis(10));
}
