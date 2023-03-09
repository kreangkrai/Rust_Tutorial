//Basic Linked List With Rust programing language
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Employee {
    empid: u32,
    name: &'static str,
    department: &'static str,
}
#[derive(Debug, Clone)]
enum Address {
    Address(Box<Node>),
    Nil,
}
#[derive(Debug, Clone)]
struct Node {
    emp: Employee,
    next: Address,
}
impl Node {
    fn new(emp: Employee) -> Self {
        Node {
            emp: emp,
            next: Address::Nil,
        }
    }
    fn append(&mut self, emp: Employee) {
        match self.next {
            Address::Address(ref mut next_address) => {
                next_address.append(emp);
            }
            Address::Nil => {
                let node = Node {
                    emp: emp,
                    next: Address::Nil,
                };
                self.next = Address::Address(Box::new(node))
            }
        }
    }
    fn len(&self) -> u32 {
        let mut length = 1;
        match self.next {
            Address::Address(ref next_address) => length += next_address.len(),
            Address::Nil => {}
        }
        return length;
    }
    fn print_list(&self) {
        print!("{:#?} => ", self.emp);
        match self.next {
            Address::Address(ref next_address) => next_address.print_list(),
            Address::Nil => {
                println!("Nil")
            }
        }
    }
}
fn main() {
    let mut head = Node::new(Employee {
        empid: 1,
        name: "A",
        department: "HQ",
    });
    head.append(Employee {
        empid: 2,
        name: "B",
        department: "HQ",
    });
    head.append(Employee {
        empid: 3,
        name: "C",
        department: "ACF",
    });
    head.append(Employee {
        empid: 4,
        name: "D",
        department: "ACF",
    });

    head.print_list();
    println!("Length {}",head.len());
}
