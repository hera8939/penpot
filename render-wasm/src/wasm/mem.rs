#[allow(unused_imports)]
use crate::error::{Error, Result};

#[no_mangle]
pub extern "C" fn alloc_bytes(len: usize) -> Result<*mut u8> {
    Ok(crate::mem::alloc_bytes(len))
}

#[no_mangle]
pub extern "C" fn free_bytes() -> Result<()> {
    crate::mem::free_bytes();
    Ok(())
}
