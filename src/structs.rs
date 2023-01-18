use colored::Colorize;
use std::io::stdin;
pub const VALID_COLORS: [&str; 19] = [
    "none",
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "purple",
    "cyan",
    "white",
    "bright black",
    "bright red",
    "bright green",
    "bright yellow",
    "bright blue",
    "bright magenta",
    "bright purple",
    "bright cyan",
    "bright white",
];

pub struct Input;

impl Input {
    pub fn raw(hint: &str, default_value: &str) -> String {
        let mut input = String::new();
        println!("{hint} (Default: {}) > ", default_value.bright_green());

        stdin()
            .read_line(&mut input)
            .expect("Error reading the line!");
        if input.trim() == "" {
            println!(
                "{}{}",
                "Using default value: ".blue(),
                default_value.to_string().bright_green()
            );
            return default_value.to_string();
        }
        return input.trim().to_string();
    }
}

impl Input {
    pub fn color(hint: &str, default_value: &str) -> String {
        loop {
            let inp = Input::raw(hint, default_value).trim().to_lowercase();
            if VALID_COLORS.contains(&inp.as_str()) {
                break inp;
            }
            println!("{}", "Input isn't a valid color.".red());

            continue;
        }
    }

    pub fn number(hint: &str, default_value: usize) -> usize {
        loop {
            let inp = Input::raw(hint, default_value.to_string().as_str())
                .trim()
                .to_lowercase();
            match inp.parse::<usize>() {
                Ok(n) => break n,
                Err(_) => {
                    println!("{}", "Input isn't a number.".red());
                    continue;
                }
            };
        }
    }
}

pub struct Settings {
    pub border_width: usize,
    pub border_color: String,
    pub bg: String,
    pub width: usize,
    pub height: usize,
}
