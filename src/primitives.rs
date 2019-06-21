use std::convert::TryFrom;

use crate::WasmError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame {
    Head(Head),
    Section(Section),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Head {
    pub version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section {
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
}

impl TryFrom<i32> for Section {
    type Error = WasmError;

    fn try_from(section: i32) -> Result<Section, Self::Error> {
        match section {
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
            _ => Err(WasmError::InvalidSection(section)),
        }
    }
}
