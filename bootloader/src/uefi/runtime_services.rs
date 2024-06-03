use crate::Hdr;
use crate::ImageHandle;
use crate::Status;
use crate::MemoryDescriptor;
use crate::VoidPtr;
use crate::GUID;
use crate::PhysicalAddress;


// ** TIME SERVICES **

pub struct Time {
    year: u16, // 1900 - 9999
    month: u8, // 1 - 12
    day: u8, // 1 - 31
    hour: u8, // 0 - 23
    minute: u8, // 0 - 59
    second: u8, // 0 - 59
    pad1: u8,
    nanosecond: u32, // 0 - 999,999,999
    timezone: i16, // -1440 to 1400 or 2047
    daylight: u8,
    pad2: u8,
}

pub struct TimeCapabilities {
    resolution: u32,
    accuracy: u32,
    set_to_zero: bool,
}

pub type GetTime = extern "efiapi" fn(
    time: *mut Time,
    capabilities: *mut TimeCapabilities,
) -> Status; 

pub type SetTime = extern "efiapi" fn( time: *mut Time) -> Status;

pub type GetWakeupTime = extern "efiapi" fn(
    enabled: *mut bool,
    pending: *mut bool,
    time: *mut Time,
) -> Status;

pub type SetWakeupTime = extern "efiapi" fn(
    enable: bool,
    time: *mut Time,
) -> Status;

// ** VIRTUAL MEMORY SERVICES **
pub type SetVirtualAddressMap = extern "efiapi" fn(
    memory_map_size: usize,
    descriptor_size: usize,
    descriptor_version: u32,
    virtual_map: *mut MemoryDescriptor,
) -> Status;

pub type ConvertPointer = extern "efiapi" fn(
    debug_disposition: usize,
    address: *mut VoidPtr,
) -> Status;


// ** VARIABLE SERVICES **
pub type GetVariable = extern "efiapi" fn(
    variable_name: *mut u16,
    vendor_guid: GUID,
    attributes: *mut u32,
    data_sizes: *mut usize,
    data: VoidPtr,
) -> Status;

pub type GetNextVariableName = extern "efiapi" fn(
    variable_name_size: *mut usize,
    variable_name: *mut u16,
    vendor_guid: *mut GUID,
) -> Status;

pub type SetVariable = extern "efiapi" fn(
    variable_name: *mut u16,
    vendor_guid: *mut GUID,
    attributes: u32,
    data_size: usize,
    data: VoidPtr,
) -> Status;

// ** MISCELLANEOUS SERVICES **
pub type GetNextHighMonotonicCount = extern "efiapi" fn(
    high_count: u32,
) -> Status;

#[repr(u32)]
pub enum ResetType {
    ResetCold,
    ResetWarm,
    ResetShutdown,
    ResetPlatformSpecific,
}

pub type ResetSystem = extern "efiapi" fn(
    reset_type: ResetType,
    reset_status: Status,
    data_size: usize,
    reset_data: VoidPtr,
);


// ** UEFI 2.0 CAPSULE SERVICES **
pub struct CapsuleHeader {
    capsule_guid: GUID,
    header_size: u32,
    flags: u32,
    capsule_image_size: u32,
}

pub type UpdateCapsule = extern "efiapi" fn(
    capsule_header_array: *mut *mut CapsuleHeader,
    capsule_count: usize,
    scatter_gather_list: PhysicalAddress,
) -> Status;

pub type QueryVariableInfo = extern "efiapi" fn(
    attrabutes: u32,
    maximum_variable_storage_size: *mut u64,
    remaining_variable_storage_size: *mut u64,
    maximum_variable_size: *mut u64,
) -> Status;

pub struct RuntimeService {
    pub header: Hdr,

    // Time Services 
    pub get_time: GetTime,
    pub set_time: SetTime,
    pub get_wakeup_time: GetWakeupTime,
    pub set_wakeup_time: SetWakeupTime,

    // Virtual Memory Services
    pub set_virtual_address_map: SetVirtualAddressMap,
    pub convert_pointer: ConvertPointer,

    // Variable Services
    get_variable: GetVariable,
    get_next_variable_name: GetNextVariableName,
    set_variable: SetVariable,

    // Miscellaneous Services
    get_next_high_monotonic_count: GetNextHighMonotonicCount,
    reset_system: ResetSystem,

    // UEFI 2.0 Capsule Services
    update_capsule: UpdateCapsule,
    query_variable_info: QueryVariableInfo,
}
