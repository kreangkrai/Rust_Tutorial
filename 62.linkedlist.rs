//Basic Linked List With Rust programing language
#[derive(Debug,Clone)]
#[allow(dead_code)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}
impl Node {
    fn new(val:i32)->Self{
      Node { val: val, next: None }  
    }
}
#[derive(Debug,Clone)]
struct LinkedList{
  head:Option<Box::<Node>>,
}
impl LinkedList {
  fn new()->Self{
    LinkedList{
      head:None
    }
  }
  fn length(&self)-> usize{
    let mut count = 0;
    let mut curr_node = self.head.as_ref();
    while let Some(node) = curr_node  {
        count += 1;
        curr_node = node.next.as_ref().map(|n| n)
    }
    count
  }
  fn get_nth_node_mut(&mut self,begin:usize,end:usize)->Option<&mut Box<Node>>{
    let mut nth_node = self.head.as_mut();
    for _ in begin..end {
      nth_node = match nth_node {
          Some(node) => node.next.as_mut().map(|node|&mut *node),
          None => return None
      }
    }
    nth_node
  }
  fn push_front(&mut self,val:i32){
    let mut new_node = Node::new(val);
    match self.head {
        Some(_) => {
          let curr_node = self.head.take().unwrap();
          new_node.next = Some(curr_node);
          self.head = Some(Box::new(new_node));
        },
        None => self.head = Some(Box::new(new_node)),
    }
  }
  fn push_back(&mut self,val:i32){
    let len = self.length();
    let new_node = Node::new(val);
    if len > 0{
      let last_node = self.get_nth_node_mut(0,len - 1);
      last_node.map(|node| node.next = Some(Box::new(new_node)));    
    }else{
      self.head = Some(Box::new(new_node));
    }
  }
  fn pop_front(&mut self){
    self
      .head
      .take()
      .map(|head| self.head = head.next.map(|node| node));
  }
  fn reverse(&self)-> Option<Box<Node>>{
    let mut prev = None;
    let mut curr_node = self.clone().head.take();
    while let Some(mut curr_node_inner) = curr_node.take() {
        let next = curr_node_inner.next.take();
        curr_node_inner.next = prev.take();
        prev = Some(curr_node_inner);
        curr_node = next;
    }
    prev.take()
  }
  fn pop_back(&mut self){  
    let length = self.length();
    if length > 0 {
      self.reverse()
      .take()
      .map(|head| self.head = head.next.map(|node| node));
      self.head = self.reverse();
    }
  }
  fn print_linkedlist(&self){
    let mut curr_node = self.head.as_ref();
    match curr_node {
        Some(node) => {print!("{:?} ",node.val);},
        None => print!("Empty")
    }
    while let Some(node) = curr_node  {
      match node.next.clone() {
        Some(n) => {
          print!("{:?} ",n.val);
        },
        None =>{}
      }   
      curr_node = node.next.as_ref().map(|n| n);
    }
    println!();
  }
}
fn main() {
  let mut linkedlist:LinkedList = LinkedList::new();
  linkedlist.push_back(9); // 9
  linkedlist.push_back(5); // 9 5
  linkedlist.push_back(7); // 9 5 7
  linkedlist.push_front(1); // 1 9 5 7
  linkedlist.push_back(4); // 1 9 5 7 4
  linkedlist.push_front(0); // 0 1 9 5 7 4
  linkedlist.print_linkedlist();
  linkedlist.pop_front(); // 1 9 5 7 4
  linkedlist.print_linkedlist();
  linkedlist.pop_back(); // 1 9 5 7
  linkedlist.print_linkedlist();
  linkedlist.pop_front(); // 9 5 7
  linkedlist.print_linkedlist();
  linkedlist.pop_back(); // 9 5
  linkedlist.print_linkedlist();
  linkedlist.push_front(99); // 99 9 5
  linkedlist.print_linkedlist();
  linkedlist.pop_back(); // 99 9
  linkedlist.pop_back(); // 99
  linkedlist.print_linkedlist();
  linkedlist.pop_front(); //Empty
  linkedlist.print_linkedlist();
}
