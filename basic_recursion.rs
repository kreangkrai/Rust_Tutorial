//Basic Recursion Function With Rust programing Language

fn func(x:i32,y:i32,z:i32)->i32{
    if x == 1 && y ==1 {
        return x+y;
    }
    if z < 1{
        return z;
    }
    return x + y + z + func(x-1,y-1,z-2);
}
fn main() {
   let ans = func(7, 8, 12);
   println!("{}",ans);
}

/*
Check Answer step by step

x + y + z + func(x-1, y-1, z-2)
Condition :
  x = 1 and y = 1 : return x + y
  z < 1 : return z

Given x = 7 , y = 8 z = 12

step 1 assign x=7,y=8,z=12
7 + 8 + 12 + func([7-1],[8-1],[12-2])  => (1) //not return

step 2 assign x=6,y=7,z=10
6 + 7 + 10 + func([6-1], [7-1],[10-2])  => (2) //not return

step 3 assign x=5,y=6,z=8
5 + 6 + 8 + func([5-1], [6-1],[8-2])  => (3) //not return

step 4 assign x=4,y=5,z=6
4 + 5 + 6 + func([4-1], [5-1],[6-2])  => (4) //not return

step 5 assign x=3,y=4,z=4
3 + 4 + 4 + func([3-1], [4-1],[4-2])  => (5) //not return

step 6 assign x=2,y=3,z=2
2 + 3 + 2 + func([2-1], [3-1],[2-2])  => (6) //not return

step 7 assign x=1,y=2,z=0
1 + 2 + 0 + func([1-1], [2-1],[0-2]) => (7) // return 0 (z=0) because z < 1

assign value each backward step

(7) = 0
(6) = [2 + 3 + 2 + (7)] = [2 + 3 + 2 + 0] = 7
(5) = [3 + 4 + 4 + (6)] = [3 + 4 + 4 + 7] = 18
(4) = [4 + 5 + 6 + (5)] = [4 + 5 + 6 +18] = 33
(3) = [5 + 6 + 8 + (4)] = [5 + 6 + 8 + 33] = 52
(2) = [6 + 7 + 10 + (3)] = [6 + 7 + 10 + 52] = 75
(1) = [7 + 8 + 12 + (2)] = [7 + 8 + 12 + 75] = 102

Answer  = 102

 */
