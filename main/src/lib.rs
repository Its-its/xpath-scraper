mod error;

pub use error::*;

pub use xpather::{
	self,
	Document,
	Node,
	Value
};

/// Used to scrape data for a struct.
///
/// An example of this would be here:
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
	fn scrape(doc: &Document, container: Option<Node>) -> Result<Self>;
}

/// A simple document evaluation fn.
/// Mainly defined for macros.
/// Allows for evaluating from the Document for from the Node in the document.
pub fn evaluate<S: Into<String>>(search: S, doc: &Document, container: Option<Node>) -> Result<Value> {
	Ok(if let Some(node) = container {
		doc.evaluate_from(search, node)?
	} else {
		doc.evaluate(search)?
	})
}

// TODO: Rename to ConvertToValue
/// Allows for Conversion from `Option<Value>` into another.
pub trait ConvertFromValue<T>: Sized {
	fn convert_from(self, doc: &Document) -> Result<T>;
}

impl ConvertFromValue<Option<String>> for Result<Value> {
	fn convert_from(self, _: &Document) -> Result<Option<String>> {
		Ok(value_to_string(self?).ok())
	}
}

impl ConvertFromValue<String> for Result<Value> {
	fn convert_from(self, _: &Document) -> Result<String> {
		value_to_string(self?)
	}
}

impl ConvertFromValue<Vec<String>> for Result<Value> {
	fn convert_from(self, _: &Document) -> Result<Vec<String>> {
		Ok(value_to_string_vec(self?))
	}
}

impl<T> ConvertFromValue<Vec<T>> for Result<Value> where T: ScraperMain {
	fn convert_from(self, doc: &Document) -> Result<Vec<T>> {
		let value = self?.into_iterset()?;
		Ok(value.map(|n| T::scrape(doc, Some(n))).collect::<Result<Vec<_>>>()?)
	}
}

/// Convert Value to an Optional String.
pub fn value_to_string(value: Value) -> Result<String> {
	match value {
		Value::Nodeset(set) => {
			set.nodes.first()
			.ok_or(Error::ConvertFromValue(None))
			.and_then(|n| value_to_string(n.value()?))
		}

		Value::String(v) => Ok(v),

		value => Err(Error::ConvertFromValue(Some(value)))
	}
}

/// Convert Value to a Vec of Strings.
pub fn value_to_string_vec(value: Value) -> Vec<String> {
	match value {
		Value::Nodeset(set) => {
			set.nodes.into_iter()
				.filter_map(|n| value_to_string(n.value().ok()?).ok())
				.collect()
		}

		Value::String(v) => vec![v],

		_ => Vec::new()
	}
}