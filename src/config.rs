use anyhow::Context;
use dirs_next::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Script {
	pub name: String,
	pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ConfigFile {
	pub scripts: Vec<Script>,
}

pub fn parse() -> anyhow::Result<ConfigFile> {
	let dir = folder_path();
	fs::create_dir_all(dir)?;
	let file = file_path();
	let does_exist = Path::new(file.as_path()).exists();
	if does_exist {
		read(file)
	} else {
		let config = ConfigFile { scripts: vec![] };
		write(config)
	}
}

pub fn folder_path() -> PathBuf {
	let mut dir = PathBuf::new();
	dir.push(home_dir().unwrap());
	dir
}

pub fn file_path() -> PathBuf {
	let mut file = folder_path();
	file.push("coco-scripts.json");
	file
}

pub fn write(config: ConfigFile) -> anyhow::Result<ConfigFile> {
	let dir = folder_path();
	fs::create_dir_all(dir)?;
	let file = file_path();
	let json = serde_json::to_string(&config).unwrap();
	let mut physical_file = fs::File::create(file.as_path())?;
	physical_file.write_all(json.as_bytes())?;
	Ok(config)
}

pub fn read(path: PathBuf) -> anyhow::Result<ConfigFile> {
	let file = fs::File::open(path.as_path());
	match serde_json::from_reader(file.unwrap()) {
		Ok(json) => Ok(json),
		Err(err) => Err(anyhow::Error::new(err))
			.with_context(|| "Error parsing coco-scripts.json. Try deleting the file."),
	}
}
