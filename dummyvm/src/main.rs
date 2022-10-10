use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use aptos_crypto::hash::CryptoHash;
use serde::{Deserialize, Serialize};
use aptos_types::{
    transaction::Transaction,
};
use aptos_crypto::{HashValue};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BLOCKS: Arc<Mutex<HashMap<HashValue, bool>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Debug, Serialize, Deserialize)]
struct ExecuteBlockRequest {
    block_hash: HashValue,
    parent_block_id: HashValue,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommitBlockRequest {
    block_ids: Vec<HashValue>,
}

async fn execute_block(item: web::Json<ExecuteBlockRequest>) -> HttpResponse {

    /*
        simple state machine - hash every transaction in a way:
        accumulator = parent_block_id
        accumulator = hash(accumulator + transaction_hash) for every transaction
        result = accumulator

        TODO: make this server statefull
    */

    let mut blocks = BLOCKS.lock().unwrap();

    let mut hashes: Vec<HashValue> = Vec::new();
    for i in item.transactions.iter() {
        hashes.push(i.hash());
    }

    let mut accumulator: [u8; 32] = *item.parent_block_id.as_ref();


    for h in hashes.iter() {
        let next_hash: [u8; 32] = *h.as_ref();
        let bytes: [u8; 64] = [accumulator, next_hash].concat().try_into().unwrap();
        accumulator = *HashValue::sha3_256_of(&bytes).as_ref();
    }

    let result = accumulator;

    blocks.insert(item.block_hash, false);

    HttpResponse::Ok().json(result)
}

async fn commit_blocks_ext(item: web::Json<CommitBlockRequest>) -> HttpResponse {

    let mut blocks = BLOCKS.lock().unwrap();

    let mut commited_counter: u64 = 0;
    for id in item.block_ids.iter() {
        match blocks.get(id) {
            Some(commited) => {
                if !commited {
                    commited_counter += 1;
                    blocks.entry(*id).and_modify(|el| *el = true);
                }
            }
            None => {}
        }
    }
    HttpResponse::Ok().json(commited_counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/execute_block").route(web::post().to(execute_block)))
            .service(web::resource("/commit_blocks_ext").route(web::post().to(commit_blocks_ext)))
        })
    .bind(("127.0.0.1", 8383))?
    .run()
    .await
}