use clap::Parser;

#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
pub struct Args {
    #[arg(short, long)]
    pub question: bool,

    #[arg(short, long)]
    pub no : bool,

    #[arg(short, long)]
    pub yes : bool,

    #[arg(short, long)]
    pub graduate_multicolor : bool,

    #[arg(short, long)]
    pub colored : bool,

    #[arg(short, long)]
    pub debug : bool,
    
    #[arg(short, long)]
    pub help : bool,
}