use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum WasmError {
    InvalidSection(u32),
    InvalidMagicNumber(u32),
    InvalidVersion(u32),
    InvalidType(i32),
    EOF,
}

impl Error for WasmError {}

impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WasmError::InvalidSection(code) => write!(f, "invalid section {:?}", code),
            WasmError::InvalidMagicNumber(magic_number) => {
                write!(f, "invalid magic number {:?}", magic_number)
            }
            WasmError::InvalidVersion(version) => write!(f, "invalid version {:?}", version),
            WasmError::InvalidType(t) => write!(f, "invalid type {:?}", t),
            WasmError::EOF => write!(f, "parser EOF"),
        }
    }
}
