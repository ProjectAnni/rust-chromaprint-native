use bindings::*;
use std::ffi::{CStr, CString};

use std::os::raw::c_char;
use std::ptr;

pub struct Context {
    c_ctx: *mut ChromaprintContext,
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            Context {
                c_ctx: chromaprint_new(CHROMAPRINT_ALGORITHM_DEFAULT),
            }
        }
    }

    pub fn start(&mut self, sample_rate: i32, num_channels: i32) -> bool {
        unsafe { chromaprint_start(self.c_ctx, sample_rate, num_channels) == 0 }
    }

    pub fn feed(&mut self, data: &[i16]) -> bool {
        unsafe { chromaprint_feed(self.c_ctx, data.as_ptr(), data.len() as i32) == 0 }
    }

    pub fn finish(&mut self) -> bool {
        unsafe { chromaprint_finish(self.c_ctx) == 0 }
    }

    pub fn fingerprint(&mut self) -> String {
        unsafe {
            let mut result = ptr::null_mut::<c_char>();
            chromaprint_get_fingerprint(self.c_ctx, &mut result);

            CString::from_raw(result).into_string().unwrap()
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { chromaprint_free(self.c_ctx) }
    }
}

pub fn version() -> &'static str {
    unsafe { CStr::from_ptr(chromaprint_get_version()).to_str().unwrap() }
}
