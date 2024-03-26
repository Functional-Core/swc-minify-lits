pub use crate::error::Error;

pub type Result<'i, T> = std::result::Result<T, Error>;
