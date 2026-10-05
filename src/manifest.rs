use crate::types::Manifest;
use std::{fs, io, os::unix::fs::MetadataExt, path::Path};

pub fn manifest_create(path: &Path) -> Result<Manifest, io::Error> {
    let md = match fs::metadata(path) {
        Ok(md) => md,
        Err(e) => return Err(e),
    };
    let mut fiemap = Manifest::default();
    fiemap.name = path.file_name().unwrap().to_string_lossy().to_string();
    fiemap.size = md.size();
    fiemap.blocks_count = md.blocks();

    Ok(fiemap)
}
