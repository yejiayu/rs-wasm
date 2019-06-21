#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame<'a> {
    Head { version: u32 },
    Section { section: Section, payload: &'a [u8] },
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
    Type {
        form: Type,
        params: Vec<Type>,
        returns: Vec<Type>,
    },
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
