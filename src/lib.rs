use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use chrono::{DateTime, Utc};

const TAGS_SEP: &str = ",";

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
        TodoEntry {
            id: 0,
            title: String::new(),
            desc: String::new(),
            due: Utc::now(),
            project: String::new(),
            tags: Vec::new(),
        }
    }
}

impl Default for TodoEntry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TodoPersistDao {
    path: String,
    entries: Vec<TodoEntry>,
}

impl TodoPersistDao {
    pub fn new(path: &str) -> Self {
        TodoPersistDao {
            path: String::from(path),
            entries: Vec::new(),
        }
    }

    pub fn load(&mut self) -> Result<(), io::Error> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if line.is_ok() {
                let line = line.unwrap();
                if line.ends_with("\tdescription\n") {
                    continue;
                }
                let mut entry = TodoEntry::new();
                let mut tokens = line.split('\t');

                entry.id = tokens.next().expect("Could not parse task id!").parse().unwrap();
                entry.project.insert_str(0, tokens.next().expect("Could not read task's project!"));
                for tag in tokens.next().expect("Can't read tags").split(TAGS_SEP) {
                    entry.tags.push(tag.to_string());
                }
                entry.due = DateTime::parse_from_rfc3339(tokens.next().expect("Could not read date")).expect("Could not parse due date!").with_timezone(&Utc);
                entry.title.insert_str(0, tokens.next().expect("Could not read title"));
                entry.desc.insert_str(0, tokens.next().expect("Could not read description"));
                self.entries.push(entry);
            }
        }
        Ok(())
    }


    pub fn dump(&self) -> Result<(), io::Error> {
        if !Path::new(&self.path).exists() {
            File::create(&self.path)?;
        }
        let file = File::open(&self.path)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "id\tproject\ttags\tdue date\ttitle\tdescription").expect("Could not write to file!");
        for entry in &self.entries {
            writeln!(writer, "{}\t:{}\t+{}\t{}\t\"{}\"\t\"{}\"", entry.id, entry.project, entry.tags.join(TAGS_SEP), entry.due.to_rfc3339(), entry.title, entry.desc).expect("Could not write to file!");
        }
        Ok(())
    }
}
