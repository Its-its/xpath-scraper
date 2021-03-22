use thiserror::Error as ThisError;

use mime::FromStrError as MimeConvertError;
use url::ParseError as UrlParseError;
use reqwest::Error as ReqwestError;
use reqwest::header::ToStrError as HeaderToStrError;
use xpath::Error as XpathError;
use regex::Error as RegexError;

use std::io::Error as IoError;

pub type Result<V> = std::result::Result<V, Error>;


#[derive(ThisError, Debug)]
pub enum Error {
	// External
	#[error("Reqwest Error: {0}")]
	Reqwest(ReqwestError),
	#[error("Xpath Error: {0}")]
	Xpath(XpathError),
	#[error("Regex Error: {0}")]
	Regex(RegexError),
	#[error("URL Error: {0}")]
	UrlParseError(UrlParseError),

	#[error("Header To String Error: {0}")]
	HeaderToStrError(HeaderToStrError),

	#[error("MIME Convert Error: {0}")]
	MimeConvertError(MimeConvertError),

	#[error("IO Error: {0}")]
	Io(IoError),

	// Internal
	#[error("Test Error: {0}")]
	Test(TestError),

	#[error("Download Error: {0}")]
	Download(DownloadError),

	#[error("Request: Invalid Method")]
	InvalidMethod,

	#[error("Process: Couldn't find data")]
	ProcessDataResultMissing,

	#[error("Can't convert from Value")]
	ConvertFromValue
}

#[derive(ThisError, Debug)]
pub enum TestError {
	// List
	#[error("Unable to Get List. Url is not defined.")]
	UnableToGetListUrlNotDefined,
	#[error("Unable to Get List. Parser is not defined.")]
	UnableToGetListParserNotDefined,

	#[error("Unable to find Container.")]
	UnableToFindContainer,
	#[error("Unable to find Container Item.")]
	UnableToFindContainerItem,

	// Item
	#[error("Unable to Get Item. Parser is not defined.")]
	UnableToGetItemParserNotDefined,
}

#[derive(ThisError, Debug)]
pub enum DownloadError {
	#[error("Request returned invalid status {0}")]
	RequestReturnedInvalidStatus(u16)
}



impl From<TestError> for Error {
	fn from(error: TestError) -> Self {
		Error::Test(error)
	}
}

impl From<DownloadError> for Error {
	fn from(error: DownloadError) -> Self {
		Error::Download(error)
	}
}


impl From<ReqwestError> for Error {
	fn from(error: ReqwestError) -> Self {
		Error::Reqwest(error)
	}
}

impl From<XpathError> for Error {
	fn from(error: XpathError) -> Self {
		Error::Xpath(error)
	}
}

impl From<RegexError> for Error {
	fn from(error: RegexError) -> Self {
		Error::Regex(error)
	}
}

impl From<IoError> for Error {
	fn from(error: IoError) -> Self {
		Error::Io(error)
	}
}

impl From<UrlParseError> for Error {
	fn from(error: UrlParseError) -> Self {
		Error::UrlParseError(error)
	}
}

impl From<MimeConvertError> for Error {
	fn from(error: MimeConvertError) -> Self {
		Error::MimeConvertError(error)
	}
}

impl From<HeaderToStrError> for Error {
	fn from(error: HeaderToStrError) -> Self {
		Error::HeaderToStrError(error)
	}
}