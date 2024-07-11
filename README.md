mpd-fresh
=========

mpd-fresh is a small utility that scans your MPD library for any new releases from your artists.

It relies on the [MusicBrainz API](https://musicbrainz.org/) to check for new releases for a given artist. This API is limited to one request per second so scanning a large music library may take a while to iterate through all artists.

mpd-fresh is designed to only notify you of new releases, not all missing releases. A new release for a given artist is considered any release more recent than the most recent release already in your library. For example, consider the following artist & albums:

```
Artist:
  Album 1 (2020) <- Present in library
  Album 2 (2021) <- NOT present in library
  Album 3 (2022) <- Present in library
  Album 4 (2023) <- NOT present in library
```

In this case, mpd-fresh would only identify "Album 4" as a new album as it is not present in the library already AND is it newer than the most recent album in the library, Album 3.

mpd-fresh will prompt you to ignore any found new releases so as to not keep asking about new releases you may never include in your library. This ignore file is stored in YAML format at `~/.config/mpd_fresh_ignored.yml`.

## Installation

```bash
$ cargo install mpd-fresh
$ mpd-fresh --help
```

## Usage

If your MPD server is running on the standard port and local host then simply running `mpd-fresh` should be sufficient.

Unless your library is already fairly complete, on the first run it is likely useful to use `--ignore` to automatically ignore all found new releases.

Full options include:

```
Usage: mpd-fresh [OPTIONS]

Options:
  -s, --server <server>      MPD server to connect to [default: localhost]
  -p, --port <port>          MPD port to connect to [default: 6600]
  -w, --password <password>  MPD password to use
  -i, --ignore               Ignore all new releases (useful for an initial run to avoid many prompts)
  -a, --artist <artist>      Only check a single artist
  -v, --verbose              Be louder
  -h, --help                 Print help
```

## Development

With Rust & Cargo installed, the following will compile the program:

```bash
$ cargo build
```

To both build and run and include command line arguments:

```bash
$ cargo run -- [command line args]
```

### Publishing

For updating the release on crates.io:

```bash
$ cargo build --release
$ cargo publish
```

## License

GPLv3
