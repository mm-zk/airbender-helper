use execution_utils::ProgramProof;
use js_sys::Array;
use serde_json::Result;
use wasm_bindgen::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SequencerProof {
    block_number: usize,
    proof: String,
}

pub fn guess_program_proof(program_proof_data: &str) -> Option<ProgramProof> {
    // Try parsing program proof as json directly.
    let input_program_proof: Result<execution_utils::ProgramProof> =
        serde_json::from_str(&program_proof_data);

    if let Ok(input_program_proof) = input_program_proof {
        return Some(input_program_proof);
    }

    // Try parsing the byte64 encoded version from sequencer.
    let input: Result<SequencerProof> = serde_json::from_str(&program_proof_data);
    if let Ok(input) = input {
        let decoded = base64::decode(&input.proof).expect("Failed to decode base64 program proof");
        let program_proof: ProgramProof =
            bincode::deserialize(&decoded).expect("Failed to deserialize program proof");
        return Some(program_proof);
    }

    None
}

#[wasm_bindgen]
pub fn verify_all_program_proof(program_proof_data: &str) -> Array {
    use cli_lib::prover_utils::{
        generate_oracle_data_from_metadata_and_proof_list,
        proof_list_and_metadata_from_program_proof,
    };

    let input_program_proof = guess_program_proof(program_proof_data).unwrap();

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
