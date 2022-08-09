use std::fs;
use clap::Parser;

//alias: alias new_cat="/home/USER/Documents/rust/new_cat/target/release/new_cat -f"    
//path: export PATH="$/home/USER/Documents/rust/new_cat/target/release/:$PATH" 

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    ///File to read. Set 'cat -f ' to an alias/path for use without a flag
    #[clap(short, long, value_parser)]
    f: String,
}


fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(args.f).expect("File could not be read, check permissions");
    println!{"{}", file};
}
