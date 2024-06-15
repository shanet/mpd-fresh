use crate::common;
use reqwest;

pub struct MusicBrainz {}

impl MusicBrainz {
  pub fn releases_for_artist(artist: &str) -> Result<Vec<common::Release>, reqwest::Error> {
    let artist_arguments = format!("query=artist:{artist}");
    let response = Self::query("artist", &artist_arguments)?;
    let mut artists = response.json::<common::Artists>()?;

    artists.artists.sort_by_key(|artist| artist.score);
    let artist_id = &artists.artists[0].id;

    let release_arguments = format!("artist={artist_id}&type=album");
    let response = Self::query("release-group", &release_arguments)?;
    let mut releases = response.json::<common::Releases>()?;

    releases.release_groups.sort_by_key(|album| album.date.clone());

    return Ok(releases.release_groups);
  }

  pub fn query(entity: &str, arguments: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    // println!("{}", format!("https://musicbrainz.org/ws/2/{entity}?{arguments}"));

    let response = reqwest::blocking::Client::new().get(format!("https://musicbrainz.org/ws/2/{entity}?{arguments}"))
        .header(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json"))
        .header(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("mpd-fresh/1.0 ( https://github.com/shanet/mpd-fresh )"))
        .send()?;

    if response.status().is_success() {
      return Ok(response);
    } else {
      return Err(response.error_for_status().unwrap_err());
    }
  }
}
