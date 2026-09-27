use std::{
    fs, io,
    os::unix::fs::MetadataExt,
    path::{self, Path},
};

#[derive(Default)]
struct FileMetadata {
    name: String,
    size: u64,
    blocks_count: u64,
}

pub fn sender(path: &Path, ip_addr: String) -> io::Result<()> {
    let md = fs::metadata(path)?;
    let mut fiemap = FileMetadata::default();
    fiemap.name = path.file_name().unwrap().to_string_lossy().to_string();
    fiemap.size = md.size();
    fiemap.blocks_count = md.blocks();

    Ok(())
}
