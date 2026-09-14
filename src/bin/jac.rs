use clap::Parser;
use std::io;


fn print_coffee() {
    const ASCII : &str = include_str!("../../coffee");
    println!("{}", ASCII);
}

fn show_details() {
    let mut details: String = String::new();
    // Usage
    details += "usages : jac [option]";
    details += "If you want a coffee, the script print a ascii coffee to remind you to drink one.\n\n";
    // -q
    details += "-q      --question  :   Show a question asking user to drink a coffee ( unactive by default )\n";
    // -y
    details += "-y      --yes   :   Skip the question by auto-responding \"yes\"\n";
    // -n
    details += "-n      --no    :   Skip the question by auto-responding \"no\"\n";
    // -h
    details += "-h      --help  :   Show arguments and usage of the command"; 

    println!("{}", details);
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut buffer = String::new();
    let args: Args = Args::parse();

    if args.help{
        show_details();
    }
    else if !(args.question) && !(args.no) {
        print_coffee();
    }
    else if !(args.no) {
        if args.yes {
            print_coffee();
        }else {
            println!("Want a coffee ?");
            io::stdin().read_line(&mut buffer)?;

            if !(buffer.trim().to_lowercase() == "n") {
                print_coffee();
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
    help : bool
}