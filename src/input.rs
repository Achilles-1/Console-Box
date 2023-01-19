use crate::colors::VALID_COLORS;
use colored::Colorize;
use std::io::stdin;
pub struct Input;

impl Input {
    pub fn raw(hint: &str, default_value: &str) -> String {
        let mut input = String::new();
        if default_value.trim() != "" {
            println!("{hint} (Default: {}) >", default_value.bright_green());
        } else {
            println!("{hint} >");
        }

        stdin()
            .read_line(&mut input)
            .expect("Error reading the line!");
        if input.trim() == "" && default_value.trim() != "" {
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
