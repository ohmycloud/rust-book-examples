use bytes::Bytes;
use parquet_parser::ensure_header_footer_magic;

#[test]
pub fn magic_correct_header_and_footer() {
    let data = Bytes::from("PAR1xyzPAR1");
    assert!(ensure_header_footer_magic(data).is_ok());
}
