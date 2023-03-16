//Remove Max Two Element to Make the Array Strictly Increasing With Rust Programing Language
#[allow(dead_code)]
fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] < w[1])
}
struct Solution {}
#[allow(dead_code)]
impl Solution {
    fn can_be_increasing(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            let mut n = nums.clone();
            n.remove(i);
            let b = is_sorted(&n);
            if b {
                return true;
            }
            for j in (i + 1)..n.len() {
                let mut temp_n = n.clone();
                temp_n.remove(j);
                let b = is_sorted(&temp_n);
                if b {
                    return true;
                }              
            }
        }
        false
    }
}

fn main() {

}
#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_can_be_increasing() {
        assert_eq!(Solution::can_be_increasing(vec![1, 2, 1, 5, 2, 6]), true);
        assert_eq!(Solution::can_be_increasing(vec![1, 2, 1, 5, 2, 5]), false);
        assert_eq!(Solution::can_be_increasing(vec![1, 2, 1, 5, 2, 3]), false);
        assert_eq!(Solution::can_be_increasing(vec![1, 1, 1]), true);
        assert_eq!(Solution::can_be_increasing(vec![1, 1, 1, 1]), false);
        assert_eq!(Solution::can_be_increasing(vec![1]), true);
        assert_eq!(Solution::can_be_increasing(vec![1, 2, 2]), true);
    }
}
