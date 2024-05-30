pub type ImageHandle = u64;
pub type Status = usize;

#[repr(C)]
pub struct SystemTable{
    header: Hdr, 
    firmware_vendor: u64,
    firmware_revision:u32,
    input_handle: ImageHandle,
    input: u64,
    output_handle: ImageHandle,
    pub output: *const TextOutputProtocol,
    error_handle: ImageHandle,
    error:  u64,
    runtime: u64,
    boot: u64,
    no_of_entries: usize,
    config_table: u64,
}

pub struct Hdr {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

//u64 are pointers to functions that will not be used
#[repr(C)]
pub struct TextOutputProtocol{
    reset: u64,
    pub output_string: OutputString,
    test_output: u64,
    query_mode: u64,
    set_mode: u64,
    set_attribute: u64,
    clear_screen:  u64,
    set_cursor_position: u64,
    enable_cursor: u64,
    mode: u64,
}

type OutputString = extern "efiapi" fn(
 output_protocol:*const TextOutputProtocol,
 string :*const u16
 )-> Status;


// ** TASK PRIORITY SERVICES **
pub type NewTpl = usize;
pub type RaiseTpl = extern "efiapi" fn(efi_tpl: NewTpl) -> NewTpl;
pub type RestoreTpl = extern "efiapi" fn(old_tpl: NewTpl);


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
    memory: &mut PhysicalAddress, 
) -> Status;

pub type FreePages = extern "efiapi" fn(
    memory: *mut PhysicalAddress,
    pages: usize, ) -> Status;

pub type VirtualAddress = u64;

pub struct MemoryDescriptor {
    r#type: u32, 
    physical_start: &mut PhysicalAddress,
    virtual_start: &mut VirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

pub type GetMemoryMap = extern "efiapi" fn(
    memory_map_size: &mut usize,
    memory_map: MemoryDescriptor,
    map_key: &mut usize,
    descriptor_size: &mut usize,
    descriptor_version: &mut u32,
) -> Status;

pub type VoidPtr = *mut core::ffi::c_void;

pub type AllocatePool = extern "efiapi" fn(
    pool_type: &mut MemoryType,
    size: usize,
    buffer: *mut VoidPtr, 
) -> Status;

pub type FreePool = extern "efiapi" fn( buffer: VoidPtr) -> Status;


// ** EVENT & TIMER SERVICES
pub type Event = VoidPtr;

pub type CreateEvent = extern "efiapi" fn( 
    r#type: u32,
    notify_tpl: NewTpl,
    event: Event,
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
    trigger_time: u64,
) -> Status;

pub type WaitForEvent = extern "efiapi" fn(
    number_of_events: usize,
    event: *mut Event,
    index: *mut usize,
) -> Status;

pub type SignalEvent = extern "efiapi" fn( event: Event ) -> Status;
pub type CloseEvent = extern "efiapi" fn( event: Event ) -> Status;
pub type CheckEvent = extern "efiapi" fn( event: Event ) -> Status;


// ** PROTOCOL HANDLER SERVICES **
pub struct GUID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: u8,
}

#[repr(u32)]
pub enum InterfaceType {
    NativeInterface,
}

pub type InstallProtocolInterface = extern "efiapi" fn(
    handle: *mut ImageHandle,
    protocol: *mut GUID,
    interface_type: InterfaceType,
    interface: VoidPtr,
) -> Status;

pub type ReinstallProtocolInterface = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    old_interface: VoidPtr,
    new_interface: VoidPtr,
) -> Status;

pub type UninstallProtocolInterface = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    interface: VoidPtr,
) -> Status;

pub type HandleProtocol = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    interface: *mut VoidPtr,
) -> Status;

pub type RegisterProtocolNotify = extern "efiapi" fn(
    protocol: *mut GUID,
    event: Event,
    registration: *mut VoidPtr,
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
    buffer: ImageHandle,
) -> Status;

pub type LocateDevicePath = extern "efiapi" fn(
    protocol: *mut GUID,
    device_path: *mut DevicePathProtocol,
    device: *mut ImageHandle,
) -> Status;

pub type InstallConfigurationTable = extern "efiapi" fn(
    guid: *mut GUID,
    table: VoidPtr,
) -> Status;


//** IMAGE SERVICES **
pub type LoadImage = extern "efiapi" fn(
    boot_policy: bool,
    parent_image_handle: ImageHandle,
    device_path: *mut *mut DevicePathProtocol,
    source_size: usize,
    image_handle: *mut ImageHandle, 
) -> Status;

pub type StartImage = extern "efiapi" fn(
    image_handle: ImageHandle,
    exit_data_size: *mut usize,
) -> Status;

pub type Exit = extern "efiapi" fn(
    image_handle: ImageHandle,
    exit_status: Status,
    exit_data_size: usize,
) -> Status;

pub type UnloadImage = extern "efiapi" fn( image_handle: ImageHandle ) -> Status;
pub type ExitBootServices = extern "efiapi" fn(
    image_handle: ImageHandle,
    map_key: usize,
);


