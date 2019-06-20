mod errors;
pub mod execution;

pub use errors::WasmError;

#[cfg(test)]
mod tests {
    use crate::execution;

    #[test]
    fn it_works() {
        let code = include_bytes!("../scripts/hello.wasm");
        execution::run(code).unwrap();
    }
}
