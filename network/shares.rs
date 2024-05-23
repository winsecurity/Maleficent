

use  winapi::um::lmshare::*;
use winapi::ctypes::*;
use winapi::shared::lmcons::*;
use winapi::um::errhandlingapi::*;
use winapi::um::lmapibuf::*;
use winapi::um::processthreadsapi::*;
use crate::helpers::utils::*;
use std::collections::*;


#[derive(Debug)]
#[repr(C)]
pub struct netshares{
    pub servername:*mut c_void,
    pub level:u32,
    pub bufferpointer: Vec<*mut u8>,
    entriesread: u32,
    totalentries: u32,
    resumehandle: *mut u32
}


impl netshares{
    pub fn new(servername:*mut c_void,level:u32) -> Self{
        Self{
            servername,level,bufferpointer:vec![0 as *mut u8],
            entriesread:0,totalentries:0,resumehandle:0 as *mut u32
        }

    }


    fn getbuffertop(&mut self) -> Option<&mut *mut u8>{
        match self.bufferpointer.len(){
            0=>None,
            n=> Some(&mut self.bufferpointer[n-1])
        }
    }

    pub fn getbuffer(&mut self) {
        let res = unsafe{NetShareEnum(self.servername as LMSTR ,self.level,
                                      self.getbuffertop().unwrap(),MAX_PREFERRED_LENGTH,&mut self.entriesread,
                                      &mut self.totalentries, self.resumehandle)};
        if res==0{
            //self.bufferpointer.push(0 as *mut u8);
        }
    }


    pub fn getsharenamesonly(&mut self) -> Vec<String>{
        self.level=0;
        self.getbuffer();
        let mut sharenames:Vec<String> = Vec::new();
        if *self.getbuffertop().unwrap()!=0 as *mut u8{
            for i in 0..self.entriesread{
                let share = parsememory::<SHARE_INFO_0>(unsafe{GetCurrentProcess()},
                                                        (self.bufferpointer[self.bufferpointer.len()-1] as usize+(i as usize*std::mem::size_of::<SHARE_INFO_0>())) as *const c_void).unwrap();

                let sharename = readunicodestringfrommemory2(
                    unsafe{GetCurrentProcess()},share.shi0_netname as *const c_void
                );
                sharenames.push(sharename);

            }
        }
        self.bufferpointer.push(0 as *mut u8);
        sharenames
    }


    pub fn getsharenametypedesc(&mut self) -> HashMap<String,String>{
        self.level = 1;
        self.getbuffer();

        let mut sharenametypedesc: HashMap<String,String> = HashMap::new();
        if *self.getbuffertop().unwrap()!=0 as *mut u8{
            for i in 0..self.entriesread{
                let share = parsememory::<SHARE_INFO_1>(unsafe{GetCurrentProcess()},
                                                        (self.bufferpointer[self.bufferpointer.len()-1] as usize+(i as usize*std::mem::size_of::<SHARE_INFO_1>())) as *const c_void).unwrap();

                let sharename = readunicodestringfrommemory2(
                    unsafe{GetCurrentProcess()},share.shi1_netname as *const c_void
                );
                let sharedesc = readunicodestringfrommemory2(
                    unsafe{GetCurrentProcess()},share.shi1_remark as *const c_void
                );
                //println!("share description: {}",sharedesc);
                sharenametypedesc.insert(sharename,sharedesc);

            }
        }

        self.bufferpointer.push(0 as *mut u8);
        sharenametypedesc
    }


    pub fn getsharenamepermpath(&mut self)-> HashMap<String,String>{
        self.level = 2;
        self.getbuffer();

        let mut sharenamepath: HashMap<String,String> = HashMap::new();

        if *self.getbuffertop().unwrap()!=0 as *mut u8{
            for i in 0..self.entriesread{
                let share = parsememory::<SHARE_INFO_2>(unsafe{GetCurrentProcess()},
                                                        (self.bufferpointer[self.bufferpointer.len()-1] as usize+(i as usize*std::mem::size_of::<SHARE_INFO_2>())) as *const c_void).unwrap();

                let sharename = readunicodestringfrommemory2(
                    unsafe{GetCurrentProcess()},share.shi2_netname as *const c_void
                );
                let sharedesc = readunicodestringfrommemory2(
                    unsafe{GetCurrentProcess()},share.shi2_remark as *const c_void
                );
                let sharepass = readunicodestringfrommemory2(
                    unsafe{GetCurrentProcess()},share.shi2_passwd as *const c_void
                );
                let sharepath = readunicodestringfrommemory2(
                    unsafe{GetCurrentProcess()},share.shi2_path as *const c_void
                );
                /*println!("share name: {}",sharename);
                println!("share pass: {}",sharepass);
                println!("share path: {}",sharepath);
                println!();*/
                sharenamepath.insert(sharename,sharepath);
            }
        }

        self.bufferpointer.push(0 as *mut u8);
        sharenamepath
    }


}

impl Drop for netshares{
    fn drop(&mut self) {
       if self.bufferpointer.len()>0{
           for i in 0..self.bufferpointer.len(){
               if self.bufferpointer[i]!=0 as *mut u8{
                   //println!("freeing memory at: {:x?}",self.bufferpointer[i]);
                   unsafe{NetApiBufferFree(self.bufferpointer[i] as *mut c_void)};

               }

           }
       }
    }
}


pub fn getsharenames2(){

    let mut buffer = 0 as *mut u8;
    let mut entriesread = 0 ;
    let mut totalentries = 0 ;
    let mut resumehandle = 0 as *mut u32;
    let res = unsafe{NetShareEnum(std::ptr::null_mut(),0,
    &mut buffer,MAX_PREFERRED_LENGTH,&mut entriesread,
    &mut totalentries, resumehandle)};
    if res==0{// NERR_Success
        println!("buffer: {:?}",buffer);
        println!("entries read: {}",entriesread);
        println!("total entries: {}",totalentries);
        for i in 0..entriesread{
            let share = parsememory::<SHARE_INFO_0>(unsafe{GetCurrentProcess()},
                                    (buffer as usize+(i as usize*std::mem::size_of::<SHARE_INFO_0>())) as *const c_void).unwrap();

            println!("sharename: {}",readunicodestringfrommemory2(
                unsafe{GetCurrentProcess()},share.shi0_netname as *const c_void
            ));
        }
        unsafe{NetApiBufferFree(buffer as *mut c_void)};

    }
    else{
        println!("NetShareEnum failed: {}",unsafe{GetLastError()});
        unsafe{NetApiBufferFree(buffer as *mut c_void)};

    }

}


