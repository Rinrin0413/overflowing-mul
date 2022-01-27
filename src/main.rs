use rand::Rng;
use std::io;

struct MulFormula {
    mul_er: u16,
    mul_cand: u16,
}

impl MulFormula {
    pub fn new(mul_er: u16, mul_cand: u16) -> Self {
        Self { mul_er, mul_cand }
    }
    pub fn format(&self) -> String {
        format!("{} × {} =", self.mul_er, self.mul_cand)
    }
    pub fn result(&self) -> u32 {
        self.mul_er as u32 * self.mul_cand as u32
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        let q = MulFormula::new(rng.gen_range(10..999), rng.gen_range(1..99));
        let q_fmt = q.format();
        let q_rslt = q.result();
        println!("解を答えてください:\n{}", q_fmt);
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("読み取りに失敗");
            let mut input: i32 = match input.trim().parse() {
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
                input = i32::abs(input);
                println!("不正解:\n{} の解は {} ではありません。", q_fmt, input);
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
