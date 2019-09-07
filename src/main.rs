extern crate csv;
#[macro_use]
extern crate clap;

mod tasks;

use std::error::Error;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Navin Karkera <navin@disroot.org>")
        (about: "Dead simple password vault")
        (@subcommand add =>
            (about: "Add site")
            (version: "1.0")
            (author: "Navin Karkera <navin@disroot.org>")
            (@arg site: +required "Website name")
            (@arg user: +required "Username or email")
            (@arg password: +required "Password")
            (@arg p_type: +required "Login type")
        )
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
    )
    .get_matches();
    match matches.subcommand() {
        ("list", Some(_)) => {
            println!("{}", tasks::list()?);
        }
        ("add", Some(add_matches)) => {
            let result = tasks::add(tasks::Record {
                site: add_matches.value_of("site").unwrap().to_string(),
                identifier: add_matches.value_of("user").unwrap().to_string(),
                password: add_matches.value_of("password").unwrap().to_string(),
                identifier_type: add_matches.value_of("p_type").unwrap().to_string(),
            })?;
            println!("{}", result);
        }
        ("get", Some(get_matches)) => {
            println!("{}", tasks::get(get_matches.value_of("site").unwrap())?);
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
