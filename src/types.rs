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

pub struct PackageReference {
  pub name: String,
  pub version: String,
}

impl PackageReference {
  pub fn new(name: &str, version: &str) -> PackageReference {
    PackageReference {
      name: name.to_string(),
      version: version.to_string(),
    }
  }
}

pub struct CSProject {
  pub package_refs: Vec<PackageReference>,
  pub project_refs: Vec<String>,
}

impl CSProject {
  pub fn new(package_refs: Vec<PackageReference>, project_refs: Vec<String>) -> CSProject {
    CSProject { package_refs, project_refs }
  }
}