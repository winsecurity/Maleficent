use ntapi::ntexapi::*;
use ntapi::ntldr::*;
use ntapi::ntpebteb::*;
use ntapi::ntpsapi::*;
use ntapi::ntrtl::*;
use winapi::ctypes::*;
use winapi::shared::ntdef::*;
use winapi::shared::ntstatus::*;
use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::*;
use winapi::um::handleapi::*;
use winapi::um::memoryapi::*;
use winapi::um::processthreadsapi::*;
use winapi::um::psapi::*;
use winapi::um::tlhelp32::*;
use winapi::um::winnt::*;
use crate::helpers::utils::*;

use std::collections::HashMap;
use winapi::shared::windef::*;
use winapi::um::sysinfoapi::*;
use winapi::um::winuser::{EnumWindows, GetWindowTextA, GetWindowTextLengthA};

pub fn enumallprocesses(){

    let mut bytesneeded:u32 = 100;
    let mut counter = 1;

    let mut pidarray = loop{
        let mut pidarray:Vec<u32> = vec![0;bytesneeded as usize];
        let res = unsafe{EnumProcesses(pidarray.as_mut_ptr() as *mut u32,pidarray.len() as u32,&mut bytesneeded)};
        println!("our array length: {}",pidarray.len());
        println!("bytesneeded: {}",bytesneeded);
        if pidarray.len()>bytesneeded as usize{
            break pidarray;
        }
        bytesneeded *= 2;
    };
    let res = unsafe{EnumProcesses(pidarray.as_mut_ptr() as *mut u32,pidarray.len() as u32,&mut bytesneeded)};
    if res==0{
        println!("Enumprocesses failed: {}",unsafe{GetLastError()});
    }
    else{
        for i in 0..pidarray.len(){
            if pidarray[i]!=0{
                counter+=1;
            }
        }
        println!("pidarray: {:?}",pidarray);
        println!("number of processes: {}",counter);
    }

}



pub fn enumallprocesses2() -> Result<HashMap<u32,String>,String>{

    let mut counter = 0;
    let mut processes:HashMap<u32,String> = HashMap::new();
    let snaphandle = unsafe{CreateToolhelp32Snapshot(0x00000002,0)};
    if snaphandle!=INVALID_HANDLE_VALUE{
        let mut pentry = unsafe{std::mem::zeroed::<PROCESSENTRY32W>()};
        pentry.dwSize = unsafe{std::mem::size_of::<PROCESSENTRY32W>()} as u32;
        let res = unsafe{ Process32FirstW(snaphandle, &mut pentry)};
        if res==1{

            processes.insert(pentry.th32ProcessID,String::from_utf16_lossy(
                unsafe{std::mem::transmute(&pentry.szExeFile[..])}
            ).trim_end_matches("\0").to_string());
            counter+=1;
            loop{
                pentry = unsafe{std::mem::zeroed::<PROCESSENTRY32W>()};
                pentry.dwSize = unsafe{std::mem::size_of::<PROCESSENTRY32W>()} as u32;
                let res2 = unsafe{Process32NextW(snaphandle,&mut pentry)};
                if res2==0||res2==ERROR_NO_MORE_FILES as i32{
                    break;
                }
                processes.insert(pentry.th32ProcessID,String::from_utf16_lossy(
                    unsafe{std::mem::transmute(&pentry.szExeFile[..])}
                ).trim_end_matches("\0").to_string());
                counter+=1;
            }
        }



    }
    if processes.len()>1{
        return Ok(processes);
    }
    Err(format!("CreateToolhelp32Snapshot failed: {}",unsafe{GetLastError()}))

}


pub fn getparentpid(pid:u32) -> Result<u32,String>{


    let snaphandle = unsafe{CreateToolhelp32Snapshot(0x00000002,0)};
    if snaphandle!=INVALID_HANDLE_VALUE{
        let mut pentry = unsafe{std::mem::zeroed::<PROCESSENTRY32W>()};
        pentry.dwSize = unsafe{std::mem::size_of::<PROCESSENTRY32W>()} as u32;
        let res = unsafe{ Process32FirstW(snaphandle, &mut pentry)};
        if res==1{

            if pentry.th32ProcessID==pid{
                return Ok(pentry.th32ParentProcessID);
            }

            loop{
                pentry = unsafe{std::mem::zeroed::<PROCESSENTRY32W>()};
                pentry.dwSize = unsafe{std::mem::size_of::<PROCESSENTRY32W>()} as u32;
                let res2 = unsafe{Process32NextW(snaphandle,&mut pentry)};
                if res2==0||res2==ERROR_NO_MORE_FILES as i32{
                    break;
                }
                if pentry.th32ProcessID==pid{
                    return Ok(pentry.th32ParentProcessID);
                }

            }
        }



    }

    Err(format!("CreateToolhelp32Snapshot failed: {}",unsafe{GetLastError()}))

}



