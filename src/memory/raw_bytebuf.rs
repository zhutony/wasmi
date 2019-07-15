//! An implementation of `ByteBuf` based on a plain `Vec`.

use alloc::vec::Vec;
use std::{
    slice,
    mem,
};
use super::MemoryBackend;

pub struct RawByteBuf {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

impl RawByteBuf {
    pub fn from_raw_parts(ptr: *mut u8, len: usize, cap: usize) -> Self {
        Self {
            ptr,
            len,
            cap,
        }
    }

    pub fn new(len: usize) -> Result<Self, &'static str> {
        let mut v = vec![0u8; len];
        let cap = len;
        let ptr = v.as_mut_ptr();
        mem::forget(v);

        Ok(Self {
            ptr,
            len,
            cap,
        })
    }
}

impl MemoryBackend for RawByteBuf {

    pub fn realloc(&mut self, new_len: usize) -> Result<(), &'static str> {
        if new_len > self.cap {
            return Err("exceeds cap");
        }
        self.len = new_len;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr, self.len)
        }
    }

    pub fn erase(&mut self) -> Result<(), &'static str> {
        for v in self.as_slice_mut() {
            *v = 0;
        }
        Ok(())
    }
}
