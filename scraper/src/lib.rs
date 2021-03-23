mod scraper;
mod error;
mod json;
mod downloader;
mod youtubedl;

pub use scraper::*;
pub use error::*;
pub use json::*;
pub use downloader::*;
pub use youtubedl::*;

pub use xpath::{
	self,
	Document,
	Node,
	Value
};


pub trait ScraperMain: Sized {
	fn scrape(doc: &Document, container: Option<Node>) -> Result<Self>;
}

pub fn evaluate<S: Into<String>>(search: S, doc: &Document, container: Option<Node>) -> Option<Value> {
	if let Some(node) = container {
		doc.evaluate_from(search, node)
	} else {
		doc.evaluate(search)
	}
}

pub trait ConvertFromValue<T>: Sized {
	fn convert_from(self, doc: &Document) -> Result<T>;
}

impl ConvertFromValue<Option<String>> for Option<Value> {
	fn convert_from(self, _: &Document) -> Result<Option<String>> {
		Ok(if let Some(value) = self {
			value_to_string(&value)
		} else {
			None
		})
	}
}

impl ConvertFromValue<String> for Option<Value> {
	fn convert_from(self, _: &Document) -> Result<String> {
		Ok(if let Some(value) = self {
			value_to_string(&value).ok_or(Error::ConvertFromValue(Some(value)))?
		} else {
			return Err(Error::ConvertFromValue(None));
		})
	}
}

impl ConvertFromValue<Vec<String>> for Option<Value> {
	fn convert_from(self, _: &Document) -> Result<Vec<String>> {
		Ok(if let Some(value) = self {
			value_to_string_vec(value)
		} else {
			Vec::new()
		})
	}
}

impl<T> ConvertFromValue<Vec<T>> for Option<Value> where T: ScraperMain {
	fn convert_from(self, doc: &Document) -> Result<Vec<T>> {
		Ok(if let Some(value) = self.map(|v| v.into_iterset()).transpose()? {
			value.map(|n| T::scrape(doc, Some(n))).collect::<Result<Vec<_>>>()?
		} else {
			Vec::new()
		})
	}
}


pub fn value_to_string(value: &Value) -> Option<String> {
	match value {
		Value::Nodeset(set) => {
			set.nodes.first()
			.and_then(|n| value_to_string(&n.value()))
		}

		Value::String(v) => Some(v.clone()),

		_ => None
	}
}

pub fn value_to_string_vec(value: Value) -> Vec<String> {
	match value {
		Value::Nodeset(set) => {
			set.nodes.into_iter().filter_map(|n| value_to_string(&n.value())).collect()
		}

		Value::String(v) => vec![v],

		_ => Vec::new()
	}
}