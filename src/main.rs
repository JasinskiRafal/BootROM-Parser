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
#[command(about = "STM32 BootROM hex dump parser - Converts BootROM traces to human-readable format")]
#[command(author = "Rafal Jasinski")]
struct Cli {
    /// Path to the hex dump file containing BootROM traces
    ///
    /// The file should contain space-separated hexadecimal bytes representing
    /// the memory dump from an STM32 microcontroller.
    bootrom_hex_dump_file: PathBuf,
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

    let hex_dump_file = cli.bootrom_hex_dump_file.to_owned();
    if !hex_dump_file.is_file() {
        eprintln!("Error: Hex dump file does not exist!");
        std::process::exit(1);
    }

    // Read and parse the hex dump file into 32-bit word values
    let contents = match fs::read_to_string(hex_dump_file.as_path()) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let hex_values = parser::from_hex_dump_file(contents);
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
