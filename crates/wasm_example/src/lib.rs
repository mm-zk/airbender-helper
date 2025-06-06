pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn verify_all_program_proof(program_proof_data: &str) -> Array {
    use cli_lib::prover_utils::{
        generate_oracle_data_from_metadata_and_proof_list,
        proof_list_and_metadata_from_program_proof,
    };

    let input_program_proof: execution_utils::ProgramProof =
        serde_json::from_str(&program_proof_data).expect("Failed to parse program proof as JSON");

    //serde_json::from_str(&input.unwrap()).expect("Failed to parse input_hex into ProgramProof");
    let (metadata, proof_list) = proof_list_and_metadata_from_program_proof(input_program_proof);

    let oracle_data = generate_oracle_data_from_metadata_and_proof_list(&metadata, &proof_list);
    let it = oracle_data.into_iter();

    verifier_common::prover::nd_source_std::set_iterator(it);

    // Assume that program proof has only recursion proofs.
    println!("Running continue recursive");
    assert!(metadata.reduced_proof_count > 0);
    let output = full_statement_verifier::verify_recursion_layer();
    println!("Output is: {:?}", output);

    assert!(
        verifier_common::prover::nd_source_std::try_read_word().is_none(),
        "Expected that all words from CSR were consumed"
    );

    let bb = output.map(JsValue::from);

    Array::from_iter(bb.iter().cloned())
}
