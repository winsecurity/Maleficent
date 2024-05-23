
use winapi::shared::lmcons::*;
use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::*;
use winapi::um::fileapi::*;
use winapi::um::handleapi::*;
use winapi::um::minwinbase::*;
use winapi::um::processthreadsapi::*;
use crate::helpers::utils::readunicodestringfrommemory2;
use winapi::ctypes::*;
use winapi::um::winnt::*;

pub fn createfile(filename:&str){

    let filehandle = unsafe{CreateFileA(filename.as_bytes().as_ptr() as *const i8,
    GENERIC_ALL,FILE_SHARE_READ,std::ptr::null_mut(),1,
    FILE_ATTRIBUTE_NORMAL,std::ptr::null_mut())};
    if filehandle==INVALID_HANDLE_VALUE{
        println!("CreateFileA failed: {}",unsafe{GetLastError()});
        return ();
    }
    println!("file created successfully");
    unsafe{CloseHandle(filehandle)};

}


pub fn writefile(filename:&str,contents:&str){
    let filehandle = unsafe{CreateFileA(filename.as_bytes().as_ptr() as *const i8,
                                        GENERIC_READ|GENERIC_WRITE,FILE_SHARE_READ,std::ptr::null_mut(),OPEN_ALWAYS,
                                        FILE_ATTRIBUTE_NORMAL,std::ptr::null_mut())};
    if filehandle==INVALID_HANDLE_VALUE{
        println!("CreateFileA opening file failed: {}",unsafe{GetLastError()});
        return ();
    }
    let mut byteswritten = 0;
    let res = unsafe{WriteFile(filehandle,contents.as_bytes().as_ptr() as *const c_void,
    contents.len() as u32,&mut byteswritten, std::ptr::null_mut())};
    if res==0{
        println!("WriteFile failed: {}",unsafe{GetLastError()});
        unsafe{CloseHandle(filehandle)};
        return();
    }
    println!("{} bytes written",byteswritten);
    unsafe{CloseHandle(filehandle)};

}



pub fn readfile(filename:&str){
    let filehandle = unsafe{CreateFileA(filename.as_bytes().as_ptr() as *const i8,
                                        GENERIC_READ|GENERIC_WRITE,FILE_SHARE_READ,std::ptr::null_mut(),OPEN_ALWAYS,
                                        FILE_ATTRIBUTE_NORMAL,std::ptr::null_mut())};
    if filehandle==INVALID_HANDLE_VALUE{
        println!("CreateFileA opening file failed: {}",unsafe{GetLastError()});
        return ();
    }
    let mut buffer:Vec<u8> = vec![0;1024];
    let mut bytesread = 0;
    let res = unsafe{ReadFile(filehandle,buffer.as_mut_ptr() as *mut c_void,
                               buffer.len() as u32,&mut bytesread, std::ptr::null_mut())};
    if res==0{
        println!("ReadFile failed: {}",unsafe{GetLastError()});
        unsafe{CloseHandle(filehandle)};
        return();
    }
    let filecontents = String::from_utf8_lossy(&buffer).trim_end_matches("\0").to_string();
    println!("filecontents: {}",filecontents);

}
