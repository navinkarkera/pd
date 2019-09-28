extern crate flate2;
extern crate tar;
extern crate zbox;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use zbox::{init_env, DirEntry, OpenOptions, RepoOpener};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub site: String,
    pub identifier: String,
    pub password: String,
    pub identifier_type: String,
}

pub fn init_or_open_dir(password: &str) -> Result<zbox::Repo, zbox::Error> {
    init_env();
    let store_path = format!("file://{}", store().to_str().unwrap());
    let repo = RepoOpener::new().create(true).open(&store_path, password)?;
    Ok(repo)
}

fn store() -> PathBuf {
    let store_path = match env::var("PASSWORD_STORE") {
        Ok(store_dir) => PathBuf::from(store_dir),
        Err(_) => PathBuf::from(&env::var("HOME").unwrap()).join(".pass_store"),
    };
    store_path
}

fn backup_dir() -> PathBuf {
    let bk_dir = match env::var("PASSWORD_STORE_BACKUP") {
        Ok(store_dir) => PathBuf::from(store_dir),
        Err(_) => PathBuf::from(&env::var("HOME").unwrap()),
    };
    bk_dir
}

pub fn add(record: Record, password: &str) -> Result<String, zbox::Error> {
    let mut repo = init_or_open_dir(password)?;
    let file_name = format!("/{}__{}", record.site, record.identifier_type);
    let mut file = OpenOptions::new().create(true).open(&mut repo, file_name)?;
    let contents = format!(
        "Site: {}\nIdentifier: {}\nPassword: {}\nIdentifier_type: {}",
        record.site, record.identifier, record.password, record.identifier_type
    );
    file.write_all(&contents.as_bytes())?;
    file.finish()?;
    Ok(format!(
        "Site: {}, Type: {}, added to store",
        record.site, record.identifier_type
    ))
}

pub fn list(password: &str) -> Result<String, zbox::Error> {
    let mut records = String::new();
    let repo = init_or_open_dir(password)?;
    for entry in repo.read_dir("/")? {
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
    entry.file_name().contains(site)
}

pub fn get(site: &str, password: &str) -> Result<String, zbox::Error> {
    let mut files = Vec::new();
    let mut repo = init_or_open_dir(password)?;
    for entry in repo.read_dir("/")? {
        if is_matching_site(&entry, site) {
            files.push(entry);
        }
    }
    let mut content = String::new();
    for entry in files {
        let mut file = repo.open_file(entry.path())?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;
        content.push_str(&file_content);
        content.push_str("\n============================================\n")
    }
    Ok(content)
}

pub fn backup_store() -> Result<(), std::io::Error> {
    let store = store();
    let backup_dir = backup_dir();
    let archive_path = backup_dir.join("password_vault_backup.tar.gz");
    let tar_gz = File::create(archive_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", store)?;
    Ok(())
}
