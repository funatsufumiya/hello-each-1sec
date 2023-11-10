// just print hello each 1sec to stdout (with -e option, print to stderr)
// stop with Ctrl-C

use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    let mut stderr = false;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "-e" {
            stderr = true;
        }

        // show help
        if args[1] == "-h" || args[1] == "--help" {
            // const VERSION: &'static str = env!("CARGO_PKG_VERSION");
            const COMMAND_NAME: &'static str = env!("CARGO_PKG_NAME");
            println!("Usage: {} [OPTIONS]", COMMAND_NAME);
            println!("  -e: print to stderr");
            return;
        }

        // show version
        if args[1] == "-v" || args[1] == "--version" {
            const VERSION: &'static str = env!("CARGO_PKG_VERSION");
            const COMMAND_NAME: &'static str = env!("CARGO_PKG_NAME");
            println!("{} v{}", COMMAND_NAME, VERSION);
            return;
        }
    }
    loop {
        if stderr {
            eprintln!("hello");
        } else {
            println!("hello");
        }
        thread::sleep(Duration::from_secs(1));
    }
}