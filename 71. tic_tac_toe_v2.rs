use std::io::stdin;
use std::collections::VecDeque;
struct TICTACTOE {
    first:&'static str,
    arr:Vec<&'static str>,
    o_queue:VecDeque<usize>,
    x_queue:VecDeque<usize>,
}
impl TICTACTOE {
    fn new(first:&'static str) -> Self{
        TICTACTOE{
            first :first,
            arr : vec!["1","2","3","4","5","6","7","8","9"],
            o_queue : VecDeque::new(),
            x_queue : VecDeque::new()
        }
    }
    fn print(&self){
        println!();
        for i in 0..3{
            for j in (i*3)..(i*3 + 3){  
                if self.arr[j] == "X"{
                    print!("\x1b[91m {} \x1b[0m",self.arr[j]);
                }else if self.arr[j] == "O"{
                    print!("\x1b[93m {} \x1b[0m",self.arr[j]);
                }else{
                    print!(" {} ",self.arr[j]);
                }
                if j < (i*3) + 2 {
                    print!("|");
                }
                if j == (i*3) + 2{
                    println!("");
                }
            }
            if  i < 2 {
                println!("-----------");
            }
        }
        println!();
    }
    fn is_won(&self)->bool{
        for tmp in 0..3 {
            if self.arr[tmp] == self.arr[tmp + 3] && self.arr[tmp] == self.arr[tmp + 6] {
                return true;
            }
    
            let tmp = tmp * 3;
    
            if self.arr[tmp] == self.arr[tmp + 1] && self.arr[tmp] == self.arr[tmp + 2] {
                return true;
            }
        }
    
        if (self.arr[0] == self.arr[4] && self.arr[0] == self.arr[8])
            || (self.arr[2] == self.arr[4] && self.arr[2] == self.arr[6])
        {
            return true;
        }
    
        false
    }
    fn is_valid(&self,position:usize)->bool{
        if position == 0 || position > 9{
            return false;
        }
        let check = self.arr[position-1];
        if check != "X" && check != "O"{
            return true;
        }
        return false;
    }
    fn is_over(&self)->bool{
        let check = self.arr.clone().into_iter().all(|f| (f == "X" || f == "O"));
        if check {
            return true;
        }
        return false;
    }
    fn play(&mut self){
        self.print();
        let mut p = self.first;
        let temp = self.arr.clone();
        loop {
            println!("Player '{}'" ,p);
            let remain_position = self.arr.clone().into_iter().filter(|p| *p != "X" && *p != "O").collect::<Vec<_>>();
            println!("Remain Position {:?}",remain_position);
            let mut str_in = String::new();
            stdin().read_line(&mut str_in).unwrap();
            let position:usize = str_in.trim().parse::<usize>().expect("Incorrect Format , Number Only");            
            if self.is_valid(position){

                if self.o_queue.len() > 3{
                    let o = self.o_queue.pop_front().unwrap();
                    self.arr[o-1] = temp[o-1];
                }
                if self.x_queue.len() > 3{
                    let x = self.x_queue.pop_front().unwrap();
                    self.arr[x-1] = temp[x-1];
                }
                
                self.arr[position-1] = p;
                self.print();
        
                if self.is_won(){
                    println!("'{}' is Won",p);
                    break;
                }
                if self.is_over(){
                    println!("Game Over!!!");
                    break;
                }
               
                if p == "X" {
                    p = "O";
                    self.o_queue.push_back(position);                   
                }else{
                    p = "X";
                    self.x_queue.push_back(position);                    
                }              
            }else{
                println!("'{}' try again",p);
            }
        }
    }
}

fn main() {
    let mut game = TICTACTOE::new("O");
    game.play();
}
