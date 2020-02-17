use std::string::String;
use std::convert::TryFrom;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Project {
    pub title: String,
    pub status: Option<Status>,
    // pub velocity: u32,
    // pub velocity: u32,
    pub tasks: Vec<Task>,
}

// impl Project {
//     pub fn time_estimate(p: Self) -> Option<usize> {
//         p.tasks.into_iter().sum()
//
//     }
// }

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Task {
    pub title: std::string::String,
    pub done: bool,
    pub time_spent: u32,
    pub time_estimate: Option<usize>,
    pub tasks: Vec<Task>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Ignore,
    Abandoned,
    Maybe,
    Paused,
    Active
}

impl TryFrom<&str> for Status {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ignore" => Ok(Self::Ignore),
            "abandoned" => Ok(Self::Abandoned),
            "maybe" => Ok(Self::Maybe),
            "paused" => Ok(Self::Paused),
            "active" => Ok(Self::Active),
            _ => Err(())
        }
    }
}
