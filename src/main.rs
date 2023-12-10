use dotenvy::dotenv;
use reqwest::blocking::Client;
use reqwest::cookie::Jar;
use reqwest::Url;
use std::boxed::Box;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use toml_edit::{value, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");
    if env::args().len() == 1 {
        println!(
            r#"Usage: edit .env file and `cargo run -- gen` to generate workspace
or `cargo run -- fetch DAY` to download input"#
        );
        Ok(())
    } else {
        let action = env::args().nth(1).unwrap();
        let year = env::var("AOC_year").expect("edit in .env file");
        let year_path = Path::new(&year);
        match action.as_str() {
            "gen" => {
                if !year_path.exists() {
                    // copy cargo.toml
                    fs::create_dir(year_path)?;
                    let src = Path::new("template");
                    fs::copy(src.join("Cargo.toml"), year_path.join("Cargo.toml"))?;

                    for i in 1..=25 {
                        let src = Path::new("template").join("day_template");
                        let day_string = format!("day{:0>2}", i);
                        let day_path = year_path.join(&day_string);

                        // day: copy and fix cargo
                        fs::create_dir(&day_path)?;
                        let mut doc =
                            fs::read_to_string(src.join("Cargo.toml"))?.parse::<Document>()?;
                        doc["package"]["name"] = value(day_string);
                        fs::write(day_path.join("Cargo.toml"), doc.to_string())?;

                        // day: copy main.rs
                        fs::create_dir(day_path.join("src"))?;
                        fs::copy(
                            src.join("src").join("main.rs"),
                            day_path.join("src").join("main.rs"),
                        )?;
                    }
                    Ok(())
                } else {
                    Err("year_path existed".into())
                }
            }
            "fetch" => {
                let day = env::args()
                    .nth(2)
                    .expect("Expect DAY")
                    .parse::<u32>()
                    .expect("DAY should be a number");
                if day < 1 || day > 25 {
                    panic!("DAY should be 1..25");
                }
                let dst = year_path
                    .join(format!("day{:0>2}", day))
                    .join("src")
                    .join("input.txt");
                let session = env::var("AOC_session")?;
                let url = Url::from_str("https://adventofcode.com/")?;
                let get_url = url.join(&format!("{year}/day/{day}/input"))?;

                let jar = Jar::default();
                jar.add_cookie_str(&format!("session={session}"), &url);

                let input = Client::builder()
                    .cookie_provider(std::sync::Arc::new(jar))
                    .build()?
                    .get(get_url)
                    .send()?
                    .text()?;
                fs::write(dst, input)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
