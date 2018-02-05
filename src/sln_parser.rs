use std::io::{Read, BufReader, BufRead};
use types::VSProject;
use regex::Regex;

pub fn parse_sln<T>(source: T) -> Vec<VSProject> where T: Read {
  lazy_static! {
    static ref PROJECT_RE: Regex = Regex::new(r#"Project\([^)]+\)[ ]*=[ ]*"([^"]+)"[ ]*,[ ]*"([^"]+)"[ ]*,[ ]"\{([a-zA-Z0-9\-]+)\}""#).unwrap();
  }

  let reader = BufReader::new(source);
  let mut projects = Vec::new();
  for line in reader.lines().map(|l| l.unwrap()) {
    match PROJECT_RE.captures(&line) {
      Some(caps) => if caps.len() == 4 {
        projects.push(VSProject::new(&caps[3], &caps[1], &caps[2]));
      },
      _ => ()
    }
  }

  projects
}