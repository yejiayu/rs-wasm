use std::convert::TryFrom;

use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;

use crate::primitives::{Frame, Section, Type};
use crate::{WasmError, WasmResult};
use crate::parser::bytes_reader::BytesReader;

const WASM_MAGIC_NUMBER: u32 = 0x6d736100;
// const WASM_EXPERIMENTAL_VERSION: u32 = 0xd;
const WASM_SUPPORTED_VERSION: u32 = 0x1;

pub struct Reader<'a> {
    bytes: BytesReader<'a>,
    initialize: bool,
}

impl<'a> Reader<'a> {
    pub fn new(code: &'a Bytes) -> Self {
        Self {
            bytes: BytesReader::new(code),
            initialize: false,
        }
    }

    pub fn read(&mut self) -> WasmResult<Frame<'a>> {
        if !self.initialize {
            let head = self.head()?;
            self.initialize = true;
            Ok(head)
        } else {
            self.section()
        }

        // self.section()
    }

    fn head(&mut self) -> WasmResult<Frame<'a>> {
        let magic_number = self.bytes.read_u32()?;
        if magic_number != WASM_MAGIC_NUMBER {
            return Err(WasmError::InvalidMagicNumber(magic_number));
        }

        let version = self.bytes.read_u32()?;
        if version != WASM_SUPPORTED_VERSION {
            return Err(WasmError::InvalidVersion(version));
        }
        Ok(Frame::Head { version })
    }

    fn section(&mut self) -> WasmResult<Frame<'a>> {
        let section_code = self.bytes.read_u7()?;
        let section_type = match  section_code {
            1 => Ok(Section::Type),
            2 => Ok(Section::Import),
            3 => Ok(Section::Function),
            4 => Ok(Section::Table),
            5 => Ok(Section::Memory),
            6 => Ok(Section::Global),
            7 => Ok(Section::Export),
            8 => Ok(Section::Start),
            9 => Ok(Section::Element),
            10 => Ok(Section::Code),
            11 => Ok(Section::Data),
            _ => Err(WasmError::InvalidSection(section_code)),
        };

        let payload_len = self.bytes.read_u7()? as usize;
        let payload = self.bytes.read_range(payload_len)?;

        Ok(Frame::Section { section, payload })
    }

    fn read_type_section(&mut self) -> WasmResult<Section> {
        self
    }

//     fn read_type(&mut self) -> WasmResult<Type> {
//         let type_code = self.read_i7()?;
//         let t = match type_code {
//             -0x01 => Ok(Type::I32),
//             -0x02 => Ok(Type::I64),
//             -0x03 => Ok(Type::F32),
//             -0x04 => Ok(Type::F64),
//             -0x05 => Ok(Type::V128),
//             -0x10 => Ok(Type::AnyFunc),
//             -0x11 => Ok(Type::AnyRef),
//             -0x20 => Ok(Type::Func),
//             -0x40 => Ok(Type::EmptyBlockType),
//             _ => Err(WasmError::InvalidType(type_code)),
//         };
//         Ok(t)
//     }
// }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::parser::Reader;

    #[test]
    fn test_head() {
        let code = include_bytes!("../../scripts/module.wasm");
        let code = Bytes::from(code.as_ref());
        let mut r = Reader::new(&code);
        r.head().unwrap();
    }

    #[test]
    fn test_read() {
        let code = include_bytes!("../../scripts/func.wasm");
        let code = Bytes::from(code.as_ref());
        let mut r = Reader::new(&code);
        r.read().unwrap();
        let s = r.read().unwrap();
        println!("section {:?}", s);
    }
}
