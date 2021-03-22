use std::process::Stdio;

use tokio::process::Command;

use crate::Result;

pub struct YoutubeDL;

impl YoutubeDL {
	pub async fn is_url_supported(url: &str) -> Result<bool> {
		let mut dl = Command::new("youtube-dl");

		dl.stdout(Stdio::null());
		dl.stderr(Stdio::null());

		dl.args(&[
			"-s",
			"-g",
			url
		]);

		let status = dl.spawn()?
			.wait()
			.await?;

		Ok(status.success())
	}

	pub async fn download(url: &str, directory: &str) -> Result<bool> {
		let mut dl = Command::new("youtube-dl");

		dl.current_dir(directory);

		dl.args(&[
			"--print-json",
			"--restrict-filenames",
			"-o",
			"%(title)s.%(ext)s",
			url
		]);

		let output = dl.output().await?;

		println!("OUT: {}", unsafe { String::from_utf8_unchecked(output.stdout) });
		println!("ERR: {}", unsafe { String::from_utf8_unchecked(output.stderr) });

		Ok(output.status.success())
	}
}