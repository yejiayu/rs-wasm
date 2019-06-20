use wasmparser::{
    OperatorValidatorConfig, ParserState, Range, SectionCode, ValidatingParser,
    ValidatingParserConfig, WasmDecoder,
};

use crate::WasmError;

pub fn run(code: &[u8]) -> Result<(), WasmError> {
    // let config = ValidatingParserConfig::default();
    let mut parser = ValidatingParser::new(
        code,
        Some(ValidatingParserConfig {
            operator_config: OperatorValidatorConfig {
                enable_threads: false,
                enable_reference_types: false,
                enable_simd: false,
                enable_bulk_memory: false,
            },
            mutable_global_imports: false,
        }),
    );

    loop {
        match *parser.read() {
            ParserState::Error(err) => log::error!("{:?}", err),
            ParserState::Initial => log::debug!("wasm initial"),
            ParserState::BeginWasm { version } => begin_wasm(version),
            ParserState::EndWasm => break,
            ParserState::BeginSection { code, range } => begin_section(code, range),
            ParserState::EndSection => break,
            _ => println!("opcode"),
        }
    }

    Ok(())
}

fn begin_wasm(version: u32) {
    log::debug!("wasm version {:?}", version);
}

fn begin_section<'a>(code: SectionCode<'a>, range: Range) {}
