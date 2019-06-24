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

    pub fn read_var_u8(&mut self) -> WasmResult<u32> {
        let byte = self.peek_byte(self.position)?;
        if (byte & 0x80) != 0 {
            return Err(WasmError::InvalidLEB128);
        }

        self.position += 1;
        Ok(LittleEndian::read_u32(&[byte, 0, 0, 0]))
    }

    pub fn read_var_i8(&mut self) -> WasmResult<i32> {
        let byte = self.peek_byte(self.position)?;
        if (byte & 0x80) != 0 {
            return Err(WasmError::InvalidLEB128);
        }

        self.position += 1;
        Ok(LittleEndian::read_i32(&[byte, 0, 0, 0]))
    }

    pub fn read_var_u32(&mut self) -> WasmResult<u32> {
        let mut raw = self.read_var_raw(4)?;
        let len = raw.len();

        for _ in 0..(4 - raw.len()) {
            raw.push(0)
        }

        self.position += len;
        Ok(LittleEndian::read_u32(&raw))
    }

    pub fn read_var_i32(&mut self) -> WasmResult<i32> {
        let mut raw = self.read_var_raw(4)?;
        let len = raw.len();

        for _ in 0..(4 - raw.len()) {
            raw.push(0)
        }

        self.position += len;
        Ok(LittleEndian::read_i32(&raw))
    }

    pub fn read_var_u64(&mut self) -> WasmResult<u64> {
        let mut raw = self.read_var_raw(8)?;
        let len = raw.len();

        for _ in 0..(8 - raw.len()) {
            raw.push(0)
        }

        self.position += len;
        Ok(LittleEndian::read_u64(&raw))
    }

    pub fn read_var_i64(&mut self) -> WasmResult<i64> {
        let mut raw = self.read_var_raw(8)?;
        let len = raw.len();

        for _ in 0..(8 - raw.len()) {
            raw.push(0)
        }

        self.position += len;
        Ok(LittleEndian::read_i64(&raw))
    }

    pub fn read_var_f32(&mut self) -> WasmResult<f32> {
        let mut raw = self.read_var_raw(4)?;
        let len = raw.len();

        for _ in 0..(8 - raw.len()) {
            raw.push(0)
        }

        self.position += len;
        Ok(LittleEndian::read_u32(&raw) as f32)
    }

    pub fn read_var_f64(&mut self) -> WasmResult<f64> {
        let mut raw = self.read_var_raw(4)?;
        let len = raw.len();

        for _ in 0..(8 - raw.len()) {
            raw.push(0)
        }

        self.position += len;
        Ok(LittleEndian::read_u64(&raw) as f64)
    }

    pub fn read_u8(&mut self) -> WasmResult<u8> {
        let n = self.code.get(self.position).ok_or(WasmError::EOF)?;
        self.position += 1;
        Ok(*n)
    }
    fn peek_byte(&self, position: usize) -> WasmResult<u8> {
        let byte = self.code.get(position).ok_or(WasmError::EOF)?;
        Ok(*byte)
    }

    fn read_var_raw(&self, max_len: usize) -> WasmResult<Vec<u8>> {
        let byte = self.peek_byte(self.position)?;
        let mut bytes = vec![byte];
        if (bytes[0] & 0x80) == 0 {
            return Ok(bytes);
        }

        let mut len = 1;
        loop {
            if len > max_len {
                return Err(WasmError::InvalidLEB128);
            }
            len += 1;

            let byte = self.peek_byte(self.position + len)?;
            bytes[len - 1] = byte;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        Ok(bytes)
    }
}
