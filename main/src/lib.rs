mod error;

pub use error::*;

pub use xpather::{
	self,
	factory::ProduceIter,
	value::Node,
	Document,
	Value
};

pub use scraper_macros::Scraper;

/// Used to scrape data for a struct.
///
/// An example of this would look like with macros:
/// ```rust
/// pub struct RedditListItem {
///     pub url: String
/// }
///
/// impl ScraperMain for RedditListItem {
///     fn scrape(doc: &Document, container: Option<Node>) -> Result<Self> {
///        Ok(Self {
///             url: evaluate(".//a[@data-click-id=\"body\"]/@href", doc, container).convert_from(doc)?
///         })
///     }
/// }
/// ```
pub trait ScraperMain: Sized {
	fn scrape(doc: &Document, container: Option<&Node>) -> Result<Self>;
}

/// A simple [Document] evaluation fn.
///
/// Mainly defined for macros.
///
/// Allows for evaluating from the start of the [Document] or from a [Node] in the Document.
pub fn evaluate<'a, S: Into<String>>(search: S, doc: &'a Document, container: Option<&'a Node>) -> Result<ProduceIter<'a>> {
	Ok(if let Some(node) = container {
		doc.evaluate_from(search, node)?
	} else {
		doc.evaluate(search)?
	})
}

/// Allows for Conversion from [ProduceIter] into another.
pub trait ConvertToValue<T>: Sized {
	fn convert_from(self, doc: &Document) -> Result<T>;
}


impl<'a> ConvertToValue<Option<String>> for Result<ProduceIter<'a>> {
	fn convert_from(self, _: &Document) -> Result<Option<String>> {
		self?.next().map(value_to_string).transpose()
	}
}

impl<'a> ConvertToValue<String> for Result<ProduceIter<'a>> {
	fn convert_from(self, _: &Document) -> Result<String> {
		self?.next()
			.map(value_to_string)
			.transpose()?
			.ok_or(Error::ConvertFromValue(None))
	}
}

impl<'a> ConvertToValue<Vec<String>> for Result<ProduceIter<'a>> {
	fn convert_from(self, _: &Document) -> Result<Vec<String>> {
		Ok(value_to_string_vec(self?).into_iter().filter_map(|v| v.ok()).collect())
	}
}

impl<'a> ConvertToValue<Vec<Result<String>>> for Result<ProduceIter<'a>> {
	fn convert_from(self, _: &Document) -> Result<Vec<Result<String>>> {
		Ok(value_to_string_vec(self?))
	}
}



impl<'a, T> ConvertToValue<Vec<T>> for Result<ProduceIter<'a>> where T: ScraperMain {
	fn convert_from(self, doc: &Document) -> Result<Vec<T>> {
		self?.map(|n| T::scrape(doc, Some(n?.as_node()?))).collect::<Result<Vec<_>>>()
	}
}

impl<'a, T> ConvertToValue<Option<T>> for Result<ProduceIter<'a>> where T: ScraperMain {
	fn convert_from(self, doc: &Document) -> Result<Option<T>> {
		self?.next().map(|n| T::scrape(doc, Some(n?.as_node()?))).transpose()
	}
}

/// Converts [Value] to an [Result]<[String]>.
pub fn value_to_string(value: xpather::Result<Value>) -> Result<String> {
	match value? {
		Value::Node(node) => {
			value_to_string(node.value())
		}

		Value::String(v) => Ok(v),

		value => Err(Error::ConvertFromValue(Some(value)))
	}
}

/// Converts [Value] to [Vec]<[String]>.
pub fn value_to_string_vec(iter: ProduceIter) -> Vec<Result<String>> {
	iter.map(value_to_string).collect()
}