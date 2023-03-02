// Queue Data Structure With Rust Programing Language

#[derive(Debug,Clone)]
struct Queue<T> {
    queue: Vec<T>,
}
#[allow(dead_code)]
impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }
    fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }
    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }
    fn length(&self) -> usize {
        self.queue.len()
    }
    fn isempty(&self) -> bool {
        self.queue.is_empty()
    }
    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
    fn last(&self) -> Option<&T>{
        self.queue.last()
    }
    fn front(&self)->Option<&T>{
        self.queue.first()
    }
}
fn main() {
    let mut q:Queue<i32> = Queue::new();
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    q.enqueue(4);
    q.enqueue(5);

    while !q.isempty() {
        println!("Dequue : {}",q.dequeue())
    }
}
