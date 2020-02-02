//! Decompresses the input from stdin and writes the result to stdout.

extern crate lzw;

use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
    let mut decoder = lzw::Decoder::new(lzw::LsbReader::new(), 8);
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    loop {
        let len = {
            let buf = stdin.fill_buf()?;
            if buf.is_empty() {
                break;
            }
            let (len, bytes) = decoder.decode_bytes(buf)?;
            stdout.write_all(bytes)?;
            len
        };
        stdin.consume(len);
    }
    Ok(())
}