pub fn myenumwindowsproc(handle: HWND,param:u32 ) -> u8{

    let len1 = unsafe{GetWindowTextLengthA(handle)};
    if len1>0{

        let mut buffer:Vec<u8> = vec![0;(len1+1) as usize];
        let res = unsafe{GetWindowTextA(handle,buffer.as_mut_ptr() as *mut i8,buffer.len() as i32)};
        if res!=0{
            let title = String::from_utf8_lossy(&buffer[..]).to_string();
            if title.to_lowercase().contains("title{"){
                println!("{}",title);
            }
        }

    }

    1
}


pub fn enumallwindows(){

    let res = unsafe{EnumWindows(std::mem::transmute(myenumwindowsproc as *mut c_void),0)};
    if res==0{
        println!("EnumWindows failed: {}",unsafe{GetLastError()});
    }
}


pub fn enumallprocesses3(){

    let mut bytesneeded = 1;
    let buffer = loop{
        let mut buffer:Vec<u8> = vec![0;bytesneeded as usize];
        let res = unsafe{NtQuerySystemInformation(5, buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32, &mut bytesneeded)};
        if NT_SUCCESS(res){
            break buffer;
        }
    };
    let p = unsafe{std::mem::zeroed::<SYSTEM_PROCESS_INFORMATION>()};
    let pidoffset = (&p.UniqueProcessId as *const _ as u64 - &p as *const _ as u64);
    let nameoffset = (&p.ImageName as *const _ as u64 - &p as *const _ as u64);

    let mut counter =0;

    let mut currentpos = buffer.as_ptr() as u64;
    //println!("first process id: {}",unsafe{std::ptr::read((currentpos+pidoffset) as *const u64)});
    loop{
        let nextentry = unsafe{std::ptr::read(currentpos as *const u32)};
        let pid = unsafe{std::ptr::read((currentpos+pidoffset) as *const u64)};
        println!("pid: {}",pid);
        let pname = unsafe{std::ptr::read((currentpos+nameoffset) as *const UNICODE_STRING)};
        println!("process name: {}",readunicodestringfrommemory(
            unsafe{GetCurrentProcess()},pname.Buffer as *const c_void
        ));

        counter += 1;
        if nextentry == 0{
            break;
        }

        currentpos += nextentry as u64;
    }

    println!("number of processes: {}",counter);

}

pub fn enumprocessthreads(pid: u32) -> Result<Vec<u32>,String>{

    let mut threadids:Vec<u32> = Vec::new();
    let snaphandle = unsafe{CreateToolhelp32Snapshot(0x00000004,unsafe{GetCurrentProcessId()})};
    if snaphandle!=INVALID_HANDLE_VALUE{
        let mut tentry = unsafe{std::mem::zeroed::<THREADENTRY32>()};
        tentry.dwSize = unsafe{std::mem::size_of::<THREADENTRY32>()} as u32;

        let res = unsafe{Thread32First(snaphandle,&mut tentry)};
        if res==1{
            loop{
                if tentry.th32OwnerProcessID==pid{
                    threadids.push(tentry.th32ThreadID);
                }
                tentry = unsafe{std::mem::zeroed::<THREADENTRY32>()};
                tentry.dwSize = unsafe{std::mem::size_of::<THREADENTRY32>()} as u32;
                let res2 = unsafe{Thread32Next(snaphandle,&mut tentry)};
                if res2==0||res2==ERROR_NO_MORE_FILES as i32{
                    break;
                }

            }
        return Ok(threadids);
        }

    }
    return Err(format!("CreateToolhelp32Snapshot failed: {}",unsafe{GetLastError()}));
}


