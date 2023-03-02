// Stack Data Structure With Rust Programing Language

#[derive(Debug, Clone)]
struct Stack<T> {
    stack: Vec<T>,
}
#[allow(dead_code)]
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    fn push(&mut self, item: T) {
        self.stack.push(item)
    }
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    fn length(&self) -> usize {
        self.stack.len()
    }
    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
    fn read(&mut self) -> Vec<T> {
        let mut jobs = vec![];
        while !self.stack.is_empty() {
            let j = self.pop().unwrap();
            jobs.push(j);
        }
        jobs
    }
}
fn main() {
    let mut s:Stack<i32> = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    s.push(4);
    s.push(5);

    while !s.is_empty() {
        println!("POP : {}",s.pop().unwrap())
    }
}
