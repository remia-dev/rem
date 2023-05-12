// Okay I want to create a command line that opens a markdown when called
// markdown template should have tags along with title
// rem [title] (Optional)[author] [tag]

use std::fs::File;
// use std::io::prelude::*;
use std::path::Path;

use std::io::Write;

// Parsing arguments
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short)]
    action: String,

    title: String,

    author: Option<String>,

    #[arg(long)]
    tag: Option<String>,
}

fn main() {
    print!("> ");
    std::io::stdout().flush().expect("Error");
    let args = Args::parse();

    // Create a file

    if args.action == "add" {
        let title2 = args.title + ".md";
        let title = Path::new(&title2);
        let display = title.display();
        let mut _file = match File::create(&title) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("succesfully wrote to {}", display),
        };
    }else if args.action =="draw"{

    }
}
