use crate::Hdr;
use crate::ImageHandler;
use crate::Status;

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

pub type SetTime = extern"efiapi" fn( time: *mut Time) -> Status;

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
pub type GetVariable = extern "efiapi" fn() -> Status;
pub type GetNextVariableName = extern "efiapi" fn() -> Status;
pub type SetVariable = extern "efiapi" fn() -> Status;

// ** MISCELLANEOUS SERVICES **
pub type GetNextHighMonoCount = extern "efiapi" fn() -> Status;
pub type ResetSystem = extern "efiapi" fn() -> Status;


// ** UEFI 2.0 CAPSULE SERVICES **
pub type UpdateCapsule = extern "efiapi" fn() -> Status;
pub type QueryVariableInfo = extern "efiapi" fn() -> Status;
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
    get_next_high_monotonic_count: GetNextHighMonoCount,
    reset_system: ResetSystem,

    // UEFI 2.0 Capsule Services
    update_capsule: UpdateCapsule,
    query_variable_info: QueryVariableInfo,
}
