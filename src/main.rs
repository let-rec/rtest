use clap::{ Parser, Subcommand};
use std::io::{stdin};

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {
        #[arg(required = true)]
        arg: String,
    },
    Set {
        key: i32,
        value: String,
        #[arg(required = false)]
        is_true: bool
    },
    HelpCommand
}


pub fn get_something(arg: String) {
    println!("Get fn: {}", arg);
}
pub fn set_something(key: i32, value: String, is_true: bool) {
    if is_true {
        println!("Set fn: {},{},{}", key, value, is_true)
    }
    println!("IS NOT TRUE MF")
}
pub fn show_commands() {
    println!(r#"COMMANDS:
get <KEY> - Gets the value of a given key and displays it. If no key given, retrieves all values and displays them.
set <KEY> <VALUE> - Sets the value of a given key.
    Flags: --is-true
"#);
}

pub fn create_name() -> String {
    let name: String = String::new();
    name
}

#[tokio::main]
async fn main() {

    // let chunks: Vec<Result<_, ::std::io::Error>> = vec![
    // Ok("hello"),
    // Ok(" "),
    // Ok("world"),
    // ];

    // let stream = futures_util::stream::iter(chunks);

    // let body = Body::wrap_stream(stream);   

    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Couldn't parse stdin");
        let line = buf.trim();
        let args = shlex::split(line).ok_or("error: invalid quoting").unwrap();

        println!("ITS BUF: {:?}", line);
        match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) =>{
                println!("THIS IS CLI COMMAND: {:?}", cli.cmd);
                match cli.cmd {
                    Commands::Get { arg } => get_something(arg),
                    Commands::Set { key, value, is_true } => set_something(key, value, is_true),
                    Commands::HelpCommand => show_commands(),
                }
            }
            Err(e) => println!("Not valid command: {}", e)
        };
    }

}



// -- LEARN this --
// #[derive(Default)]
// struct Foo{}





// -- JUST READ FILE WITH CLI --

// If you want to use this code, which searches and parses text from data.txt, 
//   please comment out all the working code above and uncomment this code.

/* 

use std::env;
use std::process;
use funcs::{Config, run};

mod funcs;



fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arg: {}" , err);
        process::exit(1);
    });
    
    println!("Search for {}", config.query);
    println!("In file {}", config.filename);
    
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
*/
// --------------- //






/*
let args = Cli::parse();
let content = std::fs::read_to_string(&args.path).expect("could not read file");

for line in content.lines() {
    if line.contains(&args.pattern) {
        print!("Line: {}", line);
    }
}
// println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
*/

/*
// #[arg(short = 'o', long = "output")]
#[derive(Debug, Parser)]
#[clap(author,version,about)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}
*/