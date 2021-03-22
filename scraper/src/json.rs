use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};
use xpath::{Document, Value};
use regex::RegexBuilder;
use reqwest::Method;

use crate::{DownloadType, Error, Result, TestError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scraper {
	pub base_url: String,
	pub list_url: Option<String>,
	pub item_url: Option<String>,

	pub test_urls: Option<Vec<String>>,

	pub title: Option<String>,
	pub collection: String,

	pub list: Option<ListRequest>,
	pub item: Option<PageScraper>,
	pub downloader: Option<DownloadType>
}


// Item

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageItem {
	pub url: String,

	pub title: Option<String>,
	pub description: Option<String>,
	pub thumbnail: Option<String>,

	pub media_urls: Option<Vec<String>>
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageScraper {
	pub title_path: Option<String>,
	pub description_path: Option<String>,
	pub thumbnail_path: Option<String>,

	pub url_path: Option<Vec<UrlParser>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrlParser {
	Xpath(String),
	Regex(String, bool),
	Request(RequestUrl),
	RequestPrev(Option<String>)
}

impl UrlParser {
	pub fn is_request(&self) -> bool {
		matches!(self, Self::Request(_) | Self::RequestPrev(_))
	}

	pub async fn scrape(&self, values: Vec<String>, page_doc: &Document) -> Result<Option<Vec<String>>> {
		match self {
			UrlParser::Xpath(xpath) => {
				path_to_string(xpath.as_str(), &page_doc)
			}

			UrlParser::Regex(pattern, is_case_sensitive) => {
				let regex = RegexBuilder::new(pattern.as_str())
					.case_insensitive(*is_case_sensitive)
					.build()?;

				let body = &values[0];

				Ok(
					regex.find(body.as_str())
					.map(|m| vec![m.as_str().to_string()])
				)
			}

			UrlParser::RequestPrev(url_fix) => {
				let body = &values[0];

				let url = if let Some(fix) = url_fix {
					fix.replace("{0}", &body)
				} else {
					body.clone()
				};

				let resp = reqwest::get(url).await?;
				let body = resp.text().await?;

				Ok(Some(vec![body]))
			}

			UrlParser::Request(props) => {
				let mut builder = reqwest::ClientBuilder::new()
					.build()?
					.request(
						Method::from_bytes(
							props.method.as_deref().unwrap_or("GET").to_uppercase().as_bytes()
						).map_err(|_| Error::InvalidMethod)?,
						&props.url
					);

				if let Some(form) = props.form_data.as_ref() {
					let mut fixed_map = HashMap::new();

					let body = &values[0];

					for (key, val) in form {
						fixed_map.insert(
							key.clone(),
							val.replace("{0}", body.as_str())
						);
					}

					builder = builder.form(&fixed_map);
				}

				let resp = builder.send().await?;
				let body = resp.text().await?;

				Ok(Some(vec![body]))
			}
		}
	}
}

fn path_to_string(search: &str, page_doc: &Document) -> Result<Option<Vec<String>>> {
	Ok(
		value_to_string(
			page_doc.evaluate(search)
			.ok_or_else(|| Error::from(TestError::UnableToFindContainerItem))?
		)
	)
}

fn value_to_string(value: Value) -> Option<Vec<String>> {
	match value {
		Value::Nodeset(set) => {
			Some(
				set.nodes
					.into_iter()
					.filter_map(|n| value_to_string(n.value()))
					.flatten()
					.collect()
			)
		}

		Value::String(v) => Some(vec![v]),

		_ => None
	}
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestUrl {
	pub url: String,
	pub method: Option<String>,
	pub form_data: Option<HashMap<String, String>>
}


// List

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListRequest {
	Xpath(ListRequestXpath),
	Url(RequestUrl)
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRequestXpath {
	pub container_path: Option<String>,

	pub unique_id_path: Option<String>,
	pub url_path: Option<String>,

	pub title_path: Option<String>,
	pub description_path: Option<String>,
	pub thumbnail_path: Option<String>,

	pub next_page_path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListItem {
	pub unique_id: Option<String>,
	pub url: Option<String>,

	pub title: Option<String>,
	pub description: Option<String>,
	pub thumbnail: Option<String>,

	pub next_page_url: Option<String>
}

impl ListItem {
	pub async fn scrape(&self, scraper: &Scraper) -> Result<Option<PageItem>> {
		if let Some(item_url) = self.url.as_deref() {
			scraper.scrape_item(item_url).await.map(Some)
		} else {
			Ok(None)
		}
	}
}
