use reqwest::header::CONTENT_TYPE;
use serde_derive::{Serialize, Deserialize};

use std::path::PathBuf;

use mime::Mime;
use tokio::io::AsyncWriteExt;
use tokio::fs::OpenOptions;
use crypto::sha3::Sha3;
use crypto::digest::Digest;

use crate::{DownloadError, Result, YoutubeDL};


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DownloadErrorHandler {
	IgnoreErrors,
	ReturnIfError
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DownloadType {
	Request,
	DownloadAll,
	DownloadSingle
}

impl DownloadType {
	pub async fn download(self, urls: Vec<String>, directory: &str) -> Result<bool> {
		let mut info = DownloadInfo {
			files: Vec::new()
		};

		match self {
			Self::Request => {
				// TODO: Multiple requests at once.

				for url in urls {
					let file_name = self.download_file(&url, directory).await;

					info.files.push(DownloadedFile {
						url,
						file_name
					});
				}
			}

			Self::DownloadAll => {
				for url in urls {
					if !YoutubeDL::is_url_supported(&url).await? {
						break;
					}

					let file_name = self.download_file(&url, directory).await;

					info.files.push(DownloadedFile {
						url,
						file_name
					});
				}
			}

			Self::DownloadSingle => {
				for url in urls {
					if YoutubeDL::is_url_supported(&url).await? {
						let file_name = self.download_file(&url, directory).await;

						info.files.push(DownloadedFile {
							url,
							file_name
						});

						return Ok(true);
					}
				}
			}
		}

		Ok(false)
	}

	pub async fn download_file(self, url: &str, directory: &str) -> Result<String> {
		println!("Downloading: {}", url);

		match self {
			Self::Request => {
				// TODO: Multiple requests at once.

				let resp = reqwest::get(url).await?;

				if resp.status().is_success() {
					let ext = if let Some(value) = resp.headers().get(CONTENT_TYPE) {
						let value = value.to_str()?;

						let mime = value.parse::<Mime>()?;

						// TODO: Only allow cetain mime types for ext

						mime.subtype().to_string()
					} else {
						todo!("Content Type Header");
					};


					let bytes = resp.bytes().await?;

					let file_name = {
						let mut sha = Sha3::sha3_256();
						sha.input(&bytes);

						sha.result_str()
					};


					let mut path = PathBuf::new();
					path.push(directory);
					path.set_file_name(file_name);
					path.set_extension(ext);

					let mut file = OpenOptions::new()
						.write(true)
						.truncate(true)
						.create(true)
						.open(path)
						.await?;

					file.write_all(&bytes).await?;
					file.flush().await?;
				} else {
					return Err(DownloadError::RequestReturnedInvalidStatus(resp.status().as_u16()).into());
				}
			}

			Self::DownloadSingle |
			Self::DownloadAll => {
				let resp = YoutubeDL::download(url, directory).await?;

				println!("{}", resp);

				// DO
			}
		}

		Ok("".to_string())
	}
}


pub struct DownloadInfo {
	pub files: Vec<DownloadedFile>,

	//
}

#[derive(Debug)]
pub struct DownloadedFile {
	pub url: String,
	pub file_name: Result<String>
}