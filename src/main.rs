use clap::Parser;
use std::fs;
use std::path::PathBuf;

use crate::trace::Trace;

mod parser;
mod trace;

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "Simple BootROM hex dump parser")]
struct Cli {
    bootrom_hex_dump_file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let hex_dump_file = cli.bootrom_hex_dump_file.to_owned();
    if !hex_dump_file.is_file() {
        eprintln!("Hex dump file does not exist!");
        return;
    }

    // format the file into a vector of word values
    let contents =
        fs::read_to_string(hex_dump_file.as_path()).expect("The file needs to be readable");
    let hex_values = parser::from_hex_dump_file(contents);
    let traces = Trace::from_parsed_hex_dump(&hex_values);

    for trace in traces {
        println!("{}", trace);
    }
}
