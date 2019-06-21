use std::convert::TryFrom;

use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;

use crate::primitives::{Frame, Head, Section};
use crate::{WasmError, WasmResult};

const WASM_MAGIC_NUMBER: u32 = 0x6d736100;
const WASM_EXPERIMENTAL_VERSION: u32 = 0xd;
const WASM_SUPPORTED_VERSION: u32 = 0x1;

pub struct Reader<'a> {
    code: &'a Bytes,
    position: usize,
    initialize: bool,
}

impl<'a> Reader<'a> {
    pub fn new(code: &'a Bytes) -> Self {
        Self {
            code,
            position: 0,
            initialize: false,
        }
    }

    pub fn read(&mut self) -> WasmResult<Frame> {
        if !self.initialize {
            let head = self.head()?;
            self.initialize = true;
            return Ok(head);
        }

        self.section()
    }

    fn head(&mut self) -> WasmResult<Frame> {
        let magic_number = self.read_u32()?;
        if magic_number != WASM_MAGIC_NUMBER {
            return Err(WasmError::InvalidMagicNumber(magic_number));
        }
        self.offset(4);

        let version = self.read_u32()?;
        if version != WASM_SUPPORTED_VERSION {
            return Err(WasmError::InvalidVersion(version));
        }
        self.offset(4);
        Ok(Frame::Head(Head { version }))
    }

    fn section(&mut self) -> WasmResult<Frame> {
        let i7 = self.read_i7()?;
        self.offset(1);
        let section = Section::try_from(i7)?;
        Ok(Frame::Section(section))
    }

    fn read_range(&self, len: usize) -> WasmResult<&'a [u8]> {
        self.code
            .get(self.position..self.position + len)
            .ok_or(WasmError::EOF)
    }

    fn read_u32(&self) -> WasmResult<u32> {
        let buf = self.read_range(4)?;
        Ok(LittleEndian::read_u32(buf))
    }

    fn read_i7(&self) -> WasmResult<i32> {
        let buf = self.code.get(self.position).ok_or(WasmError::EOF)?;
        Ok(LittleEndian::read_i32(&[*buf, 0, 0, 0]))
    }

    fn offset(&mut self, offset: usize) {
        self.position += offset
    }
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
