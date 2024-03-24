
use std::io;
use base64::{engine::general_purpose::STANDARD, read::DecoderReader, write::EncoderWriter};

// https://docs.rs/base64/latest/base64/

// encode will encode the data using base64 encoding, and print the result to stdout
pub(crate) fn encode(data: &str) {
    let mut output = io::stdout();
    let mut r = io::Cursor::new(data);
    let mut encoder: EncoderWriter<'_, base64::engine::GeneralPurpose, &mut io::Stdout> = EncoderWriter::new(&mut output, &STANDARD);
    _ = io::copy(&mut r, &mut encoder);
}

// decode will decode the data using base64 decoding, and print the result to stdout
pub(crate) fn decode(data: &str) {
    let s = io::Cursor::new(data);
    let mut decoder = DecoderReader::new(s, &STANDARD);
    _ = io::copy(&mut decoder, &mut io::stdout());
}
