use std::error::Error;
use std::fmt;

use wasmparser::BinaryReaderError;

#[derive(Debug, Clone)]
pub enum WasmError {
    BinaryReader(BinaryReaderError),
}

impl Error for WasmError {}

impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WasmError::BinaryReader(err) => write!(f, "{:?}", err),
        }
    }
}
