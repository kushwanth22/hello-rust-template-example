use std::result;

// clap command line tool to play with marco polo
use clap::Parser;
#[derive(Debug, Parser)]

#[clap(version = "1.0", author = "Kushwanth Kandala", about = "a macro polo game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}
#[derive(Debug, Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Kushwanth Kandala")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let cli_args = Cli::parse();
    match cli_args.command {
        Some(Commands::Play{name}) => {
            // if name == "Macro" {
            // println!("{} Polo is playing the game!", name)
            // } else {
            //     println!("What's your name?");
            // }
            let result = hello_marco::marco_polo(&name);
            println!("{}", result)
        }
        None => println!("No Command given."),
    }
}