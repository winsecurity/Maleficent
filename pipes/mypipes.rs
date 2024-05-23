use std::net::Shutdown::Write;
use winapi::um::errhandlingapi::*;
use winapi::um::fileapi::*;
use winapi::um::handleapi::*;
use winapi::um::namedpipeapi::*;
use winapi::um::winbase::*;
use winapi::ctypes::*;

pub fn createpipe(){

    let serverpipe = unsafe{CreateNamedPipeA("\\\\.\\pipe\\PIPEFLAG{MAYTHERE_B_LIGHT_AT_THE_ENDOF_THIS_TUNNEL}\0".as_bytes().as_ptr() as *const i8,
                            PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE,
    2,1024,1024,0,std::ptr::null_mut())};

    if serverpipe==INVALID_HANDLE_VALUE{
        println!("CreateNamedPipeA failed: {}",unsafe{GetLastError()});
        return ();
    }
    unsafe{ConnectNamedPipe(serverpipe,std::ptr::null_mut())};
    /*let mut buffer = vec![0u8;1024];
    let mut bytesread = 0;
    unsafe{ReadFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,
                    buffer.len() as u32,&mut bytesread, std::ptr::null_mut())};

    let mut msg = String::from_utf8_lossy(&buffer).trim_end_matches("\0").to_string();
    if msg.to_lowercase().contains("gimme_the_flag"){
        msg = "CLIENTPIPEFLAG{NICE_JOB_ON_SENDING_MSG_TO_US}".to_string();
    }
    else{
        msg = "Sorry, Try Again".to_string();
    }
    buffer = msg.bytes().collect::<Vec<u8>>();
    buffer.push(0);
    unsafe{WriteFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,std::ptr::null_mut(),std::ptr::null_mut())};
*/


    /*unsafe{ConnectNamedPipe(serverpipe,std::ptr::null_mut())};


    let mut buffer = vec![0u8;1024];
    let mut bytesread = 0;
    unsafe{ReadFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,
    buffer.len() as u32,&mut bytesread, std::ptr::null_mut())};

    let mut msg = String::from_utf8_lossy(&buffer).trim_end_matches("\0").to_string();
    println!("{}",msg);

    loop{
        if msg.contains("quit"){
            break;
        }


        buffer = msg.bytes().collect::<Vec<u8>>();
        buffer.push(0);
        unsafe{WriteFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,std::ptr::null_mut(),std::ptr::null_mut())};

        buffer = vec![0u8;1024];
        let mut bytesread = 0;
        unsafe{ReadFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,
                        buffer.len() as u32,&mut bytesread, std::ptr::null_mut())};
        msg = String::from_utf8_lossy(&buffer).trim_end_matches("\0").to_string();
        println!("{}",msg);

    }*/

}



pub fn createpipe2(){

    let serverpipe = unsafe{CreateNamedPipeA("\\\\.\\pipe\\myserverpipe123\0".as_bytes().as_ptr() as *const i8,
                                             PIPE_ACCESS_DUPLEX,PIPE_TYPE_MESSAGE,
                                             2,1024,1024,0,std::ptr::null_mut())};

    if serverpipe==INVALID_HANDLE_VALUE{
        println!("CreateNamedPipeA failed: {}",unsafe{GetLastError()});
        return ();
    }
    unsafe{ConnectNamedPipe(serverpipe,std::ptr::null_mut())};
    let mut buffer = vec![0u8;1024];
    let mut bytesread = 0;
    unsafe{ReadFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,
                    buffer.len() as u32,&mut bytesread, std::ptr::null_mut())};

    unsafe{CloseHandle(serverpipe)};
    /*let mut buffer = vec![0u8;1024];
    let mut bytesread = 0;
    unsafe{ReadFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,
                    buffer.len() as u32,&mut bytesread, std::ptr::null_mut())};

    let mut msg = String::from_utf8_lossy(&buffer).trim_end_matches("\0").to_string();
    if msg.to_lowercase().contains("gimme_the_flag"){
        msg = "CLIENTPIPEFLAG{NICE_JOB_ON_SENDING_MSG_TO_US}".to_string();
    }
    else{
        msg = "Sorry, Try Again".to_string();
    }
    buffer = msg.bytes().collect::<Vec<u8>>();
    buffer.push(0);
    unsafe{WriteFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,std::ptr::null_mut(),std::ptr::null_mut())};
*/


    /*unsafe{ConnectNamedPipe(serverpipe,std::ptr::null_mut())};


    let mut buffer = vec![0u8;1024];
    let mut bytesread = 0;
    unsafe{ReadFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,
    buffer.len() as u32,&mut bytesread, std::ptr::null_mut())};

    let mut msg = String::from_utf8_lossy(&buffer).trim_end_matches("\0").to_string();
    println!("{}",msg);

    loop{
        if msg.contains("quit"){
            break;
        }


        buffer = msg.bytes().collect::<Vec<u8>>();
        buffer.push(0);
        unsafe{WriteFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,buffer.len() as u32,std::ptr::null_mut(),std::ptr::null_mut())};

        buffer = vec![0u8;1024];
        let mut bytesread = 0;
        unsafe{ReadFile(serverpipe,buffer.as_mut_ptr() as *mut c_void,
                        buffer.len() as u32,&mut bytesread, std::ptr::null_mut())};
        msg = String::from_utf8_lossy(&buffer).trim_end_matches("\0").to_string();
        println!("{}",msg);

    }*/

}


