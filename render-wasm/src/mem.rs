use std::alloc::{alloc, Layout};

pub const LAYOUT_ALIGN: usize = 4;

// Estado global directo - MÁXIMA VELOCIDAD
static mut BUFFERU8: Vec<u8> = Vec::new();
static mut BUFFER_ERROR: u8 = 0;

pub fn alloc_bytes(len: usize) -> *mut u8  {
    unsafe {
        let layout = Layout::from_size_align_unchecked(len, LAYOUT_ALIGN);
        let ptr = alloc(layout);
        if ptr.is_null() {
            return std::ptr::null_mut();
        }
        std::ptr::write_bytes(ptr, 0, len);
        Vec::from_raw_parts(ptr, len, len);
    }
    std::ptr::null_mut()
}

#[inline(always)]
pub fn clear_error_code() {
    unsafe {
        BUFFER_ERROR = 0;
    }
}

#[inline(always)]
pub fn set_error_code(code: u8) {
    unsafe {
        BUFFER_ERROR = code;
    }
}

#[inline(always)]
pub fn read_error_code() -> u8 {
    unsafe  {
        BUFFER_ERROR
    }
}

pub fn write_bytes(mut bytes: Vec<u8>) -> *mut u8 {
    let ptr = bytes.as_mut_ptr();
    unsafe {
        BUFFERU8 = bytes;
    }
    ptr
}

pub fn bytes() -> Vec<u8> {
    unsafe {
        std::mem::take(&mut BUFFERU8)
    }
}

pub fn bytes_or_empty() -> Vec<u8> {
    unsafe {
        if BUFFERU8.is_empty() {
            Vec::new()
        } else {
            std::mem::take(&mut BUFFERU8)
        }
    }
}

pub fn free_bytes() {
    unsafe {
        BUFFERU8 = Vec::new();
    }
}

// Para el trait SerializableResult
pub trait SerializableResult: From<Self::BytesType> + Into<Self::BytesType> {
    type BytesType;
    fn clone_to_slice(&self, slice: &mut [u8]);
}

pub fn write_vec<T: SerializableResult>(result: Vec<T>) -> *mut u8 {
    let elem_size = std::mem::size_of::<T::BytesType>();
    let bytes_len = 4 + result.len() * elem_size;
    let mut result_bytes = vec![0; bytes_len];

    result_bytes[0..4].clone_from_slice(&result.len().to_le_bytes());

    for (i, item) in result.iter().enumerate() {
        let base = 4 + i * elem_size;
        item.clone_to_slice(&mut result_bytes[base..base + elem_size]);
    }

    write_bytes(result_bytes)
}
