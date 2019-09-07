extern crate csv;
#[macro_use]
extern crate clap;

mod tasks;

use std::error::Error;
use std::fs::OpenOptions;
use std::process;

use serde::{Deserialize, Serialize};
use tasks::add;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    site: String,
    user: String,
    password: String,
    p_type: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Navin Karkera <navin@disroot.org>")
        (about: "Dead simple password vault")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@subcommand add =>
            (about: "Add site")
            (version: "1.0")
            (author: "Navin Karkera <navin@disroot.org>")
            (@arg site: +required "Website name")
            (@arg user: +required "Username or email")
            (@arg password: +required "Password")
            (@arg p_type: +required "Login type")
        )
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand list =>
            (about: "List available sites")
            (version: "1.0")
            (author: "Navin Karkera <navin@disroot.org>")
        )
        (@subcommand get =>
            (about: "Get site password")
            (version: "1.0")
            (author: "Navin Karkera <navin@disroot.org>")
            (@arg site: +required "Webiste name to get details of")
        )
        (@subcommand update =>
            (about: "Update site details")
            (version: "1.0")
            (author: "Navin Karkera <navin@disroot.org>")
            (@arg site: +required "Webiste name to get details of")
            (@arg user: +required "Username or email")
            (@arg password: +required "Password")
            (@arg p_type: +required "Login type")
        )
    )
    .get_matches();
    let file_str_path = "/home/navin/.config/pass.csv";
    let file_path = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open(file_str_path)?;
    match matches.subcommand() {
        ("list", Some(_)) => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file_path);
            for result in reader.deserialize() {
                let record: Record = result?;
                println!("Site: {}, Type: {}", record.site, record.p_type);
            }
        }
        ("add", Some(add_matches)) => {
            // let mut wtr = csv::WriterBuilder::new()
            //     .has_headers(false)
            //     .from_writer(file_path);
            // wtr.serialize(Record {
            //     site: add_matches.value_of("site").unwrap().to_string(),
            //     user: add_matches.value_of("user").unwrap().to_string(),
            //     password: add_matches.value_of("password").unwrap().to_string(),
            //     p_type: add_matches.value_of("p_type").unwrap().to_string(),
            // })?;
            // wtr.flush()?;
            let result = add(tasks::Record {
                site: add_matches.value_of("site").unwrap().to_string(),
                identifier: add_matches.value_of("user").unwrap().to_string(),
                password: add_matches.value_of("password").unwrap().to_string(),
                identifier_type: add_matches.value_of("p_type").unwrap().to_string(),
            })?;
            println!("{}", result)
        }
        ("update", Some(update_matches)) => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_path(file_str_path)?;
            let mut all_records: Vec<Record> = Vec::new();
            for result in reader.deserialize() {
                let record: Record = result?;
                if (record.site == update_matches.value_of("site").unwrap())
                    & (record.p_type == update_matches.value_of("p_type").unwrap())
                {
                    continue;
                }
                all_records.push(record);
            }
            let mut wtr = csv::WriterBuilder::new()
                .has_headers(false)
                .from_path(file_str_path)?;
            for record in all_records {
                wtr.serialize(record)?;
            }
            wtr.serialize(Record {
                site: update_matches.value_of("site").unwrap().to_string(),
                user: update_matches.value_of("user").unwrap().to_string(),
                password: update_matches.value_of("password").unwrap().to_string(),
                p_type: update_matches.value_of("p_type").unwrap().to_string(),
            })?;
            wtr.flush()?;
        }
        ("get", Some(get_matches)) => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file_path);
            for result in reader.deserialize() {
                let record: Record = result?;
                if record.site == get_matches.value_of("site").unwrap() {
                    println!("========================================");
                    println!("Site: {}", record.site);
                    println!("User: {}", record.user);
                    println!("Password: {}", record.password);
                    println!("PType: {}", record.p_type);
                    println!("----------------------------------------");
                    println!("To update use below command:");
                    println!(
                        "pd update {} {} {} {}",
                        record.site, record.user, record.password, record.p_type
                    );
                }
            }
        }
        _ => (),
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
