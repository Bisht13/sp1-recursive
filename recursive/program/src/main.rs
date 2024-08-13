#![no_main]
sp1_zkvm::entrypoint!(main);

use ark_bn254::Fr;
use num_bigint::BigUint;
use plonk_verifier::verify;

pub fn main() {
    let proof = sp1_zkvm::io::read_vec()[8..].to_vec();
    let vk = sp1_zkvm::io::read_vec()[8..].to_vec();
    let vkey_hash = Fr::from(BigUint::from_bytes_be(&sp1_zkvm::io::read_vec()[8..]));
    let committed_values_digest = Fr::from(BigUint::from_bytes_be(&sp1_zkvm::io::read_vec()[8..]));

    println!("cycle-tracker-start: setup");
    if verify(&proof, &vk, &[vkey_hash, committed_values_digest]) {
        println!("Proof is valid");
    } else {
        println!("Proof is invalid");
    }
    println!("cycle-tracker-end: setup");
}
