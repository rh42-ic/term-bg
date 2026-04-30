use std::env;
use std::process;

#[derive(Debug, PartialEq)]
enum OutputMode {
    DarkLight,
    Rgb,
    Luma,
}

#[derive(Debug)]
struct Config {
    mode: OutputMode,
    timeout_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: OutputMode::DarkLight,
            timeout_ms: 50,
        }
    }
}

fn parse_args() -> Config {
    let mut config = Config::default();
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-d" => config.mode = OutputMode::DarkLight,
            "-r" => config.mode = OutputMode::Rgb,
            "-l" => config.mode = OutputMode::Luma,
            "-t" => {
                if let Some(val) = args.next() {
                    if let Ok(ms) = val.parse::<u64>() {
                        config.timeout_ms = ms;
                    } else {
                        eprintln!("Invalid timeout value");
                        process::exit(1);
                    }
                } else {
                    eprintln!("Missing timeout value");
                    process::exit(1);
                }
            }
            "-h" | "--help" => {
                println!("Usage: term-bg [-d|-r|-l] [-t <ms>]");
                println!("  -d  Output 'dark' or 'light' (default)");
                println!("  -r  Output RGB hex (e.g., #RRGGBB)");
                println!("  -l  Output luma value (0-255)");
                println!("  -t  Timeout in milliseconds (default 50)");
                process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                process::exit(1);
            }
        }
    }
    config
}

fn main() {
    let config = parse_args();
    println!("{:?}", config);
}
