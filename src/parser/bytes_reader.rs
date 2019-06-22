use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;

use crate::{WasmError, WasmResult};

pub struct BytesReader {
    code: Bytes,
    position: usize,
}

impl BytesReader {
    pub fn new(code: Bytes, position: usize) -> Self {
        Self { code, position }
    }

    pub fn read_range(&mut self, len: usize) -> WasmResult<&[u8]> {
        let buf = self
            .code
            .get(self.position..self.position + len)
            .ok_or(WasmError::EOF)?;
        self.position += len;
        Ok(buf)
    }

    pub fn read_u7(&mut self) -> WasmResult<u32> {
        let byte = self.peek_byte(self.position)?;
        if (byte & 0x80) != 0 {
            return Err(WasmError::InvalidLEB128);
        }

        self.position += 1;
        Ok(LittleEndian::read_u32(&[byte, 0, 0, 0]))
    }

    pub fn read_i7(&mut self) -> WasmResult<i32> {
        let byte = self.peek_byte(self.position)?;
        if (byte & 0x80) != 0 {
            return Err(WasmError::InvalidLEB128);
        }

        self.position += 1;
        Ok(LittleEndian::read_i32(&[byte, 0, 0, 0]))
    }

    pub fn read_u32(&mut self) -> WasmResult<u32> {
        let byte = self.peek_byte(self.position)?;
        let mut bytes = vec![byte, 0, 0, 0];
        if (bytes[0] & 0x80) == 0 {
            self.position += 1;
            return Ok(LittleEndian::read_u32(&bytes));
        }

        let mut len = 1;
        loop {
            if len > 4 {
                return Err(WasmError::InvalidLEB128);
            }
            len += 1;

            let byte = self.peek_byte(self.position + len)?;
            bytes[len - 1] = byte;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        self.position += len;
        Ok(LittleEndian::read_u32(&bytes))
    }

    pub fn read_i32(&mut self) -> WasmResult<i32> {
        let byte = self.peek_byte(self.position)?;
        let mut bytes = vec![byte, 0, 0, 0];
        if (bytes[0] & 0x80) == 0 {
            self.position += 1;
            return Ok(LittleEndian::read_i32(&bytes));
        }

        let mut len = 1;
        loop {
            if len > 4 {
                return Err(WasmError::InvalidLEB128);
            }
            len += 1;

            let byte = self.peek_byte(self.position + len)?;
            bytes[len - 1] = byte;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        self.position += len;
        Ok(LittleEndian::read_i32(&bytes))
    }

    fn peek_byte(&self, position: usize) -> WasmResult<u8> {
        let byte = self.code.get(position).ok_or(WasmError::EOF)?;
        Ok(*byte)
    }
}
