use serde;

#[derive(serde::Deserialize)]
pub struct Artist {
  pub id: String,
  pub score: i32,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Album {
  pub title: String,
  #[serde(rename="first-release-date", skip_serializing)]
  pub date: Option<String>,
}

impl PartialEq for Album {
  fn eq(&self, other: &Self) -> bool {
    return self.title == other.title;
  }
}

#[derive(serde::Deserialize)]
pub struct Artists {
  pub artists: Vec<Artist>,
}

#[derive(serde::Deserialize)]
pub struct ReleaseGroups {
  #[serde(rename = "release-groups")]
  pub release_groups: Vec<Album>,
}
