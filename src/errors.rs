use std::string::FromUtf8Error;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum WasmError {
    InvalidSection(u32),
    InvalidMagicNumber(u32),
    InvalidVersion(u32),
    InvalidType(i32),
    InvalidKindType(u32),
    InvalidOperator(u8),

    InvalidLEB128,

    FromUtf8(FromUtf8Error),
    EOF,
}

impl Error for WasmError {}

impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WasmError::InvalidSection(code) => write!(f, "[wasm]: invalid section {:?}", code),
            WasmError::InvalidMagicNumber(magic_number) => {
                write!(f, "[wasm]: invalid magic number {:?}", magic_number)
            }
            WasmError::InvalidVersion(version) => {
                write!(f, "[wasm]: invalid version {:?}", version)
            }
            WasmError::InvalidType(t) => write!(f, "[wasm]: invalid type {:?}", t),
            WasmError::InvalidKindType(t) => write!(f, "[wasm]: invalid kind type {:?}", t),
            WasmError::InvalidOperator(o) => write!(f, "[wasm]: invalid operator code {:?}", o),
            WasmError::InvalidLEB128 => write!(f, "[wasm]: invalid leb-128"),

            WasmError::FromUtf8(ref err) => write!(f, "[wasm]: {:?}", err),
            WasmError::EOF => write!(f, "parser EOF"),
        }
    }
}

impl From<FromUtf8Error> for WasmError {
    fn from(err: FromUtf8Error) -> Self {
        WasmError::FromUtf8(err)
    }
}
