use winapi::ctypes::*;
use winapi::um::errhandlingapi::*;
use winapi::um::securitybaseapi::*;
use winapi::shared::ntstatus::*;
use winapi::shared::sddl::ConvertSidToStringSidA;
use winapi::shared::winerror::*;
use winapi::um::processthreadsapi::*;
use winapi::um::winbase::LocalFree;
use winapi::um::winnt::*;

use super::super::helpers::utils::*;
use crate::mydatatypes::mywinapi::*;

pub fn getmytokeninfo(tokenhandle: *mut c_void,tokeninfoclass: u32) -> Vec<u8>{

    let mut bytesneeded: u32 = 0;
    let buffer = loop{
        let mut buffer:Vec<u8> = vec![0;bytesneeded as usize];
        let res = unsafe{GetTokenInformation(tokenhandle,tokeninfoclass,buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,&mut bytesneeded)};
        if res!=0{
            break buffer;
        }

    };
    return buffer;

}



pub fn sidtostring2(psid:PSID) -> Result<String,String>{
    let mut spointer = 0 as *mut i8;
    let res = unsafe{ConvertSidToStringSidA(psid, &mut spointer)};
    if res==0{
        return Err(format!("convertsidtostringsida failed:{}",unsafe{GetLastError()}));
    }
    let sidstring = ReadStringFromMemory(unsafe{GetCurrentProcess()},spointer as *const c_void);
    unsafe{LocalFree(spointer as *mut c_void)};
    Ok(sidstring)
}



