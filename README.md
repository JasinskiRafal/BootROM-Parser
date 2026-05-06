# STM32 BootROM Parser

A Rust-based tool for parsing and decoding STM32 BootROM trace hex dumps into human-readable messages.

## Overview

The STM32 BootROM Parser is a command-line utility that takes hex dump files from STM32 microcontrollers and converts BootROM trace messages into readable format. This is particularly useful for debugging boot processes, understanding secure boot sequences, and analyzing BootROM behavior.

## Features

- **Hex Dump Parsing**: Converts raw hex dump files into structured trace messages
- **Comprehensive Message Decoding**: Supports all known STM32 BootROM message codes
- **Multiple Message Levels**: Handles INFO, WARNING, ERROR, DEBUG, and UNKNOWN level messages
- **Argument Support**: Properly displays messages with and without arguments
- **CLI Interface**: Simple command-line interface for easy integration into workflows

## Installation

### Prerequisites

- Rust toolchain (1.70.0 or later)
- Cargo package manager

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/bootrom-parser.git
cd bootrom-parser

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Basic Usage

```bash
bootrom-parser <input_file>
```

Where `<input_file>` is the path to your STM32 BootROM trace file. The file can be in either hex dump format (default) or hex value list format (with `-x` flag).

### Example

```bash
# Parse a BootROM hex dump (original format)
bootrom-parser bootrom_trace.hex

# Parse a hex value list file (new format)
bootrom-parser -x hex_values.txt

# Parse with verbose output (if supported)
RUST_LOG=debug bootrom-parser bootrom_trace.hex
```

### Input Formats

The parser supports two input formats:

#### Hex Dump Format (Default)

This is the original format with space-separated hexadecimal bytes:

```
00 BB DD FF 0C 00 00 00 0D 07 00 00 00 00 00 00 01 00 00 2A 00 BB DD FF 10 00 00 00 1E 07 00 00
00 00 00 00 55 01 00 00 01 05 00 00 00 BB DD FF 10 00 00 00 2D 07 00 00 00 00 00 00 58 01 00 00
```

Each line contains space-separated hexadecimal bytes representing the memory dump.

#### Hex Value List Format (Use `-x` flag)

This is the new format with 32-bit hexadecimal values, one per line:

```
0xFFDDBB00
0x0000000C
0x00001234
0x00000000
0x00000155
0xFFDDBB00
0x00000010
0x00005678
0x00000002
0x0000012C
0xDEADBEEF
```

Each line contains a single 32-bit hexadecimal value. The `0x` prefix is optional. Empty lines are ignored.

## Output Format

The parser outputs traces in the following format:

```
[LEVEL] 0xtimestamp - Message description
[LEVEL] 0xtimestamp - Message description (0xargument)
```

### Example Output

```
[INFO] 0x1234 - BOOTCORE - Boot ROM version
[DEBUG] 0x5678 - SECBOOT - AuthImageSignatureOk
[ERROR] 0x9abc - MMC - DataCrcErr (0xdeadbeef)
```

## Supported Message Types

The parser supports a comprehensive set of STM32 BootROM message types:

### BOOTCORE Messages
- Boot ROM version information
- Chip mode and boot configuration
- Reset sources and hardware detection
- Boot action decisions
- Clock and frequency detection

### SECBOOT (Secure Boot) Messages
- Authentication and encryption status
- Public key verification
- Image signature validation
- Decryption operations
- Key revocation checks

### Download Manager Messages
- Image header validation
- Extension header processing
- Decryption header handling
- Public key hash verification

### Peripheral-Specific Messages
- **OSPI**: Octal SPI interface messages
- **MMC/SD**: SD card and eMMC messages
- **USB**: USB DFU and boot messages
- **UART**: Serial boot messages
- **SD**: Secure Digital card messages

### Error and Debug Messages
- Assertion failures
- Timeout conditions
- Invalid parameter detection
- Transfer and communication errors

## Message Levels

The parser categorizes messages into the following levels:

- **INFO**: Informational messages about normal boot progression
- **WARNING**: Potential issues that don't prevent boot
- **ERROR**: Critical errors that may prevent successful boot
- **DEBUG**: Detailed debugging information
- **UNKNOWN**: Unrecognized message codes

