use reqwest;

pub struct MusicBrainz {}

impl MusicBrainz {
  pub fn query() -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::Client::new().get("https://musicbrainz.org/ws/2/artist/?query=artist:Radiohead&fmt=json")
        .header(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("mpd-fresh/1.0 ( https://github.com/shanet/mpd-fresh )"))
        .send()?;

    if response.status().is_success() {
      let body = response.text()?;
      println!("Response Body: {}", body);
      return Ok(body);
    } else {
      return Err(response.error_for_status().unwrap_err());
    }
  }
}
