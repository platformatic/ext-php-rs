//! Types and functions used for throwing exceptions from Rust to PHP.

use std::{ffi::{c_int, c_uint, CString}, fmt::Debug};

use crate::{
    class::RegisteredClass,
    error::{Error, Result},
    ffi::{zend_throw_exception_ex, zend_throw_exception_object},
    flags::ClassFlags,
    types::{ZendStr, Zval},
    zend::{ce, ClassEntry},
};

/// Result type with the error variant as a [`PhpException`].
pub type PhpResult<T = ()> = std::result::Result<T, PhpException>;

/// Represents a PHP exception which can be thrown using the `throw()` function.
/// Primarily used to return from a [`Result<T, PhpException>`] which can
/// immediately be thrown by the `ext-php-rs` macro API.
///
/// There are default [`From`] implementations for any type that implements
/// [`ToString`], so these can also be returned from these functions. You can
/// also implement [`From<T>`] for your custom error type.
#[derive(Debug)]
pub struct PhpException {
    message: String,
    code: i32,
    ex: &'static ClassEntry,
    object: Option<Zval>,
}

impl PhpException {
    /// Creates a new exception instance.
    ///
    /// # Parameters
    ///
    /// * `message` - Message to contain in the exception.
    /// * `code` - Integer code to go inside the exception.
    /// * `ex` - Exception type to throw.
    pub fn new(message: String, code: i32, ex: &'static ClassEntry) -> Self {
        Self {
            message,
            code,
            ex,
            object: None,
        }
    }

    /// Creates a new default exception instance, using the default PHP
    /// `Exception` type as the exception type, with an integer code of
    /// zero.
    ///
    /// # Parameters
    ///
    /// * `message` - Message to contain in the exception.
    pub fn default(message: String) -> Self {
        Self::new(message, 0, ce::exception())
    }

    /// Creates an instance of an exception from a PHP class type and a message.
    ///
    /// # Parameters
    ///
    /// * `message` - Message to contain in the exception.
    pub fn from_class<T: RegisteredClass>(message: String) -> Self {
        Self::new(message, 0, T::get_metadata().ce())
    }

    /// Set the Zval object for the exception.
    ///
    /// Exceptions can be based of instantiated Zval objects when you are
    /// throwing a custom exception with stateful properties.
    ///
    /// # Parameters
    ///
    /// * `object` - The Zval object.
    pub fn set_object(&mut self, object: Option<Zval>) {
        self.object = object;
    }

    /// Throws the exception, returning nothing inside a result if successful
    /// and an error otherwise.
    pub fn throw(self) -> Result<()> {
        match self.object {
            Some(object) => throw_object(object),
            None => throw_with_code(self.ex, self.code, &self.message),
        }
    }
}

impl From<PhpException> for String {
    fn from(ex: PhpException) -> Self {
        ex.message
    }
}

impl From<String> for PhpException {
    fn from(str: String) -> Self {
        Self::default(str)
    }
}

impl From<&str> for PhpException {
    fn from(str: &str) -> Self {
        Self::default(str.into())
    }
}

#[cfg(feature = "anyhow")]
impl From<anyhow::Error> for PhpException {
    fn from(err: anyhow::Error) -> Self {
        Self::new(format!("{:#}", err), 0, crate::zend::ce::exception())
    }
}

/// Throws an exception with a given message. See [`ClassEntry`] for some
/// built-in exception types.
///
/// Returns a result containing nothing if the exception was successfully
/// thrown.
///
/// # Parameters
///
/// * `ex` - The exception type to throw.
/// * `message` - The message to display when throwing the exception.
///
/// # Examples
///
/// ```no_run
/// use ext_php_rs::{zend::{ce, ClassEntry}, exception::throw};
///
/// throw(ce::compile_error(), "This is a CompileError.");
/// ```
pub fn throw(ex: &ClassEntry, message: &str) -> Result<()> {
    throw_with_code(ex, 0, message)
}

