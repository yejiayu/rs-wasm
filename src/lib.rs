mod errors;
mod execution;
mod parser;
pub mod primitives;

pub use errors::WasmError;
pub use parser::Reader;

pub type WasmResult<T> = Result<T, WasmError>;
