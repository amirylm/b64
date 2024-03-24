#[path = "./encoding.rs"]
pub mod encoding;

#[cfg(test)]
mod tests {
    use std::io::BufWriter;
    use crate::encoding::{encode,decode};

    #[test]
    fn encode_test() {
        let data = "hello";
        let mut writer = BufWriter::new(Vec::new());
        encode(data, &mut writer);
        let result = writer.into_inner().unwrap();
        assert_eq!(result, b"aGVsbG8=");
    }

    #[test]
    fn decode_test() {
        let data = "aGVsbG8=";
        let mut writer = BufWriter::new(Vec::new());
        decode(data, &mut writer);
        let result = writer.into_inner().unwrap();
        assert_eq!(result, b"hello");
    }
}