/// Throws an exception with a given message and status code. See [`ClassEntry`]
/// for some built-in exception types.
///
/// Returns a result containing nothing if the exception was successfully
/// thrown.
///
/// # Parameters
///
/// * `ex` - The exception type to throw.
/// * `code` - The status code to use when throwing the exception.
/// * `message` - The message to display when throwing the exception.
///
/// # Examples
///
/// ```no_run
/// use ext_php_rs::{zend::{ce, ClassEntry}, exception::throw_with_code};
///
/// throw_with_code(ce::compile_error(), 123, "This is a CompileError.");
/// ```
pub fn throw_with_code(ex: &ClassEntry, code: i32, message: &str) -> Result<()> {
    let flags = ex.flags();

    // Can't throw an interface or abstract class.
    if flags.contains(ClassFlags::Interface) || flags.contains(ClassFlags::Abstract) {
        return Err(Error::InvalidException(flags));
    }

    // SAFETY: We are given a reference to a `ClassEntry` therefore when we cast it
    // to a pointer it will be valid.
    unsafe {
        zend_throw_exception_ex(
            (ex as *const _) as *mut _,
            code as _,
            CString::new("%s")?.as_ptr(),
            CString::new(message)?.as_ptr(),
        )
    };
    Ok(())
}

/// Throws an exception object.
///
/// Returns a result containing nothing if the exception was successfully
/// thrown.
///
/// # Parameters
///
/// * `object` - The zval of type object
///
/// # Examples
///
/// ```no_run
/// use ext_php_rs::prelude::*;
/// use ext_php_rs::exception::throw_object;
/// use crate::ext_php_rs::convert::IntoZval;
///
/// #[php_class]
/// #[extends(ext_php_rs::zend::ce::exception)]
/// pub struct JsException {
///     #[prop(flags = ext_php_rs::flags::PropertyFlags::Public)]
///     message: String,
///     #[prop(flags = ext_php_rs::flags::PropertyFlags::Public)]
///     code: i32,
///     #[prop(flags = ext_php_rs::flags::PropertyFlags::Public)]
///     file: String,
/// }
///
/// #[php_module]
/// pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
///     module
/// }
///
/// let error = JsException { message: "A JS error occurred.".to_string(), code: 100, file: "index.js".to_string() };
/// throw_object( error.into_zval(true).unwrap() );
/// ```
pub fn throw_object(zval: Zval) -> Result<()> {
    let mut zv = core::mem::ManuallyDrop::new(zval);
    unsafe { zend_throw_exception_object(core::ptr::addr_of_mut!(zv).cast()) };
    Ok(())
}

use std::sync::RwLock;

static has_observer: RwLock<bool> = RwLock::new(false);
static error_observers: RwLock<Vec<Box<
    dyn Fn(i32, &mut ZendStr, usize, &mut ZendStr) + Send + Sync
>>> = RwLock::new(Vec::new());

/// Register an error observer.
///
/// # Arguments
///
/// * `observer` - The error observer to register.
///
/// # Example
///
/// ```
/// use ext_php_rs::exception::register_error_observer;
///
/// register_error_observer(|error_type, filename, line, message| {
///     // Handle the error
/// });
/// ```
pub fn register_error_observer<F>(observer: F)
where
    F: Fn(i32, &mut ZendStr, usize, &mut ZendStr) + Send + Sync + 'static,
{
    {
        if !*has_observer.read().unwrap() {
            let mut w = has_observer.write()
                .expect("should acquire write lock for has_observer");
            *w = true;

            unsafe {
                crate::ffi::zend_observer_error_register(Some(error_observer_dispatcher));
            }
        }
    }

    {
        let mut w = error_observers.write()
            .expect("should acquire write lock for error_observers");
        w.push(Box::new(observer));
    }
}

#[no_mangle]
extern "C" fn error_observer_dispatcher(
    error_type: c_int,
    filename: *mut ZendStr,
    line: c_uint,
    message: *mut ZendStr
) {
    let observers = error_observers.read()
        .expect("should acquire read lock for error_observers");

    if observers.is_empty() {
        return;
    }

    let file = unsafe { &mut *filename };
    let message = unsafe { &mut *message };

    let line = line as usize;

    for observer in observers.iter() {
        observer(error_type, file, line, message);
    }
}
