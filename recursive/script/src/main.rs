//! A simple script to generate and verify the proof of a given program.
extern crate dotenv;

use dotenv::dotenv;
use num_bigint::BigUint;
use num_traits::Num;
use sp1_sdk::{
    proto::network::ProofMode, utils, NetworkProver, Prover, ProverClient,
    SP1ProofWithPublicValues, SP1Stdin,
};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");

fn main() {
    dotenv().ok();

    // Setup logging.
    utils::setup_logger();

    let circuits_dir = dirs::home_dir()
        .unwrap()
        .join(".sp1")
        .join("circuits")
        .join("plonk_bn254");

    let vk_dir_entry = std::fs::read_dir(circuits_dir)
        .expect("Failed to read circuits directory")
        .next()
        .expect("No directories found in circuits directory")
        .unwrap()
        .path();

    let vk_bin_path = vk_dir_entry.join("vk.bin");

    let vk = std::fs::read(vk_bin_path).unwrap();
    let proof = SP1ProofWithPublicValues::load("proof.bin").unwrap();
    let raw_proof = hex::decode(proof.clone().proof.try_as_plonk().unwrap().raw_proof).unwrap();
    let public_inputs = proof.proof.try_as_plonk().unwrap().public_inputs.clone();
    let vkey_hash = BigUint::from_str_radix(&public_inputs[0], 10)
        .unwrap()
        .to_bytes_be();
    let committed_values_digest = BigUint::from_str_radix(&public_inputs[1], 10)
        .unwrap()
        .to_bytes_be();

    let mut stdin = SP1Stdin::new();
    stdin.write(&raw_proof);
    stdin.write(&vk);
    stdin.write(&vkey_hash);
    stdin.write(&committed_values_digest);

    // Generate the proof for the given program and input.
    // let client = NetworkProver::new_from_key(&std::env::var("SP1_PRIVATE_KEY").unwrap());
    // let (_, vk) = client.setup(ELF);
    // let proof = client
    //     .prove(ELF, stdin, ProofMode::Core, None)
    //     .unwrap();

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let proof = client.prove(&pk, stdin).run().expect("proving failed");

    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    println!("Successfully verified proof for the program!")
}
