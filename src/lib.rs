#![allow(unused_unsafe)]

extern crate libc;
extern crate sqlite3_sys as raw;

use std::path::Path;

macro_rules! raise(
    ($message:expr) => (
        return Err(::Error { code: ::ResultCode::Error, message: Some($message.to_string()) })
    );
    ($code:expr, $message:expr) => (
        return Err(::Error { code: $code, message: $message })
    );
);

macro_rules! success(
    ($result:expr) => (
        match $result {
            ::raw::SQLITE_OK => {},
            code => raise!(::result::code_from_raw(code), None),
        }
    );
);

macro_rules! path_to_c_str(
    ($path:expr) => ({
        match $path.to_str() {
            Some(path) => match ::std::ffi::CString::new(path) {
                Ok(string) => string.as_ptr(),
                Err(_) => raise!("failed to process a path"),
            },
            None => raise!("failed to process a path"),
        }
    });
);

macro_rules! str_to_c_str(
    ($string:expr) => (
        match ::std::ffi::CString::new($string) {
            Ok(string) => string.as_ptr(),
            Err(_) => raise!("failed to process a string"),
        }
    );
);

macro_rules! c_str_to_string(
    ($cstr:expr) => (
        String::from_utf8_lossy(::std::ffi::CStr::from_ptr($cstr as *const _).to_bytes())
               .into_owned()
    );
);

mod database;
mod error;
mod result;
mod statement;

pub use database::{Database, ExecuteCallback};
pub use error::Error;
pub use result::{Result, ResultCode};
pub use statement::{Statement, Binding, Value};

/// Open a database.
#[inline]
pub fn open(path: &Path) -> Result<Database> {
    Database::open(path)
}
