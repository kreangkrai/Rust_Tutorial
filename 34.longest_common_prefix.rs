//Find the longest common prefix string amongst an array of strings With Rust Programing Language

struct Solution{}
#[allow(dead_code)]
impl Solution {
    fn longest_common_prefix(str:Vec<&str>) -> &str{
        let min = str.iter().map(|x| x.len()).min().unwrap();
        for i in (1..min + 1).rev(){
            let prefix = &str[0][0..i];
            if str.iter().all(|x| x.find(prefix) == Some(0)){
                return prefix;
            }
        }
        ""
    }
}
fn main() {
}
#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_longest_common_prefix(){
        assert_eq!(Solution::longest_common_prefix(vec!["flower","flow","flight"]),"fl");
        assert_eq!(Solution::longest_common_prefix(vec!["dog","racecar","car"]),"");
    }
}