pub fn enumprocessthreads2(){
    let mut bytesneeded = 1;
    let buffer = loop{
        let mut buffer:Vec<u8> = vec![0;bytesneeded as usize];
        let res = unsafe{NtQuerySystemInformation(5, buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32, &mut bytesneeded)};
        if NT_SUCCESS(res){
            break buffer;
        }
    };
    let p = unsafe{std::mem::zeroed::<SYSTEM_PROCESS_INFORMATION>()};
    let pidoffset = (&p.UniqueProcessId as *const _ as u64 - &p as *const _ as u64);
    let nameoffset = (&p.ImageName as *const _ as u64 - &p as *const _ as u64);
    let threadoffset = (&p.Threads as *const _ as u64 - &p as *const _ as u64);


    let mut counter =0;

    let mut currentpos = buffer.as_ptr() as u64;
    //println!("first process id: {}",unsafe{std::ptr::read((currentpos+pidoffset) as *const u64)});
    loop{
        let nextentry = unsafe{std::ptr::read(currentpos as *const u32)};
        let pid = unsafe{std::ptr::read((currentpos+pidoffset) as *const u64)};
        //println!("pid: {}",pid);
        let pname = unsafe{std::ptr::read((currentpos+nameoffset) as *const UNICODE_STRING)};
        //println!("process name: {}",readunicodestringfrommemory(
        //    unsafe{GetCurrentProcess()},pname.Buffer as *const c_void
        //));


        let nthreads = unsafe{std::ptr::read((currentpos+4) as *const u32)};

        //let nthreads = ((currentpos+nextentry as u64) - (currentpos+threadoffset));


        for i in 0..nthreads{
            let threadinfo = unsafe{std::ptr::read(
                (currentpos + threadoffset + (i as u64 * std::mem::size_of::<SYSTEM_THREAD_INFORMATION>() as u64)) as *const SYSTEM_THREAD_INFORMATION
            )};

            if threadinfo.ClientId.UniqueProcess as u32 == unsafe{GetCurrentProcessId()}{
                println!("thread id: {}",threadinfo.ClientId.UniqueThread as u64) ;

            }
        }


        counter += 1;
        if nextentry == 0{
            break;
        }

        currentpos += nextentry as u64;
    }
    println!("our thread id: {}",unsafe{GetCurrentThreadId()});



}


pub fn getosversion() -> Result<String,String>{

    let mut osinfo = unsafe{std::mem::zeroed::<RTL_OSVERSIONINFOEXW>()};
    osinfo.dwOSVersionInfoSize = unsafe{std::mem::size_of::<RTL_OSVERSIONINFOEXW>()} as u32;

    let res = unsafe{RtlGetVersion(&mut osinfo as *mut _ as *mut RTL_OSVERSIONINFOW)};
    if res==STATUS_SUCCESS{
       return Ok(format!("{}.{}.{}",osinfo.dwMajorVersion,osinfo.dwMinorVersion,osinfo.dwBuildNumber));

    }
    return Err(format!("RtlGetVersion error: {}",unsafe{GetLastError()}));
}


pub fn getosram() -> Result<String,String>{

    let mut memorystatus = unsafe{std::mem::zeroed::<MEMORYSTATUSEX>()};
    memorystatus.dwLength = unsafe{std::mem::size_of::<MEMORYSTATUSEX>()} as u32;
    let res = unsafe{GlobalMemoryStatusEx(&mut memorystatus)};
    if res==0{
        return Err(format!("GlobalMemoryStatusEx failed: {}",unsafe{GetLastError()}));
    }
    else{
        //println!("total physical memory: {}",memorystatus.ullTotalPhys);
        //println!("available physical memory: {}",memorystatus.ullAvailPhys);
       //println!("total virtual memory: {}",memorystatus.ullTotalVirtual);
        //println!("available virtual memory: {}",memorystatus.ullAvailVirtual);

        return Ok(format!("{}.{}",memorystatus.ullTotalPhys,memorystatus.ullTotalVirtual));
    }


}



pub fn getcpucount() -> String{
    let mut sysinfo = unsafe{std::mem::zeroed::<SYSTEM_INFO>()};
    unsafe{GetSystemInfo(&mut sysinfo)};

    //println!("number of cpus: {}",sysinfo.dwNumberOfProcessors);
    return format!("{}",sysinfo.dwNumberOfProcessors);
}


