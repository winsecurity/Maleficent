
use winapi::ctypes::*;
use winapi::um::errhandlingapi::*;
use winapi::um::handleapi::*;
use winapi::um::memoryapi::*;
use winapi::um::processthreadsapi::*;
use winapi::um::winnt::*;


use std::process::Command;

pub fn executeexe(exepath:&str) -> Result<String,String>{
    let output = Command::new(exepath).
        output().unwrap();


    if output.stdout.len()>0{

        return Ok(String::from_utf8_lossy(&output.stdout)
            .to_string().trim_end_matches("\n").to_string()
            .trim_end_matches("\r\n").to_string()
            .trim_end_matches("\0").to_string());
    }
    else{
        return Err(String::from_utf8_lossy(&output.stderr)
            .to_string().trim_end_matches("\n").to_string()
            .trim_end_matches("\r\n").to_string()
            .trim_end_matches("\0").to_string());
    }



}







#[derive(Debug)]
pub struct selfhandle{
    pub handle: *mut c_void
}
impl selfhandle{
    pub fn new()->Result<Self,String>{
        let prochandle = unsafe{GetCurrentProcess()};
        if prochandle.is_null(){
            return Err(format!("Getting handle to current process error: {}",unsafe{GetLastError()}));
        }
        else{
            Ok(Self{handle:prochandle})
        }

    }

}
impl Drop for selfhandle{
    fn drop(&mut self){
        if !self.handle.is_null(){

            unsafe{CloseHandle(self.handle)};
        }
    }
}



#[derive(Debug)]
pub struct remotehandle{
    pid: u32,
    access: u32,
    pub handle: *mut c_void
}
impl remotehandle{
    pub fn open(pid: u32, access: u32) -> Result<Self, String>{
        let prochandle = unsafe{OpenProcess(access, 0 , pid)};
        if prochandle.is_null(){
            return Err(format!("Opening handle to process: {} failed with error: {}",pid,unsafe{GetLastError()}));
        }
        return Ok(Self{pid,access,handle:prochandle});
    }
}
impl Drop for remotehandle{
    fn drop(&mut self){

        if !self.handle.is_null(){
            unsafe{CloseHandle(self.handle)};
        }
    }
}




pub struct processalloc<'a>{
    buffer: &'a Vec<u8>,
    handle: &'a *mut c_void,
    pub baseaddress: *mut c_void,
    alloctype: u32,
    protecttype: u32
}

impl<'a> processalloc<'a>{
    pub fn new(prochandle: &'a *mut c_void, buffer: &'a Vec<u8> ,alloctype: u32, protecttype: u32) -> Result<Self,String>{
        let baseaddress = unsafe{VirtualAllocEx(*prochandle,core::ptr::null_mut(),
        buffer.len(),alloctype,protecttype)};
        if baseaddress as usize ==0{
            return Err(format!("VirtualAllocEx failed: {}",unsafe{GetLastError()}));
        }
        let mut byteswritten = 0;
        let res = unsafe{WriteProcessMemory(*prochandle,baseaddress,(*buffer).as_ptr() as *const c_void,(*buffer).len(),&mut byteswritten)};
        if res==0{
            unsafe{VirtualFreeEx(*prochandle,baseaddress,0,MEM_RELEASE)};
            return Err(format!("Writing process memory failed: {}",unsafe{GetLastError()}));
        }
        return Ok(Self{buffer,handle:prochandle,baseaddress,alloctype,protecttype});
    }
}

impl<'a> Drop for processalloc<'a>{
    fn drop(&mut self){
        if !self.baseaddress.is_null() && *self.handle as usize!=0{
            unsafe{VirtualFreeEx(*self.handle, self.baseaddress,0,MEM_RELEASE)};
        }
    }
}

