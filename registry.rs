use winapi::um::winnt::*;
use winapi::um::winreg::*;
use winapi::ctypes::*;
use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::*;
use winapi::um::processthreadsapi::*;
use crate::peparser::ReadStringFromMemory;


pub fn getcpuname() -> Result<String,String>{

    let key = "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\0";
    let mut reghandle = 0 as *mut c_void;
    let res = unsafe{RegOpenKeyExA(HKEY_LOCAL_MACHINE,
    key.as_bytes().as_ptr() as *const i8,
    0,KEY_READ|KEY_EXECUTE,
    std::mem::transmute(&mut reghandle))};

    //println!("RegOpenKeyExA status: {}",res);

    if res as i32==(ERROR_SUCCESS as i32){

        let mut i=0;
        'outerloop: loop{
            let mut buffer= vec![0u8;1024];
            let mut bytesneeded = buffer.len() as u32;
            let mut class= vec![0u8;1024];
            let mut classlength = buffer.len() as u32;
            let t = unsafe{RegEnumKeyExA(std::mem::transmute(reghandle),
                                         i,
                                         buffer.as_mut_ptr() as *mut i8,&mut bytesneeded,
                                         std::ptr::null_mut(),class.as_mut_ptr() as *mut i8,&mut classlength,std::ptr::null_mut())};

            if t ==ERROR_NO_MORE_ITEMS as i32{
                break;
            }
            if t as i32==ERROR_SUCCESS as i32{
                let subkey = String::from_utf8_lossy(&buffer).trim_end_matches('\0').to_string();
                //println!("{}",subkey);
                //println!("{}",String::from_utf8_lossy(&class));

                let mut j = 0;
                'innerloop: loop{

                    let mykey = (key.trim_end_matches("\0").to_string()+"\\"+&subkey+"\0");
                    let mykeybuffer = mykey.bytes().collect::<Vec<u8>>();
                    //println!("{}",mykey);
                    let mut keyhandle = 0 as *mut c_void;
                    let res2 = unsafe{RegOpenKeyExA(HKEY_LOCAL_MACHINE,
                                                   mykeybuffer.as_ptr() as *const i8,
                                                   0,KEY_READ|KEY_EXECUTE,
                                                   std::mem::transmute(&mut keyhandle))};



                    let mut buffer2= vec![0u8;100];
                    let mut bytesneeded2 = buffer2.len() as u32;
                    let mut data = vec![0u8;100];
                    let mut datalength = data.len() as u32;
                    let mut type1 = 0;
                    let t2 = unsafe{RegEnumValueA(std::mem::transmute(keyhandle),
                                                 j,
                                                 buffer2.as_mut_ptr() as *mut i8,&mut bytesneeded2,
                                                 std::ptr::null_mut(),
                                                  &mut type1,data.as_mut_ptr() as *mut u8,
                                                  &mut datalength)};

                    if t2 ==ERROR_NO_MORE_ITEMS as i32{
                        break;
                    }
                    if t2 as i32==ERROR_SUCCESS as i32{
                        let valuename = String::from_utf8_lossy(&buffer2).trim_end_matches("\0").to_string();


                        if valuename.to_lowercase().contains("processornamestring"){
                            //println!("value name: {}",valuename);

                            //let valuecontent = String::from_utf8_lossy(&data).trim_end_matches("\0").to_string();
                            let valuecontent = ReadStringFromMemory(unsafe{GetCurrentProcess()},data.as_mut_ptr()  as *const c_void);

                            unsafe{RegCloseKey(std::mem::transmute(keyhandle))};
                            unsafe{RegCloseKey(std::mem::transmute(reghandle))};
                            return Ok(valuecontent);
                            break 'outerloop;
                            //println!("{:?}",data);
                        }

                    }

                    j +=1;
                    unsafe{RegCloseKey(std::mem::transmute(keyhandle))};


                }




            }
            i += 1;
        }


        unsafe{RegCloseKey(std::mem::transmute(reghandle))};
    }

    Err(format!("RegOpenKeyExA failed: {}",unsafe{GetLastError()}))
}


