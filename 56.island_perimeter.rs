//Find sum perimeter of Island
/*
     ⬜🟥⬜⬜
     🟥🟥🟥⬜
     ⬜🟥⬜⬜
     🟥🟥⬜⬜
     [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]] = 16

     🟥
     [[1]] = 4

     🟥⬜
     [[1,0]] = 4
 */
#[allow(dead_code)]
struct Solution {}
#[allow(dead_code,unused_assignments)]
impl Solution {
    fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut sum_perimeter = 0;
        let mut perimeter = 4;
        let lenght = grid.len();

        for i in 0..lenght {
            for j in 0..grid[i].len() {
                perimeter = 4;
                if grid[i][j] == 1 {
                    if i < lenght - 1 {
                        if grid[i + 1][j] == 1 {
                            perimeter -= 1;
                        }
                    }
                    if i > 0 {
                        if grid[i - 1][j] == 1 {
                            perimeter -= 1;
                        }
                    }
                    if j < grid[i].len() - 1 {
                        if grid[i][j + 1] == 1 {
                            perimeter -= 1;
                        }
                    }
                    if j > 0 {
                        if grid[i][j - 1] == 1 {
                            perimeter -= 1;
                        }
                    }
                } else {
                    perimeter = 0;
                }
                sum_perimeter += perimeter;
            }
        }
        sum_perimeter
    }
}
fn main() {

}
#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_island_perimeter(){
        let grid = vec![vec![0,1,0,0],vec![1,1,1,0],vec![0,1,0,0],vec![1,1,0,0]];
        assert_eq!(Solution::island_perimeter(grid),16);

        let grid = vec![vec![1]];
        assert_eq!(Solution::island_perimeter(grid),4);

        let grid = vec![vec![1,0]];
        assert_eq!(Solution::island_perimeter(grid),4);
    }
}
