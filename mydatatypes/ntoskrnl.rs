



#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct ListEntry{
    flink: u64,
    blink: u64
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct PsLoadedModuleList{
    list:ListEntry
}