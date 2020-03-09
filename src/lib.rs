use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;

use derive_more::Display;
use std::convert::From;

pub mod types;
mod parser;

use types::{ Stored, Project };

#[derive(Debug, Display)]
pub enum MarkdownizerError {
    #[display(fmt = "IO Error")]
    IOError,

    #[display(fmt = "ParseError on file {} : {}", _0, _1)]
    ParseError(String, String),
}

impl From<std::io::Error> for MarkdownizerError {
    fn from(_: std::io::Error) -> MarkdownizerError {
        MarkdownizerError::IOError
    }
}

// impl From<nom::Err> for MarkdownizerError {
//     fn from(_: nom::Err) -> MarkdownizerError {
//         MarkdownizerError::ParseError("bbbad".into())
//     }
// }

pub struct Markdownizer {
    root: PathBuf,
}


impl Markdownizer {
    // pub fn new(root: &str) -> Markdownizer {
    //     let root = std::path::PathBuf::from(root);
    //     let root = root.as_path();
    //     Markdownizer { root: path }
    // }
    pub fn new(root: &PathBuf) -> Markdownizer {
        Markdownizer { root: PathBuf::from(root) }
    }

    pub fn _project_list(&self) -> Result<Vec<Project>, MarkdownizerError> {
        fs::read_dir(&self.root)?.into_iter().map (|entry|{
            let project_path = entry?.path();
            let project = read_project(&project_path);
            project
            // read_project(&entry?.path())
        }).collect()
    }

    pub fn project_list(&self) -> Result<Vec<Stored<Project>>, MarkdownizerError> {
        fs::read_dir(&self.root)?.into_iter().map (|entry| {
            let location = entry?.path();
            read_project(&location).map(|entity|
               Stored { entity, location }
           )
        }).collect()
    }
}

fn read_project(path: &Path) -> Result<Project, MarkdownizerError> {
    let mut file = std::fs::File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    match parser::project(&s) {
        Ok((_, project)) => Ok(project),
        Err(e) => Err(MarkdownizerError::ParseError(path.display().to_string(), format!("{:?}", e)))
    }
}

