use colored::Colorize;
fn print_item(name: &str, value: String) {
    let val = if value == "0" { "none" } else { value.as_str() };
    println!("{}: {}", name.bright_blue(), val.bright_green());
}

pub fn print(
    width: (usize, usize),
    height: (usize, usize), // (user width, total width)
    bg: &String,

    border: (usize, &String), // (border width, border color)
) {
    print_item("Inner Width", width.0.to_string());
    print_item("Inner Height", height.0.to_string());
    println!();

    print_item("Total Width", width.1.to_string());
    print_item("Total Height", height.1.to_string());

    println!();
    print_item("Background Color", bg.to_string());
    println!();
    print_item("Border Width", border.0.to_string());
    print_item("Border Color", border.1.to_string());
}
