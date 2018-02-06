use std::io::{Read, BufReader};
use types::CSProject;
use xml::reader::{EventReader, XmlEvent, Result as XmlResult};

pub fn parse_sln<T>(source: T) -> CSProject where T: Read {
  let parser = EventReader::new(BufReader::new(source));
  let package_refs = Vec::new();
  let project_refs = Vec::new();

  for element in parser {

  }

  CSProject::new(package_refs, project_refs)
}

pub fn parse_item_group<T>(parser: EventReader<T>) where T: Read {

}

pub fn is_eof(result: &XmlResult<XmlEvent>) -> bool {
  match result {
    &Ok(XmlEvent::EndDocument) => true,
    &Err(_) => true,
    _ => false
  }
}