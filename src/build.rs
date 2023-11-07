use git2::{Error, Repository};
use std::process::Command;
use std::fs;
use std::path::{Path, PathBuf};

pub fn clone(pkg: String, dir: String) -> Result<Repository, Error> {
    let repo = format!("https://aur.archlinux.org/{}.git", pkg);
    Repository::clone(repo.as_str(), dir.as_str())
}

pub fn copy(dir: String, local_repo: String) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        match std::env::var("GPGKEY") {
            Ok(_) => {
                zst(file_path.clone(), local_repo.clone())?;
                if file_path.is_file() && file_path.extension().unwrap_or_default() == "sig" {
                    let file_name = file_path.file_name().unwrap();
                    let local_repo = Path::new(local_repo.as_str()).join(file_name);
                    fs::copy(&file_path, &local_repo)?;
                }
            }
            Err(_) => {
                zst(file_path, local_repo.clone())?
            }
        }
    }
    Ok(())
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::var("GPGKEY") {
        Ok(value) => {
            Command::new("makepkg")
                .arg("-s")
                .arg("--sign")
                .arg(format!("--key={}", value))
                .arg("--noconfirm")
                .output()?;
        },
        Err(_) => {
            Command::new("makepkg")
                .arg("-s")
                .arg("--noconfirm")
                .output()?;

        },
        }
    Ok(())
    }


pub fn delete(dir: String) -> std::io::Result<()> {
    fs::remove_dir_all(dir)
}

pub fn repo_add() -> Result<(), Box<dyn std::error::Error>> {
    let files: Vec<String> = glob::glob("*.pkg.tar.zst")
        .expect("Failed to read glob pattern")
        .filter_map(|entry| entry.ok())
        .filter_map(|path| path.to_str().map(String::from))
        .collect();

    if files.is_empty() {
        println!("No matching files found.");
    }

    let vec_of_str_refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
    Command::new("repo-add")
        .arg("repo.db.tar.gz")
        .arg(format!("{}", vec_of_str_refs.join(", ")))
        .output()?;
    Ok(())
}

fn zst(file_path: PathBuf, local_repo: String) -> Result<(), Box<dyn std::error::Error>> {
    if file_path.is_file() && file_path.extension().unwrap_or_default() == "zst" {
        let file_name = file_path.file_name().unwrap();
        let local_repo = Path::new(local_repo.as_str()).join(file_name);
        fs::copy(&file_path, &local_repo)?;
    }
    Ok(())
}