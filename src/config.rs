use once_cell;
use std::io::{self, Write};

static mut PARSED_ARGS: once_cell::sync::Lazy<clap::ArgMatches> = once_cell::sync::Lazy::new(|| parse_arguments());

pub fn is_verbose() -> bool {
  unsafe {
    return *PARSED_ARGS.get_one::<bool>("verbose").unwrap();
  }
}

pub fn ignore_all_albums() -> bool {
  unsafe {
    return *PARSED_ARGS.get_one::<bool>("ignore").unwrap();
  }
}

pub fn mpd_host() -> String {
  unsafe {
    return format!("{}:{}", PARSED_ARGS.get_one::<String>("server").unwrap(), PARSED_ARGS.get_one::<String>("port").unwrap());
  }
}

pub fn mpd_password() -> Option<String> {
  unsafe {
    return PARSED_ARGS.get_one::<String>("password").cloned();
  }
}

pub fn print_status(message: &str) {
  static mut LAST_MESSAGE_LENGTH: usize = 0;

  if is_verbose() {
    println!("{}", message);
  } else {
    unsafe {
      for _ in 0..LAST_MESSAGE_LENGTH {
        print!("\x08 \x08");
      }
    }

    print!("{}", message);
    let _ = io::stdout().flush();
  }

  unsafe {
    LAST_MESSAGE_LENGTH = message.chars().count();
  }
}

fn parse_arguments() -> clap::ArgMatches {
  return clap::Command::new("mpd-fresh")
    .arg(clap::Arg::new("server")
      .short('s')
      .long("server")
      .help("MPD server to connect to")
      .default_value("localhost"))
    .arg(clap::Arg::new("port")
      .short('p')
      .long("port")
      .help("MPD port to connect to")
      .default_value("6600"))
    .arg(clap::Arg::new("password")
      .short('w')
      .long("password")
      .help("MPD password to use"))
    .arg(clap::Arg::new("ignore")
      .short('i')
      .long("ignore")
      .help("Ignore all new releases (useful for an initial run to avoid many prompts)")
      .num_args(0))
    .arg(clap::Arg::new("verbose")
      .short('v')
      .long("verbose")
      .help("Be louder")
      .num_args(0))
    .get_matches();
}
