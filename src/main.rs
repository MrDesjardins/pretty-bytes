use clap::Parser;
use pretty_bytes_rust::{pretty_bytes, PrettyBytesOptions};
use std::io::stdin;
use std::io::IsTerminal;
use std::process;
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(short, long)]
    bytes: Option<u64>,
    #[arg(short, long)]
    use_1024_instead_of_1000: Option<bool>,
    #[arg(short, long)]
    number_of_decimal: Option<usize>,
    #[arg(short, long)]
    remove_zero_decimal: Option<bool>,
}

fn main() {
    let args = Cli::parse();
    let my_bytes: u64;
    if std::io::stdin().is_terminal() {
        my_bytes = args.bytes.unwrap();
    } else {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    return;
                } else {
                    my_bytes = input.trim().parse::<u64>().unwrap();
                }
            }
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }
    }
    let result = pretty_bytes(
        my_bytes,
        Some(PrettyBytesOptions {
            number_of_decimal: args.number_of_decimal,
            use_1024_instead_of_1000: args.use_1024_instead_of_1000,
            remove_zero_decimal: args.remove_zero_decimal,
        }),
    );
    println!("{}", result);
    process::exit(0);
}
