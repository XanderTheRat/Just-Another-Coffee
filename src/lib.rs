use colored::Colorize;

pub fn print_colored_coffee() {
    const ASCII : &str = include_str!("../coffee");
    for line in ASCII.lines() {
        for c in line.chars() {
            let colored_char = match c {
            	// To change color, edit the truecolor values bellow.
                '▓' => c.to_string().truecolor(111, 78, 55),
                '▒' => c.to_string().truecolor(255, 255, 255),
                '░' => c.to_string().truecolor(200, 200, 200),
                '\0'..='\u{1f}' | '!'..='\u{d7ff}' | '\u{e000}'..='\u{10ffff}' => c.to_string().into(),
                ' ' => c.to_string().into(),
            };
            print!("{}", colored_char);
        }
        println!();
    }
}