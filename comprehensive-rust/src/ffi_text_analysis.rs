// src/ffi_text_analysis
use std::ffi::c_char;
use std::os::raw::c_void;

#[repr(C)]
pub struct TextAnalyst {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub start: *const c_char,
    pub length: usize,
    pub index: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TAError {
    Ok = 0,
    NullPointer = 1,
    OutOfMemory = 2,
    Other = 3,
}

pub type Tokenizer =
    Option<unsafe extern "C" fn(token: *mut Token, extra_context: *mut c_void) -> bool>;
pub type TokenCallback = Option<
    unsafe extern "C" fn(user_context: *mut c_void, token: *mut Token, result: *mut c_void) -> bool,
>;

unsafe extern "C" {
    pub fn ta_new() -> *mut TextAnalyst;
    pub fn ta_free(ta: *mut TextAnalyst);
    pub fn ta_reset(ta: *mut TextAnalyst);
    pub fn ta_set_tokenizer(ta: *mut TextAnalyst, func: *const Tokenizer);
    pub fn ta_set_text(
        ta: *mut TextAnalyst,
        text: *const c_char,
        len: usize,
        make_copy: bool,
    ) -> TAError;
    pub fn ta_foreach_token(
        ta: *const TextAnalyst,
        callback: *const TokenCallback,
        user_context: *mut c_void,
    ) -> usize;
    pub fn ta_error_string(error: TAError) -> *const c_char;
}
