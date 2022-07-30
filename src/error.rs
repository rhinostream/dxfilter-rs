use std::fmt::Debug;

pub type DxResult<T> = Result<T, DxCapError>;

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum DxCapError {
    Ok,
    BadParam(String),
    Unknown(String),
}
