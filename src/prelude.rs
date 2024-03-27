pub use crate::error::Error;
pub use crate::settings::*;

pub type Result<T> = std::result::Result<T, Error>;
