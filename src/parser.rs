use nom::{combinator, bytes};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Project {
    pub title: std::string::String,
    // pub device: std::string::String,
    // pub mount_point: std::string::String,
    // pub file_system_type: std::string::String,
    // pub options: std::vec::Vec<std::string::String>,
}

pub fn project(input: &str) -> nom::IResult<&str, Project> {
    let (input, title) = parsers::title(input)?;
    Ok((input, Project { title: title.into() }))
    // combinator::map(bytes::complete::is_not("\r\n"), |str: &str| Project {title: str.into()})(i)
}


// pub fn projects() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
// 	let file = std::fs::File::open("/proc/mounts")?;
// 	let buf_reader = std::io::BufReader::new(file);
// 	for line in buf_reader.lines() {
// 		match parsers::parse_line(&line?[..]) {
// 			Ok( (_, m) ) => {
// 				println!("{}", m);
// 			},
// 			Err(_) => return Err(ParseError::default().into())
// 		}
// 	}
// 	Ok(())
// }

pub(self) mod parsers {
    use nom::{combinator, bytes, branch, character};
    use super::Project;

    fn title_hash(input: &str) -> nom::IResult<&str, &str> {
        let (input, _) = bytes::complete::tag("#")(input)?;
        let (input, _) = character::complete::space1(input)?;
        let (input, title) = character::complete::not_line_ending(input)?;
        let (input, _) = character::complete::line_ending(input)?;
        Ok((input, title))
    }

    fn title_underline(input: &str) -> nom::IResult<&str, &str> {
        let (input, _) = character::complete::space0(input)?;
        let (input, title) = character::complete::not_line_ending(input)?;
        let (input, _) = character::complete::line_ending(input)?;
        let (input, _) = bytes::complete::is_a("=")(input)?;
        let (input, _) = character::complete::line_ending(input)?;
        Ok((input, title))
    }

    pub fn title(input: &str) -> nom::IResult<&str, &str> {
        branch::alt((title_hash, title_underline))(input)
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_title_hash() {
            assert_eq!(title_hash("# toto\n"), Ok(("", "toto")));
            assert_eq!(title_hash("# titi\naa"), Ok(("aa", "titi")));
            assert_eq!(title_hash("#   titi\naa"), Ok(("aa", "titi")));
            assert_eq!(title_hash(" # toto"),  Err(nom::Err::Error((" # toto", nom::error::ErrorKind::Tag))));
        }

        #[test]
        fn test_title_underline() {
            assert_eq!(title_underline("toto\n===\n"), Ok(("", "toto")));
            assert_eq!(title_underline("toto\n===\naaa"), Ok(("aaa", "toto")));
            assert_eq!(title_underline("toto\r\n===\n"), Ok(("", "toto")));
            assert_eq!(title_underline("toto\n===aaa"), Err(nom::Err::Error(("aaa", nom::error::ErrorKind::CrLf))));
        }

        #[test]
        fn test_title() {
            assert_eq!(title("# toto\n"), Ok(("", "toto")));
            assert_eq!(title("toto\n===\naaa"), Ok(("aaa", "toto")));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project() {
        assert_eq!(project("# abcd\nefg"), Ok(("efg", Project { title: "abcd".into() })));
        assert_eq!(project("abcd\n===\n"), Ok(("", Project { title: "abcd".into() })));
    }
}
