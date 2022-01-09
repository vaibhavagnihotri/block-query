pub mod block_input;
pub mod block_response;

pub use self::block_input::BlockInput;
pub use self::block_response::BlockResponse;

pub type Result<T> = std::result::Result<T, failure::Error>;
