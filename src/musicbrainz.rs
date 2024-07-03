use crate::common;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest;
use std::thread;
use std::time;

pub struct MusicBrainz {}

impl MusicBrainz {
  pub fn albums_for_artist(artist: &str) -> Result<Vec<common::Album>, reqwest::Error> {
    let encoded_artist = utf8_percent_encode(artist, NON_ALPHANUMERIC).to_string();
    let artist_arguments = format!("query=artist:{encoded_artist}");
    let response = Self::query("artist", &artist_arguments)?;
    let mut artists = response.json::<common::Artists>()?;

    // No artists found with that name
    if artists.artists.len() == 0 {
      return Ok(Vec::new());
    }

    let artist_id = Self::best_matching_artist(artist, &mut artists);
    let release_arguments = format!("artist={artist_id}&type=album");

    let response = Self::query("release-group", &release_arguments)?;

    let mut releases = response.json::<common::ReleaseGroups>()?;
    releases.release_groups.sort_by_key(|album| album.date.clone());

    return Ok(releases.release_groups);
  }

  pub fn query(entity: &str, query: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let response = reqwest::blocking::Client::new().get(format!("https://musicbrainz.org/ws/2/{entity}?{query}"))
        .header(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json"))
        .header(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("mpd-fresh/1.0 ( https://github.com/shanet/mpd-fresh )"))
        .send()?;

    if response.status().is_success() {
      return Ok(response);
    } else {
      return Err(response.error_for_status().unwrap_err());
    }
  }

  fn best_matching_artist(artist_name: &str, artists: &mut common::Artists) -> String {
    // Sort by score to get the best match (0-100 scale so we need it descending)
    artists.artists.sort_by_key(|artist| artist.score);
    artists.artists.reverse();

    // First try to find an exact name match
    for artist in &artists.artists {
      if artist.name.eq_ignore_ascii_case(artist_name) {
        return artist.id.clone();
      }
    }

    // If that fails just go with the highest score
    return artists.artists[0].id.clone();
  }

  pub fn rate_limit_wait() {
    // The MusicBrainz rate limit is 1 request/second/IP so wait just a bit longer than that
    thread::sleep(time::Duration::from_millis(1100));
  }
}
