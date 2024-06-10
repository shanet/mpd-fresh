use std::collections::{HashMap, HashSet};
use std::process;
use clap;

mod mpd;
mod musicbrainz;

fn main() {
  let arguments = parse_args();
  let host = format!("{}:{}", arguments.get_one::<String>("server").unwrap(), arguments.get_one::<String>("port").unwrap());

  let _albums = mpd_albums(&host);
  new_albums();
}

fn mpd_albums(host: &str) -> HashMap<String, HashSet<String>> {
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

  match mpd_client.get_artist_albums() {
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

fn new_albums() {
  match musicbrainz::MusicBrainz::query() {
    Ok(response) => {
      print!("{}", response);
    }

    Err(_error) => {
      eprintln!("API error");
    }
  }
      //   for (artist, albums) in artists {
      //   println!("{}", artist);

      //   for album in albums {
      //     println!("\t{}", album);
      //   }
      // }
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
}
