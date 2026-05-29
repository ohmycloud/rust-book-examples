use anyhow::Result;
use bytes::Bytes;

pub fn read_file_metadata(data: Bytes) -> Result<FileMetaData> {
    let metadata_size = data.slice(data.len() - 8..).get_u32_le() as usize;
    let metadata_bytes = data.slice(data.len() - 8 - metadata_size..data.len() - 8);

    let (metadata, remaining) = read_thrift_metadata::<FileMetaData>(metadata_bytes)?;
    Ok(metadata)
}
