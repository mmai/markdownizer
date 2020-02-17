use std::{env, fs, error};
use std::path::{Path, PathBuf, StripPrefixError};
use clap::{Arg, App, SubCommand};
use std::io::Read;

mod types;
mod parser;

// use parser::hello_parser;

fn main() -> Result<(), Box<dyn error::Error>> {
    let app = App::new("Markdown dashboard")
        .version("1.0")
        .author("Henri Bourcereau <henri@bourcereau.fr>")
        .about("A markdown based GTD system manager")
        .arg(Arg::with_name("directory")
             .short("d")
             .long("directory")
             .value_name("ROOT")
             .help("Directory path of the markdown files")
             .takes_value(true))
        .arg(Arg::with_name("relative_from")
             .short("r")
             .long("relative-to")
             .value_name("RELATIVETO")
             .help("Path from which display relative paths")
             .takes_value(true))
        .subcommand(SubCommand::with_name("list")
                    .about("show projects"));

    let matches = app.get_matches();
    let current = get_current_dir();
    let root = matches.value_of("directory").unwrap_or(&current);
    let relative_from = matches.value_of("relative_from").unwrap_or(&current);

	// println!("{:?}", hello_parser("hello world"));
	// println!("{:?}", hello_parser("goodbye hello again"));
    // println!("Value for directory: {}", root);
    //
    traverse(root, relative_from).unwrap();
        Ok(())
}

fn get_current_dir() -> String {
    env::current_dir()
    .map( |cd| 
          String::from(cd.as_path().to_str().unwrap())
    ).expect("Can't find current path")
}

fn traverse(root: &str, relative_from: &str) -> Result<(), std::io::Error> {
// fn traverse(root: &str) -> Result<(), Box<dyn error::Error>> {
    let path = std::path::PathBuf::from(root);
    let path = path.as_path();
    let relative_from = std::path::PathBuf::from(relative_from);
    let relative_from = relative_from.as_path();
    for entry in fs::read_dir(path)? {
        let path: PathBuf = entry?.path();

        let mut file = std::fs::File::open(&path)?;
        // let buf_reader = std::io::BufReader::new(file);

        // let relative_path: &PathBuf = path.as_path().strip_prefix(relative_from)?.into();
        let relative_path = path.clone();
        let relative_path: &Path = relative_path.as_path().strip_prefix(relative_from).unwrap_or(&path);
        // let relative_path = &path.strip_prefix(relative_from).or_else(path.clone());

        let mut s = String::new();
        file.read_to_string(&mut s)?; 
        // let (_, project) = parser::project(&s)?;
        match parser::project(&s) {
            Ok((_, project)) => println!("({})[{:?}] ({})", project.title, relative_path, project.tasks.len()),
            e => println!("Not a project: {:?} {:?}", path, e)
            // e => println!("Not a project: {:?} {:?}", path, e)
        }

        // println!("{:?}", );
        // {
        // match file.read_to_string(&mut s) {
        //     Err(why) => panic!("couldn't read {}: {}", display,
        //                        why.description()),
        //     Ok(_) => print!("{} contains:\n{}", display, s),
        // }
        //

        // println!("{:?}", path);
    }

    Ok(())
}
