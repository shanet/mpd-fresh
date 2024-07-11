use crate::common;
use crate::config;
use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};
use std::net::{self, TcpStream};

pub struct MpdClient<'a> {
  pub host: &'a str,
  password: Option<&'a str>,
  socket: Option<TcpStream>,
}

impl<'a> MpdClient<'a> {
  pub fn new(host: &'a str, password: Option<&'a str>) -> MpdClient<'a> {
    return MpdClient {
      socket: None,
      host: host,
      password: password,
    };
  }

  pub fn connect(&mut self) -> Result<(), io::Error> {
    match TcpStream::connect(self.host) {
      Ok(socket) => {
        self.socket = Some(socket);

        match self.password {
          Some(_) => {
            return self.authenticate();
          }

          None => {
            return Ok(());
          }
        }
      }

      Err(error) => {
        return Err(error);
      }
    }
  }

  pub fn disconnect(&mut self) -> io::Result<()> {
    match self.socket {
      Some(ref mut socket) => {
        return socket.shutdown(net::Shutdown::Both);
      }

      None => {
        return Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to server"));
      }
    }
  }

  pub fn all_albums(&mut self, artist: Option<&str>) -> io::Result<BTreeMap<String, Vec<common::Album>>> {
    let mut artists = BTreeMap::new();

    let command = match artist {
      Some(artist) => {
        artists.insert(artist.to_owned(), Vec::new());
        format!("list album artist {}", artist)
      }
      None => { "list album group artist".to_string() }
    };

    let response = self.send_command(&command)?;

    let mut recent_artist = None;
    let artist_count = self.artist_count().unwrap_or(0);
    let mut artist_index = 1;

    for line in response {
      // The response format is a flat list of "Artist:" lines followed by "Album:" lines so we need
      // to track what the last encountered artist was so it can be matched to it's following albums
      if line.starts_with("Artist: ") {
        let Some(artist) = line.strip_prefix("Artist: ") else { continue; };

        // Skip any blank artists
        if artist == "" {
          recent_artist = None;
          continue;
        }

        artists.insert(artist.to_owned(), Vec::new());
        recent_artist = Some(artist.to_owned());

        config::print_status(&format!("{}/{}: {}", artist_index, artist_count, artist));
        artist_index += 1;
      } else if line.starts_with("Album: ") {
        let Some(ref artist) = recent_artist else { continue; };
        let Some(albums) = artists.get_mut(artist) else { continue; };
        let Some(album) = line.strip_prefix("Album: ") else { continue; };

        let Ok(release_date) = self.album_release_date(artist, album) else {
          if config::is_verbose() { eprintln!("Release date not found for: {} - {}", artist, album); }
          continue;
        };

        let release = common::Album {
          title: album.to_owned(),
          date: Some(release_date.to_owned()),
        };

        // It's kind of inefficient to sort after every push here, but most artists will only have a few albums so it's not a big deal
        albums.push(release);
        albums.sort_by_key(|album| album.date.clone());
      }
    }

    return Ok(artists);
  }

  pub fn album_release_date(&mut self, artist: &str, album: &str) -> io::Result<String> {
    let response = self.send_command(&format!("find album \"{album}\" artist \"{artist}\""))?;

    for line in response {
      if line.starts_with("Date: ") {
        let Some(release_date) = line.strip_prefix("Date: ") else { continue; };
        return Ok(release_date.to_string());
      }
    }

    return Err(io::Error::new(io::ErrorKind::Other, "No release date found for album"));
  }

  fn artist_count(&mut self) -> io::Result<i32> {
    let response = self.send_command("list artist")?;
    let mut count = 0;

    for line in response {
      if line.starts_with("Artist: ") {
        count += 1;
      }
    }

    return Ok(count);
  }

  fn authenticate(&mut self) -> Result<(), io::Error> {
    let command = format!("password {}", self.password.unwrap());

    match self.send_command(&command) {
      Ok(response) => {
        if response.len() == 1 && response[0] == "OK" {
          return Ok(());
        } else {
          let message = format!("MPD authentication failed: {}", response.join("\n"));
          return Err(io::Error::new(io::ErrorKind::Other, message));
        }
      }

      Err(error) => {
        return Err(error);
      }
    }
  }

  fn send_command(&mut self, command: &str) -> io::Result<Vec<String>> {
    self.send(command)?;
    return self.receive();
  }

  fn send(&mut self, command: &str) -> io::Result<()> {
    match self.socket {
      Some(ref mut socket) => {
        socket.write_all(command.as_bytes())?;
        return socket.write_all(b"\n");
      }

      None => {
        return Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to server"));
      }
    }
  }

  fn receive(&mut self) -> io::Result<Vec<String>> {
    match self.socket {
      Some(ref socket) => {
        let reader = io::BufReader::new(socket);
        let mut response = Vec::new();

        for line in reader.lines() {
          let line = line?;

          if line == "OK" || line.starts_with("ACK") {
            break;
          }

          response.push(line);
        }

        return Ok(response);
      }

      None => {
        return Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to server"));
      }
    }
  }
}