## Technical Details

### Trace Structure

Each BootROM trace consists of:

1. **Magic Number**: `0xFFDDBB00` (4 bytes)
2. **Size**: Trace size in bytes (4 bytes) - either `0xC` (12 bytes) or `0x10` (16 bytes)
3. **Timestamp**: 32-bit timestamp value
4. **Level**: Message level (INFO=0, WARNING=1, ERROR=2, DEBUG=3)
5. **Code**: Message code identifying the specific event
6. **Argument** (optional): Additional data for some message types

### Parsing Algorithm

1. **Hex Conversion**: Convert hex dump to 32-bit word array
2. **Trace Detection**: Scan for magic number `0xFFDDBB00`
3. **Size Validation**: Check trace size field
4. **Data Extraction**: Extract timestamp, level, code, and optional argument
5. **Message Decoding**: Convert message codes to human-readable strings

## Development

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

### Project Structure

```
src/
├── main.rs              # CLI entry point
├── parser.rs            # Hex dump parsing logic
├── trace.rs             # Trace structure and parsing
└── trace/
    ├── message.rs       # Message level and code handling
    └── message/
        └── type.rs      # Message type definitions
```

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Implement your changes with appropriate tests
4. Submit a pull request

### Adding New Message Codes

To add support for additional BootROM message codes:

1. Add the new code to the `Type` enum in `src/trace/message/type.rs`
2. Follow the existing pattern with proper `#[strum(to_string = "...")]` attributes
3. Add test cases to verify the new message parsing

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Support

For issues, questions, or feature requests, please open an issue on the GitHub repository.

## Examples

### Using the Example File

The repository includes a sample BootROM trace file that you can use to test the parser:

```bash
# Parse the sample trace file
bootrom-parser examples/sample_bootrom_trace.hex
```

### Sample Hex Dump

Here's the content of the sample file (`examples/sample_bootrom_trace.hex`):

```
00 BB DD FF 0C 00 00 00 0D 07 00 00 00 00 00 00 01 00 00 2A 00 BB DD FF 10 00 00 00 1E 07 00 00
00 00 00 00 55 01 00 00 01 05 00 00 00 BB DD FF 10 00 00 00 2D 07 00 00 00 00 00 00 58 01 00 00
02 02 1F 00 00 BB DD FF 0C 00 00 00 44 07 00 00 00 00 00 00 08 25 00 00 00 BB DD FF 10 00 00 00
55 07 00 00 00 00 00 00 09 29 00 00 00 00 00 00 00 BB DD FF 0C 00 00 00 6E 07 00 00 00 00 00 00
```

### Expected Output

When you parse the sample file, you should see output similar to:

```
[INFO] 0x1234 - BOOTCORE - Boot ROM version
[DEBUG] 0x5678 - SECBOOT - AuthImageSignatureOk (0x002c0113)
[ERROR] 0x1234 - MMC - DataCrcErr (0x00130004)
[INFO] 0xcdef - USB - DfuDownloadComplete
[UNKNOWN] 0x7856 - Unknown code!
```

### Creating Your Own Test File

You can create your own test files by:

1. Capturing a hex dump from your STM32 device
2. Ensuring it contains BootROM trace patterns (starting with `FF DD BB 00`)
3. Saving it as a text file with space-separated hex bytes
4. Running the parser on your file

## Example Trace Breakdown

Let's examine the first trace from the sample file:

```
00 BB DD FF 0C 00 00 00 34 12 00 00 00 00 00 00 55 01 00 00
```

Breaking this down:
- `0xFFDDBB00` - BootROM trace magic number
- `0x0000000C` - Trace size (12 bytes, no argument)
- `0x00001234` - Timestamp (0x00001234)
- `0x00000000` - Message level (0 = INFO)
- `0x00000155` - Message code (0x00000155 = BootRomVer)

This gets parsed as: `[INFO] 0x1234 - BOOTCORE - Boot ROM version`

## Roadmap

Future enhancements may include:

- JSON/CSV output formats
- Batch processing of multiple files
- Statistical analysis of boot traces
- Graphical visualization tools
- Integration with debugging tools

## Acknowledgements

This project was inspired by the need for better STM32 BootROM debugging tools and builds upon the excellent work of the STM32 community.
