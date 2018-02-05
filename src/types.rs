pub enum FileType {
    Unknown,
    Sln,
    Csproj
}

pub struct VSProject {
  pub guid: String,
  pub name: String,
  pub path: String,
}

impl VSProject {
  pub fn new(guid: &str, name: &str, path: &str) -> VSProject {
    VSProject {
      guid: guid.to_string(),
      name: name.to_string(),
      path: path.to_string()
    }
  }
}