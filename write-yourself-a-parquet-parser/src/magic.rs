use anyhow::{Ok, Result, bail};
use bytes::Bytes;

pub fn ensure_header_footer_magic(data: Bytes) -> Result<()> {
    if data.len() < 8 || !data.starts_with(b"PAR1") || !data.ends_with(b"PAR1") {
        bail!("Magic: not a parquet file")
    }
    Ok(())
}
