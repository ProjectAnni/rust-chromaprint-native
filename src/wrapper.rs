use bindings::*;
use std::ffi::CStr;

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
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { chromaprint_dealloc(self.c_ctx) }
    }
}

pub fn version() -> &'static str {
    unsafe { CStr::from_ptr(chromaprint_get_version()).to_str().unwrap() }
}
