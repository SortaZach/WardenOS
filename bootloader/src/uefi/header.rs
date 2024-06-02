pub type ImageHandle = u64;
pub type Status = usize;

// Header
pub struct Hdr {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

