//Transportation Problem (Least Cost Cell Method) With Rust Programing Language
#[derive(Debug, Clone)]
struct Data {
    value: i32,
    result: i32,
    flag: bool,
}
#[derive(Debug, Clone)]
struct LeastCostCell {
    supply: Vec<i32>,
    demand: Vec<i32>,
    values: Vec<Vec<Data>>,
}
impl LeastCostCell {
    fn new(s: Vec<i32>, d: Vec<i32>, v: Vec<Vec<Data>>) -> Self {
        LeastCostCell {
            supply: s,
            demand: d,
            values: v,
        }
    }
    fn minimum_value_row(&self, row: usize) -> usize {
        let mut min = i32::MAX;
        let mut index = 0;
        for (i, v) in self.values[row as usize].clone().into_iter().enumerate() {
            if !v.flag {
                if v.value < min {
                    min = v.value;
                    index = i;
                }
            }
        }
        return index;
    }
    fn calculate(&mut self) {
        for (i, _) in self.values.clone().into_iter().enumerate() {
            while self.supply[i] > 0 {
                let index_min = self.minimum_value_row(i);
                if self.supply[i] > self.demand[index_min] {
                    self.values[i][index_min].result = self.demand[index_min];
                    self.demand[index_min] -= self.values[i][index_min].result;
                    self.supply[i] -= self.values[i][index_min].result;
                    self.values[i][index_min].flag = true;
                } else {
                    self.values[i][index_min].result = self.supply[i];
                    self.demand[index_min] -= self.values[i][index_min].result;
                    self.supply[i] -= self.values[i][index_min].result;
                    self.values[i][index_min].flag = true;
                }
            }
        }
    }
    fn print(&self) {
        println!("==========================================");
        for (i, v) in self.values.clone().into_iter().enumerate() {
            print!(" |");
            for (j, val) in v.clone().into_iter().enumerate() {
                if val.result > 0 {
                    let f = format!("{:>8}", format!("({}*{})", val.result, val.value));
                    print!("{}", f);
                } else {
                    let f = format!("{:>8}", val.value);
                    print!("{}", f);
                }

                if j < v.len() - 1 {
                    print!(" |");
                } else {
                    println!(" |");
                }
            }
            if i < self.values.len() - 1 {
                println!("------------------------------------------");
            }
        }
        println!("==========================================");

        let mut sum = 0;
        let mut str = vec![];
        for v in self.values.clone().into_iter() {
            for val in v.clone().into_iter() {
                if val.result > 0 {
                    sum += val.result * val.value;
                    str.push(format!("({}*{})", val.result, val.value));
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

    let mut values: Vec<Vec<Data>> = Vec::new();
    let v1 = vec![
        Data {
            value: 3,
            result: 0,
            flag: false,
        },
        Data {
            value: 1,
            result: 0,
            flag: false,
        },
        Data {
            value: 7,
            result: 0,
            flag: false,
        },
        Data {
            value: 4,
            result: 0,
            flag: false,
        },
    ];
    let v2 = vec![
        Data {
            value: 2,
            result: 0,
            flag: false,
        },
        Data {
            value: 6,
            result: 0,
            flag: false,
        },
        Data {
            value: 5,
            result: 0,
            flag: false,
        },
        Data {
            value: 9,
            result: 0,
            flag: false,
        },
    ];
    let v3 = vec![
        Data {
            value: 8,
            result: 0,
            flag: false,
        },
        Data {
            value: 3,
            result: 0,
            flag: false,
        },
        Data {
            value: 3,
            result: 0,
            flag: false,
        },
        Data {
            value: 2,
            result: 0,
            flag: false,
        },
    ];
    values.push(v1);
    values.push(v2);
    values.push(v3);

    let mut lcc = LeastCostCell::new(supply, demand, values);
    lcc.calculate();
    lcc.print();
}
