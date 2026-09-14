use clap::Parser;
use colored::Colorize;
use std::io;


fn print_colored_coffee() {
    const ASCII : &str = include_str!("../../coffee");
    for line in ASCII.lines() {
        for c in line.chars() {
            let colored_char = match c {
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

fn print_coffee(gm: bool) {
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

fn show_details() {
    let mut details: String = String::new();
    // Usage
    details += "usages : jac [option]";
    details += "If you want a coffee, the script print a ascii coffee to remind you to drink one.\n\n";
    // -q
    details += "-q      --question      :   Show a question asking user to drink a coffee ( unactive by default )\n";
    // -y
    details += "-y      --yes           :   Skip the question by auto-responding \"yes\"\n";
    // -n
    details += "-n      --no            :   Skip the question by auto-responding \"no\"\n";
    // --graduate_multicolor
    details += "--graduate-multicolor   :   Color the ascii output based on the position (x, y) of the pixel\n";
    // -c 
    details += "-c      --colored       :   Color the cup and the coffee. May not work on all device, colorisation follow rules ( see readme ).\n";
    // -h
    details += "-h      --help  :   Show arguments and usage of the command"; 

    println!("{}", details);
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut buffer = String::new();
    let args: Args = Args::parse();

    // Debug tool
    if args.debug {
        println!("{:?}", args);
    }

    if args.help{
        show_details();
    }
    else if args.colored {
        print_colored_coffee();
    }
    else if !(args.question) && !(args.no) {
        print_coffee(args.graduate_multicolor);
    }
    else if !(args.no) {
        if args.yes {
            print_coffee(args.graduate_multicolor);
        }else {
            println!("Want a coffee ?");
            io::stdin().read_line(&mut buffer)?;

            if !(buffer.trim().to_lowercase() == "n") {
                print_coffee(args.graduate_multicolor);
            }
            else {
                println!("No more coffee.");
            }
        }
    } else if args.no {
        println!("No more coffee.");
    }
    else {
        panic!("wtf ?");
    }


    

    Ok(())

}


#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
struct Args {
    #[arg(short, long)]
    question: bool,

    #[arg(short, long)]
    no : bool,

    #[arg(short, long)]
    yes : bool,

    #[arg(short, long)]
    graduate_multicolor : bool,

    #[arg(short, long)]
    colored : bool,

    #[arg(short, long)]
    debug : bool,
    
    #[arg(short, long)]
    help : bool,

}