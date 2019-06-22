#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame {
    Head { version: u32 },
    Section(Section),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
    V128,
    AnyFunc,
    AnyRef,
    Func,
    EmptyBlockType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section {
    Type(Vec<SectionTypeEntity>),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionTypeEntity {
    pub form: Type,
    pub params: Vec<Type>,
    pub returns: Vec<Type>,
}
