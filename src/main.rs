use rand::Rng;
use std::{fmt, io};

struct Problem {
    multiplicand: u16,
    multiplier: u16,
}

impl Problem {
    fn new(multiplicand: u16, multiplier: u16) -> Self {
        Self {
            multiplicand,
            multiplier,
        }
    }
    fn answer(&self) -> u32 {
        self.multiplicand as u32 * self.multiplier as u32
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} × {} =", self.multiplicand, self.multiplier)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        let problem = Problem::new(rng.gen_range(10..999), rng.gen_range(1..99));
        let formula = problem.to_string();
        let answer = problem.answer();
        println!("{}", formula);
        loop {
            let mut input = String::new();
            if let Err(why) = io::stdin().read_line(&mut input) {
                eprintln!("Read line error: {}", why);
                continue;
            }
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
            } else if answer != input as u32 {
                println!("不正解:\n{} の解は {} ではありません。", formula, input.abs());
                continue;
            } else {
                println!("正解: {} {}", formula, answer);
                println!("================================");
                println!("次の問題");
                break;
            }
        }
    }
}
