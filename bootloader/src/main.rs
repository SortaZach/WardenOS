#![no_std]
#![no_main]
use core::panic::PanicInfo;

pub mod uefi;
use uefi::*;

use crate::boot_services::*;

#[no_mangle]
pub extern "efiapi" fn efi_main(
       _handle: ImageHandle, 
        system_table:*const SystemTable
){
 
 let string = "hello\n\r";
 for character in string.chars(){
  
  let mut buffer:[u16;1] = [0];
  let utf16 = character.encode_utf16(&mut buffer);
  unsafe{
   let _status = ((*(*system_table).output).output_string)(
    (*system_table).output, 
    &utf16[0],
  );
 }
 }
 
 let string_arr = ['h' as u16, 'm' as u16, '!' as u16, '\n' as u16, '\0' as u16];
 
 unsafe{
  let _status = ((*(*system_table).output).output_string)(
   (*system_table).output, 
   &string_arr[0],
  );
 }
 loop{}
}
#[panic_handler]
fn panic(_info: &PanicInfo) ->!{
 loop{}
}


