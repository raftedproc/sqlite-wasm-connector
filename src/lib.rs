#![allow(unused_unsafe)]

extern crate libc;
extern crate sqlite3_sys as raw;

#[cfg(test)]
extern crate temporary;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::from($message)));
);

macro_rules! failure(
    ($database:expr, $code:expr) => (
        match ::error::last($database) {
            Some(error) => return Err(error),
            None => return Err(::Error::from(::error::kind_from_code($code))),
        }
    );
);

macro_rules! success(
    ($database:expr, $result:expr) => (
        match $result {
            ::raw::SQLITE_OK => {},
            code => failure!($database, code),
        }
    );
    ($result:expr) => (
        match $result {
            ::raw::SQLITE_OK => {},
            code => return Err(::Error::from(::error::kind_from_code(code))),
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
mod statement;

pub use database::Database;
pub use error::{Error, ErrorKind};
pub use statement::{Statement, Binding, Value, State};

/// A result.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Open a database.
#[inline]
pub fn open<'l>(path: &std::path::Path) -> Result<Database<'l>> {
    Database::open(path)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use temporary::Directory;

    macro_rules! ok(
        ($result:expr) => ($result.unwrap());
    );

    pub fn setup() -> (PathBuf, Directory) {
        let directory = ok!(Directory::new("sqlite"));
        (directory.path().join("database.sqlite3"), directory)
    }
}
