mod errors;
mod parser;
pub mod primitives;

pub use errors::WasmError;
pub use parser::Parser;

pub type WasmResult<T> = Result<T, WasmError>;
