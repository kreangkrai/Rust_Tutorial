// Minimum Coin Change Problem : Greedy Algorithm With Rust programing Language

use std::collections::HashMap;

#[derive(Debug)]
struct Coin {
    coin: u32,
    amount: u32,
}
struct Coins {
    coins: Vec<Coin>,
}
impl Coins {
    pub fn new(coins: Coins) -> Self {
        coins
    }
    fn calculate(&mut self, mut n: u32) -> (HashMap<u32,u32>,&Vec<Coin>,u32){
        let mut check_amount_remain = false;
        let mut hm: HashMap<u32, u32> = HashMap::new();
        while n > 0 && !check_amount_remain {
            for c in 0..self.coins.len() {
                if n >= self.coins[c].coin && self.coins[c].amount > 0 {
                    n = n - self.coins[c].coin;
                    self.coins[c].amount -= 1;
                    *hm.entry(self.coins[c].coin).or_insert(0) += 1;
                    check_amount_remain = self.coins.iter().all(|a| a.amount == 0);
                    break;
                }
            }
        }
        (hm,&self.coins,n)      
    }
}
fn main() {
    let coins: Coins = Coins {
        coins: vec![
            Coin {
                coin: 10,
                amount: 5,
            },
            Coin {
                coin: 5,
                amount: 25,
            },
            Coin {
                coin: 2,
                amount: 10,
            },
            Coin {
                coin: 1,
                amount: 30,
            },
        ],
    };

    let mut ans = Coins::new(coins);
    let n = 199;
    let (hm,coins,m) = ans.calculate(n);

    println!("Answer {:#?}", hm);
    println!("Remain Coins {:#?}", coins);
    println!("Remain {:?}", m);

}
