use thiserror::Error as ThisError;

use xpather::{Error as XpathError, Value};

use std::io::Error as IoError;

pub type Result<V> = std::result::Result<V, Error>;


#[derive(ThisError, Debug)]
pub enum Error {
	// External
	#[error("Xpath Error: {0}")]
	Xpath(XpathError),

	#[error("IO Error: {0}")]
	Io(IoError),

	#[error("Can't convert from Value")]
	ConvertFromValue(Option<Value>),

	#[error("Field \"{0}\" Value Error {1}")]
	FieldValueError(&'static str, Box<Error>)
}

impl From<XpathError> for Error {
	fn from(error: XpathError) -> Self {
		Error::Xpath(error)
	}
}

impl From<IoError> for Error {
	fn from(error: IoError) -> Self {
		Error::Io(error)
	}
}