#[derive(Clone, Default, Debug, PartialEq)]
pub struct Project {
    pub title: std::string::String,
    pub status: std::string::String,
    // pub velocity: u32,
    // pub velocity: u32,
    pub tasks: Vec<Task>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Task {
    pub title: std::string::String,
    pub done: bool,
    pub time_spent: u32,
    pub time_estimate: Option<usize>,
    pub tasks: Vec<Task>,
}
