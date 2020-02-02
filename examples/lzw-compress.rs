//! Compresses the input from stdin and writes the result to stdout.

extern crate lzw;

use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut encoder = lzw::Encoder::new(lzw::LsbWriter::new(io::stdout()), 8)?;
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    loop {
        let len = {
            let buf = stdin.fill_buf()?;
            encoder.encode_bytes(buf)?;
            buf.len()
        };
        if len == 0 {
            break Ok(());
        }
        stdin.consume(len);
    }
}
