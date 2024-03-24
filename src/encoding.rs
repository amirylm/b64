
use std::io;
use base64::{engine::general_purpose::STANDARD, read::DecoderReader, write::{EncoderWriter}};

// https://docs.rs/base64/latest/base64/

// encode will encode the data using base64 encoding, and print the result to stdout
pub(crate) fn encode<W: ?Sized>(data: &str, mut writer: &mut W) where W: io::Write {
    let mut r = io::Cursor::new(data);
    let mut encoder: EncoderWriter<'_, base64::engine::GeneralPurpose, &mut dyn io::Write> = EncoderWriter::new(&mut writer, &STANDARD);
    _ = io::copy(&mut r, &mut encoder);
}

// decode will decode the data using base64 decoding into the given writer, and print the result to stdout
pub(crate) fn decode<W: ?Sized>(data: &str, writer: &mut W) where W: io::Write {
    let s = io::Cursor::new(data);
    let mut decoder = DecoderReader::new(s, &STANDARD);
    _ = io::copy(&mut decoder, writer);
}
