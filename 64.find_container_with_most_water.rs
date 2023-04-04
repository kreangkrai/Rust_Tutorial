//Find Container With Most Water
/*
8|     |___________________|________
7|     |                   |       |
6|     |   |               |       |
5|     |   |       |       |       |
4|     |   |       |   |   |       |
3|     |   |       |   |   |   |   |
2|     |   |   |   |   |   |   |   |
1|__|__|___|___|___|___|___|___|___|_   Max => ( 7 * 7 ) = 49

2|   ___
1|__|___|_  Max => (1 * 1) = 1

 */
#[allow(dead_code)]
struct Solution {}
#[allow(dead_code)]
impl Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut max = i32::MIN;
        let mut i = 0;
        let mut j = len - 1;
        while i < j {
            if height[i] <= height[j] {
                let area = height[i] * (j-i) as i32;
                if area > max {
                    max = area;
                }
                i += 1;
            }else if height[i] > height[j] {
                let area = height[j] * (j-i) as i32;
                if area > max {
                    max = area;
                }
                j -= 1;
            }
        }
        max
    }
}
  
fn main() {
}
#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_max_area(){
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]),49);
        assert_eq!(Solution::max_area(vec![1,1]),1);
    }
}
