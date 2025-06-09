#![no_main]
sp1_zkvm::entrypoint!(main);

use sp1_poseidon2_bn256::{
    fields::bn256::{FpBN256, bytes_to_fp_elements_mont, fp_elements_to_bytes},
    merkle_tree::merkle_tree_fp::MerkleTree,
    poseidon2::{poseidon2::Poseidon2, poseidon2_instance_bn256::POSEIDON2_BN256_PARAMS},
};

pub fn main() {
    // Load raw Montgomery-encoded field byte
    let input_bytes: Vec<u8> = sp1_zkvm::io::read_vec();
    let field_inputs: Vec<FpBN256> = bytes_to_fp_elements_mont(&input_bytes);

    // Compute Poseidon2 root of merkle tree
    let poseidon = Poseidon2::new(&POSEIDON2_BN256_PARAMS);
    let mut merkle = MerkleTree::new(poseidon);
    let root = merkle.accumulate(&field_inputs);

    // Convert back to canonical bytes
    let output_bytes = fp_elements_to_bytes(&[root]);
    sp1_zkvm::io::commit_slice(&output_bytes);
}