pub fn getos2(){

    let mut buffer:Vec<u8> = vec![0;unsafe{std::mem::size_of::<PROCESS_BASIC_INFORMATION>()}];
    let mut byteswritten = 0;
    let res = unsafe{NtQueryInformationProcess(GetCurrentProcess(),0,
    buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,&mut byteswritten)};
    if res==STATUS_SUCCESS{
        let mut ppeb = unsafe{*(buffer.as_mut_ptr() as *mut PROCESS_BASIC_INFORMATION)}.PebBaseAddress;
        let peb = unsafe{*(ppeb as *mut PEB)};
        println!("os major version: {}",peb.OSMajorVersion);
        println!("os minor version: {}",peb.OSMinorVersion);
        println!("build number: {}",peb.OSBuildNumber);
        println!("is being debugged: {}",peb.BeingDebugged);

    }

}

pub fn getdlls(pid:u32) -> Result<HashMap<String,usize>,String>{

    let mut dlls:HashMap<String,usize> = HashMap::new();
   let snaphandle = unsafe{CreateToolhelp32Snapshot(0x00000008|0x10,pid)};
    if snaphandle!=INVALID_HANDLE_VALUE {

        let mut mentry = unsafe{std::mem::zeroed::<MODULEENTRY32>()};
        mentry.dwSize = unsafe{std::mem::size_of::<MODULEENTRY32>()} as u32;

        let res = unsafe{Module32First(snaphandle,&mut mentry)};
        if res==1{
            dlls.insert(String::from_utf8_lossy(
                unsafe{std::mem::transmute(&mentry.szModule[..])}).trim_end_matches("\0").to_string(),
                        mentry.modBaseAddr as usize);


            loop{
                mentry = unsafe{std::mem::zeroed::<MODULEENTRY32>()};
                mentry.dwSize = unsafe{std::mem::size_of::<MODULEENTRY32>()} as u32;

                let res2 = unsafe{Module32Next(snaphandle,&mut mentry)};
                if res2==0||res2==ERROR_NO_MORE_FILES as i32{
                    break;
                }
                dlls.insert(String::from_utf8_lossy(
                    unsafe{std::mem::transmute(&mentry.szModule[..])}).trim_end_matches("\0").to_string(),
                            mentry.modBaseAddr as usize);

            }
            return Ok(dlls);
        }
    }
    Err(format!("CreateToolhelp32Snapshot failed: {}",unsafe{GetLastError()}))

}


pub fn getdlls2(prochandle:*mut c_void){

    let mut buffer:Vec<u8> = vec![0;unsafe{std::mem::size_of::<PROCESS_BASIC_INFORMATION>()}];
    let mut byteswritten = 0;
    let res = unsafe{NtQueryInformationProcess(prochandle,0,
                                               buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,&mut byteswritten)};
    if res==STATUS_SUCCESS{
        let mut ppeb = parsememory::<PROCESS_BASIC_INFORMATION>(unsafe{GetCurrentProcess()},buffer.as_ptr() as *const c_void).unwrap().PebBaseAddress;
        let  peb = parsememory::<PEB>(unsafe{prochandle},ppeb as *const c_void).unwrap();

        let ldr = parsememory::<PEB_LDR_DATA>(unsafe{prochandle},peb.Ldr    as *const c_void).unwrap();

        let mut ldrdatatable =  unsafe{
            *(ldr.InLoadOrderModuleList.Flink as *mut LDR_DATA_TABLE_ENTRY)};

        println!("{}: {:x?}",readunicodestringfrommemory(prochandle,ldrdatatable.BaseDllName.Buffer as *const c_void)
        ,ldrdatatable.DllBase);


        loop{
            ldrdatatable = unsafe{*(ldrdatatable.InLoadOrderLinks.Flink as *mut LDR_DATA_TABLE_ENTRY) };
            if ldrdatatable.InLoadOrderLinks.Flink as u64==(peb.Ldr as u64+0x10){
                println!("{}: {:x?}",readunicodestringfrommemory(prochandle,ldrdatatable.BaseDllName.Buffer as *const c_void)
                         ,ldrdatatable.DllBase);
                break;
            }
            println!("{}: {:x?}",readunicodestringfrommemory(prochandle,ldrdatatable.BaseDllName.Buffer as *const c_void)
                     ,ldrdatatable.DllBase);


        }






    }


}

