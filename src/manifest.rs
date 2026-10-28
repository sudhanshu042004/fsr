use crate::types::Manifest;
use std::fs::File;
use std::mem::size_of;
use std::os::fd::AsRawFd;
use std::{fs, io, os::unix::fs::MetadataExt, path::Path};

const FS_IOC_FIEMAP: libc::c_ulong = 0xC020660B;

const FIEMAP_FLAG_SYNC: u32 = 0x0000_0001;
const FIEMAP_EXTENT_LAST: u32 = 0x0000_0001;
const FIEMAP_EXTENT_UNKNOWN: u32 = 0x0000_0002;
const FIEMAP_EXTENT_DELALLOC: u32 = 0x0000_0004;
const FIEMAP_EXTENT_UNWRITTEN: u32 = 0x0000_0800;
const FIEMAP_EXTENT_ENCODED: u32 = 0x0000_0008;
const FIEMAP_EXTENT_DATA_ENCRYPTED: u32 = 0x0000_0080;

const EXTENTS_PER_REQUEST: usize = 128;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct FiemapExtent {
    logical: u64,
    physical: u64,
    length: u64,
    reserved64: [u64; 2],
    flags: u32,
    reserved: [u32; 3],
}

#[repr(C)]
struct Fiemap {
    start: u64,
    length: u64,
    flags: u32,
    mapped_extents: u32,
    extent_count: u32,
    reserved: u32,
}

fn get_extents(file: &File, start: u64) -> io::Result<(Vec<FiemapExtent>, bool)> {
    let header_size = size_of::<Fiemap>();
    let extent_size = size_of::<FiemapExtent>();

    // Allocate an aligned buffer containing the header and
    // enough space for EXTENTS_PER_REQUEST extent records.
    let bytes = header_size + EXTENTS_PER_REQUEST * extent_size;
    let words = bytes.div_ceil(size_of::<u64>());
    let mut buffer = vec![0u64; words];

    let header = buffer.as_mut_ptr().cast::<Fiemap>();

    unsafe {
        (*header).start = start;
        (*header).length = u64::MAX - start;
        (*header).flags = FIEMAP_FLAG_SYNC;
        (*header).mapped_extents = 0;
        (*header).extent_count = EXTENTS_PER_REQUEST as u32;
        (*header).reserved = 0;

        let result = libc::ioctl(file.as_raw_fd(), FS_IOC_FIEMAP, header);

        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        let count = (*header).mapped_extents as usize;

        if count > EXTENTS_PER_REQUEST {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Kernel returned an invalid extent count",
            ));
        }

        let extent_ptr = buffer
            .as_ptr()
            .cast::<u8>()
            .add(header_size)
            .cast::<FiemapExtent>();

        let mut extents = Vec::with_capacity(count);

        for i in 0..count {
            extents.push(*extent_ptr.add(i));
        }

        let is_last = extents
            .last()
            .is_some_and(|e| e.flags & FIEMAP_EXTENT_LAST != 0);

        Ok((extents, is_last))
    }
}

pub fn manifest_create(path: &Path) -> Result<Manifest, io::Error> {
    let md = match fs::metadata(path) {
        Ok(md) => md,
        Err(e) => return Err(e),
    };
    let mut fiemap = Manifest::default();
    fiemap.name = path.file_name().unwrap().to_string_lossy().to_string();
    fiemap.size = md.size();
    fiemap.blocks_count = md.blocks();

    println!("blocks count: {}", fiemap.blocks_count);

    Ok(fiemap)
}
