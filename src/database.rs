use crate::utils::{check_db_file, get_db_file_path};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek, Write};

// Data record
pub struct Record {
    pub id: i32,
    pub content: String,
}

// Parse record line
impl From<&str> for Record {
    fn from(line: &str) -> Self {
        let fields: Vec<&str> = line.split(',').collect();
        // handle empty line
        if fields.len() == 1 {
            return Record {
                id: 0,
                content: "".to_string(),
            };
        }
        let content = fields[1..].join(",");
        Record {
            id: fields[0].parse::<i32>().unwrap(),
            content,
        }
    }
}

// Database
pub struct Database {
    pub file: File,
}

impl Database {
    // Open database file
    pub fn open() -> Database {
        check_db_file().unwrap();

        let db_file = get_db_file_path();

        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(db_file)
            .unwrap();
        Database { file }
    }

    // Add record
    pub fn add_record(&mut self, record: &Record) -> Result<(), io::Error> {
        let line = format!("{},{}", record.id, record.content);
        writeln!(self.file, "{}", line)
    }

    // Remove record
    pub fn remove_record(&mut self, id: i32) -> Result<(), io::Error> {
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();
        let line = lines.find(|(_, line)| {
            let record = Record::from(line.as_ref().unwrap().as_str());
            record.id == id
        });
        match line {
            Some((i, _)) => {
                let db_file = get_db_file_path();
                let contents = fs::read_to_string(db_file).unwrap();
                let new_contents = contents
                    .lines()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, line)| format!("{}\n", line))
                    .collect::<Vec<_>>()
                    .join("");
                self.file.seek(io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_contents.as_bytes()).unwrap();
                self.file.set_len(new_contents.len() as u64).unwrap();
                Ok(())
            }
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("No such record: {}", id),
            )),
        }
    }

    // Read records
    pub fn read_records(&self) -> Vec<Record> {
        let reader = BufReader::new(&self.file);
        reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| Record::from(line.as_str()))
            .collect()
    }
}
