#![feature(str_split_at)]
#![feature(path_ext)]
extern crate url;
extern crate hyper;

pub mod url_type {
  use std::fmt;

  pub fn url_type(url: &str) -> UrlType {
    if url.contains("youtube") {
      return UrlType::Youtube;
    } else if url.contains("torrent:") {
      return UrlType::Torrent;
    }

    UrlType::Direct
  }

  pub enum UrlType {
    Youtube,
    Direct,
    Torrent
  }

  impl fmt::Display for UrlType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let display = match *self {
        UrlType::Youtube => "YouTube",
        UrlType::Direct => "Direct",
        UrlType::Torrent => "Torrent"
      };

      write!(f, "{}", display)
    }
  }
}

pub mod query {
  use url::{Url};

  pub struct Query {
    pub url: String,
    pub quality: String,
    pub path: String
  }

  pub fn parse(full_url: &str) -> Query {
    let url = Url::parse(&full_url).unwrap();
    let pairs = Url::query_pairs(&url).unwrap();

    let mut query = Query {
      url: "".to_string(),
      quality: "".to_string(),
      path: "Other".to_string()
    };

    for x in &pairs {
      let (key, value) = x.clone();

      let value = value.clone();
      match key.as_ref() {
        "url" => query.url = value,
        "path" => query.path = value,
        "quality" => query.quality = value,
        _ => {}
      };
    }

    query
  }
}

pub mod download {
  use super::query::Query;
  use std::fs::{File};
  use std::io::prelude::*;

  use hyper::Client;
  use std::path::{Path, PathBuf};

  pub fn direct(query: &Query) -> Result<PathBuf, String> {
    let url = &query.url;
    let client = Client::new();

    let mut res = client.get(url).send().unwrap();

    let name = Path::new(url).file_name().unwrap();
    let path = Path::new("/srv/ftp").join(&query.path).join(name);

    if path.exists() {
      return Err("File already exists".to_string());
    }

    let mut file = try!(File::create(path.clone()).map_err(|e| e.to_string()));

    let mut buffer = Vec::new();
    try!(res.read_to_end(&mut buffer).map_err(|e| e.to_string()));

    match file.write_all(&buffer) {
      Ok(..) => Ok(path),
      Err(e) => Err(e.to_string())
    }
  }

  use std::process::Command;

  pub fn youtube(query: &Query) -> Result<PathBuf, String> {
    let url = &query.url;
    let mut command = Command::new("youtube-dl");

    let path = Path::new("/srv/ftp").join(&query.path);
    command.current_dir(path.clone());

    command.arg(url);
    if !query.quality.is_empty() {
      command.arg("-f");
      command.arg(&query.quality);
    }


    match command.output() {
      Ok(output) => {
        if output.status.success() {
          println!("before stdout");
          let stdout = String::from_utf8(output.stdout).unwrap();
          println!("stdout {}", stdout);

          let keyword = "Destination: ";
          let destination = match stdout.find(keyword) {
            Some(value) => value,
            None => return Err("File already exists".to_string())
          };
          let start_index = destination + keyword.len();

          let (_, sec) = stdout.split_at(start_index);
          let linebreak_index = sec.find("\n").unwrap();

          let (file_name, _) = sec.split_at(linebreak_index);

          Ok(path.join(file_name))
        } else {
          Err("youtube-dl error: ".to_string())
        }
      },
      Err(e) => Err("Internal error: ".to_string() + &e.to_string())
    }
  }
}
