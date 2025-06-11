//! Functions relating to the Zend Memory Manager, used to allocate
//! request-bound memory.

use crate::{ffi::{_efree, _emalloc, _estrdup}};
use std::{alloc::Layout, ffi::{c_char, c_void, CString}};

/// Uses the PHP memory allocator to allocate request-bound memory.
///
/// # Parameters
///
/// * `layout` - The layout of the requested memory.
///
/// # Returns
///
/// A pointer to the memory allocated.
pub fn emalloc(layout: Layout) -> *mut u8 {
    // TODO account for alignment
    let size = layout.size();

    (unsafe {
        #[cfg(php_debug)]
        {
            _emalloc(size as _, std::ptr::null_mut(), 0, std::ptr::null_mut(), 0)
        }
        #[cfg(not(php_debug))]
        {
            _emalloc(size as _)
        }
    }) as *mut u8
}

/// Frees a given memory pointer which was allocated through the PHP memory
/// manager.
///
/// # Parameters
///
/// * `ptr` - The pointer to the memory to free.
///
/// # Safety
///
/// Caller must guarantee that the given pointer is valid (aligned and non-null)
/// and was originally allocated through the Zend memory manager.
pub unsafe fn efree(ptr: *mut u8) {
    #[cfg(php_debug)]
    {
        _efree(
            ptr as *mut c_void,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
            0,
        )
    }
    #[cfg(not(php_debug))]
    {
        _efree(ptr as *mut c_void)
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn estrdup(string: impl Into<Vec<u8>>) -> *mut c_char {
    let string = CString::from_vec_unchecked(string.into()).into_raw();

    #[cfg(php_debug)]
    let result = _estrdup(
        string,
        std::ptr::null_mut(),
        0,
        std::ptr::null_mut(),
        0,
    );

    #[cfg(not(php_debug))]
    let result = _estrdup(string);

    drop(unsafe { CString::from_raw(string) });
    result as *mut c_char
}
