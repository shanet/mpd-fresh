use std::collections::{HashMap};
use std::process;
use std::fs;
use std::io;
use clap;

mod common;
mod mpd;
mod musicbrainz;

fn main() {
  let arguments = parse_args();
  let host = format!("{}:{}", arguments.get_one::<String>("server").unwrap(), arguments.get_one::<String>("port").unwrap());

  // let albums = mpd_albums(&host);

  // for (artist, albums) in albums {
    // check_for_new_releases(&artist, albums);
  // }

  load_ignored_releases();

  let new_releases = check_for_new_releases("Chvrches", vec![common::Release {title: "Screen Violence".to_string(), date: "2020-01-01".to_string()}]);

  for release in new_releases {
    println!("New release from {}: {} ({})", "Chvrches", release.title, release.date);
  }
  // print new release
  // ask to ignore?
  // write to ignored yaml file
}

fn load_ignored_releases() {
  let mut file = fs::File::open("ignored.yml");
  let mut contents = String::new();
  file.read_to_string(&contents);
  let config = serde_yaml::from_str(&contents);
}

fn mpd_albums(host: &str) -> HashMap<String, Vec<common::Release>> {
  let mut mpd_client = mpd::MpdClient::new(host);

  match mpd_client.connect() {
    Ok(()) => {
      println!("Connected to MPD server at {}", mpd_client.host);
    }

    Err(error) => {
      eprintln!("Failed to connect to MPD: {}", error);
      process::exit(1);
    }
  }

  match mpd_client.all_albums() {
    Ok(artists) => {
      let _ = mpd_client.disconnect();
      return artists;
    }

    Err(error) => {
      eprintln!("Failed to get albums from MPD: {}", error);
      process::exit(1);
    }
  }
}

fn check_for_new_releases(artist: &str, albums: Vec<common::Release>) -> Vec<common::Release> {
  if artist != "Chvrches" { return Vec::new(); }

  match musicbrainz::MusicBrainz::releases_for_artist(artist) {
    Ok(releases) => {
      // let Some(most_recent_release) = &releases.last() else { return Vec::new(); };
      let Some(last_album) = &albums.last() else { return Vec::new(); };

      let mut new_releases = Vec::new();

      for release in releases {
        if release.date > last_album.date { // todo: and not ignored already
          new_releases.push(release);
        }
      }

      return new_releases;
    }

    Err(error) => {
      match error.status() {
        Some(status) => { eprintln!("API error: {}", status.as_str()); }
        None => { eprintln!("API error: Unknown error"); }
      }

      return Vec::new();
    }
  }
}

fn parse_args() -> clap::ArgMatches {
  return clap::Command::new("mpd-fresh")
    .arg(clap::Arg::new("server")
      .short('s')
      .long("server")
      .help("MPD server to connect to")
      .default_value("localhost"))
    .arg(clap::Arg::new("port")
      .short('p')
      .long("port")
      .help("MPD port to connect to")
      .default_value("6600"))
    .get_matches();

    // TODO: support passwords
    // TODO: support singles and EP release types
}
