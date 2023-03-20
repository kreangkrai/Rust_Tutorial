//Transportation Problem (NorthWest Corner Method) With Rust Programing Language
#[derive(Debug, Clone)]
struct NorthWestCorner {
    supply: Vec<i32>,
    demand: Vec<i32>,
    values: Vec<Vec<i32>>,
    matrix: Vec<Vec<i32>>,
}
impl NorthWestCorner {
    fn new(s: Vec<i32>, d: Vec<i32>, v: Vec<Vec<i32>>) -> Self {
        NorthWestCorner {
            supply: s.clone(),
            demand: d.clone(),
            values: v,
            matrix: vec![vec![0; d.len()]; s.len()],
        }
    }
    fn sum_current_col(&self, col: usize) -> i32 {
        let mut sum = 0;
        for i in 0..self.matrix.len() {
            for j in i..self.matrix[i].len() {
                if i == col {
                    sum += self.matrix[col][j];
                }
            }
        }
        return sum;
    }
    fn sum_current_row(&self, row: usize) -> i32 {
        let mut sum = 0;
        for i in 0..self.matrix.len() {
            sum += self.matrix[i][row];
        }
        return sum;
    }
    fn calculate(&mut self) {
        for (i, m) in self.matrix.clone().into_iter().enumerate() {
            let sum_cur_col = self.sum_current_col(i);
            for (j, _) in m.into_iter().enumerate() {
                let sum_cur_row = self.sum_current_row(j);
                if sum_cur_col < self.demand[j] || sum_cur_row < self.supply[i] {
                    if self.supply[i] > self.demand[j] {
                        self.matrix[i][j] = self.demand[j] - sum_cur_col;
                        self.demand[j] -= self.matrix[i][j];
                        self.supply[i] -= self.matrix[i][j];
                    } else {
                        self.matrix[i][j] = self.supply[i] - sum_cur_row;
                        self.demand[j] -= self.matrix[i][j];
                        self.supply[i] -= self.matrix[i][j];
                    }
                } else {
                    break;
                }
            }
        }
    }
    fn print(&self) {
        println!("=========================================");
        for (i, m) in self.matrix.clone().into_iter().enumerate() {
            print!("|");
            for (j, val) in m.clone().into_iter().enumerate() {
                if val > 0 {
                    let f = format!("{:>8}", format!("({}*{})", val, self.values[i][j]));
                    print!("{}", f);
                } else {
                    let f = format!("{:>8}", val);
                    print!("{}", f);
                }

                if j < m.len() - 1 {
                    print!(" |");
                } else {
                    println!(" |");
                }
            }
            if i < self.values.len() - 1 {
                println!("-----------------------------------------");
            }
        }
        println!("=========================================");

        let mut sum = 0;
        let mut str = vec![];
        for (i, m) in self.matrix.clone().into_iter().enumerate() {
            for (j, val) in m.clone().into_iter().enumerate() {
                if val > 0 {
                    sum += val * self.values[i][j];
                    str.push(format!("({}*{})", val, self.values[i][j]));
                }
            }
        }

        for (i, s) in str.iter().enumerate() {
            if i < str.len() - 1 {
                print!("{} + ", s);
            } else {
                println!("{} = {}", s, sum);
            }
        }
    }
}
fn main() {
    let supply = vec![300, 400, 500];
    let demand = vec![250, 350, 400, 200];

    let values = vec![vec![3, 1, 7, 4], vec![2, 6, 5, 9], vec![8, 3, 3, 2]];
    let mut nwc = NorthWestCorner::new(supply, demand, values);
    nwc.calculate();
    nwc.print();
}
