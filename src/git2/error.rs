extern crate libc;
use self::libc::c_uchar;
use std::string::raw::from_buf;
use std::fmt::{Show, Formatter, FormatError};

use git2;
extern {
    fn giterr_last() -> *mut _GitError;
}

#[deriving(Show)]
enum GitErrorType {
    GITERR_NOMEMORY,
    GITERR_OS,
    GITERR_INVALID,
    GITERR_REFERENCE,
    GITERR_ZLIB,
    GITERR_REPOSITORY,
    GITERR_CONFIG,
    GITERR_REGEX,
    GITERR_ODB,
    GITERR_INDEX,
    GITERR_OBJECT,
    GITERR_NET,
    GITERR_TAG,
    GITERR_TREE,
    GITERR_INDEXER,
    GITERR_SSL,
    GITERR_SUBMODULE,
    GITERR_THREAD,
    GITERR_STASH,
    GITERR_CHECKOUT,
    GITERR_FETCHHEAD,
    GITERR_MERGE,
}

pub struct _GitError {
    message: *const c_uchar,
    klass: GitErrorType
}
pub struct GitError {
    pub message: String,
    pub class: GitErrorType
}

impl Show for GitError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(format!("<GitError {}: {}>", self.class, self.message).as_bytes())

    }
}

pub fn get_last_error() -> GitError {
    let e: *mut _GitError = unsafe {giterr_last()};
    unsafe {
        match e.to_option() {
            None => fail!("Asked for error, but there was no error"),
            Some(er) => { 
                GitError{message: from_buf(er.message), class: er.klass}
            }
        }
    }
}
