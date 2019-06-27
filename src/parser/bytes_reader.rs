use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
use codicon::Decoder;
use lebicon::Leb128;

use crate::{WasmError, WasmResult};

pub struct BytesReader {
    code: Bytes,
    position: usize,
}

impl BytesReader {
    pub fn new(code: Bytes, position: usize) -> Self {
        Self { code, position }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == self.code.len()
    }

    pub fn len(&self) -> usize {
        self.position
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
        self.read_var::<u32>(1)
    }

    pub fn read_var_i8(&mut self) -> WasmResult<i32> {
        self.read_var::<i32>(1)
    }

    pub fn read_var_u32(&mut self) -> WasmResult<u32> {
        self.read_var::<u32>(4)
    }

    pub fn read_var_i32(&mut self) -> WasmResult<i32> {
        self.read_var::<i32>(4)
    }

    // pub fn read_var_u64(&mut self) -> WasmResult<u64> {
    //     self.read_var::<u64>(8)
    // }

    pub fn read_var_i64(&mut self) -> WasmResult<i64> {
        let a = self.read_var::<i64>(9)?;
        Ok(a)
    }

    pub fn read_var_f32(&mut self) -> WasmResult<u32> {
        let u = self.peek_u32(self.position)?;
        self.position += 4;
        Ok(u)
    }

    pub fn read_var_f64(&mut self) -> WasmResult<u64> {
        let u1 = u64::from(self.peek_u32(self.position)?);
        let u2 = u64::from(self.peek_u32(self.position + 4)?);
        self.position += 8;
        Ok(u1 | (u2 << 32))
    }

    pub fn read_u32(&mut self) -> WasmResult<u32> {
        let buf = self.read_range(4)?;
        Ok(LittleEndian::read_u32(buf))
    }

    pub fn read_str(&mut self, len: usize) -> WasmResult<String> {
        let bytes = self.read_range(len)?;
        let s = String::from_utf8(bytes.to_owned())?;
        Ok(s)
    }

    pub fn read_u8(&mut self) -> WasmResult<u8> {
        let byte = self.code.get(self.position).ok_or(WasmError::EOF)?;
        self.position += 1;
        Ok(*byte)
    }

    fn peek_byte(&self, position: usize) -> WasmResult<u8> {
        let byte = self.code.get(position).ok_or(WasmError::EOF)?;
        Ok(*byte)
    }

    fn peek_u32(&self, position: usize) -> WasmResult<u32> {
        let b1 = u32::from(self.peek_byte(position)?);
        let b2 = u32::from(self.peek_byte(position + 1)?);
        let b3 = u32::from(self.peek_byte(position + 2)?);
        let b4 = u32::from(self.peek_byte(position + 3)?);

        Ok(b1 | (b2 << 8) | (b3 << 16) | (b4 << 24))
    }

    fn read_var<T: Decoder<Leb128>>(&mut self, max_len: usize) -> WasmResult<T> {
        let byte = self.peek_byte(self.position)?;
        let mut bytes = vec![byte];
        if (bytes[0] & 0x80) == 0 {
            let value = T::decode(&mut &bytes[..], Leb128).map_err(|_| WasmError::InvalidLEB128)?;
            self.position += 1;
            return Ok(value);
        }

        let mut len = bytes.len();
        loop {
            if len > max_len {
                return Err(WasmError::InvalidLEB128);
            }
            len += 1;

            let byte = self.peek_byte(self.position + len - 1)?;
            bytes.push(byte);
            if (byte & 0x80) == 0 {
                break;
            }
        }

        let value = T::decode(&mut &bytes[..], Leb128).map_err(|_| WasmError::InvalidLEB128)?;
        self.position += bytes.len();
        Ok(value)
    }
}
