// just print hello each 1sec to stdout (with -e option, print to stderr)
// stop with Ctrl-C

use std::thread;
use std::time::Duration;
use std::env;
use std::time::Instant;

fn main() {
    let mut stderr = false;
    let mut exit_sec: f64 = 0.0; // 0 means no exit
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "-e" || args.len() > 2 && args[2] == "-e" || args.len() > 3 && args[3] == "-e" {
            stderr = true;
        }

        // specify exit sec by "-t 10"
        if args[1] == "-t" {
            if args.len() > 2 {
                let sec: f64 = args[2].parse().unwrap();
                exit_sec = sec;
            }
        }

        // show help
        if args[1] == "-h" || args[1] == "--help" {
            // const VERSION: &'static str = env!("CARGO_PKG_VERSION");
            const COMMAND_NAME: &'static str = env!("CARGO_PKG_NAME");
            println!("Usage: {} [OPTIONS]", COMMAND_NAME);
            println!("  -e: print to stderr");
            println!("  -t [sec]: specify exit sec (default: 0 / means no exit)");
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
    let start_time = Instant::now();
    loop {
        if stderr {
            eprintln!("hello");
        } else {
            println!("hello");
        }
        thread::sleep(Duration::from_secs(1));
        if exit_sec > 0.0 {
            let elapsed = start_time.elapsed();
            let elapsed_sec = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
            if elapsed_sec > exit_sec {
                break;
            }
        }
    }
}