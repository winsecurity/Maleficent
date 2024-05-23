use winapi::ctypes::*;
use winapi::um::errhandlingapi::*;
use winapi::um::securitybaseapi::*;
use winapi::shared::ntstatus::*;
use winapi::shared::winerror::*;
use winapi::um::processthreadsapi::*;
use winapi::um::winnt::*;



pub trait tokentrait{}


pub struct TOKEN_GROUP{
    pub groupcount: u32,
    pub groups: Vec<SID_AND_ATTRIBUTES>
}





impl tokentrait for TOKEN_GROUP{

}
impl tokentrait for TOKEN_USER{

}