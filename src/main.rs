mod colors;
mod info;
mod input;

use colored::Colorize;
use colors::VALID_COLORS;
use input::Input;

struct Settings {
    border_width: usize,
    border_color: String,
    bg: String,
    width: usize,
    height: usize,
}

fn main() {
    let mut settings = Settings {
        border_width: 0,
        border_color: "none".to_string(),
        bg: "white".to_string(),
        width: 30,
        height: 15,
    };

    println!("{}", "Welcome to console box".magenta().bold());
    println!(
        "{}",
        "You'll be asked questions about how the box should look. Eg. Color, Size, etc."
            .bright_white()
            .bold()
    );
    println!("\nHere is a list of colors possible");
    for (index, color) in VALID_COLORS.iter().enumerate() {
        println!(
            "{} {}",
            ((index + 1).to_string() + ".").bright_green(),
            color.bright_blue()
        );
    }
    settings.width = Input::number("Width", settings.width);
    settings.height = Input::number("Height", settings.height);
    settings.border_width = Input::number("Border Width", settings.border_width);

    // if border width is 0 then
    // it means no borders and the
    // border color input should be taken.
    if settings.border_width > 0 {
        settings.border_color = Input::color("Border Color", "blue");
    }

    settings.bg = Input::color("Background", &settings.bg);

    let total_width = settings.border_width + settings.width;
    let total_height = settings.border_width + settings.height;

    println!("\n\n{}", "Box Info:".bold());
    info::print(
        (settings.width, total_width),
        (settings.height, total_height),
        &settings.bg,
        (settings.border_width, &settings.border_color),
    );
    for i in 0..total_height {
        for j in 0..total_width {
            let mut ch: String = String::from("██");
            if (0..settings.border_width).contains(&j)
                || (settings.width..total_width).contains(&j)
                || (0..settings.border_width).contains(&i)
                || (settings.height..total_height).contains(&i)
            {
                ch = ch.color(settings.border_color.as_str()).to_string();
            } else if settings.bg != "none" {
                ch = ch.color(settings.bg.as_str()).to_string();
            } else {
                ch = "  ".to_string();
            }
            print!("{}", ch);
        }
        println!();
    }
}
