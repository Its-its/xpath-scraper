use std::io::Cursor;

use scraper_macros::*;
use scraper::{
	xpath,
	ConvertFromValue,
	ScraperMain,
	Result,
	// Scraper,
	// DownloadType,
	// ListRequest,
	// PageScraper,
	// UrlParser,
	// ListRequestXpath,
	// RequestUrl
};

#[derive(Debug, Scraper)]
pub struct RedditList {
	#[scrape(xpath = r#"//div[contains(@class, "Post") and not(contains(@class, "promotedlink"))]"#)]
	list: Vec<RedditListItem>
}


#[derive(Debug, Scraper)]
pub struct RedditListItem {
	#[scrape(xpath = r#".//a[@data-click-id="body"]/@href"#)]
	pub url: String,

	#[scrape(xpath = r#".//a[@data-click-id="body"]/div/h3/text()"#)]
	pub title: String,

	#[scrape(xpath = r#".//a[@data-click-id="timestamp"]/text()"#)]
	pub timestamp: String,

	#[scrape(xpath = r#".//div[@data-click-id="image"]/../@href"#)]
	pub thumb: String,

	#[scrape(xpath = r#".//a[@data-click-id="comments"]/span/text()"#)]
	pub comment_count: String,

	#[scrape(xpath = r#".//button[@data-click-id="upvote"]/../div/text()"#)]
	pub votes: String,
}


#[tokio::main]
async fn main() -> Result<()> {
	let resp = reqwest::get("https://www.reddit.com/r/nocontextpics/").await?;
	let data = resp.text().await?;
	let document = xpath::parse_doc(&mut Cursor::new(data));

	let list = RedditList::scrape(&document, None)?;

	println!("{:#?}", list);

	Ok(())
}
