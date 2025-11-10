use std::{io, path::Path};

use crate::structures::entry::Entry;

pub trait LoadData {
    fn load_from_file<P>(&mut self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>;
}

pub trait WriteData {
    fn write_current_to_file<P>(&self, file_path: P) -> io::Result<()>
    where
        P: AsRef<Path>;

    fn set_entries(&mut self, entries: Vec<Entry>);
}

pub trait ParseData {
    fn parse_data(&mut self) -> Result<(), String>;
}

pub trait ReturnData {
    fn get_data(&self) -> &Vec<Entry>;
}
