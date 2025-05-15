use halo2curves::bn256::Bn256;
use zk_engine::{
    nova::{
        provider::{ipa_pc, Bn256EngineIPA, Bn256EngineKZG, GrumpkinEngine},
        spartan,
        traits::Dual,
    },
    utils::logging::init_logger,
    wasm_ctx::{WASMArgsBuilder, WASMCtx},
    wasm_snark::{StepSize, WasmSNARK},
};

use std::fs;
use std::path::Path;
use std::io::Write;
use wat::parse_str;
use clap::{Arg, Command};

// Ensure you use the correct cycle pairing
type E = Bn256EngineKZG;
type E2 = GrumpkinEngine;
type EE1 = zk_engine::nova::provider::hyperkzg::EvaluationEngine<Bn256, E>;
type EE2 = zk_engine::nova::provider::ipa_pc::EvaluationEngine<E2>;
type S1 = zk_engine::nova::spartan::batched::BatchedRelaxedR1CSSNARK<E, EE1>; 
type S2 = zk_engine::nova::spartan::snark::RelaxedR1CSSNARK<E2, EE2>; 

fn main() {
    init_logger();

    // Parse command-line arguments using clap v4
    let matches = Command::new("zkEngine Party CLI")
        .version("1.0")
        .author("Wyatt")
        .about("Runs a zkProver on a WebAssembly module")
        .arg(
            Arg::new("wat_file")
                .short('w')
                .long("wat")
                .value_name("FILE")
                .help("Path to the .wat file")
                .required(true),
        )
        .arg(
            Arg::new("invoke")
                .short('i')
                .long("invoke")
                .value_name("FUNCTION")
                .help("Function to call in the WebAssembly module")
                .default_value("fib"),
        )
        .arg(
            Arg::new("func_args")
                .short('a')
                .long("args")
                .value_name("ARGS")
                .help("Comma-separated list of function arguments (e.g., '16,42')")
                .default_value("16"),
        )
        .arg(
            Arg::new("step_size")
                .short('s')
                .long("step-size")
                .value_name("NUM")
                .help("Step size for proving")
                .default_value("20"),
        )
        // Removed the output directory argument as we'll always use /proofs
        .get_matches();

    // Get arguments from CLI (clap v4 uses `.get_one::<T>()`)
    let wat_file = matches.get_one::<String>("wat_file").unwrap();
    let invoke = matches.get_one::<String>("invoke").unwrap();
    let func_args: Vec<String> = matches
        .get_one::<String>("func_args")
        .unwrap()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let step_size: usize = matches
        .get_one::<String>("step_size")
        .unwrap()
        .parse()
        .expect("Step size must be an integer");
    let output_dir = "proofs";

    println!("🚀 Starting zkProver");
    println!("📜 WAT File: {}", wat_file);
    println!("🔧 Function: {}", invoke);
    println!("🔢 Args: {:?}", func_args);
    println!("⏳ Step Size: {}", step_size);
    println!("💾 Output Directory: {} (project root)", output_dir);

    // Ensure the output directory exists
    if !Path::new(output_dir).exists() {
        println!("📁 Creating output directory: {}", output_dir);
        match fs::create_dir_all(output_dir) {
            Ok(_) => println!("✅ Output directory created successfully"),
            Err(e) => {
                eprintln!("❌ Failed to create output directory: {:?}", e);
                return;
            }
        }
    }

    // Convert step size
    let step_size = StepSize::new(step_size);
    println!("🔨 Generating public parameters...");
    
    let pp = WasmSNARK::<E, S1, S2>::setup(step_size);

    // Load the .wat file dynamically
    let wat_content = match fs::read_to_string(wat_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to load WAT file {}: {:?}", wat_file, e);
            return;
        }
    };

    // Convert WAT to WASM bytes
    let wasm_bytes = match parse_str(&wat_content) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("❌ Error parsing WAT: {:?}", e);
            return;
        }
    };

    println!("✅ WAT successfully converted to WASM!");

    // Build WASM execution context
    let wasm_args = WASMArgsBuilder::default()
        .bytecode(wasm_bytes)
        .invoke(invoke)  // Use the dynamic function name
        .func_args(func_args.clone()) // Use dynamic function arguments
        .build();

    println!("🔄 Creating WASM execution context...");

    let wasm_ctx = WASMCtx::new(wasm_args);

    println!("🔍 Starting proof generation...");

    let (snark, instance) = match WasmSNARK::<E, S1, S2>::prove(&pp, &wasm_ctx, step_size) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("❌ Proving error: {:?}", e);
            return;
        }
    };

    println!("✅ Proving complete! Verifying proof...");

    let str_instance = serde_json::to_string(&instance).unwrap();
    let str_snark = serde_json::to_string(&snark).unwrap();

    // Generate simple filenames
    let instance_filename = format!("{}/instance_{}.json", output_dir, invoke);
    let snark_filename = format!("{}/snark_{}.json", output_dir, invoke);

    // Save instance to file
    println!("💾 Saving instance to {}", instance_filename);
    match fs::File::create(&instance_filename) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(str_instance.as_bytes()) {
                eprintln!("❌ Failed to write instance data: {:?}", e);
            } else {
                println!("✅ Instance saved successfully");
            }
        },
        Err(e) => eprintln!("❌ Failed to create instance file: {:?}", e),
    }

    // Save SNARK to file
    println!("💾 Saving SNARK to {}", snark_filename);
    match fs::File::create(&snark_filename) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(str_snark.as_bytes()) {
                eprintln!("❌ Failed to write SNARK data: {:?}", e);
            } else {
                println!("✅ SNARK saved successfully");
            }
        },
        Err(e) => eprintln!("❌ Failed to create SNARK file: {:?}", e),
    }

    match snark.verify(&pp, &instance) {
        Ok(_) => println!("🎉 Proof verified successfully!"),
        Err(e) => eprintln!("❌ Verification error: {:?}", e),
    }
}