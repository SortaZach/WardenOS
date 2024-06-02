pub mod header;
pub mod boot_services;
pub mod runtime_services;
use header::*;
use boot_services::*;
use runtime_services::*;

pub type ImageHandle = u64;
pub type Status = usize;

pub struct Hdr {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
pub struct SystemTable {
    header: Hdr,
    firmware_vendor: u64,
    firmware_revision: u32,
    input_handle: ImageHandle,
    input: u64,
    output_handle: ImageHandle,
    pub output: *const TextOutputProtocol,
    error_handle: ImageHandle,
    error: u64,
    runtime: u64,
    boot: u64,
    no_of_entries: usize,
    config_table: u64,
}

#[repr(C)]
pub struct TextOutputProtocol {
    reset: u64,
    pub output_string: OutputString,
    test_output: u64,
    query_mode: u64,
    set_mode: u64,
    set_attribute: u64,
    clear_screen: u64,
    set_cursor_position: u64,
    enable_cursor: u64,
    mode: u64,
}

type OutputString = extern "efiapi" fn(
    output_protocol: *const TextOutputProtocol,
    string: *const u16
) -> Status;


