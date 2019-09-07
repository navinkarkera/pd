use std::env;
use std::fs::{create_dir_all, read_dir, DirEntry, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub site: String,
    pub identifier: String,
    pub password: String,
    pub identifier_type: String,
}

fn store() -> PathBuf {
    let store_path = match env::var("PASSWORD_STORE") {
        Ok(store_dir) => PathBuf::from(store_dir),
        Err(_) => PathBuf::from(&env::var("HOME").unwrap()).join(".pass_store"),
    };
    create_dir_all(&store_path).expect("Could not create store folder!!");
    store_path
}

fn create_password_file(site_name: &str, identifier_type: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(store().join(format!("{}__{}", site_name, identifier_type)))
}

pub fn add(record: Record) -> Result<String, io::Error> {
    let mut file = create_password_file(&record.site, &record.identifier_type)?;
    let contents = format!(
        "Site: {}\nIdentifier: {}\nPassword: {}\nIdentifier_type: {}",
        record.site, record.identifier, record.password, record.identifier_type
    );
    file.write_all(&contents.as_bytes())?;
    Ok(format!(
        "Added site: {}, type: {} to store",
        record.site, record.identifier_type
    ))
}

pub fn list() -> Result<String, io::Error> {
    let mut records = String::new();
    for entry in read_dir(store())? {
        let entry = entry?;
        match entry.path().file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_name) => records = format!("{}{}\n", records, file_name),
                None => (),
            },
            None => (),
        }
    }
    Ok(records)
}

fn is_matching_site(entry: &DirEntry, site: &str) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.contains(site))
        .unwrap_or(false)
}

pub fn get(site: &str) -> Result<String, io::Error> {
    let mut files = Vec::new();
    for entry in read_dir(store())? {
        let entry = entry?;
        if is_matching_site(&entry, site) {
            files.push(entry);
        }
    }
    let mut content = String::new();
    for entry in files {
        let mut file = File::open(entry.path())?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;
        content.push_str(&file_content);
        content.push_str("\n============================================\n")
    }
    Ok(content)
}
