use crate::types::{Project, Task, Status};

#[derive(Clone, Debug, PartialEq)]
pub struct MetaData {
    pub status: Option<Status>,
    pub tags: std::vec::Vec<std::string::String>,
}

use nom::{multi, character, combinator};
// use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

pub fn project(input: &str) -> nom::IResult<&str, Project> {
    let (input, meta) = combinator::opt(parsers::front_matter)(input)?;
    let (input, _) = multi::many0(character::complete::line_ending)(input)?;
    let (input, title) = parsers::title(input)?;
    let (input, _description) = parsers::description(input)?;
    let (input, tasks) = parsers::tasks(input)?;
    Ok((input, Project {
        title: title.into(),
        status: meta.map(|m| m.status).unwrap_or(Some(Status::Maybe)),
        tasks
    }))
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
    use nom::{combinator, bytes, branch, character, sequence, multi};
    use super::{MetaData, Task};
    use yaml_rust::YamlLoader;
    use std::convert::TryInto;

    fn yaml_delimiter(input: &str) -> nom::IResult<&str, ()> {
        let (input, _) = combinator::opt(character::complete::line_ending)(input)?;
        let (input, _) = bytes::complete::tag("---")(input)?;
        let (input, _) = character::complete::line_ending(input)?;
        Ok((input, ()))
    }

    pub fn front_matter(input: &str) -> nom::IResult<&str, MetaData> {
        let (input, yaml) = sequence::delimited( yaml_delimiter, 
                                                 bytes::complete::take_until("\n---"), 
                                                 yaml_delimiter
        )(input)?;
        let docs = YamlLoader::load_from_str(yaml).unwrap();
        let doc = &docs[0];

        // println!("debug {:?}", doc);
        let metadata = MetaData {
            status: doc["status"].as_str().unwrap().try_into().ok(),
            tags: vec![]
        };
        Ok((input, metadata))
    }

    pub fn description(input: &str) -> nom::IResult<&str, String> {
        let (input, description) = bytes::complete::take_until("\n## ")(input)?;
        Ok((input, description.into()))
    }

    fn task_status(input: &str) -> nom::IResult<&str, bool> {
        branch::alt((
            combinator::value(true, bytes::complete::tag("[x] ")),
            combinator::value(false, combinator::opt(bytes::complete::tag("[ ] ")))
        ))(input)
    }

    fn task_estimate(input: &str) -> nom::IResult<&str, usize> {
        let (input, estimate_val) = character::complete::digit1(input)?;
        let (input, estimate_unit) = branch::alt((
                bytes::complete::tag("j"),
                bytes::complete::tag("d"),
                bytes::complete::tag("h"),
                bytes::complete::tag("mn"),
        ))(input)?;
        let (input, _) = bytes::complete::tag(" ")(input)?;
        let multiplicator = match estimate_unit {
            "j" => 24 * 60,
            "d" => 24 * 60,
            "h" => 60,
            "mn" => 1,
            _ => 0
        };
        let estimate_val: usize = estimate_val.parse().unwrap();
        Ok((input, estimate_val * multiplicator))
    }

    fn task(level: usize) -> impl Fn(&str) -> nom::IResult<&str, Task>
    {
        move |input: &str| {
            let (input, _) = multi::many_m_n(level, level, bytes::complete::tag("  "))(input)?;
            let (input, _) = bytes::complete::tag("* ")(input)?;
            let (input, done) = task_status(input)?;
            let (input, time_estimate) = combinator::opt(task_estimate)(input)?;
            let (input, title) = character::complete::not_line_ending(input)?;
            let (input, _) = character::complete::line_ending(input)?;
            let (input, tasks) = multi::many0(task(level + 1))(input)?;
            let task = Task {
                title: title.into(),
                done: done,
                time_spent: 0,
                time_estimate: time_estimate,
                tasks: tasks,
            };
            Ok((input, task))
        }

    }

    pub fn tasks(input: &str) -> nom::IResult<&str, Vec<Task>> {
        let (input, _) = combinator::opt(character::complete::line_ending)(input)?;
        let (input, _) = bytes::complete::tag("## Tasks")(input)?;
        let (input, _) = multi::many1(character::complete::line_ending)(input)?;
        let (input, tasks) = multi::many0(task(0))(input)?;
        Ok((input, tasks))
    }

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
        fn test_front_matter() {
            let meta = "---\nstatus: active\ntags:\n---\no";
            assert_eq!(front_matter(meta), Ok(("o", MetaData {status: "active".try_into().ok(), tags: vec![]})));
        }

        #[test]
        fn test_task() {
            let taskstr = "* principale\n  * [ ] sous 1\n  * [x] sous 2\n* une autre";
            assert_eq!(task(0)(taskstr), Ok(("* une autre", Task {
                title: "principale".into(),
                done: false,
                time_spent:0,
                time_estimate:None,
                tasks: vec![
                    Task {
                        title: "sous 1".into(),
                        done: false,
                        time_spent:0,
                        time_estimate:None,
                        tasks: vec![]
                    },
                    Task {
                        title: "sous 2".into(),
                        done: true,
                        time_spent:0,
                        time_estimate:None,
                        tasks: vec![]
                    }
                ]
            })));
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
    use std::convert::TryInto;

    #[test]
    fn test_project() {
        let input = "
---
status: active
---

Lectures liens
==============

## Tasks

* 1h taguer liens nons tagués
  * toread, reference
  * catégories
* compteur liens à lire
* migration wallabag ?
";
        assert_eq!(project(input), Ok(("", Project {
            title: "Lectures liens".into(),
            status: "active".try_into().ok(),
            tasks: vec![
                Task {
                    title: "taguer liens nons tagués".into(),
                    done: false,
                    time_spent:0,
                    time_estimate:Some(60),
                    tasks: vec![
                        Task {
                            title: "toread, reference".into(),
                            done: false,
                            time_spent:0,
                            time_estimate:None,
                            tasks: vec![]
                        },
                        Task {
                            title: "catégories".into(),
                            done: false,
                            time_spent:0,
                            time_estimate:None,
                            tasks: vec![]
                        }
                    ]
                },
                Task {
                    title: "compteur liens à lire".into(),
                    done: false,
                    time_spent:0,
                    time_estimate:None,
                    tasks: vec![]
                },
                Task {
                    title: "migration wallabag ?".into(),
                    done: false,
                    time_spent:0,
                    time_estimate:None,
                    tasks: vec![]
                },
            ]
        })));
    }
}
