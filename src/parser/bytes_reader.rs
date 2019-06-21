use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;

use crate::{WasmError, WasmResult};

pub struct BytesReader<'a> {
    code: &'a Bytes,
    position: usize,
}

impl<'a> BytesReader<'a> {
    pub fn new(code: &'a Bytes) -> Self {
         Self {
            code,
            position: 0,
        }
    }

    pub fn read_range(&mut self, len: usize) -> WasmResult<&'a [u8]> {
        let buf = self.code
            .get(self.position..self.position + len)
            .ok_or(WasmError::EOF)?;
        self.position += len;
        Ok(buf)
    }

    pub fn read_u7(&mut self) -> WasmResult<u32> {
        let buf = self.code.get(self.position).ok_or(WasmError::EOF)?;
        self.position += 1;
        Ok(LittleEndian::read_u32(&[*buf, 0, 0, 0]))
    }

    pub fn read_i7(&mut self) -> WasmResult<i32> {
        let buf = self.code.get(self.position).ok_or(WasmError::EOF)?;
        self.position += 1;
        Ok(LittleEndian::read_i32(&[*buf, 0, 0, 0]))
    }

    pub fn read_u32(&mut self) -> WasmResult<u32> {
        let buf = self.read_range(4)?;
        Ok(LittleEndian::read_u32(buf))
    }

    pub fn read_i32(&mut self) -> WasmResult<i32> {
        let buf = self.read_range(4)?;
        Ok(LittleEndian::read_i32(buf))
    }
}
