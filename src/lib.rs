use chrono::{Datelike, DateTime, Utc};
use chrono::format::Fixed::TimezoneName;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Structure holding all the info to represent a task item
/// id may be auto-generated
pub struct TodoEntry {
    id: u32,
    title: String,
    desc: String,
    due: DateTime<Utc>,
    project: String,
    tags: Vec<String>,
}

impl TodoEntry {
    pub fn new() -> Self {
        TodoEntry { id: 0, title: String::new(), desc: String::new(), due: Utc::now(), project: String::new(), tags: Vec::new() }
    }
}
