// Sum each point of coordinates With Rust Programing Language
// Label A
// Point [1,2]
// Point [2,5]
// SUM Point [3,7]

// Label B
// Point [3,2]
// Point [1,3]
// SUM Point [4,5]

// Label C
// Point [2,4]
// SUM Point [2,4]

use std::{collections::HashMap, fmt::Error};
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone)]
struct Coordinate {
    label: &'static str,
    point: Point,
}
#[derive(Debug, Clone)]
struct Coordinates {
    coors: Vec<Coordinate>,
}
impl Coordinates {
    fn new(coors: Vec<Coordinate>) -> Self {
        Coordinates { coors: coors }
    }
    fn calculate(&self) -> Result<Vec<Coordinate>, Error> {
        let mut hm: HashMap<&str, Vec<Point>> = HashMap::new();
        let mut new_coors: Vec<Coordinate> = vec![];
        let temp_coors = self.coors.clone();
        for c in temp_coors.into_iter() {
            hm.entry(c.label).or_insert(Vec::new()).push(c.point);
        }

        for (key, value) in hm {
            let sum = value.iter().fold(Point { x: 0, y: 0 }, |a, b| Point {
                x: a.x + b.x,
                y: a.y + b.y,
            });
            new_coors.push(Coordinate {
                label: key,
                point: sum,
            });
        }
        Ok(new_coors)
    }
}
fn main() {
    let coors = vec![
        Coordinate {
            label: "A",
            point: Point { x: 1, y: 2 },
        },
        Coordinate {
            label: "A",
            point: Point { x: 2, y: 5 },
        },
        Coordinate {
            label: "B",
            point: Point { x: 3, y: 2 },
        },
        Coordinate {
            label: "B",
            point: Point { x: 1, y: 3 },
        },
        Coordinate {
            label: "C",
            point: Point { x: 2, y: 4 },
        },
    ];
    let c = Coordinates::new(coors);
    let answer = c.calculate().unwrap();
    println!("{:#?}", answer);
}
