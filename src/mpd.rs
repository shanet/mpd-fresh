use std::io::{self, BufRead, Write};
use std::net::{self, TcpStream};
use std::collections::{HashMap, HashSet};

pub struct MpdClient<'a> {
  pub host: &'a str,
  socket: Option<TcpStream>,
}

impl<'a> MpdClient<'a> {
  pub fn new(host: &str) -> MpdClient {
    return MpdClient {
      socket: None,
      host: host,
    };
  }

  pub fn connect(&mut self) -> Result<(), io::Error> {
    match TcpStream::connect(self.host) {
      Ok(socket) => {
        self.socket = Some(socket);
        return Ok(());
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

  pub fn get_artist_albums(&mut self) -> io::Result<HashMap<String, HashSet<String>>> {
    let response = self.send_command("list album group artist")?;

    let mut artists = HashMap::new();
    let mut recent_artist = None;

    for line in response {
      if line.starts_with("Artist: ") {
        let Some(artist) = line.strip_prefix("Artist: ") else { continue; };

        artists.insert(artist.to_owned(), HashSet::new());
        recent_artist = Some(artist.to_owned());
      } else if line.starts_with("Album: ") {
        let Some(ref artist) = recent_artist else { continue; };
        let Some(albums) = artists.get_mut(artist) else { continue; };
        let Some(album) = line.strip_prefix("Album: ") else { continue; };

        albums.insert(album.to_owned());
      }
    }

    return Ok(artists);
  }

  pub fn send_command(&mut self, command: &str) -> io::Result<Vec<String>> {
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
