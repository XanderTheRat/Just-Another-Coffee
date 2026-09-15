use colored::Colorize;

pub fn print_coffee(gm: bool) {
    const ASCII : &str = include_str!("../../coffee");
    if !(gm) {
        println!("{}", ASCII);
    } else {
        // Doesn\'t need modification if you edit the ascii art.
        for (y, line) in ASCII.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let r = (x.saturating_mul(4) % 255) as u8;
                let g = (y.saturating_mul(12) % 255) as u8;
                let b = 150; 

                print!("{}", c.to_string().truecolor(r, g, b));
            }
            println!();
        }
    }
}

pub fn print_colored_coffee() {
    const ASCII : &str = include_str!("../../coffee");
    for line in ASCII.lines() {
        for c in line.chars() {
            let colored_char = match c {
            	// To change color, edit the truecolor values bellow.
                // If your ASCII art use different symbol, change it bellow.
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