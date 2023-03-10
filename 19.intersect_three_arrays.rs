//Intersection of Three Arrays With Rust Programing Language

//[4, 9, 5, 8, 9, 4]
//[9, 4, 9, 8, 5, 4]
//[9, 4, 7]

// Answer [9,4]

use std::collections::HashMap;
fn intersect(nums1:Vec<i32>,nums2:Vec<i32>,nums3:Vec<i32>) -> Vec<i32>{
    let mut hm:HashMap<i32,i32> = HashMap::new();
    let mut vec = vec![];
    for n in nums1.into_iter(){
        *hm.entry(n).or_insert(0) += 1;
    }
    for n in nums2.into_iter(){
        match hm.get_mut(&n) {
            Some(val) => {
                if *val > 0{
                    vec.push(n);
                    *val -=1;
                }
            },
            None=>{}
        }
    }

    let mut hm:HashMap<i32,i32> = HashMap::new();
    let mut ans_vec = vec![];
    for n in vec.into_iter(){
        *hm.entry(n).or_insert(0) += 1;
    }
    for n in nums3.into_iter(){
        match hm.get_mut(&n) {
            Some(val) => {
                if *val > 0{
                    ans_vec.push(n);
                    *val -=1;
                }
            },
            None =>{}
        }
    }
    return ans_vec;
}
fn main() {
    let nums1 = vec![4, 9, 5, 8, 9, 4];
    let nums2 = vec![9, 4, 9, 8, 5, 4];
    let nums3 = vec![9, 4, 7];
    let vec = intersect(nums1, nums2, nums3);
    println!("{:?}",vec);
}
