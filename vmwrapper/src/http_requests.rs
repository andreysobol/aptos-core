use reqwest::Result;
use serde::{Deserialize, Serialize};
use aptos_crypto::HashValue;
use aptos_types::transaction::Transaction;

use tokio::runtime::Runtime;

#[derive(Debug, Serialize, Deserialize)]
struct CommitBlockRequest {
    block_ids: Vec<HashValue>,
}

async fn commit_blocks_async_request(block_ids: Vec<HashValue>) -> Result<()>{

    let c = CommitBlockRequest {
        block_ids: vec![HashValue::zero(), HashValue::zero()],
    };
    
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8383/commit_blocks_ext")
        .json(&c)
        .send()
        .await?;

    Ok(())
}

pub fn commit_blocks_request(block_ids: Vec<HashValue>) {
    let _ = Runtime::new().unwrap().block_on(commit_blocks_async_request(block_ids));
}

#[derive(Debug, Serialize, Deserialize)]
struct ExecuteBlockRequest {
    block_hash: HashValue,
    parent_block_id: HashValue,
    transactions: Vec<Transaction>,
}

async fn execute_block_async_request(
    block_hash: HashValue,
    parent_block_id: HashValue,
    transactions: Vec<Transaction>
) -> Result<()>{

    let c = ExecuteBlockRequest {
        block_hash: block_hash,
        parent_block_id: parent_block_id,
        transactions: transactions,
    };
    
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8383/execute_block")
        .json(&c)
        .send()
        .await?;

    Ok(())
}

pub fn execute_block_request(
    block_hash: HashValue,
    parent_block_id: HashValue,
    transactions: Vec<Transaction>
) {
    let _ = Runtime::new().unwrap().block_on(execute_block_async_request(
        block_hash,
        parent_block_id,
        transactions
    ));
}