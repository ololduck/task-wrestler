use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, LineWriter, Write};

use chrono::{DateTime, Utc};
#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

const TAGS_SEP: &str = ",";
const DEFAULT_VALUE: &str = "-";

/// Structure holding all the info to represent a task item
/// id may be auto-generated
#[derive(Debug)]
pub struct TaskEntry {
    pub id: u32,
    pub title: String,
    pub desc: Option<String>,
    pub due: Option<DateTime<Utc>>,
    pub project: Option<String>,
    pub tags: Vec<String>,
}

impl TaskEntry {
    pub fn new() -> Self {
        TaskEntry {
            id: 0,
            title: String::from(DEFAULT_VALUE),
            desc: None,
            due: None,
            project: None,
            tags: Vec::new(),
        }
    }
}

impl Default for TaskEntry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TaskDao {
    path: String,
    pub entries: Vec<TaskEntry>,
}

impl TaskDao {
    pub fn new(path: &str) -> Self {
        TaskDao {
            path: String::from(path),
            entries: Vec::new(),
        }
    }

    /// load tasks from disk
    /// ```
    /// use task_wrestler::{TaskDao, TaskEntry};
    /// use std::fs::remove_file;
    /// let fname = "/tmp/persist_dao_test_read.txt";
    /// let mut dao = TaskDao::new(fname);
    /// let mut task = TaskEntry::default();
    /// task.id = 0;
    /// task.title = "testing".to_string();
    /// dao.entries.push(task);
    /// dao.dump().expect("Could not write to disk!");
    /// let mut dao_read = TaskDao::new(fname);
    /// assert!(dao_read.load().is_ok());
    /// dbg!(&dao_read.entries);
    /// assert_eq!(dao_read.entries.len(), 1);
    /// assert_eq!(dao_read.entries[0].id, 0);
    /// remove_file(fname);
    /// ```
    pub fn load(&mut self) -> Result<(), io::Error> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut is_first_line = true;
        for line in reader.lines() {
            if line.is_ok() {
                if is_first_line {
                    is_first_line = false;
                    continue;
                }
                let line = line.unwrap();
                let mut entry = TaskEntry::new();
                let mut tokens = line.split('\t');

                entry.id = tokens
                    .next()
                    .expect("Could not read task id!")
                    .parse()
                    .expect("Could not parse task id!");
                entry.project = Some(
                    tokens
                        .next()
                        .expect("Could not read task's project!")
                        .trim_matches(':')
                        .to_string(),
                );
                for tag in tokens
                    .next()
                    .expect("Can't read tags")
                    .trim_matches('+')
                    .split(TAGS_SEP)
                {
                    entry.tags.push(tag.to_string());
                }
                let due_token = tokens.next().expect("Could not read date");
                if due_token == "-" {
                    entry.due = None;
                } else {
                    entry.due = Some(
                        DateTime::parse_from_rfc3339(tokens.next().expect("Could not read date"))
                            .expect("Could not parse due date!")
                            .with_timezone(&Utc),
                    );
                }
                entry.title = String::from(tokens.next().expect("Could not read title"));
                entry.desc = Some(
                    tokens
                        .next()
                        .expect("Could not read description")
                        .to_string(),
                );
                self.entries.push(entry);
            }
        }
        Ok(())
    }

    /// Dumps the task entries to disk, at the path given on [`TaskDao::new()`]
    /// ```
    /// use task_wrestler::{TaskDao, TaskEntry};
    /// use std::path::Path;
    /// use std::fs::remove_file;
    /// use std::io::Read;
    /// use std::fs;
    /// let fname = "/tmp/persist_dao_test_write.txt";
    /// let mut dao = TaskDao::new(fname);
    /// let mut task = TaskEntry::default();
    /// task.id = 0;
    /// task.title = "testing".to_string();
    /// dao.entries.push(task);
    /// dao.dump().expect("Could not write to disk!");
    /// let mut buffer = fs::read_to_string(fname).unwrap();
    /// let mut lines = buffer.lines();
    /// let line = lines.next();
    /// assert!(line.is_some());
    /// assert_eq!(line.unwrap(), "id\tproject\ttags\tdue date\ttitle\tdescription");
    /// let line = lines.next();
    /// assert!(line.is_some());
    /// assert_eq!(line.unwrap(), "0\t:-\t+\t-\t\"testing\"\t\"\"");
    /// remove_file(fname);
    /// ```
    pub fn dump(&self) -> Result<(), io::Error> {
        let file = File::create(&self.path)?;
        let mut writer = LineWriter::new(file);
        writeln!(writer, "id\tproject\ttags\tdue date\ttitle\tdescription")
            .expect("Could not write to file!");
        for entry in &self.entries {
            writeln!(
                writer,
                "{}\t:{}\t+{}\t{}\t\"{}\"\t\"{}\"",
                entry.id,
                match &entry.project {
                    Some(s) => s,
                    None => "-",
                },
                entry.tags.join(TAGS_SEP),
                match entry.due {
                    Some(date) => date.to_rfc3339(),
                    None => "-".to_string(),
                },
                entry.title,
                match &entry.desc {
                    Some(s) => s,
                    None => "",
                }
            )
            .expect("Could not write to file!");
        }
        Ok(())
    }
}
