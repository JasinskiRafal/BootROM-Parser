//! STM32 BootROM Parser
//!
//! This is the main entry point for the STM32 BootROM trace parser.
//! The application takes a hex dump file as input and outputs human-readable
//! BootROM trace messages.

use clap::Parser;
use std::fs;
use std::path::PathBuf;

use crate::trace::Trace;

mod parser;
mod trace;

/// Command Line Interface structure
///
/// Defines the CLI arguments for the BootROM parser application.
#[derive(Parser, Debug)]
#[command(version)]
#[command(
    about = "STM32 BootROM hex dump parser - Converts BootROM traces to human-readable format"
)]
#[command(author = "Rafal Jasinski")]
struct Cli {
    /// Path to the input file containing BootROM traces
    ///
    /// The file can be either:
    /// - A hex dump file with space-separated bytes (default)
    /// - A hex value list file with 32-bit hex values (use --hex-values flag)
    input_file: PathBuf,

    /// Treat input file as a list of 32-bit hex values (0x12345678 format)
    ///
    /// When enabled, the parser expects each line to contain a single 32-bit
    /// hexadecimal value in the format 0x12345678 or 0xDEADBEEF.
    #[arg(long, short = 'x')]
    hex_values: bool,
}

/// Main entry point for the BootROM parser application
///
/// This function:
/// 1. Parses command line arguments
/// 2. Validates the input file
/// 3. Reads and parses the hex dump
/// 4. Extracts BootROM traces
/// 5. Outputs human-readable trace messages
fn main() {
    let cli = Cli::parse();

    let input_file = cli.input_file.to_owned();
    if !input_file.is_file() {
        eprintln!("Error: Input file does not exist!");
        std::process::exit(1);
    }

    // Read the input file
    let contents = match fs::read_to_string(input_file.as_path()) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Parse the file based on the selected format
    let hex_values = if cli.hex_values {
        parser::from_hex_value_list(contents)
    } else {
        parser::from_hex_dump_file(contents)
    };
    let traces = Trace::from_parsed_hex_dump(&hex_values);

    // Print summary if no traces found
    if traces.is_empty() {
        eprintln!("No BootROM traces found in the input file.");
        return;
    }

    // Output each parsed trace
    for trace in traces {
        println!("{}", trace);
    }
}
