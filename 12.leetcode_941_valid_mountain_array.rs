//Leetcode 941 Valid Mountain Array
pub struct Solution{}
fn find_max(arr:&Vec<i32>) ->(usize,i32){
    let mut max = 0;
    let mut index = 0;
    for (i,&v) in arr.into_iter().enumerate(){
        if max < v{
            max = v;
            index = i;
        }
    }
    return (index,max);
}
impl Solution {
     pub fn valid_mountain_array(arr:Vec<i32>) -> bool{
        let len = arr.len();
        let (index,m) = find_max(&arr);
        let mut b = false;
        if index == len - 1 || index == 0{
            return false;
        } 
        if len > 2{
            for i in 0..arr.len() - 1{
                if arr[i] == m{
                    b = true;
                }
                if !b{
                    if arr[i] >= arr[i+1]{
                        return false;
                    }
                }else{
                    if arr[i] <= arr[i+1]{
                        return false;
                    }
                }
            }
        }else{
            return false;
        }
        return true;
    }
}

fn main() {

}
#[cfg(test)]
mod tests{
    use crate::Solution;
    #[test]
    fn test_valid_mountain_array(){ 
        assert_eq!(Solution::valid_mountain_array(vec![2,1]),false);
        assert_eq!(Solution::valid_mountain_array(vec![3,5,5]),false);
        assert_eq!(Solution::valid_mountain_array(vec![0,3,2,1]),true);
    }
}
