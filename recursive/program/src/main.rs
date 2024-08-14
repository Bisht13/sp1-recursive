#![no_main]
sp1_zkvm::entrypoint!(main);

use ark_bn254::Fr;
use gnark_bn254_verifier::{verify, ProvingSystem};
use num_bigint::BigUint;

pub fn main() {
    let proof = sp1_zkvm::io::read_vec()[8..].to_vec();
    let vk = sp1_zkvm::io::read_vec()[8..].to_vec();
    let vkey_hash = Fr::from(BigUint::from_bytes_be(&sp1_zkvm::io::read_vec()[8..]));
    let committed_values_digest = Fr::from(BigUint::from_bytes_be(&sp1_zkvm::io::read_vec()[8..]));

    println!("cycle-tracker-start: setup");
    if verify(
        &proof,
        &vk,
        &[vkey_hash, committed_values_digest],
        ProvingSystem::Plonk,
    ) {
        println!("Proof is valid");
    } else {
        println!("Proof is invalid");
    }
    println!("cycle-tracker-end: setup");
}
