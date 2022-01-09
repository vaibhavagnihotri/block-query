use crate::api::http_response;
use crate::types::{BlockInput, BlockResponse, Result};
use crate::utils::get_block_id;
use actix_web::get;
use actix_web::web;
use actix_web::{HttpResponse, Responder};
use futures::future;
use serde::Serialize;
use std::collections::HashMap;
use web3::api::Eth;
use web3::transports::WebSocket;
use web3::types::U64;
use web3::types::{BlockId, BlockNumber, H2048, H256};

#[get("/api/block")]
pub async fn get(input: web::Query<BlockInput>) -> impl Responder {
    http_response(get_latest_blocks(input.limit.unwrap_or(10)).await)
}

async fn get_latest_blocks(limit: u32) -> Result<BlockResponse> {
    if limit == 0 {
        return Ok(BlockResponse::new());
    }

    //Get latest block
    let latest_block: Option<web3::types::Block<H256>> = get_block(None).await?;

    println!("COMES");

    let mut response = BlockResponse::new();

    match latest_block {
        Some(v) => {
            response.add(&v);
            if let Some(number) = v.number {
                let mut handlers = vec![];
                //GET next N blocks in descending order
                for i in 1..limit {
                    handlers.push(tokio::spawn(get_block(Some(number - i))));
                }

                for result in future::try_join_all(handlers).await? {
                    match result {
                        Ok(v) => match v {
                            Some(block) => {
                                response.add(&block);
                            }
                            None => {}
                        },
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
                Ok(response)
            } else {
                //First block is in pending state
                Ok(BlockResponse::new())
            }
        }
        None => {
            //No block found
            Ok(BlockResponse::new())
        }
    }
}

async fn get_block(block_number: Option<U64>) -> web3::Result<Option<web3::types::Block<H256>>> {
    let websocket = web3::transports::WebSocket::new(
        "wss://mainnet.infura.io/ws/v3/7d20d175e75f4d54b085fd8590d7e72c",
    )
    .await?;
    println!("block {:?}", block_number);
    let eth_client = web3::Web3::new(websocket).eth();

    eth_client.block(get_block_id(block_number)).await
}
