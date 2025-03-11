# 
```
    ____                   ____   ____  ___         __           _________ 
   / __ \_________  ____  / __/  / __ \/   |  _____/ /___  __   / ____/ (_)
  / /_/ / ___/ __ \/ __ \/ /_   / /_/ / /| | / ___/ __/ / / /  / /   / / / 
 / ____/ /  / /_/ / /_/ / __/  / ____/ ___ |/ /  / /_/ /_/ /  / /___/ / /  
/_/   /_/   \____/\____/_/    /_/   /_/  |_/_/   \__/\__, /   \____/_/_/   
                                                    /____/                 
                                                                         
                                                                         
                                                      NovaNet @2025
```

# NovaNet's Proof Party Client

This is a command-line interface for generating zero-knowledge proofs from WebAssembly (WASM) modules using zkEngine. 

## Overview

The Proof Party Client allows you to:
- Load a WebAssembly module (in text format - .wat)
- Execute a specified function within that module
- Generate a zero-knowledge proof of correct execution
- Verify the proof
- Save the generated proof and instance to files

## Features

- **WebAssembly Support**: Run proofs on any computation that can be compiled to WebAssembly
- **Flexible Invocation**: Call any function in your WASM module with arbitrary arguments
- **Configurable Step Size**: Adjust the proving granularity
- **Automatic Verification**: Verify proofs immediately after generation
- **Persistent Storage**: Save proofs to disk for later use

## Prerequisites

- Rust and Cargo installed
- zkEngine dependencies

## Installation

Clone this repository and build with cargo:

```bash
git clone https://github.com/wyatt-benno/proof-party-client.git
cd proof-party-client
cargo build --release
```

## Usage

```bash
proof-party-client --wat <WAT_FILE> [OPTIONS]
```

### Options

- `-w, --wat <FILE>`: Path to the .wat file [required]
- `-i, --invoke <FUNCTION>`: Function to call (default: "fib")
- `-a, --args <ARGS>`: Comma-separated list of function arguments (default: "16")
- `-s, --step-size <NUM>`: Step size for proving (default: 10)
- `-h, --help`: Print help
- `-V, --version`: Print version

### Examples

```bash
target/release/party_cli --wat fibonacci.wat --invoke fib --args 16 --step-size 10
```

Or run it directly:

```bash
cargo run --release -- --wat wasms/fib.wat --invoke fib --args 16 --step-size 10
```

## Output

The tool will:
1. Create a `/proofs` directory in your project root if it doesn't exist
2. Generate the proof and instance
3. Save them as JSON files:
   - `/proofs/instance_<function_name>.json`
   - `/proofs/snark_<function_name>.json`

## Technical Details

This client uses zkEngine:
- It is powered by folding scheme variants of KST22 (https://eprint.iacr.org/2021/370).
- It can generate succint proofs that can be verified on EVM blockchains.
- It can work on various device sizes due to low memory usage.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[TBD/ but likely MIT]

## Acknowledgments

This project utilizes the zkEngine (https://github.com/ICME-Lab/zkEngine_dev) framework and various other open-source libraries. 