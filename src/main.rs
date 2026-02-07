use cargo_generate::{generate, GenerateArgs, TemplatePath};
use dotenvy::dotenv;
use reqwest::{blocking::Client, cookie::Jar, Url};
use std::boxed::Box;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

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
        let year_num = year.parse::<usize>()?;
        let num_of_day = match year_num {
            ..2015 => panic!("no puzzle for those years"),
            2015..2025 => 25,
            2025.. => 12,
        };
        let year_path = env::current_dir()?.join(year.clone());
        let action = env::args().nth(1).unwrap();
        match action.as_str() {
            "gen" => {
                if !year_path.exists() {
                    // make year workspace
                    println!("creating year folder");
                    fs::create_dir(&year_path)?;

                    // copy year workspace cargo.toml & bacon.toml
                    println!("copying Cargo.toml and bacon.toml");
                    let src = Path::new("year_template");
                    fs::copy(src.join("Cargo.toml"), year_path.join("Cargo.toml"))?;
                    fs::copy(src.join("bacon.toml"), year_path.join("bacon.toml"))?;

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
                    for i in 1..num_of_day {
                        generate(GenerateArgs {
                            name: Some(format!("day{:0>2}", i)),
                            template_path: day_template.clone(),
                            destination: Some(year_path.clone()),
                            ..Default::default()
                        })?;
                    }
                    // last_day
                    let day_template = TemplatePath {
                        path: Some(
                            env::current_dir()?
                                .join("year_template")
                                .join("last_day")
                                .join("{{project-name}}")
                                .to_string_lossy()
                                .into_owned(),
                        ),
                        ..Default::default()
                    };
                    generate(GenerateArgs {
                        name: Some(format!("day{:0>2}", num_of_day)),
                        template_path: day_template.clone(),
                        destination: Some(year_path.clone()),
                        ..Default::default()
                    })?;
                    println!("Finish generating template for YEAR {year}");
                    Ok(())
                } else {
                    Err("year_path existed".into())
                }
            }
            "fetch" => {
                let arg = env::args().nth(2).expect("Expect all or DAY");

                let session = env::var("AOC_session")?;
                let base_url = Url::from_str("https://adventofcode.com/")?;
                let jar = Jar::default();
                jar.add_cookie_str(&format!("session={session}"), &base_url);
                let client = Client::builder().cookie_provider(Arc::new(jar)).build()?;

                if arg == "all" {
                    println!("start downloading input for year {}", year);
                    for day in 1..=num_of_day {
                        let dst = year_path
                            .join(format!("day{:0>2}", day))
                            .join("src")
                            .join("input.txt");

                        let url = base_url.join(&format!("{year}/day/{day}/input"))?;
                        let response = client.get(url).send()?;
                        if response.status().is_success() {
                            let input = response.text()?;
                            fs::write(dst, input)?;
                            println!("finish download input for day {:0>2}", day);
                        } else {
                            let input = response.text()?;
                            println!("{}", &input);
                            break;
                        }
                    }
                } else {
                    let day = arg.parse::<usize>().expect("DAY should be a number");
                    if !(1..=num_of_day).contains(&day) {
                        panic!("DAY should be 1..={}", num_of_day);
                    }
                    let dst = year_path
                        .join(format!("day{:0>2}", day))
                        .join("src")
                        .join("input.txt");

                    let url = base_url.join(&format!("{year}/day/{day}/input"))?;
                    let response = client.get(url).send()?;

                    if response.status().is_success() {
                        let input = response.text()?;
                        println!("{}", &input);
                        fs::write(dst, input)?;
                    } else {
                        let input = response.text()?;
                        println!("{}", &input);
                    }
                }
                Ok(())
            }
            _ => Err("use gen or fetch".into()),
        }
    }
}
