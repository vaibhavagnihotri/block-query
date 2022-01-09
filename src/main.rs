use actix_web::{get, web, App, HttpServer, Responder, ResponseError};
use std::env;
use std::result::Result;
use std::str::FromStr;
use web3::api::Eth;
use web3::transports::WebSocket;
use web3::types::U64;
use web3::types::{Block, BlockId, BlockNumber, H160, H256, U256};
use web3::Web3;

pub mod api;
pub mod settings;
pub mod types;
pub mod utils;

use settings::SETTINGS;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Settings Loaded - {:?}", *SETTINGS);

    HttpServer::new(|| App::new().service(api::block::get))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
