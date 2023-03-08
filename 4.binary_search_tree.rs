// Binary Search Tree With Rust Programing Language
//                 Employee
//                   5
//                /    \
//               3      8
//              / \    / \
//             2   4  7   9
//            /      /
//           1      6

use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Debug)]
#[allow(dead_code)]
struct Employee {
    id: u32,
    name: String,
    department: String,
}
#[derive(Debug)]
pub enum BST<Employee> {
    Leaf {
        value: Employee,
        left: Box<BST<Employee>>,
        right: Box<BST<Employee>>,
    },
    Empty,
}
#[allow(dead_code, non_shorthand_field_patterns)]
impl BST<Employee> {
    fn new() -> Self {
        BST::Empty
    }
    fn create(emp: Employee) -> Self {
        BST::Leaf {
            value: emp,
            left: Box::new(BST::Empty),
            right: Box::new(BST::Empty),
        }
    }
    fn insert(&mut self, new_emp: Employee) {
        match self {
            BST::Leaf {
                value: ref emp,
                left: ref mut left,
                right: ref mut right,
            } => match new_emp.id.cmp(&emp.id) {
                Ordering::Less => left.insert(new_emp),
                Ordering::Greater => right.insert(new_emp),
                _ => {}
            },
            BST::Empty => {
                *self = BST::create(new_emp);
            }
        }
    }
    fn is_empty(&self) -> bool {
        match self {
            BST::Empty => true,
            BST::Leaf { .. } => false,
        }
    }
    fn find(&self, find_emp: u32) {
        match self {
            BST::Leaf {
                value: ref emp,
                left: ref left,
                right: ref right,
            } => match find_emp.cmp(&emp.id) {
                Ordering::Less => {
                    println!("EmpId : {} -> Left", &emp.id);
                    left.find(find_emp)
                }
                Ordering::Greater => {
                    println!("EmpId : {} -> Right", &emp.id);
                    right.find(find_emp)
                }
                Ordering::Equal => println!("Found : {:?}", &emp),
            },
            BST::Empty => println!("Not Found"),
        }
    }
}

fn main() {
    let mut bst = BST::new();
    bst.insert(Employee {
        id: 5,
        name: "A".to_string(),
        department: "ACF".to_string(),
    });
    bst.insert(Employee {
        id: 3,
        name: "B".to_string(),
        department: "CES".to_string(),
    });
    bst.insert(Employee {
        id: 8,
        name: "C".to_string(),
        department: "CES".to_string(),
    });
    bst.insert(Employee {
        id: 4,
        name: "D".to_string(),
        department: "ACF".to_string(),
    });
    bst.insert(Employee {
        id: 7,
        name: "E".to_string(),
        department: "CES".to_string(),
    });
    bst.insert(Employee {
        id: 2,
        name: "F".to_string(),
        department: "HQ".to_string(),
    });
    bst.insert(Employee {
        id: 9,
        name: "G".to_string(),
        department: "CES".to_string(),
    });
    bst.insert(Employee {
        id: 1,
        name: "H".to_string(),
        department: "HBO".to_string(),
    });
    bst.insert(Employee {
        id: 6,
        name: "I".to_string(),
        department: "CES".to_string(),
    });
    
    bst.find(6);
}
