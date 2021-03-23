use std::io::Cursor;

use scraper_macros::*;
use scraper_main::{
	xpath,
	ConvertFromValue,
	ScraperMain
};


#[derive(Debug, Scraper)]
pub struct RedditList(
	#[scrape(xpath = r#"//div[contains(@class, "Post") and not(contains(@class, "promotedlink"))]"#)]
	Vec<RedditListItem>
);


#[derive(Debug, Scraper)]
pub struct RedditListItem {
	#[scrape(xpath = r#".//a[@data-click-id="body"]/@href"#)]
	pub url: Option<String>,

	#[scrape(xpath = r#".//a[@data-click-id="body"]/div/h3/text()"#)]
	pub title: Option<String>,

	#[scrape(xpath = r#".//a[@data-click-id="timestamp"]/text()"#)]
	pub timestamp: Option<String>,

	// xpath not working.
	#[scrape(xpath = r#".//div[@data-click-id="background"]/div/div[1]/div/div/a/@href"#)]
	pub thumb: Option<String>,

	#[scrape(xpath = r#".//a[@data-click-id="comments"]/span/text()"#)]
	pub comment_count: Option<String>,

	#[scrape(xpath = r#"./div[1]/div/div/text()"#)]
	pub votes: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let resp = reqwest::get("https://www.reddit.com/r/nocontextpics/").await?;
	let data = resp.text().await?;
	let document = xpath::parse_doc(&mut Cursor::new(data));

	let list = RedditList::scrape(&document, None)?;

	println!("{:#?}", list);

	Ok(())
}
