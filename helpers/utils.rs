use winapi::ctypes::*;
use winapi::shared::sddl::*;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::memoryapi::*;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winbase::{GlobalAlloc, GlobalFree, GlobalLock, GMEM_MOVEABLE, LocalFree, LookupAccountSidA, LookupPrivilegeNameA};

use std::arch::asm;
use winapi::um::winnt::{HANDLE, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, PLUID, PSID};
use winapi::um::winuser::*;

pub fn mylasterror(){

}

pub fn parsememory<T>(prochandle: *mut c_void, source: *const c_void) -> Result<T, i32> {
    let mut a: T = unsafe { core::mem::zeroed::<T>() };
    let mut bytesread = 0;
    let res = unsafe { ReadProcessMemory(prochandle, source, &mut a as *mut _ as *mut c_void, core::mem::size_of::<T>(), &mut bytesread) };
    if res == 0 {
        return Err(res);
    }
    Ok(a)
}


pub struct sidtostring{
    sidpointer: *mut c_void,
    unicodesid: *mut c_void
}

impl sidtostring{
    pub fn fromsidpointer(sidpointer:*mut c_void) -> Result<String,String>{
        let mut bufferpointer = 0 as *mut u16;
        let res = unsafe{ConvertSidToStringSidW(sidpointer,&mut bufferpointer)};
        if res==0{
            return Err(format!("ConvertSidToStringSidW error: {}",unsafe{GetLastError()}));
        }
        //self.sidpointer = sidpointer;
        //self.unicodesid = bufferpointer as *mut c_void;
        let sidstring = readunicodestringfrommemory(unsafe{GetCurrentProcess()},bufferpointer as *mut c_void);
        unsafe{
            LocalFree(bufferpointer as *mut c_void);
        };
        Ok(sidstring)
    }
}

impl Drop for sidtostring{
    fn drop(&mut self) {
        if self.unicodesid!=0 as *mut c_void{
            unsafe{
                LocalFree(self.unicodesid)
            };
        }
    }
}





pub fn readunicodestringfrommemory(prochandle: *mut c_void, base: *const c_void) -> String {
    unsafe {
        let mut buffer: Vec<u16> = Vec::new();
        let mut i = 0;

        loop {
            let mut bytesread = 0;
            let mut temp: Vec<u16> = vec![0; 2];
            ReadProcessMemory(
                prochandle,
                (base as usize + (i * 2)) as *const c_void,
                temp.as_mut_ptr() as *mut c_void,
                2,
                &mut bytesread,
            );

            i += 1;
            if temp[0] == 0 && temp[1] == 0 {
                break;
            }

            buffer.push(temp[0]);
            buffer.push(temp[1]);
        }

        return String::from_utf16_lossy(&buffer).trim().to_string();
    }
}


pub fn readunicodestringfrommemory2(prochandle: *mut c_void, base: *const c_void) -> String {
    unsafe {
        let mut buffer: Vec<u8> = Vec::new();
        let mut i = 0;

        loop {
            let mut bytesread = 0;
            let mut temp:u16 = 0;
            ReadProcessMemory(
                prochandle,
                (base as usize + (i * 2)) as *const c_void,
                &mut temp as *mut _ as *mut c_void,
                2,
                &mut bytesread,
            );

            i += 1;
            if temp&0xFF ==0 {
                break;
            }
            else{
                buffer.push((temp&0xff)as u8);
            }


        }

        return String::from_utf8_lossy(&buffer).to_string();
    }
}





pub fn ReadStringFromMemory(prochandle: *mut c_void, base: *const c_void) -> String {
    unsafe {
        let mut i: isize = 0;
        let mut s = String::new();
        loop {
            let mut a: [u8; 1] = [0];
            ReadProcessMemory(
                prochandle,
                (base as isize + i) as *const c_void,
                a.as_mut_ptr() as *mut c_void,
                1,
                std::ptr::null_mut(),
            );

            if a[0] == 0 || i == 100 {
                return s;
            }
            s.push(a[0] as char);
            i += 1;
        }
    }
}

pub fn sidtousername(psid:PSID) -> Result<String,String>{

    let mut username:Vec<u8> = vec![0;1024];
    let mut domainname:Vec<u8> = vec![0;1024];
    let mut userlength = username.len() as u32;
    let mut domainlength = domainname.len() as u32;
    let mut sidtype = 0;
    let res = unsafe{LookupAccountSidA(std::ptr::null_mut(),psid,username.as_mut_ptr() as *mut i8,
    &mut userlength, domainname.as_mut_ptr() as *mut i8, &mut domainlength,
    &mut sidtype)};
    if res!=0{
        let fullname = format!("{}\\{}", String::from_utf8_lossy(&domainname)
            .trim_end_matches("\0"),String::from_utf8_lossy(&username));
        return Ok(fullname);
    }
    return Err(format!("LookupAccountSidA error: {}",unsafe{GetLastError()}));
}

pub fn luidtoprivilege(pluid:PLUID) -> Result<String,String>{

    let mut namebuffer:Vec<u8> = vec![0;1024];
    let mut namelength = namebuffer.len() as u32;
    let res = unsafe{LookupPrivilegeNameA(std::ptr::null_mut(),pluid,
    namebuffer.as_mut_ptr() as *mut i8, &mut namelength)};
    if res!=0{
        return Ok(format!("{}",String::from_utf8_lossy(&namebuffer)));
    }
    return Err(format!("LookupPrivilegeNameA failed: {}",unsafe{GetLastError()}));

}


pub fn getclipboardtext() -> Result<String,String>{

    let opened = unsafe{OpenClipboard(std::ptr::null_mut())};
    if opened==1{
        let clip = unsafe{GetClipboardData(CF_OEMTEXT)};
        if clip==0 as HANDLE{
            return Err(format!("GetClipboardData failed: {}",unsafe{GetLastError()} ));
        }
        let data = ReadStringFromMemory(unsafe{GetCurrentProcess()},clip as *const c_void);

        unsafe{CloseClipboard()};
        return Ok(data);
    }
    Err(format!("OpenClipboard failed: {}",unsafe{GetLastError()}))

}
























pub fn setclipboardtext(mut s:Vec<u8>) -> Result<(),String>{



    //let mut base = s.bytes().collect::<Vec<u8>>();
    let opened = unsafe{OpenClipboard(std::ptr::null_mut())};
    if opened==1{



        unsafe{EmptyClipboard()};
        let clip = unsafe{SetClipboardData(CF_TEXT,
                                          s.as_mut_ptr() as HANDLE)};
        if clip==0 as HANDLE{
            unsafe{CloseClipboard()};

            return Err(format!("SetClipboardData failed: {}",unsafe{GetLastError()} ));
        }

        unsafe{CloseClipboard()};

    }
    Err(format!("OpenClipboard failed: {}",unsafe{GetLastError()}))

}
