use crate::common;
use std::collections::{BTreeMap};
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;

const IGNORED_FILENAME: &'static str = "mpd_fresh_ignored.yml";

pub struct DataStore {
  map: BTreeMap<String, Vec<common::Album>>,
}

impl DataStore {
  pub fn new() -> Result<DataStore, Box<dyn Error>> {
    match Self::load() {
      Ok(config) => {
        return Ok(DataStore {
          map: config,
        });
      }

      Err(error) => {
        if let Some(_) = error.downcast_ref::<serde_yaml::Error>() {
          return Err(error);
        }

        return Ok(DataStore {
          map: BTreeMap::new(),
        });
      }
    }
  }

  fn load() -> Result<BTreeMap<String, Vec<common::Album>>, Box<dyn Error>> {
    let contents = fs::read_to_string(Self::ignored_config_path())?;
    let config: BTreeMap<String, Vec<common::Album>> = serde_yaml::from_str(&contents)?;
    return Ok(config);
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let contents = serde_yaml::to_string(&self.map)?;
    let mut file = fs::File::create(Self::ignored_config_path())?;
    file.write_all(contents.as_bytes())?;
    return Ok(());
  }

  pub fn ignored_albums_for_artist(&self, artist: &str) -> Vec<common::Album> {
    let empty_vector = Vec::new();
    let ignored_albums = self.map.get(artist).unwrap_or(&empty_vector);
    return ignored_albums.to_vec();
  }

  pub fn add_ignored_album_for_artist(&mut self, artist: &str, release: common::Album) {
    self.map.entry(artist.to_string())
      .and_modify(|albums| albums.push(release.clone()))
      .or_insert_with(|| vec![release]);
  }

  fn ignored_config_path() -> String {
    match env::var("HOME") {
      Ok(home_directory) => {
        return format!("{}/.config/{}", home_directory, IGNORED_FILENAME);
      }

      Err(_) => {
        eprintln!("WARNING: $HOME not found, using current directory for ignored config file");
        return IGNORED_FILENAME.to_string();
      }
    }
  }
}
