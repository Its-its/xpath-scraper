#![warn(
	clippy::all,
	clippy::pedantic
)]

use std::io::Cursor;

use scraper_macros::Scraper;
use scraper_main::{
	xpather,
	ConvertFromValue,
	ScraperMain
};

// Structure which is used to be able to scrape data from reddit.
#[derive(Debug, Scraper)]
pub struct RedditList(
	// Uses XPATH to find the item containers
	#[scrape(xpath = r#"//div[contains(@class, "Post") and not(contains(@class, "promotedlink"))]"#)]
	Vec<RedditListItem>
);


// Transform received "/r/.." into "https://reddit.com/r/.."
fn url_transform(received: Option<String>) -> Option<String> {
	received.map(|mut url| {
		url.insert_str(0, "https://reddit.com");
		url
	})
}

#[derive(Debug, Scraper)]
pub struct RedditListItem {
	// URL of the post
	#[scrape(xpath = r#".//a[@data-click-id="body"]/@href"#)]
	#[scrape(transform = "url_transform")]
	pub url: Option<String>,

	// Title of the post
	#[scrape(xpath = r#".//a[@data-click-id="body"]/div/h3/text()"#)]
	pub title: Option<String>,

	// When it was posted
	#[scrape(xpath = r#".//a[@data-click-id="timestamp"]/text()"#)]
	pub timestamp: Option<String>,

	// Thumbnail image. Not working currently. Seems to be a Document issue (?)
	#[scrape(xpath = r#".//div[@data-click-id="background"]/div/div[1]/div/div/a/@href"#)]
	pub thumb: Option<String>,

	// Amount of comments.
	#[scrape(xpath = r#".//a[@data-click-id="comments"]/span/text()"#)]
	pub comment_count: Option<String>,

	// Vote count.
	#[scrape(xpath = r#"./div[1]/div/div/text()"#)]
	pub votes: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Request subreddit
	let resp = reqwest::get("https://www.reddit.com/r/nocontextpics/").await?;

	// Return page data.
	let data = resp.text().await?;

	// Parse request into a Document.
	let document = xpather::parse_doc(&mut Cursor::new(data));

	// Scrape RedditList struct.
	let list = RedditList::scrape(&document, None)?;

	// Output the scraped.
	println!("{:#?}", list);

	Ok(())
}
