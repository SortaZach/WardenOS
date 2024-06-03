use crate::Hdr;
use crate::ImageHandle;
use crate::Status;



// ** TASK PRIORITY SERVICES **
pub type NewTpl = usize;
pub type RaiseTpl = extern "efiapi" fn(efi_tpl: NewTpl) -> NewTpl;
pub type RestoreTpl = extern "efiapi" fn(old_tpl: NewTpl) -> NewTpl;

// ** MEMORY SERVICES **
#[repr(u32)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[repr(u32)]
pub enum MemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    MaxMemoryType,
}

pub type PhysicalAddress = u64;

pub type AllocatePages = extern "efiapi" fn(
    r#type: AllocateType,
    memory_type: MemoryType,
    pages: usize,
    memory: &mut PhysicalAddress
) -> Status;

pub type FreePages = extern "efiapi" fn(
    memory: PhysicalAddress,
    pages: usize
) -> Status;

pub type VirtualAddress = u64;

pub struct MemoryDescriptor {
    pub r#type: u32,
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64,
}

pub type GetMemoryMap = extern "efiapi" fn(
    memory_map_size: &mut usize,
    memory_map: *mut MemoryDescriptor,
    map_key: &mut usize,
    descriptor_size: &mut usize,
    descriptor_version: &mut u32
) -> Status;

pub type VoidPtr = *mut core::ffi::c_void;

pub type AllocatePool = extern "efiapi" fn(
    pool_type: MemoryType,
    size: usize,
    buffer: *mut VoidPtr
) -> Status;

pub type FreePool = extern "efiapi" fn(buffer: VoidPtr) -> Status;

// ** EVENT & TIMER SERVICES **
pub type Event = VoidPtr;

pub type CreateEvent = extern "efiapi" fn(
    r#type: u32,
    notify_tpl: NewTpl,
    event: *mut Event
) -> Status;

#[repr(u32)]
pub enum TimerDelay {
    TimerCancel,
    TimerPeriodic,
    TimerRelative,
}

pub type SetTimer = extern "efiapi" fn(
    event: Event,
    r#type: TimerDelay,
    trigger_time: u64
) -> Status;

pub type WaitForEvent = extern "efiapi" fn(
    number_of_events: usize,
    event: *mut Event,
    index: *mut usize
) -> Status;

pub type SignalEvent = extern "efiapi" fn(event: Event) -> Status;
pub type CloseEvent = extern "efiapi" fn(event: Event) -> Status;
pub type CheckEvent = extern "efiapi" fn(event: Event) -> Status;

// ** PROTOCOL HANDLER SERVICES **
#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

#[repr(u32)]
pub enum InterfaceType {
    NativeInterface,
}

pub type DevicePathProtocol = extern "efiapi" fn(
    r#type: u8,
    sub_type: u8,
    length: u8
);

pub type InstallProtocolInterface = extern "efiapi" fn(
    handle: *mut ImageHandle,
    protocol: *mut GUID,
    interface_type: InterfaceType,
    interface: VoidPtr
) -> Status;

pub type ReinstallProtocolInterface = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    old_interface: VoidPtr,
    new_interface: VoidPtr
) -> Status;

pub type UninstallProtocolInterface = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    interface: VoidPtr
) -> Status;

pub type HandleProtocol = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    interface: *mut VoidPtr
) -> Status;

pub type RegisterProtocolNotify = extern "efiapi" fn(
    protocol: *mut GUID,
    event: Event,
    registration: *mut VoidPtr
) -> Status;

#[repr(u32)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

pub type LocateHandle = extern "efiapi" fn(
    search_type: LocateSearchType,
    buffer_size: *mut usize,
    buffer: *mut ImageHandle
) -> Status;

pub type LocateDevicePath = extern "efiapi" fn(
    protocol: *mut GUID,
    device_path: *mut DevicePathProtocol,
    device: *mut ImageHandle
) -> Status;

pub type InstallConfigurationTable = extern "efiapi" fn(
    guid: *mut GUID,
    table: VoidPtr
) -> Status;

// ** IMAGE SERVICES **
pub type LoadImage = extern "efiapi" fn(
    boot_policy: bool,
    parent_image_handle: ImageHandle,
    device_path: *mut *mut DevicePathProtocol,
    source_size: usize,
    image_handle: *mut ImageHandle
) -> Status;

pub type StartImage = extern "efiapi" fn(
    image_handle: ImageHandle,
    exit_data_size: *mut usize
) -> Status;

pub type Exit = extern "efiapi" fn(
    image_handle: ImageHandle,
    exit_status: Status,
    exit_data_size: usize
) -> Status;

pub type UnloadImage = extern "efiapi" fn(image_handle: ImageHandle) -> Status;
pub type ExitBootServices = extern "efiapi" fn(
    image_handle: ImageHandle,
    map_key: usize
) -> Status;

// ** MISCELLANEOUS SERVICES **
pub type GetNextMonotonicCount = extern "efiapi" fn(count: *mut u64) -> Status;
pub type Stall = extern "efiapi" fn(microseconds: usize) -> Status;
pub type SetWatchdogTimer = extern "efiapi" fn(
    timeout: usize,
    watchdog_code: u64,
    data_size: usize
) -> Status;

