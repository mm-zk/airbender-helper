use base64::Engine;
use clap::Parser;
use execution_utils::ProgramProof;
use reqwest::Client;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

async fn load_program_proof_from_file(
    file_path: &str,
) -> Result<ProgramProof, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    dbg!("parsing program");
    let program_proof: ProgramProof = serde_json::from_str(&contents)?;
    dbg!("parsing program - done ");
    Ok(program_proof)
}

async fn send_fri_rpc_call(
    server_url: &str,
    program_proof: &ProgramProof,
) -> Result<Value, Box<dyn std::error::Error>> {
    let serialized_proof = bincode::serialize(program_proof)?;
    let serialized_proof_base64 = base64::prelude::BASE64_STANDARD.encode(serialized_proof);

    let client = Client::new();
    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "sendFRI",
        "params": {
            "payload": serialized_proof_base64,
            "bytecode_hash": "aa", //program_proof.bytecode_hash,
            "public_input": "bb", //program_proof.public_input,
        },
        "id": 1
    });

    let response = client.post(server_url).json(&request_body).send().await?;
    dbg!("sent");

    println!("response.status: {}", response.status());

    let response_json: Value = response.json().await?;
    Ok(response_json)
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    proof_file: String,
    #[arg(short, long)]
    server_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let program_proof = load_program_proof_from_file(&cli.proof_file).await?;
    let response = send_fri_rpc_call(&cli.server_url, &program_proof).await?;

    println!("Response from server: {:?}", response);
    Ok(())
}
