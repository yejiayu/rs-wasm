mod bytes_reader;

use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;

use crate::parser::bytes_reader::BytesReader;
use crate::primitives::{
    Frame, KindIndex, MemArg, Operator, Section, SectionCodeEntity, SectionDataEntity,
    SectionExportEntity, SectionFuncEntity, SectionMemoryEntity, SectionTypeEntity, Type,
};
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
        let section_code = self.reader.read_var_u8()?;
        let section = match section_code {
            0x01 => self.section_type()?,
            0x02 => Section::Import,
            0x03 => self.section_function()?,
            0x04 => Section::Table,
            0x05 => self.section_memory()?,
            0x06 => Section::Global,
            0x07 => self.section_export()?,
            0x08 => Section::Start,
            0x09 => Section::Element,
            0x0A => self.section_code()?,
            0x0B => self.section_data()?,
            _ => return Err(WasmError::InvalidSection(section_code)),
        };

        Ok(Frame::Section(section))
    }

    fn section_type(&mut self) -> WasmResult<Section> {
        let mut reader = parser_payload(&mut self.reader)?;

        let entries = parser_section_type_entries(&mut reader)?;
        Ok(Section::Type(entries))
    }

    fn section_function(&mut self) -> WasmResult<Section> {
        let mut reader = parser_payload(&mut self.reader)?;

        let count = reader.read_var_u32()? as usize;
        let mut entries = Vec::with_capacity(count);
        for _ in 0..count {
            entries.push(SectionFuncEntity {
                signature_index: reader.read_var_u32()? as usize,
            });
        }

        Ok(Section::Function(entries))
    }

    fn section_export(&mut self) -> WasmResult<Section> {
        let mut reader = parser_payload(&mut self.reader)?;

        let entires = parser_section_export_entries(&mut reader)?;
        Ok(Section::Export(entires))
    }

    fn section_code(&mut self) -> WasmResult<Section> {
        let mut reader = parser_payload(&mut self.reader)?;

        let entires = parser_section_code_entries(&mut reader)?;
        Ok(Section::Code(entires))
    }

    fn section_memory(&mut self) -> WasmResult<Section> {
        let mut reader = parser_payload(&mut self.reader)?;

        let entries = parser_section_memory_entries(&mut reader)?;
        Ok(Section::Memory(entries))
    }

    fn section_data(&mut self) -> WasmResult<Section> {
        let mut reader = parser_payload(&mut self.reader)?;

        let entries = parser_section_data_entries(&mut reader)?;
        Ok(Section::Data(entries))
    }
}

fn parser_section_type_entries(reader: &mut BytesReader) -> WasmResult<Vec<SectionTypeEntity>> {
    let count = reader.read_var_u32()? as usize;

    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let form = parser_type(reader)?;
        let param_count = reader.read_var_u32()? as usize;

        let mut params = Vec::with_capacity(param_count);
        for _ in 0..param_count {
            params.push(parser_type(reader)?);
        }

        let return_count = reader.read_var_u32()? as usize;
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

fn parser_section_export_entries(reader: &mut BytesReader) -> WasmResult<Vec<SectionExportEntity>> {
    let count = reader.read_var_u32()? as usize;

    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let name = String::from_utf8(parser_bytes(reader)?)?;

        let kind = parser_kind_type(reader)?;
        let index = reader.read_var_u32()?;
        entries.push(SectionExportEntity { name, kind, index });
    }

    Ok(entries)
}

fn parser_section_code_entries(reader: &mut BytesReader) -> WasmResult<Vec<SectionCodeEntity>> {
    let count = reader.read_var_u32()? as usize;

    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let mut payload_reader = parser_payload(reader)?;
        let mut locals = vec![];
        let decl_count = payload_reader.read_var_u32()? as usize;

        for _ in 0..decl_count {
            let local_count = payload_reader.read_var_u32()? as usize;
            let t = parser_type(&mut payload_reader)?;

            for _ in 0..local_count {
                locals.push(t.clone());
            }
        }

        let expr = parser_expr(&mut payload_reader)?;

        entries.push(SectionCodeEntity { locals, expr });
    }

    Ok(entries)
}

