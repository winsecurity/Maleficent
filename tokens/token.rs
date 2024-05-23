use winapi::ctypes::*;
use winapi::um::errhandlingapi::*;
use winapi::um::securitybaseapi::*;
use winapi::shared::ntstatus::*;
use winapi::shared::winerror::*;
use winapi::um::handleapi::*;
use winapi::um::processthreadsapi::*;
use winapi::um::winnt::TOKEN_USER;
use crate::helpers::utils::{sidtostring, sidtousername};
use winapi::um::winnt::*;
use std::collections::*;
//use crate::mydatatypes::mywinapi::*;


use super::enumeration::*;


#[link(name="Advapi32")]
extern "C"{
    pub fn LookupPrivilegeDisplayNameA(
        systemname:*const i8,
        namebuffer: *const i8,
        outputbuffer: *mut c_void,
        outputbuffersize: &mut u32,
        langid: &mut u32
    ) ->u32;
}


#[derive(Debug)]
pub struct mytoken {
    tokenhandle: *mut c_void,
    tokenaccess: u32
}


impl mytoken {
    pub fn new(prochandle: *mut c_void, tokenaccess: u32)-> Result<Self,String>{
        let mut tokhandle = 0 as *mut c_void;
        let res = unsafe{OpenProcessToken(prochandle,tokenaccess, &mut tokhandle)};
        if res==0{
            return Err(format!("openprocesstoken error: {}",unsafe{GetLastError()}));
        }
        Ok(Self{tokenhandle:tokhandle,tokenaccess})
    }


    pub fn gettokenuser(&self) -> Result<String,String>{
        let mut buffer = getmytokeninfo(self.tokenhandle,1);
        let tokenuser = unsafe{*(buffer.as_mut_ptr() as *mut TOKEN_USER)};
        let usersid = sidtostring::fromsidpointer(tokenuser.User.Sid);
        let username =  sidtousername(tokenuser.User.Sid);
        if usersid.is_ok(){
            if username.is_ok(){
                return Ok(format!("{}.{}",username.unwrap(),usersid.unwrap()));
            }
        }
        return Err(format!("something went wrong"));
       // username
    }


    pub fn gettokengroups(&self) -> HashMap<String,String>{
        let mut sidgroup:HashMap<String,String> = HashMap::new();
        let mut buffer = getmytokeninfo(self.tokenhandle,2);
        let groupcount = unsafe{std::ptr::read(buffer.as_ptr() as *const u32)};
        for i in 0..groupcount {
            let group = unsafe { *((buffer.as_mut_ptr() as usize + 8 + (i as usize * std::mem::size_of::<SID_AND_ATTRIBUTES>())) as *mut SID_AND_ATTRIBUTES) };
            //println!("token sid: {}", sidtostring2(group.Sid).unwrap());
           // println!("token group: {}", sidtousername(group.Sid).unwrap());
            sidgroup.insert(format!("{}",sidtostring2(group.Sid).unwrap().trim_end_matches("\0")),
                            format!("{}",sidtousername(group.Sid).unwrap().trim_end_matches("\0")));
        }
        sidgroup

    }


    pub fn gettokenprivilegedescription(&self,privname:&str) -> Result<String,String>{
        let mut buffer:Vec<u8> = vec![0;1024];
        let mut bytesneeded = 1024 ;
        let mut languageid = 0;
        let res = unsafe{
            LookupPrivilegeDisplayNameA(std::ptr::null_mut(),
                                               privname.as_bytes().as_ptr() as *const i8,
                                               buffer.as_mut_ptr() as *mut c_void, &mut bytesneeded,
                                               &mut languageid)};
        if res!=0{
            return Ok(format!("{}",String::from_utf8_lossy(&buffer).trim_end_matches("\0")));
        }
        else{
            return Err(format!("LookupPrivilegeDisplayNameA error: {}",unsafe{GetLastError()}));
        }
    }

}

impl Drop for mytoken {
    fn drop(&mut self) {
        if self.tokenhandle!=0 as *mut c_void{
            unsafe{CloseHandle(self.tokenhandle)};
        }
    }
}



pub fn getprivilegedescription(privname:&str){

    let mut description = vec![0u8;1024];
    let mut desclength = description.len() as u32;
    let mut languageid = 0;
    let res = unsafe{LookupPrivilegeDisplayNameA(std::ptr::null_mut(),
                                       privname.as_bytes().as_ptr() as *const i8,
       description.as_mut_ptr() as *mut c_void,&mut desclength,&mut languageid
    )};
    if res==0{
         println!("LookupPrivilegeDisplayNameA failed: {}",unsafe{GetLastError()});
        return ();

    }

    println!("{}",String::from_utf8_lossy(&description));
}


