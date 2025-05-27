use std::ffi::{c_char, CStr, CString};
use crate::ffi::{
    php_ini_builder,
    php_ini_builder_prepend,
    php_ini_builder_unquoted,
    php_ini_builder_quoted,
    php_ini_builder_define,
};

/// A builder for creating INI configurations.
pub type IniBuilder = php_ini_builder;

impl IniBuilder {
    /// Creates a new INI builder.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ext_php_rs::builders::IniBuilder;
    /// let mut builder = IniBuilder::new();
    /// ```
    pub fn new() -> IniBuilder {
         IniBuilder {
            value: std::ptr::null_mut(),
            length: 0,
        }
    }

    /// Appends a value to the INI builder.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to append.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ext_php_rs::builders::IniBuilder;
    /// let mut builder = IniBuilder::new();
    /// builder.prepend("foo=bar");
    /// ```
    pub fn prepend<V: AsRef<str>>(&mut self, value: V) {
        let value = value.as_ref();
        let c_value = CString::new(value).unwrap();
        unsafe {
            php_ini_builder_prepend(self, c_value.into_raw(), value.len());
        }
    }

    /// Appends an unquoted name-value pair to the INI builder.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the pair.
    /// * `value` - The value of the pair.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ext_php_rs::builders::IniBuilder;
    /// let mut builder = IniBuilder::new();
    /// builder.unquoted("foo", "bar");
    /// ```
    pub fn unquoted<N, V>(&mut self, name: N, value: V)
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        let name = name.as_ref();
        let value = value.as_ref();
        let c_name = CString::new(name).unwrap();
        let c_value = CString::new(value).unwrap();
        unsafe {
            php_ini_builder_unquoted(self, c_name.into_raw(), name.len(), c_value.into_raw(), value.len());
        }
    }

    /// Appends a quoted name-value pair to the INI builder.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the pair.
    /// * `value` - The value of the pair.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ext_php_rs::builders::IniBuilder;
    /// let mut builder = IniBuilder::new();
    /// builder.quoted("foo", "bar");
    /// ```
    pub fn quoted<N, V>(&mut self, name: N, value: V)
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        let name = name.as_ref();
        let value = value.as_ref();
        let c_name = CString::new(name).unwrap();
        let c_value = CString::new(value).unwrap();
        unsafe {
            php_ini_builder_quoted(self, c_name.into_raw(), name.len(), c_value.into_raw(), value.len());
        }
    }

    /// Defines a value in the INI builder.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to define.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ext_php_rs::builders::IniBuilder;
    /// let mut builder = IniBuilder::new();
    /// builder.define("foo=bar");
    /// ```
    pub fn define<V: AsRef<str>>(&mut self, value: V) {
        let value = value.as_ref();
        let c_value = CString::new(value).unwrap();
        unsafe {
            php_ini_builder_define(self, c_value.into_raw());
        }
    }

    /// Finishes building the INI configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ext_php_rs::builders::IniBuilder;
    /// let mut builder = IniBuilder::new();
    /// let ini = builder.finish();
    /// ```
    pub fn finish(&mut self) -> *mut c_char {
        if self.value.is_null() {
          return std::ptr::null_mut();
        }

        unsafe { CStr::from_ptr(self.value) }.as_ptr() as *mut c_char
    }
}