use std::arch::asm;
pub fn getmydlls() -> HashMap<String,usize>{

    let mut dlls:HashMap<String,usize> = HashMap::new();
    let mut pebaddress: u64 = 0;
    unsafe{
        asm!{
        "mov {},gs:[0x60]",
        out(reg) pebaddress
        }
    }
        let mut peb = parsememory::<PEB>(unsafe{GetCurrentProcess()},pebaddress as *const c_void).unwrap();
        //let  peb = parsememory::<PEB>(unsafe{GetCurrentProcess()},ppeb as *const c_void).unwrap();

        let ldr = parsememory::<PEB_LDR_DATA>(unsafe{GetCurrentProcess()},peb.Ldr    as *const c_void).unwrap();

        let mut ldrdatatable =  unsafe{
            *(ldr.InLoadOrderModuleList.Flink as *mut LDR_DATA_TABLE_ENTRY)};

        let dllname = readunicodestringfrommemory2(unsafe{GetCurrentProcess()},ldrdatatable.BaseDllName.Buffer as *const c_void);
        let dllbase = ldrdatatable.DllBase;
        dlls.insert(dllname,dllbase as usize);


        loop{
            ldrdatatable = unsafe{*(ldrdatatable.InLoadOrderLinks.Flink as *mut LDR_DATA_TABLE_ENTRY) };
            if ldrdatatable.InLoadOrderLinks.Flink as u64==(peb.Ldr as u64+0x10){
                let dllname = readunicodestringfrommemory2(unsafe{GetCurrentProcess()},ldrdatatable.BaseDllName.Buffer as *const c_void);
                let dllbase = ldrdatatable.DllBase;
                dlls.insert(dllname,dllbase as usize);

                break;
            }
            let dllname = readunicodestringfrommemory2(unsafe{GetCurrentProcess()},ldrdatatable.BaseDllName.Buffer as *const c_void);
            let dllbase = ldrdatatable.DllBase;
            dlls.insert(dllname,dllbase as usize);



        }


        return dlls;






}




pub fn getenvironmentblock(prochandle:*mut c_void) -> Result<Vec<String>,String>{
    let mut buffer:Vec<u8> = vec![0;unsafe{std::mem::size_of::<PROCESS_BASIC_INFORMATION>()}];
    let mut byteswritten = 0;
    let res = unsafe{NtQueryInformationProcess(prochandle,0,
                                               buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,&mut byteswritten)};
    let mut envs :Vec<String> = Vec::new();
    if res==STATUS_SUCCESS{
        let mut ppeb =  parsememory::<PROCESS_BASIC_INFORMATION>(unsafe{GetCurrentProcess()},buffer.as_mut_ptr() as *mut c_void).unwrap().PebBaseAddress;
        let peb = parsememory::<PEB>(prochandle,ppeb as *mut c_void).unwrap();

        let params = parsememory::<RTL_USER_PROCESS_PARAMETERS>(prochandle,peb.ProcessParameters as *const c_void).unwrap();

        let mut temp = params.Environment as usize;
        loop{
            if temp>(params.Environment as usize+params.EnvironmentSize){
                break;
            }
            let value1 = readunicodestringfrommemory2(prochandle,temp as *const c_void);
            envs.push(value1.clone());

            temp += value1.len()*2 +2;
        }

        Ok(envs)

    }
    else{
        Err(format!("NtQueryInformationProcess error: {}",unsafe{GetLastError()}))
    }
}


pub fn getenvironmentblock2() -> Result<Vec<String>,String>{
    use std::arch::asm;
    let mut envs :Vec<String> = Vec::new();
    let mut pebaddress: u64 = 0;
    unsafe{
        asm!{
        "mov {},gs:[0x60]",
        out(reg) pebaddress
        }
    };
        let peb = parsememory::<PEB>(unsafe{GetCurrentProcess()},pebaddress as *mut c_void).unwrap();

        let params = parsememory::<RTL_USER_PROCESS_PARAMETERS>(unsafe{GetCurrentProcess()},peb.ProcessParameters as *const c_void).unwrap();

        let mut temp = params.Environment as usize;
        loop{
            if temp>(params.Environment as usize+params.EnvironmentSize){
                break;
            }
            let value1 = readunicodestringfrommemory2(unsafe{GetCurrentProcess()},temp as *const c_void);
            envs.push(value1.clone());

            temp += value1.len()*2 +2;
        }

        Ok(envs)



}