// ** MISCELLANEOUS SERVICES **
pub type GetNextMonotonicCount = extern "efiapi" fn( count: *mut u64) -> Status;
pub type Stall = extern "efiapi" fn( microseconds: usize) -> Status;
pub type SetWatchdogTimer = extern "efiapi" fn(
    timeout: usize,
    watchdog_code: u64,
    data_size: usize,
) -> Status;


// ** DRIVER SUPPORT **
pub type ConnectController = extern "efiapi" fn(
    controller_handle: ImageHandle,
    recursive: bool,
) -> Status;
pub type DisconnectController = extern "efiapi" fn(
    controller_handle: ImageHandle,
) -> Status;


// ** OPEN AND CLOSE PROTOCOL SERVICES **
pub type OpenProtocol = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    agent_handle: ImageHandle,
    controller_handle: ImageHandle,
    attributes: u32,
) -> Status;

pub type CloseProtocol = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    agent_handle: ImageHandle,
    controller_handle: ImageHandle,
) -> Status;

pub struct OpenProtocolInformationEntry {
    agent_handle: ImageHandle,
    controller_handle: ImageHandle,
    attributes: u32,
    open_count: u32,
}

pub type OpenProtocolInformation = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    entry_buffer: *mut *mut OpenProtocolInformationEntry,
    entry_count: *mut usize,
) -> Status;


//** LIBRARY SERVICES ** 
pub type ProtocolsPerHandle = extern "efiapi" fn(
    handle: ImageHandle,
    protocol_buffer: *mut *mut *mut GUID,
    protocol_buffer_count: *mut usize,
) -> Status;


pub type LocateHandleBuffer = extern "efiapi" fn(
    search_type: LocateSearchType,
    no_handle: *mut usize,
    buffer: *mut *mut ImageHandle,
) -> Status;

pub type LocateProtocol = extern "efiapi" fn(
    protocol: *mut GUID,
    interface: *mut VoidPtr,
) -> Status;

pub type InstallMultipleProtocolInterfaces = extern "efiapi" fn( handle: mut* ImageHandle ) -> Status;
pub type UninstallMultipleProtocolInterfaces = extern "efiapi" fn(
    handle: ImageHandle,
    protocol: *mut GUID,
    interface: VoidPtr,
) -> Status;


//** 32-BIT CRC SERVICES **
pub type CalculateCrc32 = extern "efiapi" fn(
    data: VoidPtr,
    data_size: usize,
    crc32: *mut u32,
) -> Status;


//** MISCELLANEOUS SERVICES **
pub type CopyMem = extern "efiapi" fn(
    destination: VoidPtr,
    source: VoidPtr,
    length: usize,
) -> Status;

pub type SetMem = extern "efiapi" fn(
    buffer: VoidPtr,
    size: usize,
    value: u8,
) -> Status;

pub type CreateEventEx = extern "efiapi" fn(
    type: u32,
    notify_tpl: NewTpl,
    event: Event,
) -> Status;

pub struct BootServices{
    header: Hdr,

    // Task Priority Services
    raise_tpl: RaiseTpl,
    restore_tpl: RestoreTpl,
    
    // Memory Services
    allocate_pages: AllocatePages,
    free_pages: FreePages,
    get_memory_map: GetMemoryMap,
    allocate_pool: AllocatePool,
    free_pool: FreePool,

    // Event & Timer Services
    create_event: CreateEvent,
    set_timer: SetTimer,
    wait_for_event: WaitForEvent,
    signal_event: SignalEvent,
    close_event: CloseEvent,
    check_event: CheckEvent,

    // Protocol Handler Services
    install_protocol_interface: InstallProtocolInterface,
    reinstall_protocol_interface: ReinstallProtocolInterface,
    uninstall_protocol_interface: UninstallProtocolInterface,
    handle_protocol: HandleProtocol,
    reserved: u32,
    register_protocol_notify: RegisterProtocolNotify,
    locate_handle: LocateHandle,
    locate_device_path: LocateDevicePath,
    install_configuration_table: InstallConfigurationTable,

    // Image Service
    load_image: LoadImage,
    start_image: StartImage,
    exit: Exit,
    unload_image: UnloadImage,
    exit_boot_service: ExitBootService,

    // Miscellaneous Services
    get_next_monotonic_count: GetNextMonotonicCount,
    stall: Stall,
    set_watchdog_timer: SetWatchdogTimer,

    // Driver Support
    connect_controller: ConnectController,
    disconnect_controller: DisconnectController,

    // Open and Close Protocol Services
    open_protocol: OpenProtocol,
    close_protocol: CloseProtocol,
    open_protocol_information: OpenProtocolInfomration,
    
    // Library Services
    protocols_per_handle: ProtocolsPerHandle,
    locate_handle_buffer: LocateHandleBuffer,
    locate_protocol: LocateProtocol,
    install_multiple_protocol_interfaces: InstallMultipleProtocolInterfaces,
    uninstall_multiple_protocol_interfaces: UninstallMultipleProtocolInterfaces,

    // 32-bit CRC Services
    calculate_crc32: CalculateCrc32,

    // Miscellaneous Services
    copy_mem: CopyMem,
    set_mem: SetMem,
    create_event_ex: CreateEventEx,
        
}

