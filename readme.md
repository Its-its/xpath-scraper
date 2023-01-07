# XPATH Scraper

Makes it easier to scrape websites with XPATH. Currently using [my xpath parser](https://github.com/Its-its/rust-xpath) which is incomplete, undocumented and used originally for teaching myself about parsing.

A Very simple example of this which is below and also in the [example](/example) folder:
```rust
use std::io::Cursor;

use scraper_main::{
    xpather,
    ConvertToValue,
    ScraperMain,
    Scraper
};

#[derive(Debug, Scraper)]
pub struct RedditList(
    // Uses XPATH to find the item containers
    #[scrape(xpath = r#"//div[contains(@class, "Post") and not(contains(@class, "promotedlink"))]"#)]
    Vec<RedditListItem>
);


#[derive(Debug, Scraper)]
pub struct RedditListItem {
    // URL of the post
    #[scrape(xpath = r#".//a[@data-click-id="body"]/@href"#)]
    pub url: Option<String>,

    // Title of the post
    #[scrape(xpath = r#".//a[@data-click-id="body"]/div/h3/text()"#)]
    pub title: Option<String>,

    // Amount of comments.
    #[scrape(xpath = r#".//a[@data-click-id="comments"]/span/text()"#)]
    pub comment_count: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Request subreddit
    let resp = reqwest::get("https://www.reddit.com/r/nocontextpics/").await?;

    // Return page data.
    let data = resp.text().await?;

    // Parse request into a Document.
    let document = xpather::parse_document(&mut Cursor::new(data))?;

    // Scrape RedditList struct.
    let list = RedditList::scrape(&document, None)?;

    // Output the scraped.
    println!("{:#?}", list);

    Ok(())
}
```