pub fn getprocessparameters(prochandle:*mut c_void) ->Result<String,String> {
    let mut buffer:Vec<u8> = vec![0;unsafe{std::mem::size_of::<PROCESS_BASIC_INFORMATION>()}];
    let mut byteswritten = 0;
    let res = unsafe{NtQueryInformationProcess(prochandle,0,
                                               buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,&mut byteswritten)};
    if res==STATUS_SUCCESS{
        let mut ppeb = parsememory::<PROCESS_BASIC_INFORMATION>(unsafe{GetCurrentProcess()},buffer.as_ptr() as *const c_void).unwrap().PebBaseAddress;
        let  peb = parsememory::<PEB>(unsafe{prochandle},ppeb as *const c_void).unwrap();


        let  params = parsememory::<RTL_USER_PROCESS_PARAMETERS>(unsafe{prochandle},peb.ProcessParameters as *const c_void).unwrap();
        let cmdline = readunicodestringfrommemory2(unsafe{prochandle},params.CommandLine.Buffer as *const c_void);
        return Ok(cmdline);
    }
    else{
        return Err(format!("NtQueryInformationProcess failed: {}",unsafe{GetLastError()}));
    }
}


pub fn getenvblock(prochandle: *mut c_void){
    let mut buffer:Vec<u8> = vec![0;unsafe{std::mem::size_of::<PROCESS_BASIC_INFORMATION>()}];
    let mut byteswritten = 0;
    let res = unsafe{NtQueryInformationProcess(prochandle,0,
                                               buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,&mut byteswritten)};
    if res==STATUS_SUCCESS{
        let mut ppeb = parsememory::<PROCESS_BASIC_INFORMATION>(unsafe{GetCurrentProcess()},buffer.as_ptr() as *const c_void).unwrap().PebBaseAddress;
        let  peb = parsememory::<PEB>(unsafe{prochandle},ppeb as *const c_void).unwrap();


        let  params = parsememory::<RTL_USER_PROCESS_PARAMETERS>(unsafe{prochandle},peb.ProcessParameters as *const c_void).unwrap();

        let mut temp= params.Environment as usize;

        loop{
            if temp as usize>(params.Environment as usize+ params.EnvironmentSize){
                break;
            }
            let env1 = readunicodestringfrommemory2(unsafe{prochandle},temp as *const c_void);
            println!("{}",env1);
            temp += (env1.len()*2+2) as usize;

        }


    }
    else{
        //return Err(format!("NtQueryInformationProcess failed: {}",unsafe{GetLastError()}));
    }
}


pub fn setprocessparameters(prochandle:*mut c_void,s:String){
    let mut buffer:Vec<u8> = vec![0;unsafe{std::mem::size_of::<PROCESS_BASIC_INFORMATION>()}];
    let mut byteswritten = 0;
    let res = unsafe{NtQueryInformationProcess(prochandle,0,
                                               buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,&mut byteswritten)};
    if res==STATUS_SUCCESS{
        let mut ppeb = parsememory::<PROCESS_BASIC_INFORMATION>(unsafe{GetCurrentProcess()},buffer.as_ptr() as *const c_void).unwrap().PebBaseAddress;
        let  peb = parsememory::<PEB>(unsafe{prochandle},ppeb as *const c_void).unwrap();


        let rtl = unsafe{std::mem::zeroed::<RTL_USER_PROCESS_PARAMETERS>()};
        let offset = (&rtl.CommandLine.Length as *const _ as  u64 - &rtl as *const _ as u64);

        let cmdlineaddress = peb.ProcessParameters as u64 + offset;
        let p = parsememory::<UNICODE_STRING>(prochandle,cmdlineaddress as *const c_void).unwrap();
        //println!("{}",readunicodestringfrommemory(prochandle,(p.Buffer) as *const c_void));
       // println!("length: {}",p.Length);
        let sbuffer = s.encode_utf16().collect::<Vec<u16>>();

        if (sbuffer.len()*2)<=p.Length as usize{
            unsafe{WriteProcessMemory(prochandle,
            p.Buffer as *mut c_void,sbuffer.as_ptr() as *const c_void,
            sbuffer.len()*2,std::ptr::null_mut())};
        }
        //println!("{}",readunicodestringfrommemory(prochandle,(p.Buffer) as *const c_void));


    }
    else{
        //return Err(format!("NtQueryInformationProcess failed: {}",unsafe{GetLastError()}));
    }
}


