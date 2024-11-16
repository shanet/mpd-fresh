use std::collections::BTreeMap;
use std::io::{self, Write};
use std::process;

mod common;
mod config;
mod data_store;
mod mpd;
mod musicbrainz;

fn main() {
  config::parse_arguments();

  let mut data_store = data_store::DataStore::new().unwrap_or_else(|error| {
    eprintln!("Failed to parse ignored YAML file: {}", error);
    process::exit(1);
  });

  println!("Getting release date info from MPD...");
  let all_albums = get_albums_from_mpd();

  println!("{}Getting new releases from MusicBrainz...", (if config::is_verbose() {""} else {"\n"}));
  let mut new_albums = BTreeMap::new();
  let mut new_albums_count = 0;
  let mut index = 0;

  for (artist, albums) in &all_albums {
    index += 1;

    let ignored_albums = data_store.ignored_albums_for_artist(&artist);
    config::print_status(&format!("({}/{}) {}", index, all_albums.len(), artist));

    let artist_new_albums = check_new_albums_for_artist(&artist, &albums, &ignored_albums);
    new_albums_count += artist_new_albums.len();
    new_albums.insert(artist, artist_new_albums);

    // Sleep between requests to avoid hitting the rate limit
    musicbrainz::MusicBrainz::rate_limit_wait();
  }

  // Print a newline so the prompts below start on their own line apart from the status output above
  println!("");

  index = 1;

  for (artist, albums) in new_albums {
    for album in albums {
      let prefix = format!("{}/{}", index, new_albums_count);
      index += 1;

      if config::ignore_all_albums() || prompt_for_ignore(&artist, &album, &prefix) {
        if config::is_verbose() { println!("Ignoring: {}/{}", artist, album.title); }

        data_store.add_ignored_album_for_artist(&artist, album);
      }
    }
  }

  if config::is_verbose() { println!("Writing results to ignore file"); }
  let _ = data_store.save().unwrap_or_else(|error| {
    eprintln!("{}Failed to saved ignored file: {}", (if config::is_verbose() {""} else {"\n"}), error);
    process::exit(1);
  });
}

fn get_albums_from_mpd() -> BTreeMap<String, Vec<common::Album>> {
  let host = config::mpd_host();
  let password = config::mpd_password();
  let mut mpd_client = mpd::MpdClient::new(&host, password.as_deref());

  match mpd_client.connect() {
    Ok(()) => {if config::is_verbose() { println!("Connected to MPD server at {}", mpd_client.host); }}

    Err(error) => {
      eprintln!("Failed to connect to MPD: {}", error);
      process::exit(1);
    }
  }

  match mpd_client.all_albums(config::artist().as_deref()) {
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

fn check_new_albums_for_artist(artist: &str, known_albums: &Vec<common::Album>, ignored_albums: &Vec<common::Album>) -> Vec<common::Album> {
  match musicbrainz::MusicBrainz::albums_for_artist(artist) {
    Ok(all_albums) => {
      let mut new_albums = Vec::new();

      // Known albums should be sorted by date. Use the last one as the most recent.
      // It doesn't make sense for an artist in the library to not have any albums so if this is the
      // case then return no new albums as this must be a compilation artist or something else odd.
      let Some(last_album) = &known_albums.last() else { return new_albums; };

      for album in all_albums {
        if album.date > last_album.date && !ignored_albums.contains(&album) {
          new_albums.push(album);
        }
      }

      return new_albums;
    }

    Err(error) => {
      match error.status() {
        Some(status) => {
          if status.as_str() == "503" {
            eprintln!("{}API error: Rate limited", (if config::is_verbose() {""} else {"\n"}));
          } else {
            eprintln!("{}API error: {}", (if config::is_verbose() {""} else {"\n"}), status.as_str());
          }
        }

        None => eprintln!("{}API error: Unknown error", (if config::is_verbose() {""} else {"\n"}))
      }

      return Vec::new();
    }
  }
}

fn prompt_for_ignore(artist: &str, album: &common::Album, prefix: &str) -> bool {
  print!("({}) New release: {} - {} ({}). Ignore? (Y,n) ", prefix, artist, album.title, album.date.as_deref().unwrap_or("unknown date"));
  let _ = io::stdout().flush();

  let mut input = String::new();
  let _ = io::stdin().read_line(&mut input);

  input.make_ascii_lowercase();
  let answer = input.trim();

  return answer == "y" || answer == "";
}
