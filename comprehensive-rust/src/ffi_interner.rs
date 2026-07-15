use std::marker::PhantomData;
use std::os::raw::c_char;

#[repr(C)]
pub struct StringInternerRaw {
    _opaque: [u8; 0],
    _pin: PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}

unsafe extern "C" {
    pub fn interner_new() -> *mut StringInternerRaw;
    pub fn interner_free(interner: *mut StringInternerRaw);
    pub fn interner_intern(interner: *mut StringInternerRaw, s: *const c_char) -> *const c_char;
    pub fn interner_count(interner: *const StringInternerRaw) -> usize;
}
