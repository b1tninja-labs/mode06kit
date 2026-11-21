//! Command-line entry point for `mode06kit`: a read-only reader over GM's Mode $06 definitions.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (code, text) = mode06kit::cli::run(&args);
    if code == 0 {
        if !text.is_empty() {
            println!("{text}");
        }
        ExitCode::SUCCESS
    } else {
        eprintln!("{text}");
        ExitCode::from(u8::try_from(code).unwrap_or(1))
    }
}
