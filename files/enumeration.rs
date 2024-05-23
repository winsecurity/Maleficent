use winapi::shared::lmcons::LMSTR;
use winapi::shared::winerror::ERROR_FILE_NOT_FOUND;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::*;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::WIN32_FIND_DATAW;
use winapi::um::processthreadsapi::GetCurrentProcess;
use crate::helpers::utils::readunicodestringfrommemory2;
use winapi::ctypes::*;

pub fn listcontents(dir:String){

    let mut dirbuffer = dir.encode_utf16().collect::<Vec<u16>>();
    dirbuffer.push(0);

    let mut finddataw = unsafe{std::mem::zeroed::<WIN32_FIND_DATAW>()};
    let searchhandle = unsafe{FindFirstFileW(dirbuffer.as_mut_ptr() as LMSTR,&mut finddataw)};
    if searchhandle!=INVALID_HANDLE_VALUE{
            let filename = readunicodestringfrommemory2(unsafe{GetCurrentProcess()},finddataw.cFileName.as_ptr() as *const c_void);
            println!("filename: {}",filename);


            loop{
                finddataw = unsafe{std::mem::zeroed::<WIN32_FIND_DATAW>()};
                let res = unsafe{FindNextFileW(searchhandle,&mut finddataw)};
                if res==0{
                    break;
                }
                let filename = readunicodestringfrommemory2(unsafe{GetCurrentProcess()},finddataw.cFileName.as_ptr() as *const c_void);
                println!("filename: {}",filename);


            }

    }

}





pub fn listdirectory(dir:String) -> Result<Vec<String>,String>{
    let mut files:Vec<String> = Vec::new();
    let mut dirbuffer = dir.encode_utf16().collect::<Vec<u16>>();
    dirbuffer.push(0);
    let mut win32finddata = unsafe{std::mem::zeroed::<WIN32_FIND_DATAW>()};

    let searchhandle = unsafe{FindFirstFileW(dirbuffer.as_mut_ptr() as *mut u16,&mut win32finddata)};
    if searchhandle!=INVALID_HANDLE_VALUE{

        //println!("filename: {}",String::from_utf16_lossy(&win32finddata.cFileName));
        files.push(String::from_utf16_lossy(&win32finddata.cFileName)
            .trim_end_matches("\0").to_string());
        loop{
            win32finddata = unsafe{std::mem::zeroed::<WIN32_FIND_DATAW>()};
            let res = unsafe{FindNextFileW(searchhandle,&mut win32finddata)};
            if res==0{
                break;
            }
            /*println!("filename: {:?}",String::from_utf16_lossy(&win32finddata.cFileName)
                .trim_end_matches("\0").to_string());*/
            files.push(String::from_utf16_lossy(&win32finddata.cFileName)
                .trim_end_matches("\0").to_string());

        }

        Ok(files)
    }

    else{
        return Err(format!("FindFirstFileW failed: {}",unsafe{GetLastError()}));


    }

}

