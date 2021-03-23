# Reddit Code Example:
Scrapes the [/r/nocontextpics](https://www.reddit.com/r/nocontextpics/) subreddit.

## Running the example:
```sh
git pull https://github.com/Its-its/xpath-scraper

cd example

cargo run
```

## Example Output:
```rust
RedditList([
	RedditListItem {
		url: Some("/r/nocontextpics/comments/mam169/pic/"),
		title: Some("PIC"),
		timestamp: Some("12 hours ago"),
		thumb: None,
		comment_count: Some("11 comments"),
		votes: Some("2.3k"),
	},
	RedditListItem {
		url: Some("/r/nocontextpics/comments/mammt9/pic/"),
		title: Some("PIC"),
		timestamp: Some("11 hours ago"),
		thumb: None,
		comment_count: Some("11 comments"),
		votes: Some("605"),
	},
	RedditListItem {
		url: Some("/r/nocontextpics/comments/malrjf/pic/"),
		title: Some("PIC"),
		timestamp: Some("12 hours ago"),
		thumb: None,
		comment_count: Some("12 comments"),
		votes: Some("391"),
	}
	...
])
```