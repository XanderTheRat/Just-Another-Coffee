use clap::Parser;
use std::io;

use jac::asset::front::print_coffee;
use jac::asset::front::print_colored_coffee;
use jac::asset::structures::Args;

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