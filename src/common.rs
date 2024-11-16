use serde;

#[derive(serde::Deserialize)]
pub struct Artist {
  pub id: String,
  pub name: String,
  pub score: i32,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Album {
  pub title: String,
  pub date: Option<String>,
}

impl PartialEq for Album {
  fn eq(&self, other: &Self) -> bool {
    return self.title.eq_ignore_ascii_case(&other.title);
  }
}

#[derive(serde::Deserialize)]
pub struct Artists {
  pub artists: Vec<Artist>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Release {
  pub title: String,
  #[serde(rename="first-release-date", skip_serializing)]
  pub date: String,
  #[serde(rename = "primary-type")]
  pub primary_type: String,
  #[serde(rename = "secondary-types")]
  pub secondary_types: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct ReleaseGroups {
  #[serde(rename = "release-groups")]
  pub release_groups: Vec<Release>,
}
