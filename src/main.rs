use colored::*;
use rand::Rng;
use std::{fmt, io};
use toml::Value;

struct Localization {
    /// Non-English language.
    lang: Option<Value>,
    /// English is default language
    /// so it is used when there is no corresponding localization.
    english: Value,
}

impl Localization {
    fn init() -> Self {
        Self { 
            lang: LANG_FILE.map(|f| parse_lang_file(f)),
            english: parse_lang_file(ENG_LANG_FILE),
        }
    }

    /// Returns language specified at compile time.
    /// 
    /// If corresponding localization is not found, returns English localization.
    /// If English localization is also not found, returns key itself.
    fn get<'a>(&'a self, key: &'a str) -> &str {
        // If English, skip.
        if let Some(lang) = &self.lang {
            // If failed to get value with key, skip.
            if let Some(value) = lang.get(key) {
                // If value is not string, skip.
                if let Some(value) = value.as_str() {
                    return value;
                }
            }
        }

        if let Some(value) = self.english.get(key) {
            value.as_str().unwrap_or(key)
        } else {
            key
        }
    }
}

fn parse_lang_file(lang_file: &str) -> Value {
    lang_file.parse::<Value>().unwrap_or_else(|why| {
        eprintln!("{}", format!("Failed to parse language file:\n{}", why).red());
        std::process::exit(1);
    })
}

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

const LANG_FILE: Option<&str> = if cfg!(feature = "ja") {
    Some(include_str!("lang/ja.toml"))
} else {
    None
};
const ENG_LANG_FILE: &str = include_str!("lang/en.toml");

fn main() {
    let lang = Localization::init();

    let mut rng = rand::thread_rng();
    loop {
        let problem = Problem::new(rng.gen_range(10..999), rng.gen_range(1..99));
        let formula = problem.to_string();
        let answer = problem.answer();
        println!("{}", formula.blue().bold());
        loop {
            let mut input = String::new();
            if let Err(why) = io::stdin().read_line(&mut input) {
                eprintln!("{}: {}", lang.get("error_readLine"), why);
                continue;
            }
            let input: u32 = match input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("{}", lang.get("error_parse")/*, i32::MAX*/);
                    continue;
                }
            };
            if input == answer {
                println!("{} {} {}", lang.get("correct").green(), formula, answer);
                println!("================================");
                println!("{}:", lang.get("nextProblem"));
                break;
            } else {
                println!("{}", lang.get("incorrect").red());
                continue;
            }
        }
    }
}
