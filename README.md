# block-query
![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
> Querying block information in Ethereum with RUST

This provides an implementation of querying Ethereum Blocks given the infuria account key. It uses web3 as RPC client and actix-web as API framework. Tokio runtime is used for execution of parallel fetch of blocks.   

## How to setup
* Clone the repo
* Create `conf/settings.yaml` with desired value of `infuria_key` 

## Starting the application
`ulimit -n 10100 && RUST_LOG=<LOG_LEVEL> cargo run`


LOG_LEVEL can have following values -
* error
* info
* debug
* trace

Here we're setting `ulimit` as `10100` because application keeps parallel socket open for all the requests, in order to fetch information of `10000` blocks, in the worst case `10000` open socket connections will be required and OS can also have some files open.


## APIs

* Application listens at 127.0.0.1:8080
* Application consists of only one GET API `/api/block` which takes an optional query param `limit` as input defaulting to `10000`
* API returns block information in reverse order `(latest → latest - limit)`

#### Fetching Latest 100 Blocks --

``curl --location --request GET 'http://127.0.0.1:8080/api/block?limit=100'``

## Future Work
Dependency of `ulimit` is to be removed. Blocks can be fetched in batches with batch size lower than the default value of `ulimit` and then result can be achieved by combining result from those batches.