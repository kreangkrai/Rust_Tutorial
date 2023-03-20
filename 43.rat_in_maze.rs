//Rat in a Maze Problem : Backtracking Algorithm With Rust Programing Language
#[derive(Debug)]
struct Points {
    points: Vec<Vec<u32>>,
}
#[allow(unused_comparisons)]
impl Points {
    fn new(points: Vec<Vec<u32>>) -> Self {
        Points { points: points }
    }
    fn isvalid_place(&self, x: u32, y: u32) -> bool {
        if x >= 0
            && x < self.points.len() as u32
            && y >= 0
            && y < self.points[0].len() as u32
            && self.points[x as usize][y as usize] == 1{
            return true;
        }
        false
    }
    fn solve_maze(&self, x: u32, y: u32, sol: &mut Vec<Vec<u32>>) -> bool {
        let length_x = self.points.len() as u32;
        let length_y = self.points[x as usize].len() as u32;
        if x == length_x - 1 && y == length_y - 1 && self.points[x as usize][y as usize] == 1{
            sol[x as usize][y as usize] = 1;
            return true;
        }

        if self.isvalid_place(x, y) {
            if sol[x as usize][y as usize] == 1 {
                return false;
            }
            sol[x as usize][y as usize] = 1;
            if x < length_x -1{
                if self.solve_maze(x + 1, y, sol) {
                    return true;
                }
            }
            if x > 0{
                if self.solve_maze(x - 1, y, sol) {
                    return true;
                }
            }
            if y < length_y -1{         
                if self.solve_maze(x, y + 1, sol) {
                    return true;
                }
            }
            if y > 0{
                if self.solve_maze(x, y - 1, sol) {
                    return true;
                }
            }           
            sol[x as usize][y as usize] = 0;
            return false;
        }
        false
    }
    fn find_solution(&self, sol: &mut Vec<Vec<u32>>) -> bool {
        if self.solve_maze(0, 0, sol) == false {
            println!("No Route");            
            return false;
        }
        self.print_path(sol);
        return true;
    }
    fn print_path(&self, sol: &Vec<Vec<u32>>) {
        println!();
        for row in sol.iter() {
            for &col in row.iter() {
                if col == 1{
                    print!("🟥");
                }else{
                    print!("⬜");
                }
            }
            println!();
        }
    }
}
fn main() {
    let point: Vec<Vec<u32>> = vec![
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
        vec![0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1,],
        vec![0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];

    let max_row = point.len();
    let mut max_col = usize::MIN;
    for row in point.iter(){
        if max_col < row.len(){
            max_col = row.len();
        }       
    }
    
    let mut new_point = vec![vec![0;max_col];max_row];

    for row in 0..point.len(){
        for col in 0..point[row].len(){
            new_point[row][col] = point[row][col];
        }
    }
    let points = Points::new(new_point.clone());
    let mut sol = point.clone();
    for row in 0..sol.len(){
        for col in 0..sol[row].len(){
            sol[row][col]=0;
        }
    }

    points.find_solution(&mut sol);
}
