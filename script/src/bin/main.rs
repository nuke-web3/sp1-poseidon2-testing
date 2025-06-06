//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

// TODO: test against known good lib
// use zkhash::{
//     fields::bn256::FpBN256,
//     poseidon2::{poseidon2::Poseidon2, poseidon2_instance_bn256::POSEIDON2_BN256_PARAMS},
// };

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const POSEIDON2_ELF: &[u8] = include_elf!("poseidon2-program");

/// Example: Eight 32 byte hashes bn254 scalar field elements represented as [u8; 32].
pub const INPUT_BYTES: &[u8; 32*8] = &[0u8; 32*8];

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "20")]
    n: u32,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    let mut stdin = SP1Stdin::new();

    // TODO: replace with runtime CLI / runtime selection
    let input_bytes: &[u8] = INPUT_BYTES;
    stdin.write_slice(input_bytes);

    let client = ProverClient::from_env();
    if args.execute {
        // Execute the program
        let (output, report) = client.execute(POSEIDON2_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        // - poseidon2 hash = 32 bytes
        let output_hash = output.to_vec();
        println!(
            "zkVM -> hash: 0x{}",
            hex::encode(output_hash)
        );

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(POSEIDON2_ELF);

        // Generate the proof
        //
        // NOTE:
        // Using the [groth16 proof type](https://docs.succinct.xyz/docs/sp1/generating-proofs/proof-types#groth16-recommended) to trade increased proving costs & time for minimal EVM gas costs.
        let proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
