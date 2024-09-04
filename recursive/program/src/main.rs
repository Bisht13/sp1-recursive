#![no_main]
sp1_zkvm::entrypoint!(main);

use gnark_bn254_verifier::{PlonkVerifier, Verifier};
use substrate_bn::Fr;

pub fn main() {
    let proof = sp1_zkvm::io::read_vec()[8..].to_vec();
    let vk = sp1_zkvm::io::read_vec()[8..].to_vec();
    let vkey_hash = &sp1_zkvm::io::read_vec()[8..];
    let committed_values_digest_bytes = sp1_zkvm::io::read_vec()[8..].to_vec();

    let proof = proof;
    let vk = vk;
    let vkey_hash = Fr::from_slice(&vkey_hash).expect("Unable to read vkey_hash");
    println!(
        "commitment_values_digest bytes: {:?}",
        &committed_values_digest_bytes
    );
    let committed_values_digest = Fr::from_slice(&committed_values_digest_bytes)
        .expect("Unable to read committed_values_digest");
    println!(
        "[READ] commited_values_digest: {:?}",
        committed_values_digest
    );

    println!("cycle-tracker-start: setup");
    if PlonkVerifier::verify(&proof, &vk, &[vkey_hash, committed_values_digest]) {
        println!("Proof is valid");
    } else {
        panic!("Proof is invalid");
    }
    println!("cycle-tracker-end: setup");
}
