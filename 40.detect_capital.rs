//Detect capital in words
struct Solution{}
#[allow(dead_code)]
impl Solution {
    fn detect_capital_use(word: String) -> bool {
        if word.as_bytes().to_vec().iter().all(|x| x >= &65 && x <= &90){
            return true;
        }
        if word.as_bytes().to_vec().iter().all(|x| x >= &97 && x <= &122){
            return true;
        } 
        for (i,val) in word.chars().enumerate(){
            if i == 0{
                if val as u8 >=97 && val as u8 <= 122{
                    return false;
                }          
            }else{
                if val as u8 >=65 && val as u8 <= 90{
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    
}
#[cfg(test)]
mod tests{
    use crate::Solution;
    #[test]
    fn test_detect_capital_use(){
        assert_eq!(Solution::detect_capital_use(String::from("THAILAND")),true);
        assert_eq!(Solution::detect_capital_use(String::from("THaILAND")),false);
    }
}
