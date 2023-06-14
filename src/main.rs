use rand::Rng;
use std::{fmt, io};

struct MulFormula {
    multiplicand: u16,
    multiplier: u16,
}

impl MulFormula {
    fn new(multiplicand: u16, multiplier: u16) -> Self {
        Self {
            multiplicand,
            multiplier,
        }
    }
    fn result(&self) -> u32 {
        self.multiplicand as u32 * self.multiplier as u32
    }
}

impl fmt::Display for MulFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} × {} =", self.multiplicand, self.multiplier)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        let q = MulFormula::new(rng.gen_range(10..999), rng.gen_range(1..99));
        let q_fmt = q.to_string();
        let q_rslt = q.result();
        println!("解を答えてください:\n{}", q_fmt);
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("読み取りに失敗");
            let input: i32 = match input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("数値を入力してください:");
                    continue;
                }
            };
            if input < 0 {
                println!("正の数を入力してください:");
                continue;
            } else if q_rslt != input as u32 {
                println!("不正解:\n{} の解は {} ではありません。", q_fmt, input.abs());
                continue;
            } else {
                println!("正解: {} {}", q_fmt, q_rslt);
                println!("================================");
                println!("次の問題");
                break;
            }
        }
    }
}
