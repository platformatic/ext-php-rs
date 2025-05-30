use crate::ffi::{
    sapi_header_struct,
    uid_t,
    gid_t,
    php_default_post_reader,
    php_default_treat_data,
    php_default_input_filter,
    ext_php_rs_php_error
};
use crate::types::Zval;
use crate::{embed::SapiModule, error::Result};

use std::ffi::{c_char, c_int, c_void};
use std::{ffi::CString, ptr};

/// Builds a Sapi module to run PHP.
///
/// ```rust,no_run
/// use std::ffi::{c_char, c_int, c_void};
/// use ext_php_rs::{
///     builders::SapiBuilder,
///     ffi::sapi_header_struct
/// };
///
/// #[no_mangle]
/// pub extern "C" fn ub_write(str: *const i8, str_length: usize) -> usize {
///     println!("PHP wrote: {:?}", str);
///     str_length
/// }
///
/// #[no_mangle]
/// pub extern "C" fn send_header(header: *mut sapi_header_struct, server_context: *mut c_void) {
///     println!("PHP sent a header: {:?}", header);
/// }
///
/// let sapi = SapiBuilder::new("ext_php_rs", "Ext PHP RS")
///   .ub_write_function(ub_write)
///   .send_header_function(send_header)
///   .build();
/// ```
pub struct SapiBuilder {
    name: String,
    pretty_name: String,
    module: SapiModule,
    executable_location: Option<String>
}

impl SapiBuilder {
    /// Creates a new SapiBuilder
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the SAPI module.
    pub fn new<T: Into<String>, U: Into<String>>(name: T, pretty_name: U) -> Self {
        Self {
            name: name.into(),
            pretty_name: pretty_name.into(),
            module: SapiModule {
                name: ptr::null_mut(),
                pretty_name: ptr::null_mut(),
                startup: None,
                shutdown: None,
                activate: None,
                deactivate: None,
                ub_write: None,
                flush: None,
                get_stat: None,
                getenv: None,
                sapi_error: Some(ext_php_rs_php_error),
                header_handler: None,
                send_headers: None,
                send_header: None,
                read_post: None,
                read_cookies: None,
                register_server_variables: None,
                log_message: None,
                get_request_time: None,
                terminate_process: None,
                php_ini_path_override: ptr::null_mut(),
                default_post_reader: Some(php_default_post_reader),
                treat_data: Some(php_default_treat_data),
                // treat_data: None,
                executable_location: ptr::null_mut(),
                php_ini_ignore: 0,
                php_ini_ignore_cwd: 0,
                get_fd: None,
                force_http_10: None,
                get_target_uid: None,
                get_target_gid: None,
                input_filter: Some(php_default_input_filter),
                // input_filter: None,
                ini_defaults: None,
                phpinfo_as_text: 0,
                ini_entries: ptr::null_mut(),
                additional_functions: ptr::null(),
                input_filter_init: None,
            },
            executable_location: None
        }
    }

    /// Sets the startup function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called on startup.
    pub fn startup_function(mut self, func: SapiStartupFunc) -> Self {
        self.module.startup = Some(func);
        self
    }

    /// Sets the shutdown function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called on shutdown.
    pub fn shutdown_function(mut self, func: SapiShutdownFunc) -> Self {
        self.module.shutdown = Some(func);
        self
    }

    /// Sets the activate function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called on activation.
    pub fn activate_function(mut self, func: SapiActivateFunc) -> Self {
        self.module.activate = Some(func);
        self
    }

    /// Sets the deactivate function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called on deactivation.
    pub fn deactivate_function(mut self, func: SapiDeactivateFunc) -> Self {
        self.module.deactivate = Some(func);
        self
    }

    /// Sets the write function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called on write.
    pub fn ub_write_function(mut self, func: SapiUbWriteFunc) -> Self {
        self.module.ub_write = Some(func);
        self
    }

    /// Set the flush function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called on flush.
    pub fn flush_function(mut self, func: SapiFlushFunc) -> Self {
        self.module.flush = Some(func);
        self
    }

    /// Sets the get env function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP gets an environment variable.
    pub fn getenv_function(mut self, func: SapiGetEnvFunc) -> Self {
        self.module.getenv = Some(func);
        self
    }

    /// Sets the sapi error function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP encounters an error.
    // pub fn sapi_error_function(mut self, func: SapiErrorFunc) -> Self {
    //     self.module.sapi_error = Some(func);
    //     self
    // }

    // TODO: Implement header_handler and send_headers

    /// Sets the send header function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP sends a header line.
    pub fn send_header_function(mut self, func: SapiSendHeaderFunc) -> Self {
        self.module.send_header = Some(func);
        self
    }

    /// Sets the read post function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP reads the POST data.
    pub fn read_post_function(mut self, func: SapiReadPostFunc) -> Self {
        self.module.read_post = Some(func);
        self
    }

    /// Sets the read cookies function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP reads the cookies.
    pub fn read_cookies_function(mut self, func: SapiReadCookiesFunc) -> Self {
        self.module.read_cookies = Some(func);
        self
    }

    /// Sets the register server variables function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP registers server variables.
    pub fn register_server_variables_function(mut self, func: SapiRegisterServerVariablesFunc) -> Self {
        self.module.register_server_variables = Some(func);
        self
    }

    /// Sets the log message function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP logs a message.
    pub fn log_message_function(mut self, func: SapiLogMessageFunc) -> Self {
        self.module.log_message = Some(func);
        self
    }

