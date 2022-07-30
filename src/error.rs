//! contains definition of various errors.
//!
use std::fmt::Debug;

pub type DxResult<T> = Result<T, DxFilterErr>;

/// all methods in this crate throw this error
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum DxFilterErr {
    /// only included for debug
    Ok,

    /// Parameter provided is wrong/ unexpected
    BadParam(String),

    /// unexpected internal error. typically happens only with bugs in this crate.
    Unknown(String),
}
