#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Read;
    use std::path::PathBuf;

    use rs_wasm::primitives::Frame;
    use rs_wasm::Parser;

    #[test]
    fn test_parser() {
        let count = fs::read_dir("tests/wasm").unwrap().count();
        let mut current = 0;

        for entry in fs::read_dir("tests/wasm").unwrap() {
            let dir = entry.unwrap();
            if !dir.file_type().unwrap().is_file() {
                continue;
            }
            let data = read_file_data(&dir.path());
            let mut r = Parser::new(data.as_slice());
            loop {
                match r.read() {
                    Frame::End => break,
                    Frame::ParserError { err } => panic!(
                        "[test_parser] {:?}/{:?} file name {:?} err {:?}",
                        current,
                        count,
                        dir.path(),
                        err
                    ),
                    // Frame::Section { section } => println!("{:?}\n", section),
                    _ => continue,
                }
            }

            current += 1;
        }

        println!("Pass the parser test. {:?}/{:?}", current, count);
    }

    fn read_file_data(path: &PathBuf) -> Vec<u8> {
        let mut data = Vec::new();
        let mut f = File::open(path).ok().unwrap();
        f.read_to_end(&mut data).unwrap();
        data
    }
}
