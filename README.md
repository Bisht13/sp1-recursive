# SP1 Recursive Plonk Proof System

## Overview

This repository provides a practical demonstration of generating and verifying SP1 Plonk proofs within the SP1 environment itself. It serves as a guide to understanding the self-referential capabilities of the SP1 system when applied to the Plonk proof system.

## Generating and Verifying Proofs

To generate and verify proofs within the SP1 system, follow the steps outlined below. Ensure that all necessary `.env` files are filled out before proceeding.

### Proof Generation

1. Navigate to the `fibonacci` directory:
    ```bash
    cd fibonacci
    ```
2. Build the program:
    ```bash
    cd program
    cargo prove build
    ```
3. Generate the proof by running the script:
    ```bash
    cd ../script
    RUST_LOG=info cargo run --release -- --prove
    ```
   This command will create a `proof.bin` file in the `script` directory.

### Proof Verification

To verify the generated proof, execute the following steps:

1. Copy the `proof.bin` file to the `recursive` script directory:
    ```bash
    cp proof.bin ../../recursive/script
    ```
2. Build the recursive program:
    ```bash
    cd ../../recursive/program
    cargo prove build
    ```
3. Run the verification script:
    ```bash
    cd ../script
    RUST_LOG=info cargo run --release -- --prove 
    ```

Follow these instructions carefully to ensure successful proof generation and verification within the SP1 Plonk proof system.