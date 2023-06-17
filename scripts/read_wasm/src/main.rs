use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use wasmparser::{Chunk, Parser, Payload::*};

fn parse(mut reader: impl std::io::Read) -> Result<()> {
    let mut buf = Vec::new();
    let mut parser = Parser::new(0);
    let mut eof = false;

    loop {
        let (payload, consumed) = match parser.parse(&buf, eof)? {
            Chunk::NeedMoreData(hint) => {
                assert!(!eof);

                let len = buf.len();
                buf.extend((0..hint).map(|_| 0u8));
                let n = reader.read(&mut buf[len..])?;
                buf.truncate(len + n);
                eof = n == 0;
                continue;
            }

            Chunk::Parsed { consumed, payload } => (payload, consumed),
        };

        match payload {
            Version { version, range } => {
                println!("Version section - Version: {}, Range: {:?}", version, range);
            }
            TypeSection(types) => {
                println!(
                    "Type section - Count: {}, Range: {:?}",
                    types.get_count()?,
                    types.range()
                );
            }
            ImportSection(imports) => {
                println!(
                    "Import section - Count: {}, Range: {:?}",
                    imports.get_count()?,
                    imports.range()
                );
            }
            FunctionSection(functions) => {
                println!(
                    "Function section - Count: {}, Range: {:?}",
                    functions.get_count()?,
                    functions.range()
                );
            }
            TableSection(tables) => {
                println!(
                    "Table section - Count: {}, Range: {:?}",
                    tables.get_count()?,
                    tables.range()
                );
            }
            MemorySection(memories) => {
                println!(
                    "Memory section - Count: {}, Range: {:?}",
                    memories.get_count()?,
                    memories.range()
                );
            }
            GlobalSection(globals) => {
                println!(
                    "Global section - Count: {}, Range: {:?}",
                    globals.get_count()?,
                    globals.range()
                );
            }
            ExportSection(exports) => {
                println!(
                    "Export section - Count: {}, Range: {:?}",
                    exports.get_count()?,
                    exports.range()
                );
            }
            StartSection { func, range } => {
                println!("Start section - Function: {}, Range: {:?}", func, range);
            }
            ElementSection(elements) => {
                println!(
                    "Element section - Count: {}, Range: {:?}",
                    elements.get_count()?,
                    elements.range()
                );
            }
            CodeSectionStart { count, range } => {
                println!("Code section - Count: {}, Range: {:?}", count, range);
            }
            DataSection(data) => {
                println!(
                    "Data section - Count: {}, Range: {:?}",
                    data.get_count()?,
                    data.range()
                );
            }
            CustomSection { name, data, range } => {
                println!(
                    "Custom section - Name: {}, Data length: {}, Range: {:?}",
                    name,
                    data.len(),
                    range
                );
            }
            _ => {}
        }

        // Remove the consumed bytes from the buffer
        buf.drain(..consumed);
    }

    Ok(())
}

fn main() -> Result<()> {
    let file = File::open("path_to_your_wasm_file.wasm")?;
    let reader = BufReader::new(file);

    parse(reader)
}
