mod bytes_reader;

use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;

use crate::parser::bytes_reader::BytesReader;
use crate::primitives::{Frame, Section, SectionFuncEntity, SectionTypeEntity, Type};
use crate::{WasmError, WasmResult};

const WASM_MAGIC_NUMBER: u32 = 0x6d736100;
// const WASM_EXPERIMENTAL_VERSION: u32 = 0xd;
const WASM_SUPPORTED_VERSION: u32 = 0x1;

pub struct Parser {
    reader: BytesReader,
    initialize: bool,
}

impl Parser {
    pub fn new(code: Bytes) -> Self {
        Self {
            reader: BytesReader::new(code, 0),
            initialize: false,
        }
    }

    pub fn read(&mut self) -> WasmResult<Frame> {
        if !self.initialize {
            let head = self.head()?;
            self.initialize = true;
            Ok(head)
        } else {
            self.section()
        }
    }

    fn head(&mut self) -> WasmResult<Frame> {
        let magic_number = LittleEndian::read_u32(self.reader.read_range(4)?);
        if magic_number != WASM_MAGIC_NUMBER {
            return Err(WasmError::InvalidMagicNumber(magic_number));
        }

        let version = LittleEndian::read_u32(self.reader.read_range(4)?);
        if version != WASM_SUPPORTED_VERSION {
            return Err(WasmError::InvalidVersion(version));
        }
        Ok(Frame::Head { version })
    }

    fn section(&mut self) -> WasmResult<Frame> {
        let section_code = self.reader.read_u7()?;
        let section = match section_code {
            0x01 => self.section_type()?,
            0x02 => Section::Import,
            0x03 => self.section_function()?,
            0x04 => Section::Table,
            0x05 => Section::Memory,
            0x06 => Section::Global,
            0x07 => Section::Export,
            0x08 => Section::Start,
            0x09 => Section::Element,
            0x0a => Section::Code,
            0x0b => Section::Data,
            _ => return Err(WasmError::InvalidSection(section_code)),
        };

        Ok(Frame::Section(section))
    }

    fn section_type(&mut self) -> WasmResult<Section> {
        let payload_len = self.reader.read_u7()? as usize;
        let payload = self.reader.read_range(payload_len)?;

        let mut reader = BytesReader::new(Bytes::from(payload), 0);

        let entries = parser_section_type_entries(&mut reader)?;
        Ok(Section::Type(entries))
    }

    fn section_function(&mut self) -> WasmResult<Section> {
        let payload_len = self.reader.read_u7()? as usize;
        let payload = self.reader.read_range(payload_len)?;

        let mut reader = BytesReader::new(Bytes::from(payload), 0);

        let count = reader.read_u32()? as usize;
        let mut entries = Vec::with_capacity(count);
        for _ in 0..count {
            entries.push(SectionFuncEntity {
                signature_index: reader.read_u32()? as usize,
            });
        }

        Ok(Section::Function(entries))
    }
}

fn parser_section_type_entries(reader: &mut BytesReader) -> WasmResult<Vec<SectionTypeEntity>> {
    let count = reader.read_u32()? as usize;

    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let form = parser_type(reader)?;
        let param_count = reader.read_u32()? as usize;

        let mut params = Vec::with_capacity(param_count);
        for _ in 0..param_count {
            params.push(parser_type(reader)?);
        }

        let return_count = reader.read_u32()? as usize;
        let mut returns = Vec::with_capacity(return_count);
        for _ in 0..return_count {
            returns.push(parser_type(reader)?);
        }

        entries.push(SectionTypeEntity {
            form,
            params,
            returns,
        })
    }

    Ok(entries)
}

fn parser_type(reader: &mut BytesReader) -> WasmResult<Type> {
    let type_code = reader.read_i7()?;
    match type_code {
        0x7F => Ok(Type::I32),
        0x7E => Ok(Type::I64),
        0x7D => Ok(Type::F32),
        0x7C => Ok(Type::F64),

        0x40 => Ok(Type::EmptyBlockType),
        0x60 => Ok(Type::Func),
        0x70 => Ok(Type::AnyRef),
        _ => Err(WasmError::InvalidType(type_code)),
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::parser::Parser;

    #[test]
    fn test_head() {
        let code = include_bytes!("../../scripts/module.wasm");
        let code = Bytes::from(code.as_ref());
        let mut r = Parser::new(code);
        r.head().unwrap();
    }

    #[test]
    fn test_read() {
        let code = include_bytes!("../../scripts/func.wasm");
        let code = Bytes::from(code.as_ref());
        let mut r = Parser::new(code);
        r.read().unwrap();
        r.read().unwrap();
        let f = r.read().unwrap();
        println!("func {:?}", f);
    }
}