pub fn test(){

    let keystring = "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\0";
    let key = "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\0".bytes().collect::<Vec<u8>>();
    let mut keyhandle = 0 as *mut c_void;
    let res = unsafe{RegOpenKeyExA(HKEY_LOCAL_MACHINE,
    key.as_ptr() as *const i8,
    0,KEY_READ|KEY_EXECUTE,
                                   std::mem::transmute(&mut keyhandle))};

    if res!=ERROR_SUCCESS as i32{
        println!("RegOpenKeyExA failed: {}",res);
        return ();
    }

    let mut i = 0;
    'outerloop: loop{
        let mut subkeynamebuffer =vec![0u8;1024];
        let mut subkeynamebufferlength = subkeynamebuffer.len() as u32;
        let mut classnamebuffer =vec![0u8;1024];
        let mut classnamebufferlength = classnamebuffer.len() as u32;
        let res2 = unsafe{RegEnumKeyExA(std::mem::transmute( keyhandle),
                                        i,
                                        subkeynamebuffer.as_mut_ptr() as *mut i8,&mut subkeynamebufferlength,
                                        std::ptr::null_mut(),classnamebuffer.as_mut_ptr() as *mut i8,
                                        &mut classnamebufferlength,std::ptr::null_mut())};

        //println!("res2: {}",res2);
        if res2==ERROR_NO_MORE_ITEMS as i32{
            break;
        }
        if res2==ERROR_SUCCESS as i32 {
            let subkeyname = ReadStringFromMemory(unsafe{GetCurrentProcess()},subkeynamebuffer.as_ptr() as *const c_void);
            println!("subkeyname: {}",subkeyname);



            let valuekey = keystring.trim_end_matches("\0").to_string()+"\\"+&subkeyname+"\0";
            println!("valuekey: {}",valuekey);


            let mut subkeyhandle = 0 as *mut c_void;
            let res = unsafe{RegOpenKeyExA(HKEY_LOCAL_MACHINE,
                                           valuekey.as_ptr() as *const i8,
                                           0,KEY_READ|KEY_EXECUTE,
                                           std::mem::transmute(&mut subkeyhandle))};


            let mut j = 0;
            'innerloop: loop{
                let mut valuenamebuffer = vec![0u8;1024];
                let mut valuenamebufferlength = valuenamebuffer.len() as u32;
                let mut valuecontentbuffer = vec![0u8;2048];
                let mut valuecontentbufferlength = valuecontentbuffer.len() as u32;
                let mut valuetype = 0;
                let res3 = unsafe{RegEnumValueA(std::mem::transmute( subkeyhandle),
                                                j,valuenamebuffer.as_mut_ptr() as *mut i8,
                                                &mut valuenamebufferlength,
                                                std::ptr::null_mut(),&mut valuetype,
                                                valuecontentbuffer.as_mut_ptr() as *mut u8,
                                                &mut valuecontentbufferlength)};

                if res3==ERROR_NO_MORE_ITEMS as i32{
                    break 'innerloop;
                }
                if res3==ERROR_SUCCESS as i32{
                    if valuetype==REG_SZ{
                        let valuename = ReadStringFromMemory(unsafe{GetCurrentProcess()},valuenamebuffer.as_ptr() as *const c_void);
                        let valuecontent = ReadStringFromMemory(unsafe{GetCurrentProcess()},valuecontentbuffer.as_ptr() as *const c_void);

                        println!("{} : {}",valuename,valuecontent);
                    }


                }
                j +=1;
            }


            unsafe{RegCloseKey(std::mem::transmute(subkeyhandle))};

        }



        i +=1;

    }


    unsafe{RegCloseKey( std::mem::transmute(&mut keyhandle))};


}


