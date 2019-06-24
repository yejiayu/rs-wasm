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
pub enum KindIndex {
    Type,
    Func,
    Table,
    Memory,
    Global,
    Local,
    Label,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section {
    Type(Vec<SectionTypeEntity>),
    Import,
    Function(Vec<SectionFuncEntity>),
    Table,
    Memory(Vec<SectionMemoryEntity>),
    Global,
    Export(Vec<SectionExportEntity>),
    Start,
    Element,
    Code(Vec<SectionCodeEntity>),
    Data(Vec<SectionDataEntity>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    // Control Instructions
    Unreachable,               // 0x00
    Nop,                       // 0x01
    Block { t: Type },         // 0x02
    Loop { t: Type },          // 0x03
    If { t: Type },            // 0x04
    Else,                      // 0x05
    End,                       // 0x0B
    Br { label_index: u32 },   // 0x0C
    BrIf { label_index: u32 }, // 0x0D
    BrTable,                   // 0x0E
    Return,                    // 0x0F
    Call,                      // 0x10
    CallIndirect,              // 0X11

    // Parametric Instructions
    Drop,   // 0x1A
    Select, // 0X1B

    // Variable Instructions¶
    LocalGet { local_index: u32 },   // 0x20
    LocalSet { local_index: u32 },   // 0x21
    LocalTee { local_index: u32 },   // 0x22
    GlobalGet { global_index: u32 }, // 0x23
    GlobalSet { global_index: u32 }, // 0x24

    // Memory Instructions
    I32Load { memarg: MemArg },    // 0x28
    I64Load { memarg: MemArg },    // 0x29
    F32Load { memarg: MemArg },    // 0x2A
    F64Load { memarg: MemArg },    // 0x2B
    I32Load8s { memarg: MemArg },  // 0x2C
    I32Load8u { memarg: MemArg },  // 0x2D
    I32Load16s { memarg: MemArg }, // 0x2E
    I32Load16u { memarg: MemArg }, // 0x2F
    I64Load8s { memarg: MemArg },  // 0x30
    I64Load8u { memarg: MemArg },  // 0x31
    I64Load16s { memarg: MemArg }, // 0x32
    I64Load16u { memarg: MemArg }, // 0x33
    I64Load32s { memarg: MemArg }, // 0x34
    I64Load32u { memarg: MemArg }, // 0x35
    I32Store { memarg: MemArg },   // 0x36
    I64Store { memarg: MemArg },   // 0x37
    F32Store { memarg: MemArg },   // 0x38
    F64Store { memarg: MemArg },   // 0x39
    I32Store8 { memarg: MemArg },  // 0x3A
    I32Store16 { memarg: MemArg }, // 0x3B
    I64Store8 { memarg: MemArg },  // 0x3C
    I64Store16 { memarg: MemArg }, // 0x3D
    I64Store32 { memarg: MemArg }, // 0x3E
    MemorySize { size: u32 },      // 0x3F
    MemoryGrow { grow: u32 },      // 0x40

    // Numeric Instructions
    I32Const { val: i32 }, // 0x41
    I64Const,              // 0x42
    F32Const,              // 0x43
    F64Const,              // 0x44

    I32Eqz, // 0x45
    I32Eq,  // 0x46
    I32Ne,  // 0x47
    I32LtS, // 0x48
    I32LtU, // 0x49
    I32GtS, // 0x4A
    I32GtU, // 0x4B
    I32LeS, // 0x4C
    I32LeU, // 0x4D
    I32GeS, // 0x4E
    I32GeU, // 0x4F

    I64Eqz, // 0x50
    I64Eq,  // 0x51
    I64Ne,  // 0x52
    I64LtS, // 0x53
    I64LtU, // 0x54
    I64GtS, // 0x55
    I64GtU, // 0x56
    I64LeS, // 0x57
    I64LeU, // 0x58
    I64GeS, // 0x59
    I64GeU, // 0x5A

    F32Eq, // 0x5B
    F32Ne, // 0x5C
    F32Lt, // 0x5D
    F32Gt, // 0x5E
    F32Le, // 0x6F
    F32Ge, // 0x60

    F64Eq, // 0x61
    F64Ne, // 0x62
    F64Lt, // 0x63
    F64Gt, // 0x64
    F64Le, // 0x65
    F64Ge, // 0x66

    I32Clz,    // 0x67
    I32Ctz,    // 0x68
    I32Popcnt, // 0x69
    I32Add,    // 0x6A
    I32Sub,    // 0x6B
    I32Mul,    // 0x6C
    I32DivS,   // 0x6D
    I32DivU,   // 0x6E
    I32RemS,   // 0x6F
    I32RemU,   // 0x70
    I32And,    // 0x71
    I32Or,     // 0x72
    I32Xor,    // 0x73
    I32Shl,    // 0x74
    I32ShrS,   // 0x75
    I32ShrU,   // 0x76
    I32Rotl,   // 0x77
    I32Rotr,   // 0x78

    I64Clz,    // 0x79
    I64Ctz,    // 0x7A
    I64Popcnt, // 0x7B
    I64Add,    // 0x7C
    I64Sub,    // 0x7D
    I64Mul,    // 0x7E
    I64DivS,   // 0x7F
    I64DivU,   // 0x80
    I64RemS,   // 0x81
    I64RemU,   // 0x82
    I64And,    // 0x83
    I64Or,     // 0x84
    I64Xor,    // 0x85
    I64Shl,    // 0x86
    I64ShrS,   // 0x87
    I64ShrU,   // 0x88
    I64Rotl,   // 0x89
    I64Rotr,   // 0x8A

    F32Abs,      // 0x8B
    F32Neg,      // 0x8C
    F32Ceil,     // 0x8D
    F32Floor,    // 0x8E
    F32Trunc,    // 0x8F
    F32Nearest,  // 0x90
    F32Sqrt,     // 0x91
    F32Add,      // 0x92
    F32Sub,      // 0x93
    F32Mul,      // 0x94
    F32Div,      // 0x95
    F32Min,      // 0x96
    F32Max,      // 0x97
    F32Copysign, // 0x98

    F64Abs,      // 0x99
    F64Neg,      // 0x9A
    F64Ceil,     // 0x9B
    F64Floor,    // 0x9C
    F64Trunc,    // 0x9D
    F64Nearest,  // 0x9E
    F64Sqrt,     // 0x9F
    F64Add,      // 0xA0
    F64Sub,      // 0xA1
    F64Mul,      // 0xA2
    F64Div,      // 0xA3
    F64Min,      // 0xA4
    F64Max,      // 0xA5
    F64Copysign, // 0xA6

    I32WrapI64,        // 0xA7
    I32TruncSF32,      // 0xA8
    I32TruncUF32,      // 0xA9
    I32TruncSF64,      // 0xAA
    I32TruncUF64,      // 0xAB
    I64ExtendSI32,     // 0xAC
    I64ExtendUI32,     // 0xAD
    I64TruncSF32,      // 0xAE
    I64TruncUF32,      // 0xAF
    I64TruncSF64,      // 0xB0
    I64TruncUF64,      // 0xB1
    F32ConvertSI32,    // 0xB2
    F32ConvertUI32,    // 0xB3
    F32ConvertSI64,    // 0xB4
    F32ConvertUI64,    // 0xB5
    F32DemoteF64,      // 0xB6
    F64ConvertSI32,    // 0xB7
    F64ConvertUI32,    // 0xB8
    F64ConvertSI64,    // 0xB9
    F64ConvertUI64,    // 0xBA
    F64PromoteF32,     // 0xBB
    I32ReinterpretF32, // 0xBC
    I64ReinterpretF64, // 0xBD
    F32ReinterpretI32, // 0xBE
    F64ReinterpretI64, // 0xBF
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionTypeEntity {
    pub form: Type,
    pub params: Vec<Type>,
    pub returns: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionFuncEntity {
    pub signature_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionExportEntity {
    pub name: String,
    pub kind: KindIndex,
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionCodeEntity {
    pub locals: Vec<Type>,
    pub expr: Vec<Operator>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionMemoryEntity {
    pub initial: u32,
    pub max: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionDataEntity {
    pub memid: u32,
    pub expr: Vec<Operator>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}