fn parser_section_memory_entries(reader: &mut BytesReader) -> WasmResult<Vec<SectionMemoryEntity>> {
    let count = reader.read_var_u32()? as usize;

    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let flag = reader.read_var_u8()?;
        let mem = if flag == 0 {
            let initial = reader.read_var_u32()?;
            SectionMemoryEntity { initial, max: None }
        } else {
            let initial = reader.read_var_u32()?;
            let max = reader.read_var_u32()?;

            SectionMemoryEntity {
                initial,
                max: Some(max),
            }
        };

        entries.push(mem);
    }

    Ok(entries)
}

fn parser_section_data_entries(reader: &mut BytesReader) -> WasmResult<Vec<SectionDataEntity>> {
    let count = reader.read_var_u32()? as usize;

    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        // skip flag
        let memid = reader.read_var_u32()?;
        let expr = parser_expr(reader)?;
        let data = parser_bytes(reader)?;

        entries.push(SectionDataEntity { memid, expr, data });
    }

    Ok(entries)
}

fn parser_type(reader: &mut BytesReader) -> WasmResult<Type> {
    let type_code = reader.read_var_i8()?;
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

fn parser_kind_type(reader: &mut BytesReader) -> WasmResult<KindIndex> {
    let kind_code = reader.read_var_u32()?;
    match kind_code {
        0x00 => Ok(KindIndex::Func),
        0x01 => Ok(KindIndex::Table),
        0x02 => Ok(KindIndex::Memory),
        0x03 => Ok(KindIndex::Global),
        _ => Err(WasmError::InvalidKindType(kind_code)),
    }
}

fn parser_payload(reader: &mut BytesReader) -> WasmResult<BytesReader> {
    let payload_len = reader.read_var_u8()? as usize;
    let payload = reader.read_range(payload_len)?;

    Ok(BytesReader::new(Bytes::from(payload), 0))
}

fn parser_bytes(reader: &mut BytesReader) -> WasmResult<Vec<u8>> {
    let len = reader.read_var_u32()? as usize;
    Ok(reader.read_range(len)?.to_vec())
}

fn parser_expr(reader: &mut BytesReader) -> WasmResult<Vec<Operator>> {
    let mut expr = vec![];
    loop {
        let opcode = parser_operator(reader)?;
        match opcode {
            Operator::End => {
                expr.push(opcode);
                break;
            }
            _ => expr.push(opcode),
        }
    }

    Ok(expr)
}

fn parser_memarg(reader: &mut BytesReader) -> WasmResult<MemArg> {
    let align = reader.read_var_u32()?;
    let offset = reader.read_var_u32()?;

    Ok(MemArg { align, offset })
}

fn parser_operator(reader: &mut BytesReader) -> WasmResult<Operator> {
    let op_code = reader.read_var_u32()? as u8;
    let op = match op_code {
        0x00 => Operator::Unreachable,
        0x01 => Operator::Nop,
        0x02 => {
            let t = parser_type(reader)?;
            Operator::Block { t }
        }
        0x03 => {
            let t = parser_type(reader)?;
            Operator::Loop { t }
        }
        0x04 => {
            let t = parser_type(reader)?;
            Operator::If { t }
        }
        0x05 => Operator::Else,
        0x0B => Operator::End,
        0x0C => {
            let label_index = reader.read_var_u32()?;
            Operator::Br { label_index }
        }
        0x0D => {
            let label_index = reader.read_var_u32()?;
            Operator::BrIf { label_index }
        }
        0x0E => Operator::BrTable,
        0x0F => Operator::Return,
        0x10 => Operator::Call,
        0x11 => Operator::CallIndirect,

        0x1A => Operator::Drop,
        0x1B => Operator::Select,

        0x20 => {
            let local_index = reader.read_var_u32()?;
            Operator::LocalGet { local_index }
        }
        0x21 => {
            let local_index = reader.read_var_u32()?;
            Operator::LocalSet { local_index }
        }
        0x22 => {
            let local_index = reader.read_var_u32()?;
            Operator::LocalTee { local_index }
        }
        0x23 => {
            let global_index = reader.read_var_u32()?;
            Operator::GlobalGet { global_index }
        }
        0x24 => {
            let global_index = reader.read_var_u32()?;
            Operator::GlobalSet { global_index }
        }

        0x28 => Operator::I32Load {
            memarg: parser_memarg(reader)?,
        },
        0x29 => Operator::I64Load {
            memarg: parser_memarg(reader)?,
        },
        0x2A => Operator::F32Load {
            memarg: parser_memarg(reader)?,
        },
        0x2B => Operator::F64Load {
            memarg: parser_memarg(reader)?,
        },
        0x2C => Operator::I32Load8s {
            memarg: parser_memarg(reader)?,
        },
        0x2D => Operator::I32Load8u {
            memarg: parser_memarg(reader)?,
        },
        0x2E => Operator::I32Load16s {
            memarg: parser_memarg(reader)?,
        },
        0x2F => Operator::I32Load16u {
            memarg: parser_memarg(reader)?,
        },
        0x30 => Operator::I64Load8s {
            memarg: parser_memarg(reader)?,
        },
        0x31 => Operator::I64Load8u {
            memarg: parser_memarg(reader)?,
        },
        0x32 => Operator::I64Load16s {
            memarg: parser_memarg(reader)?,
        },
        0x33 => Operator::I64Load16u {
            memarg: parser_memarg(reader)?,
        },
        0x34 => Operator::I64Load32s {
            memarg: parser_memarg(reader)?,
        },
        0x35 => Operator::I64Load32u {
            memarg: parser_memarg(reader)?,
        },
        0x36 => Operator::I32Store {
            memarg: parser_memarg(reader)?,
        },
        0x37 => Operator::I64Store {
            memarg: parser_memarg(reader)?,
        },
        0x38 => Operator::F32Store {
            memarg: parser_memarg(reader)?,
        },
        0x39 => Operator::F64Store {
            memarg: parser_memarg(reader)?,
        },
        0x3A => Operator::I32Store8 {
            memarg: parser_memarg(reader)?,
        },
        0x3B => Operator::I32Store16 {
            memarg: parser_memarg(reader)?,
        },
        0x3C => Operator::I64Store8 {
            memarg: parser_memarg(reader)?,
        },
        0x3D => Operator::I64Store16 {
            memarg: parser_memarg(reader)?,
        },
        0x3E => Operator::I64Store32 {
            memarg: parser_memarg(reader)?,
        },
        0x3F => {
            let size = reader.read_var_u32()?;
            Operator::MemorySize { size }
        }
        0x40 => {
            let grow = reader.read_var_u32()?;
            Operator::MemoryGrow { grow }
        }

        0x41 => {
            let val = reader.read_var_i32()?;
            Operator::I32Const { val }
        }
        0x42 => Operator::I64Const,
        0x43 => Operator::F32Const,
        0x44 => Operator::F64Const,

        0x45 => Operator::I32Eqz,
        0x46 => Operator::I32Eq,
        0x47 => Operator::I32Ne,
        0x48 => Operator::I32LtS,
        0x49 => Operator::I32LtU,
        0x4A => Operator::I32GtS,
        0x4B => Operator::I32GtU,
        0x4C => Operator::I32LeS,
        0x4D => Operator::I32LeU,
        0x4E => Operator::I32GeS,
        0x4F => Operator::I32GeU,

        0x50 => Operator::I64Eqz,
        0x51 => Operator::I64Eq,
        0x52 => Operator::I64Ne,
        0x53 => Operator::I64LtS,
        0x54 => Operator::I64LtU,
        0x55 => Operator::I64GtS,
        0x56 => Operator::I64GtU,
        0x57 => Operator::I64LeS,
        0x58 => Operator::I64LeU,
        0x59 => Operator::I64GeS,
        0x5A => Operator::I64GeU,

        0x5B => Operator::F32Eq,
        0x5C => Operator::F32Ne,
        0x5D => Operator::F32Lt,
        0x5E => Operator::F32Gt,
        0x5F => Operator::F32Le,
        0x60 => Operator::F32Ge,

        0x61 => Operator::F64Eq,
        0x62 => Operator::F64Ne,
        0x63 => Operator::F64Lt,
        0x64 => Operator::F64Gt,
        0x65 => Operator::F64Le,
        0x66 => Operator::F64Ge,

        0x67 => Operator::I32Clz,
        0x68 => Operator::I32Ctz,
        0x69 => Operator::I32Popcnt,
        0x6A => Operator::I32Add,
        0x6B => Operator::I32Sub,
        0x6C => Operator::I32Mul,
        0x6D => Operator::I32DivS,
        0x6E => Operator::I32DivU,
        0x6F => Operator::I32RemS,
        0x70 => Operator::I32RemU,
        0x71 => Operator::I32And,
        0x72 => Operator::I32Or,
        0x73 => Operator::I32Xor,
        0x74 => Operator::I32Shl,
        0x75 => Operator::I32ShrS,
        0x76 => Operator::I32ShrU,
        0x77 => Operator::I32Rotl,
        0x78 => Operator::I32Rotr,

        0x79 => Operator::I64Clz,
        0x7A => Operator::I64Ctz,
        0x7B => Operator::I64Popcnt,
        0x7C => Operator::I64Add,
        0x7D => Operator::I64Sub,
        0x7E => Operator::I64Mul,
        0x7F => Operator::I64DivS,
        0x80 => Operator::I64DivU,
        0x81 => Operator::I64RemS,
        0x82 => Operator::I64RemU,
        0x83 => Operator::I64And,
        0x84 => Operator::I64Or,
        0x85 => Operator::I64Xor,
        0x86 => Operator::I64Shl,
        0x87 => Operator::I64ShrS,
        0x88 => Operator::I64ShrU,
        0x89 => Operator::I64Rotl,
        0x8A => Operator::I64Rotr,

        0x8B => Operator::F32Abs,
        0x8C => Operator::F32Neg,
        0x8D => Operator::F32Ceil,
        0x8E => Operator::F32Floor,
        0x8F => Operator::F32Trunc,
        0x90 => Operator::F32Nearest,
        0x91 => Operator::F32Sqrt,
        0x92 => Operator::F32Add,
        0x93 => Operator::F32Sub,
        0x94 => Operator::F32Mul,
        0x95 => Operator::F32Div,
        0x96 => Operator::F32Min,
        0x97 => Operator::F32Max,
        0x98 => Operator::F32Copysign,

        0x99 => Operator::F64Abs,
        0x9A => Operator::F64Neg,
        0x9B => Operator::F64Ceil,
        0x9C => Operator::F64Floor,
        0x9D => Operator::F64Trunc,
        0x9E => Operator::F64Nearest,
        0x9F => Operator::F64Sqrt,
        0xA0 => Operator::F64Add,
        0xA1 => Operator::F64Sub,
        0xA2 => Operator::F64Mul,
        0xA3 => Operator::F64Div,
        0xA4 => Operator::F64Min,
        0xA5 => Operator::F64Max,
        0xA6 => Operator::F64Copysign,

        0xA7 => Operator::I32WrapI64,
        0xA8 => Operator::I32TruncSF32,
        0xA9 => Operator::I32TruncUF32,
        0xAA => Operator::I32TruncSF64,
        0xAB => Operator::I32TruncUF64,
        0xAC => Operator::I64ExtendSI32,
        0xAD => Operator::I64ExtendUI32,
        0xAE => Operator::I64TruncSF32,
        0xAF => Operator::I64TruncUF32,
        0xB0 => Operator::I64TruncSF64,
        0xB1 => Operator::I64TruncUF64,
        0xB2 => Operator::F32ConvertSI32,
        0xB3 => Operator::F32ConvertUI32,
        0xB4 => Operator::F32ConvertSI64,
        0xB5 => Operator::F32ConvertUI64,
        0xB6 => Operator::F32DemoteF64,
        0xB7 => Operator::F64ConvertSI32,
        0xB8 => Operator::F64ConvertUI32,
        0xB9 => Operator::F64ConvertSI64,
        0xBA => Operator::F64ConvertUI64,
        0xBB => Operator::F64PromoteF32,
        0xBC => Operator::I32ReinterpretF32,
        0xBD => Operator::I64ReinterpretF64,
        0xBE => Operator::F32ReinterpretI32,
        0xBF => Operator::F64ReinterpretI64,
        _ => return Err(WasmError::InvalidOperator(op_code)),
    };

    Ok(op)
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
        r.read().unwrap();
        r.read().unwrap();
        r.read().unwrap();
        r.read().unwrap();
        let f = r.read().unwrap();
        println!("func {:?}", f);
    }
}
