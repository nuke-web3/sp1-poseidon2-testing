use sp1_sdk::{include_elf, HashableKey, Prover, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const POSEIDON2_ELF: &[u8] = include_elf!("poseidon2-program");

fn main() {
    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(POSEIDON2_ELF);
    println!("{}", vk.bytes32());
}
