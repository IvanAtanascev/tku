use std::fs::File;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::Path;

use crate::structures::entry::Entry;

use super::generic::LoadData;
use super::generic::ParseData;
use super::generic::ReturnData;
use super::generic::WriteData;

pub struct CsvIoHandler {
    lines: Vec<String>,
    result: Vec<Entry>,
}

impl CsvIoHandler {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            result: Vec::new(),
        }
    }
}

impl LoadData for CsvIoHandler {
    fn load_from_file<P>(&mut self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let contents = read_to_string(path)?;

        self.lines = contents.lines().map(String::from).collect();

        Ok(())
    }
}

impl ParseData for CsvIoHandler {
    fn parse_data(&mut self) -> Result<(), String> {
        for line in &self.lines {
            let mut split_line = line.splitn(3, ';');
            let (Some(original), Some(translation), Some(description)) =
                (split_line.next(), split_line.next(), split_line.next())
            else {
                return Err("failed parsing data".into());
            };
            self.result.push(Entry {
                original: original.into(),
                translation: translation.into(),
                description: description.into(),
            })
        }

        Ok(())
    }
}

impl ReturnData for CsvIoHandler {
    fn get_data(&self) -> &Vec<Entry> {
        &self.result
    }
}

impl WriteData for CsvIoHandler {
    fn write_current_to_file<P>(&self, file_path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut file = File::create(file_path)?;

        for entry in &self.result {
            writeln!(
                file,
                "{};{};{}",
                entry.original, entry.translation, entry.description
            )?;
        }

        Ok(())
    }

    fn set_entries(&mut self, entries: Vec<Entry>) {
        self.result = entries;
    }
}
