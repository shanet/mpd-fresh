use serde;

#[derive(serde::Deserialize)]
pub struct Artist {
  pub id: String,
  // name: String,
  pub score: i32,
}

// TODO: move this to a separate file for abstraction
#[derive(serde::Deserialize)]
pub struct Release {
  // id: String,
  pub title: String,
  #[serde(rename = "first-release-date")]
  pub date: String,
}

#[derive(serde::Deserialize)]
pub struct Artists {
  pub artists: Vec<Artist>,
}

#[derive(serde::Deserialize)]
pub struct Releases {
  #[serde(rename = "release-groups")]
  pub release_groups: Vec<Release>,
}
