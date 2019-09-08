#[macro_use]
extern crate clap;
extern crate dialoguer;

mod tasks;

use std::error::Error;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(PD =>
        (version: "1.0")
        (author: "Navin Karkera <navin@disroot.org>")
        (about: "Dead simple password vault")
        (@subcommand list =>
            (about: "List available sites")
            (version: "1.0")
            (author: "Navin Karkera <navin@disroot.org>")
        )
        (@subcommand get =>
            (about: "Get site password")
            (version: "1.0")
            (author: "Navin Karkera <navin@disroot.org>")
            (@arg site: +required "Website name to get details of")
        )
        (@subcommand add =>
            (about: "Add site details")
            (version: "1.0")
            (author: "Navin Karkera <navin@disroot.org>")
        )
        (@subcommand init =>
            (about: "Initialize password store")
            (version: "1.0")
            (author: "Navin Karkera <navin@disroot.org>")
        )
    )
    .get_matches();
    match matches.subcommand() {
        ("list", Some(_)) => {
            let master_password = dialoguer::PasswordInput::new()
                .with_prompt("Enter Master Password")
                .interact()?;
            println!("{}", tasks::list(&master_password)?);
        }
        ("get", Some(get_matches)) => {
            let master_password = dialoguer::PasswordInput::new()
                .with_prompt("Enter Master Password")
                .interact()?;
            println!(
                "{}",
                tasks::get(get_matches.value_of("site").unwrap(), &master_password)?
            );
        }
        ("add", Some(_)) => {
            let master_password = dialoguer::PasswordInput::new()
                .with_prompt("Enter Master Password")
                .interact()?;
            let site = dialoguer::Input::<String>::new()
                .with_prompt("Site")
                .interact()?;
            let identifier = dialoguer::Input::<String>::new()
                .with_prompt("Identifier")
                .interact()?;
            let password = dialoguer::PasswordInput::new()
                .with_prompt("Enter Site Password")
                .with_confirmation("Confirm password", "Passwords do not match!")
                .interact()?;
            let identifier_type = dialoguer::Input::<String>::new()
                .with_prompt("Identifier type")
                .interact()?;
            let result = tasks::add(
                tasks::Record {
                    site,
                    identifier,
                    password,
                    identifier_type,
                },
                &master_password,
            )?;
            println!("{}", result);
        }
        ("init", Some(_)) => {
            let master_password = dialoguer::PasswordInput::new()
                .with_prompt("Enter Master Password")
                .with_confirmation("Confirm Pasword", "Password do not match!")
                .interact()?;
            tasks::init_or_open_dir(&master_password)?;
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
