//Binary Tree
/*
                    5
                 /    \
               3        7
             /  \      /  \ 
            2    4    6    8
           /                \
          1                  9
 */

use std::collections::{VecDeque, HashMap};
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}
impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    fn left(mut self, node: TreeNode) -> Self {
        self.left = Some(Box::new(node));
        self
    }
    fn right(mut self, node: TreeNode) -> Self {
        self.right = Some(Box::new(node));
        self
    }
    fn bfs(&self) -> Vec<(usize,Vec<i32>)>{
        let mut res:Vec<(usize,i32)> = vec![];
        let mut q = VecDeque::new();
        q.push_back((0, self));
        while q.len() > 0 {
            let (level, node) = q.pop_front().unwrap();
        
            res.push((level,node.val));
            
            if let Some(ref l) = node.left {
                q.push_back((level + 1, l));
            }
            if let Some(ref r) = node.right {
                q.push_back((level + 1, r));
            }       
        }
        let mut hm:HashMap<usize,Vec<i32>> = HashMap::new();
        for r in res.into_iter(){
            hm.entry(r.0).or_insert(Vec::new()).push(r.1);
        }

        let mut hm_vec:Vec<(usize,Vec<i32>)> = hm.into_iter().collect();
        hm_vec.sort_by(|a,b| a.0.cmp(&b.0));
        hm_vec 
    }
    fn traversal_inorder(&self,node:Option<Box<TreeNode>>,res:&mut Vec<i32>){
        if let Some(n) = node{
            self.traversal_inorder(n.left.clone(),res);
            res.push(n.val);
            self.traversal_inorder(n.right.clone(),res);
        }
    }
    fn inorder(&self)->Vec<i32>{
         let mut res = vec![];
         let node = Some(Box::new(self.clone()));
         self.traversal_inorder(node,&mut res);       
         res
    }
    fn traversal_preorder(&self,node:Option<Box<TreeNode>>,res:&mut Vec<i32>){
        if let Some(n) = node{
            res.push(n.val);
            self.traversal_preorder(n.left.clone(),res);       
            self.traversal_preorder(n.right.clone(),res);
        }
    }
    fn preorder(&self)->Vec<i32>{
        let mut res = vec![];
        let node = Some(Box::new(self.clone()));
        self.traversal_preorder(node,&mut res);       
        res
   }
   fn traversal_postorder(&self,node:Option<Box<TreeNode>>,res:&mut Vec<i32>){
        if let Some(n) = node{            
            self.traversal_postorder(n.left.clone(),res);   
            self.traversal_postorder(n.right.clone(),res);
            res.push(n.val);
        }
    }
    fn postorder(&self)->Vec<i32>{
        let mut res = vec![];
        let node = Some(Box::new(self.clone()));
        self.traversal_postorder(node,&mut res);       
        res
    }
}
fn main() {
    let root = TreeNode::new(5)
        .left(TreeNode::new(3)
            .left(TreeNode::new(2)
                .left(TreeNode::new(1)))
            .right(TreeNode::new(4)))
        .right(TreeNode::new(7)
            .left(TreeNode::new(6))
            .right(TreeNode::new(8)
                .right(TreeNode::new(9)))
    );

    let res = root.bfs();
    println!("=============== BFS ==================");
    for (key,value) in res.into_iter(){
        print!("Level {:?} => ", key);
        for v in value.into_iter(){
            print!("{:?} ",v);
        }
        println!();
    }
    println!("=============== DFS ==================");
    let inorder = root.inorder();
    println!("In   Order {:?}",inorder);

    let preorder = root.preorder();
    println!("Pre  Order {:?}",preorder);

    let postoerder = root.postorder();
    println!("Post Order {:?}",postoerder);
}
