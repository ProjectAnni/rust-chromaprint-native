use bindings::*;
use errors::{handle_return, Result};
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

    pub fn start(&mut self, sample_rate: i32, num_channels: i32) -> Result<()> {
        handle_return(unsafe { chromaprint_start(self.c_ctx, sample_rate, num_channels) })
    }

    pub fn feed(&mut self, data: &[i16]) -> Result<()> {
        handle_return(unsafe { chromaprint_feed(self.c_ctx, data.as_ptr(), data.len() as i32) })
    }

    pub fn finish(&mut self) -> Result<()> {
        handle_return(unsafe { chromaprint_finish(self.c_ctx) })
    }

    pub fn fingerprint(&mut self) -> Result<String> {
        unsafe {
            let mut result = ptr::null_mut::<c_char>();
            handle_return(chromaprint_get_fingerprint(self.c_ctx, &mut result))?;

            Ok(CString::from_raw(result).into_string().unwrap())
        }
    }

    pub fn clear_fingerprint(&mut self) -> Result<()> {
        handle_return(unsafe { chromaprint_clear_fingerprint(self.c_ctx) })
    }

    #[allow(dead_code)]
    fn raw_fingerprint_size(&mut self) -> Result<i32> {
        unsafe {
            let mut result = 0;
            handle_return(chromaprint_get_raw_fingerprint_size(
                self.c_ctx,
                &mut result,
            ))?;

            Ok(result)
        }
    }

    pub fn raw_fingerprint(&mut self) -> Result<Vec<u32>> {
        unsafe {
            let mut size = 0;
            let mut result = ptr::null_mut();
            handle_return(chromaprint_get_raw_fingerprint(
                self.c_ctx,
                &mut result,
                &mut size,
            ))?;

            Ok(Vec::from_raw_parts(result, size as usize, size as usize))
        }
    }

    pub fn fingerprint_hash(&mut self) -> Result<u32> {
        unsafe {
            let mut result = 0u32;
            handle_return(chromaprint_get_fingerprint_hash(self.c_ctx, &mut result))?;

            Ok(result)
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

pub fn hash_fingerprint(fingerprint: &[u32]) -> Result<u32> {
    unsafe {
        let mut result = 0u32;
        handle_return(chromaprint_hash_fingerprint(
            fingerprint.as_ptr(),
            fingerprint.len() as i32,
            &mut result,
        ))?;

        Ok(result)
    }
}

pub fn encode_fingerprint_buffer(fingerprint: &[u32], algorithm: i32) -> Result<Vec<u32>> {
    let mut result = ptr::null_mut();
    let mut result_size = 0;
    unsafe {
        handle_return(chromaprint_encode_fingerprint(
            fingerprint.as_ptr(),
            fingerprint.len() as i32,
            algorithm,
            &mut result,
            &mut result_size,
            0,
        ))?;

        Ok(Vec::from_raw_parts(result as *mut u32, result_size as usize, result_size as usize))
    }
}

pub fn encode_fingerprint_base64(fingerprint: &[u32], algorithm: i32) -> Result<String> {
    let mut result = ptr::null_mut();
    let mut result_size = 0;
    unsafe {
        handle_return(chromaprint_encode_fingerprint(
            fingerprint.as_ptr(),
            fingerprint.len() as i32,
            algorithm,
            &mut result,
            &mut result_size,
            1,
        ))?;

        Ok(String::from_raw_parts(result as *mut u8, result_size as usize, result_size as usize))
    }
}

pub fn decode_fingerprint(fingerprint: String) -> Result<(Vec<u32>, i32)> {
    let mut result = ptr::null_mut();
    let mut result_size = 0;
    let mut algorithm = 0;
    return unsafe {
        handle_return(chromaprint_decode_fingerprint(
            fingerprint.as_ptr() as *const c_char,
            fingerprint.len() as i32,
            &mut result,
            &mut result_size,
            &mut algorithm,
            1,
        ))?;
        let fingerprint = Vec::from_raw_parts(result as *mut u32, result_size as usize, result_size as usize);
        Ok((fingerprint, algorithm as i32))
    };
}