    /// Sets the request time function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP gets the request time.
    pub fn get_request_time_function(mut self, func: SapiRequestTimeFunc) -> Self {
        self.module.get_request_time = Some(func);
        self
    }

    /// Sets the terminate process function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP terminates the process.
    pub fn terminate_process_function(mut self, func: SapiTerminateProcessFunc) -> Self {
        self.module.terminate_process = Some(func);
        self
    }

    /// Sets the get uid function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP gets the uid.
    pub fn get_target_uid_function(mut self, func: SapiGetUidFunc) -> Self {
        self.module.get_target_uid = Some(func);
        self
    }

    /// Sets the get gid function for this SAPI
    ///
    /// # Parameters
    ///
    /// * `func` - The function to be called when PHP gets the gid.
    pub fn get_target_gid_function(mut self, func: SapiGetGidFunc) -> Self {
        self.module.get_target_gid = Some(func);
        self
    }

    /// Sets the php ini path override for this SAPI
    ///
    /// # Parameters
    ///
    /// * `path` - The path to the php ini file.
    pub fn php_ini_path_override(mut self, path: &str) -> Self {
        self.module.php_ini_path_override = CString::new(path).unwrap().into_raw();
        self
    }

    /// Sets the php ini ignore for this SAPI
    ///
    /// # Parameters
    ///
    /// * `ignore` - The value to set php ini ignore to.
    pub fn php_ini_ignore(mut self, ignore: c_int) -> Self {
        self.module.php_ini_ignore = ignore;
        self
    }

    /// Sets the php ini ignore cwd for this SAPI
    ///
    /// # Parameters
    ///
    /// * `ignore` - The value to set php ini ignore cwd to.
    pub fn php_ini_ignore_cwd(mut self, ignore: c_int) -> Self {
        self.module.php_ini_ignore_cwd = ignore;
        self
    }

    /// Sets the executable location for this SAPI
    ///
    /// # Parameters
    ///
    /// * `location` - The location of the executable.
    pub fn executable_location(mut self, location: &str) -> Self {
        self.executable_location = Some(location.to_string());
        self
    }

    /// Builds the extension and returns a `SapiModule`.
    ///
    /// Returns a result containing the sapi module if successful.
    pub fn build(mut self) -> Result<SapiModule> {
        self.module.name = CString::new(self.name)?.into_raw();
        self.module.pretty_name = CString::new(self.pretty_name)?.into_raw();

        self.module.executable_location = maybe_cstr(self.executable_location)?;

        if self.module.send_header.is_none() {
            self.module.send_header = Some(dummy_send_header);
        }

        Ok(self.module)
    }
}

fn maybe_cstr<T>(m: Option<T>) -> Result<*mut c_char>
where
  T: Into<Vec<u8>>
{
  Ok(match m {
    None => std::ptr::null_mut(),
    Some(s) => CString::new(s)?.into_raw()
  })
}

/// A function to be called when PHP starts the SAPI
pub type SapiStartupFunc = extern "C" fn(sapi: *mut SapiModule) -> c_int;

/// A function to be called when PHP stops the SAPI
pub type SapiShutdownFunc = extern "C" fn(sapi: *mut SapiModule) -> c_int;

/// A function to be called when PHP activates the SAPI
pub type SapiActivateFunc = extern "C" fn() -> c_int;

/// A function to be called when PHP deactivates the SAPI
pub type SapiDeactivateFunc = extern "C" fn() -> c_int;

/// A function to be called when PHP write to the output buffer
pub type SapiUbWriteFunc = extern "C" fn(str: *const c_char, str_length: usize) -> usize;

/// A function to be called when PHP flush the output buffer
pub type SapiFlushFunc = extern "C" fn(*mut c_void);

/// A function to be called when PHP gets an environment variable
pub type SapiGetEnvFunc = extern "C" fn(name: *const c_char, name_length: usize) -> *mut c_char;

/// A function to be called when PHP encounters an error
/// TODO: Figure out variadic functions
// pub type SapiErrorFunc = extern "C" fn(type: c_int, error_msg: *const c_char, args: va_list);

/// A function to be called when PHP read the POST data
pub type SapiReadPostFunc = extern "C" fn(buffer: *mut c_char, length: usize) -> usize;

/// A function to be called when PHP read the cookies
pub type SapiReadCookiesFunc = extern "C" fn() -> *mut c_char;

/// A function to be called when PHP send a header
pub type SapiSendHeaderFunc =
    extern "C" fn(header: *mut sapi_header_struct, server_context: *mut c_void);

/// A function to be called when PHP register server variables
pub type SapiRegisterServerVariablesFunc = extern "C" fn(vars: *mut Zval);

/// A function to be called when PHP logs a message
pub type SapiLogMessageFunc = extern "C" fn(message: *const c_char, syslog_type_int: c_int);

/// A function to be called when PHP gets the request time
pub type SapiRequestTimeFunc = extern "C" fn(time: *mut f64) -> c_int;

/// A function to be called when PHP terminates the process
pub type SapiTerminateProcessFunc = extern "C" fn();

/// A function to be called when PHP gets the uid
pub type SapiGetUidFunc = extern "C" fn(uid: *mut uid_t) -> c_int;

/// A function to be called when PHP gets the gid
pub type SapiGetGidFunc = extern "C" fn(gid: *mut gid_t) -> c_int;

extern "C" fn dummy_send_header(_header: *mut sapi_header_struct, _server_context: *mut c_void) {}
