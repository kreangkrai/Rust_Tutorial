use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    fn left(mut self, node: TreeNode) -> Self {
        self.left = Some(Rc::new(RefCell::new(node)));
        self
    }
    fn right(mut self, node: TreeNode) -> Self {
        self.right = Some(Rc::new(RefCell::new(node)));
        self
    }
    fn dfs_path(&self) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut stack: Vec<(Vec<i32>, Option<Rc<RefCell<TreeNode>>>)> = Vec::new();
        stack.push((vec![self.val], Some(Rc::new(RefCell::new(self.clone())))));
        while let Some((vec, Some(node))) = stack.pop() {
            let n = node.borrow();
            if n.left.is_none() && n.right.is_none() {
                res.push(vec.clone());
            }
            if let Some(l) = n.left.clone() {
                let mut v = vec.clone();
                v.push(l.borrow().val);
                stack.push((v, Some(l)));
            }
            if let Some(r) = n.right.clone() {
                let mut v = vec.clone();
                v.push(r.borrow().val);
                stack.push((v, Some(r)));
            }
        }
        res
    }
    fn dfs_path_all(&self) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut stack: Vec<(i32, Option<Rc<RefCell<TreeNode>>>)> = Vec::new();
        stack.push((self.val, Some(Rc::new(RefCell::new(self.clone())))));
        while let Some((val, Some(node))) = stack.pop() {
            let n = node.borrow();
            res.push(val);
            if let Some(l) = n.left.clone() {
                stack.push((l.borrow().val, Some(l.clone())));
            }
            if let Some(r) = n.right.clone() {
                stack.push((r.borrow().val, Some(r.clone())));
            }
        }
        res
    }
    fn dfs_sub_left_tree(&self) -> Vec<i32> {
        if self.left.is_some(){
            let tree = TreeNode::new(self.val).left(self.clone().left.unwrap().borrow().clone());
            let res = tree.dfs_path_all();
            return res;
        }
        vec![]
    }
    fn dfs_sub_right_tree(&self) -> Vec<i32> {
        if self.right.is_some(){
            let tree = TreeNode::new(self.val).right(self.clone().right.unwrap().borrow().clone());
            let res = tree.dfs_path_all();
            return res;
        }
        vec![]
    }

    fn dfs_left_side(&self)->Vec<i32>{
        let mut res: Vec<i32> = vec![];
        let mut stack: Vec<(i32, Option<Rc<RefCell<TreeNode>>>)> = Vec::new();
        stack.push((self.val, Some(Rc::new(RefCell::new(self.clone())))));
        while let Some((val, Some(node))) = stack.pop() {
            let n = node.borrow();
            res.push(val);
            if let Some(l) = n.left.clone() {
                stack.push((l.borrow().val, Some(l.clone())));
            }
        }
        res
    }
    fn dfs_right_side(&self)->Vec<i32>{
        let mut res: Vec<i32> = vec![];
        let mut stack: Vec<(i32, Option<Rc<RefCell<TreeNode>>>)> = Vec::new();
        stack.push((self.val, Some(Rc::new(RefCell::new(self.clone())))));
        while let Some((val, Some(node))) = stack.pop() {
            let n = node.borrow();
            res.push(val);
            if let Some(r) = n.right.clone() {
                stack.push((r.borrow().val, Some(r.clone())));
            }
        }
        res
    }
    fn dfs_leaf(&self)->Vec<i32>{
        let mut res: Vec<i32> = vec![];
        let mut stack: Vec<(i32, Option<Rc<RefCell<TreeNode>>>)> = Vec::new();
        stack.push((self.val, Some(Rc::new(RefCell::new(self.clone())))));
        while let Some((val, Some(node))) = stack.pop() {
            let n = node.borrow();
            if n.left.is_none() && n.right.is_none(){
                res.push(val);
            }
            if let Some(l) = n.left.clone() {
                stack.push((l.borrow().val, Some(l.clone())));
            }
            if let Some(r) = n.right.clone() {
                stack.push((r.borrow().val, Some(r.clone())));
            }
        }
        res
    }
    fn bfs(&self) ->Vec<Vec<i32>>{
        let mut res = vec![];
        let mut v = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self.clone());
        while queue.len() > 0 {
            let bound = queue.len() - 1;
            for i in 0..=bound{
                let node = queue.pop_front().unwrap();
                v.push(node.val);
                if i == bound{
                    res.push(v.clone());
                    v = vec![];
                }
                if let Some(l) = node.left.clone() {
                    queue.push_back(l.borrow().clone());
                }
                if let Some(r) = node.right.clone(){
                    queue.push_back(r.borrow().clone());
                }
            }
        }
        res
    }
    fn bfs_all(&self) ->Vec<i32>{
        let mut res = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self.clone());
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            res.push(node.val);
            if let Some(l) = node.left.clone() {
                queue.push_back(l.borrow().clone());
            }
            if let Some(r) = node.right.clone(){
                queue.push_back(r.borrow().clone());
            }
        }
        res
    }

    fn bfs_left_side(&self)->Vec<i32>{
        let mut res = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self.clone());
        while queue.len() > 0 {
            let bound = queue.len() - 1;
            for i in 0..=bound{
                let node = queue.pop_front().unwrap();
                if i == 0{
                    res.push(node.val);
                }
                if let Some(l) = node.left.clone() {
                    queue.push_back(l.borrow().clone());
                }
                if let Some(r) = node.right.clone(){
                    queue.push_back(r.borrow().clone());
                }
            }
        }
        res
    }
    fn bfs_right_side(&self)->Vec<i32>{
        let mut res = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self.clone());
        while queue.len() > 0 {
            let bound = queue.len() - 1;
            for i in 0..=bound{
                let node = queue.pop_front().unwrap();
                if i == bound{
                    res.push(node.val);
                }
                if let Some(l) = node.left.clone() {
                    queue.push_back(l.borrow().clone());
                }
                if let Some(r) = node.right.clone(){
                    queue.push_back(r.borrow().clone());
                }
            }
        }
        res
    }
    fn bfs_inner_side(&self)->Vec<i32>{
        let mut res = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self.clone());
        while queue.len() > 0 {
            let bound = queue.len() - 1;
            for i in 0..=bound{
                let node = queue.pop_front().unwrap();
                if i > 0 && i < bound{
                    res.push(node.val);
                }
                if let Some(l) = node.left.clone() {
                    queue.push_back(l.borrow().clone());
                }
                if let Some(r) = node.right.clone(){
                    queue.push_back(r.borrow().clone());
                }
            }
        }
        res
    }
    fn bfs_leaf(&self)->Vec<i32>{
        let mut res = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self.clone());
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            if node.left.is_none() && node.right.is_none(){
                res.push(node.val);
            }
            if let Some(l) = node.left.clone() {
                queue.push_back(l.borrow().clone());
            }
            if let Some(r) = node.right.clone(){
                queue.push_back(r.borrow().clone());
            }
        }
        res
    }
    fn bfs_left_leaf(&self)->Vec<i32>{
        let mut res = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((self.clone(),false));
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            if node.0.left.is_none() && node.0.right.is_none() && node.1 == true{
                res.push(node.0.val);
            }
            if let Some(l) = node.0.left.clone() {
                queue.push_back((l.borrow().clone(),true));
            }
            if let Some(r) = node.0.right.clone(){
                queue.push_back((r.borrow().clone(),false));
            }
        }
        res
    }
    fn bfs_right_leaf(&self)->Vec<i32>{
        let mut res = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((self.clone(),false));
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            if node.0.left.is_none() && node.0.right.is_none() && node.1 == true{
                res.push(node.0.val);
            }
            if let Some(l) = node.0.left.clone() {
                queue.push_back((l.borrow().clone(),false));
            }
            if let Some(r) = node.0.right.clone(){
                queue.push_back((r.borrow().clone(),true));
            }
        }
        res
    }
    fn invert_tree(&self,node:Option<Rc<RefCell<TreeNode>>>)-> Option<Rc<RefCell<TreeNode>>>{
        if let Some(ref n) = node{
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();

            let mut n = n.borrow_mut();
            n.left = self.invert_tree(right);
            n.right = self.invert_tree(left);
        }
        node
    }
    fn invert(&self) ->TreeNode{
        let node = Some(Rc::new(RefCell::new(self.clone())));
        let res = self.invert_tree(node);
        res.unwrap().borrow().clone()
    }
    fn traversal_inorder(&self,node:Option<Rc<RefCell<TreeNode>>>,res: &mut Vec<i32>){
        if let Some(n) = node{
            self.traversal_inorder(n.borrow().left.clone(), res);
            res.push(n.borrow().val);
            self.traversal_inorder(n.borrow().right.clone(), res);
        }
    }
    fn inorder(&self)->Vec<i32>{
        let tree = self.clone();
        let mut res = vec![];
        let node = Some(Rc::new(RefCell::new(tree)));
        self.traversal_inorder(node,&mut res);
        res
    }
    fn traversal_preorder(&self,node:Option<Rc<RefCell<TreeNode>>>,res: &mut Vec<i32>){
        if let Some(n) = node{
            res.push(n.borrow().val);
            self.traversal_preorder(n.borrow().left.clone(), res);
            self.traversal_preorder(n.borrow().right.clone(), res);
        }
    }
    fn preorder(&self)->Vec<i32>{
        let tree = self.clone();
        let mut res = vec![];
        let node = Some(Rc::new(RefCell::new(tree)));
        self.traversal_preorder(node,&mut res);
        res
    }
    fn traversal_postorder(&self,node:Option<Rc<RefCell<TreeNode>>>,res: &mut Vec<i32>){
        if let Some(n) = node{ 
            self.traversal_postorder(n.borrow().left.clone(), res);
            self.traversal_postorder(n.borrow().right.clone(), res);
            res.push(n.borrow().val);
        }
    }
    fn postorder(&self)->Vec<i32>{
        let tree = self.clone();
        let mut res = vec![];
        let node = Some(Rc::new(RefCell::new(tree)));
        self.traversal_postorder(node,&mut res);
        res
    }
    fn traversal_search(&self,node:Option<Rc<RefCell<TreeNode>>>,val: i32){
        if let Some(n) = node{
            match n.borrow().val.cmp(&val) {
                std::cmp::Ordering::Equal => {
                    println!(" => Found Value [{}]",val);
                }
                std::cmp::Ordering::Less => {
                    if n.borrow().right.is_some(){
                        print!("[{}] -> Right ",n.borrow().val);
                        self.traversal_search(n.borrow().right.clone(), val);
                    }else{
                        print!("[{}] -> Right ",n.borrow().val);
                        self.traversal_search(None, val);
                    }
                }
                std::cmp::Ordering::Greater =>{
                    if n.borrow().left.is_some(){
                        print!("[{}] -> Left ",n.borrow().val);
                        self.traversal_search(n.borrow().left.clone(), val);
                    }else{
                        print!("[{}] -> Left ",n.borrow().val);
                        self.traversal_search(None, val);
                    }
                }
            }
        }else{
            println!(" => Not Found [{}]",val);
        }
    }
    fn search(&self,val:i32){
        let tree = self.clone();
        let node = Some(Rc::new(RefCell::new(tree)));
        self.traversal_search(node, val);
    }

    fn traversal_parent(&self,mut node:Option<Rc<RefCell<TreeNode>>>,val:i32)->Option<Rc<RefCell<TreeNode>>>{
        if let Some(n) = node {
            match n.borrow().val.cmp(&val) {
                std::cmp::Ordering::Equal => {
                    return Some(n.clone());
                }
                std::cmp::Ordering::Less =>{
                    if n.borrow().right.is_some(){
                        node = self.traversal_parent(n.borrow().right.clone(), val);
                    }else{
                        node = self.traversal_parent(None, val);
                    }
                }
                std::cmp::Ordering::Greater => {
                    if n.borrow().left.is_some(){
                        node = self.traversal_parent(n.borrow().left.clone(), val);
                    }else{
                        node = self.traversal_parent(None, val);
                    }
                }
            }
        }else{
            return None;
        }
        node
    }
    fn child_of_parent(&self,val:i32)->Vec<(usize,Vec<i32>)>{
        let node = Some(Rc::new(RefCell::new(self.clone())));
        let child = self.traversal_parent(node, val);

        let mut res:Vec<(usize,i32)> = vec![];
        let mut q = std::collections::VecDeque::new();
        if let Some(c) = &child.clone(){
            q.push_back((0,c.clone()));
            while q.len() > 0 {
                let (level,c) = q.pop_front().unwrap();
                res.push((level,c.borrow().val));
                if let Some(ref l) = c.clone().borrow().left{
                    q.push_back((level + 1 , l.clone()));
                }
                if let Some(ref r) = c.clone().borrow().right{
                    q.push_back((level + 1 ,r.clone()));
                }
            }
            let mut hm:std::collections::HashMap<usize,Vec<i32>> = std::collections::HashMap::new();
            for r in res.into_iter(){
                hm.entry(r.0).or_insert(Vec::new()).push(r.1);
            }
            let mut hm_vec:Vec<(usize,Vec<i32>)> = hm.into_iter().collect();
            hm_vec.sort_by(|a,b| a.0.cmp(&b.0));
            hm_vec
        }else{
            vec![]
        }
    }
    // fn insert(&mut self, val: i32) {
    //     if self.val == val {
    //         return;
    //     }
    //     let node = if val < self.val {
    //         &mut self.left
    //     } else {
    //         &mut self.right
    //     };
    //     match node {
    //         Some(n) => n.borrow_mut().insert(val),
    //         None => {
    //             let new_node = TreeNode {
    //                 val: val,
    //                 left: None,
    //                 right: None,
    //             };
    //             *node = Some(Rc::new(RefCell::new(new_node)));
    //         }
    //     }
    // }
}

fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());
    while queue.len() > 0 {
        let bound = queue.len() - 1;
        for i in 0..=bound{
            let node = queue.pop_front().unwrap();
            if i == bound{
                if let Some(n) = node.clone(){
                    res.push(n.borrow().val);
                }
            }
            if let Some(n) = node.clone(){
                if let Some(l) = n.clone().borrow().left.clone() {
                    queue.push_back(Some(l.clone()));
                }
            }
            if let Some(n) = node.clone(){
                if let Some(r) = n.clone().borrow().right.clone(){
                    queue.push_back(Some(r.clone()));
                }
            }
        }
    }
    res
}
fn main() {
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
    // let mut tree = TreeNode::new(5);
    // tree.insert(3);
    // tree.insert(2);
    // tree.insert(1);
    // tree.insert(4);
    // tree.insert(7);
    // tree.insert(6);
    // tree.insert(8);
    // tree.insert(9);
    let root = TreeNode::new(5)
        .left(
            TreeNode::new(3)
                .left(TreeNode::new(2).left(TreeNode::new(1)))
                .right(TreeNode::new(4)),
        )
        .right(
            TreeNode::new(7)
                .left(TreeNode::new(6))
                .right(TreeNode::new(8).right(TreeNode::new(9))),
    );



    let res = right_side_view(None);
    println!("{:?}",res);

    let dfs = root.dfs_path();
    println!("DFS Traversal Split Path => {:?}", dfs);

    let dfs_all = root.dfs_path_all();
    println!("DFS Traversal Path => {:?}",dfs_all);

    let dfs_sub_left_side = root.dfs_sub_left_tree();
    println!("DFS Sub Left Side => {:?}", dfs_sub_left_side);

    let dfs_sub_right_side = root.dfs_sub_right_tree();
    println!("DFS Sub Right Side => {:?}", dfs_sub_right_side);

    let dfs_left_side = root.dfs_left_side();
    println!("DFS Left Side => {:?}",dfs_left_side);

    let dfs_right_side = root.dfs_right_side();
    println!("DFS Right Side => {:?}",dfs_right_side);
    
    let dfs_leaf = root.dfs_leaf();
    println!("DFS Leaf => {:?}",dfs_leaf);

    let bfs = root.bfs();
    println!("BFS Traversal Split Level => {:?}",bfs);

    let bfs_all = root.bfs_all();
    println!("BFS Traversal => {:?}",bfs_all);

    let bfs_left_side = root.bfs_left_side();
    println!("BFS Left Side => {:?}",bfs_left_side);

    let bfs_right_side = root.bfs_right_side();
    println!("BFS Right Side => {:?}",bfs_right_side);

    let bfs_inner_side = root.bfs_inner_side();
    println!("BFS Inner Side => {:?}",bfs_inner_side);

    let bfs_leaf = root.bfs_leaf();
    println!("BFS Leaf => {:?}",bfs_leaf);
    
    let bfs_left_leaf = root.bfs_left_leaf();
    println!("BFS Left Leaf => {:?}",bfs_left_leaf);

    let bfs_right_leaf = root.bfs_right_leaf();
    println!("BFS Right Leaf => {:?}",bfs_right_leaf);

    let inorder = root.inorder();
    println!("Inorder => {:?}",inorder);

    let preorder = root.preorder();
    println!("Preorder => {:?}",preorder);

    let postorder = root.postorder();
    println!("Postorder => {:?}",postorder);

    print!("Search => ");
    root.search(9);

    let child_of_parent = root.child_of_parent(3);
    println!("Child of Parent => {:?}",child_of_parent);
    if child_of_parent.len() > 0 {
        for (key,val) in child_of_parent.into_iter(){
            print!("Level {:?} => ",key);
            for v in val.into_iter(){
                print!("{:?} ",v);
            }
            println!();
        }
    }else{
        println!("Empty");
    }

    let invert = root.invert();
    println!("Invert Tree => {:#?}",invert);
}
