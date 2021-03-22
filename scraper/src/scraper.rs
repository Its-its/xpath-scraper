use std::io::Cursor;

use xpath::{Document, Node, Value};

use crate::{value_to_string, Error, ListItem, ListRequest, PageItem, Result, Scraper, TestError};


impl Scraper {
	pub async fn test(&self) -> Result<()> {
		if let Some(test_urls) = self.test_urls.as_deref() {
			println!("Running URL Tests.");

			for url in test_urls {
				println!("Testing URL: {:?}", url);

				let items = self.scrape_list(url).await?;

				println!("Found {} item(s) to scrape.", items.len());

				if let Some(item) = items.first() {
					println!("Testing single item:\n{:#?}", item);

					if let Some(item_url) = item.url.as_deref() {
						let values = self.scrape_item(item_url).await?;

						println!("Scraped from item:\n{:#?}", values);
					}
				}
			}

			println!("Finished URL Test.");
		} else {
			println!("No Test URLS found.")
		}

		Ok(())
	}


	pub async fn scrape_list(&self, list_url: &str) -> Result<Vec<ListItem>> {
		let resp = reqwest::get(list_url).await?;
		let data = resp.text().await?;

		if let Some(req) = self.list.as_ref() {
			let page_doc = xpath::parse_doc(&mut Cursor::new(data));

			let mut items = Vec::new();

			match req {
				ListRequest::Url(_url_req) => {
					//
				}

				ListRequest::Xpath(xpath_req) => {
					if let Some(container_path) = xpath_req.container_path.as_deref() {
						let nodes = page_doc.evaluate(container_path)
							.ok_or_else(|| Error::from(TestError::UnableToFindContainer))?
							.into_iterset()?;

						for node in nodes {
							items.push(ListItem {
								unique_id: node_path_to_string(&xpath_req.unique_id_path, &node, &page_doc)?,
								url: node_path_to_string(&xpath_req.url_path, &node, &page_doc)?.map(|u| self.fix_url(u)),
								title: node_path_to_string(&xpath_req.title_path, &node, &page_doc)?,
								description: node_path_to_string(&xpath_req.description_path, &node, &page_doc)?,
								thumbnail: node_path_to_string(&xpath_req.thumbnail_path, &node, &page_doc)?.map(|url| self.fix_url(url)),
								next_page_url: xpath_req.next_page_path.as_deref()
									.and_then(|xpath| page_doc.evaluate(xpath))
									.and_then(value_to_string)
									.map(|url| self.fix_url(url))
							});
						}
					}
				}
			}

			Ok(items)
		} else {
			Err(TestError::UnableToGetListParserNotDefined.into())
		}
	}


	pub async fn scrape_item(&self, item_url: &str) -> Result<PageItem> {
		if let Some(scraper) = self.item.as_ref() {
			let resp = reqwest::get(item_url).await?;
			let body = resp.text().await?;

			let page_doc = xpath::parse_doc(&mut Cursor::new(body.clone()));

			let mut page_item = PageItem {
				url: item_url.to_string(),

				title: path_to_string(&scraper.title_path, &page_doc)?,
				description: path_to_string(&scraper.description_path, &page_doc)?,
				thumbnail: path_to_string(&scraper.thumbnail_path, &page_doc)?.map(|u| self.fix_url(u)),

				.. Default::default()
			};

			if let Some(url_parser) = scraper.url_path.as_deref() {
				let mut prev_body = vec![body];
				let mut prev_doc = page_doc;

				for parser in url_parser {
					let res = parser.scrape(prev_body, &prev_doc)
							.await?
							.ok_or(Error::ProcessDataResultMissing)?;

					if parser.is_request() {
						prev_doc = xpath::parse_doc(&mut Cursor::new(res.first().unwrap().clone()));
						prev_body = res;
					} else {
						prev_body = res;
					}
				}

				page_item.media_urls = Some(prev_body.into_iter().map(|url| self.fix_url(url)).collect());
			}

			Ok(page_item)
		} else {
			Err(TestError::UnableToGetItemParserNotDefined.into())
		}
	}

	pub fn fix_url(&self, mut url_str: String) -> String {
		if url_str.starts_with("http") {
			url_str
		} else {
			let is_https = self.base_url.starts_with("https");

			if url_str.starts_with("//") {
				let base = if is_https {
					"https:"
				} else {
					"http:"
				};

				url_str.insert_str(0, base);

				url_str
			} else {
				url_str
			}
		}
	}
}





fn node_path_to_string(path: &Option<String>, node: &Node, page_doc: &Document) -> Result<Option<String>> {
	Ok(path.as_deref()
	.map(|xpath| node.evaluate_from(xpath, page_doc).ok_or_else(|| Error::from(TestError::UnableToFindContainerItem)))
	.transpose()?
	.and_then(value_to_string))
}

fn path_to_string(path: &Option<String>, page_doc: &Document) -> Result<Option<String>> {
	Ok(path.as_deref()
	.map(|xpath| page_doc.evaluate(xpath).ok_or_else(|| Error::from(TestError::UnableToFindContainerItem)))
	.transpose()?
	.and_then(value_to_string))
}