use cargo_generate::{generate, GenerateArgs, TemplatePath};
use dotenvy::dotenv;
use reqwest::{blocking::Client, cookie::Jar, Url};
use std::boxed::Box;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");
    if env::args().len() == 1 {
        println!(
            r#"Usage: edit .env file and `cargo run -- gen` to generate workspace
or `cargo run -- fetch DAY` to download input"#
        );
        Ok(())
    } else {
        let year = env::var("AOC_year").expect("edit in .env file");
        let year_path = env::current_dir()?.join(year.clone());
        let action = env::args().nth(1).unwrap();
        match action.as_str() {
            "gen" => {
                if !year_path.exists() {
                    // make year workspace
                    fs::create_dir(year_path.clone())?;

                    // copy year workspace cargo.toml
                    let src = Path::new("year_template");
                    fs::copy(src.join("Cargo.toml"), year_path.join("Cargo.toml"))?;

                    // day template for cargo-generate
                    let day_template = TemplatePath {
                        path: Some(
                            env::current_dir()?
                                .join("year_template")
                                .join("{{project-name}}")
                                .to_string_lossy()
                                .into_owned(),
                        ),
                        ..Default::default()
                    };

                    // cargo-generate day template
                    for i in 1..=25 {
                        generate(GenerateArgs {
                            name: Some(format!("day{:0>2}", i)),
                            template_path: day_template.clone(),
                            destination: Some(year_path.clone()),
                            ..Default::default()
                        })?;
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
                if !(1..=25).contains(&day) {
                    panic!("DAY should be 1..=25");
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

                let response = Client::builder()
                    .cookie_provider(std::sync::Arc::new(jar))
                    .build()?
                    .get(get_url)
                    .send()?;
                if response.status().is_success() {
                    let input = response.text()?;
                    println!("{}", &input);
                    fs::write(dst, input)?;
                } else {
                    let input = response.text()?;
                    println!("{}", &input);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