// ** DRIVER SUPPORT **
pub type ConnectController = extern "efiapi" fn(
    controller_handle: ImageHandle,
    recursive: bool
) -> Status;

pub type DisconnectController = extern "efiapi" fn(
    controller_handle: ImageHandle
) -> Status;

// ** OPEN AND CLOSE PROTOCOL SERVICES **
pub type OpenProtocol = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    agent_handle: ImageHandle,
    controller_handle: ImageHandle,
    attributes: u32
) -> Status;

pub type CloseProtocol = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    agent_handle: ImageHandle,
    controller_handle: ImageHandle
) -> Status;

pub struct OpenProtocolInformationEntry {
    pub agent_handle: ImageHandle,
    pub controller_handle: ImageHandle,
    pub attributes: u32,
    pub open_count: u32,
}

pub type OpenProtocolInformation = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    entry_buffer: *mut *mut OpenProtocolInformationEntry,
    entry_count: *mut usize
) -> Status;

// ** LIBRARY SERVICES **
pub type ProtocolsPerHandle = extern "efiapi" fn(
    handle: ImageHandle,
    protocol_buffer: *mut *mut *mut GUID,
    protocol_buffer_count: *mut usize
) -> Status;

pub type LocateHandleBuffer = extern "efiapi" fn(
    search_type: LocateSearchType,
    no_handle: *mut usize,
    buffer: *mut *mut ImageHandle
) -> Status;

pub type LocateProtocol = extern "efiapi" fn(
    protocol: *mut GUID,
    interface: *mut VoidPtr
) -> Status;

pub type InstallMultipleProtocolInterfaces = extern "efiapi" fn(
    handle: *mut ImageHandle
) -> Status;

pub type UninstallMultipleProtocolInterfaces = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    interface: VoidPtr
) -> Status;

// ** 32-BIT CRC SERVICES **
pub type CalculateCrc32 = extern "efiapi" fn(
    data: VoidPtr,
    data_size: usize,
    crc32: *mut u32
) -> Status;

// ** MISCELLANEOUS SERVICES **
pub type CopyMem = extern "efiapi" fn(
    destination: VoidPtr,
    source: VoidPtr,
    length: usize
) -> Status;

pub type SetMem = extern "efiapi" fn(
    buffer: VoidPtr,
    size: usize,
    value: u8
) -> Status;

pub type CreateEventEx = extern "efiapi" fn(
    r#type: u32,
    notify_tpl: NewTpl,
    event: *mut Event
) -> Status;

pub struct BootServices {
    pub header: Hdr,

    // Task Priority Services
    pub raise_tpl: RaiseTpl,
    pub restore_tpl: RestoreTpl,

    // Memory Services
    pub allocate_pages: AllocatePages,
    pub free_pages: FreePages,
    pub get_memory_map: GetMemoryMap,
    pub allocate_pool: AllocatePool,
    pub free_pool: FreePool,

    // Event & Timer Services
    pub create_event: CreateEvent,
    pub set_timer: SetTimer,
    pub wait_for_event: WaitForEvent,
    pub signal_event: SignalEvent,
    pub close_event: CloseEvent,
    pub check_event: CheckEvent,

    // Protocol Handler Services
    pub install_protocol_interface: InstallProtocolInterface,
    pub reinstall_protocol_interface: ReinstallProtocolInterface,
    pub uninstall_protocol_interface: UninstallProtocolInterface,
    pub handle_protocol: HandleProtocol,
    pub reserved: u32,
    pub register_protocol_notify: RegisterProtocolNotify,
    pub locate_handle: LocateHandle,
    pub locate_device_path: LocateDevicePath,
    pub install_configuration_table: InstallConfigurationTable,

    // Image Services
    pub load_image: LoadImage,
    pub start_image: StartImage,
    pub exit: Exit,
    pub unload_image: UnloadImage,
    pub exit_boot_services: ExitBootServices,

    // Miscellaneous Services
    pub get_next_monotonic_count: GetNextMonotonicCount,
    pub stall: Stall,
    pub set_watchdog_timer: SetWatchdogTimer,

    // Driver Support
    pub connect_controller: ConnectController,
    pub disconnect_controller: DisconnectController,

    // Open and Close Protocol Services
    pub open_protocol: OpenProtocol,
    pub close_protocol: CloseProtocol,
    pub open_protocol_information: OpenProtocolInformation,

    // Library Services
    pub protocols_per_handle: ProtocolsPerHandle,
    pub locate_handle_buffer: LocateHandleBuffer,
    pub locate_protocol: LocateProtocol,
    pub install_multiple_protocol_interfaces: InstallMultipleProtocolInterfaces,
    pub uninstall_multiple_protocol_interfaces: UninstallMultipleProtocolInterfaces,

    // 32-bit CRC Services
    pub calculate_crc32: CalculateCrc32,

    // Miscellaneous Services
    pub copy_mem: CopyMem,
    pub set_mem: SetMem,
    pub create_event_ex: CreateEventEx,
}

