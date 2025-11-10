use crate::{
    data_io::generic::{LoadData, ParseData, ReturnData, WriteData},
    structures::entry::Entry,
};

pub struct CardsLogic<T>
where
    T: LoadData + ParseData + ReturnData + WriteData,
{
    io_handler: T,
    current_entry_index: usize,
    current_entries: Vec<Entry>,
}

impl<T> CardsLogic<T>
where
    T: LoadData + ParseData + ReturnData + WriteData,
{
    pub fn new(io_handler: T) -> Self {
        Self {
            io_handler,
            current_entries: Vec::new(),
            current_entry_index: 0,
        }
    }

    pub fn init_entries(&mut self, path: String) {
        let _ = self.io_handler.load_from_file(path);
        let _ = self.io_handler.parse_data();
        self.current_entries = self.io_handler.get_data().clone();
    }

    pub fn match_current_translation(&self, user_string: &str) -> bool {
        if self.current_entries[self.current_entry_index].translation == user_string {
            return true;
        }

        return false;
    }

    pub fn increment_current_entry(&mut self) {
        self.current_entry_index = (self.current_entry_index + 1) % self.current_entries.len();
    }

    pub fn get_current_translation(&self) -> String {
        return self.current_entries[self.current_entry_index]
            .translation
            .clone();
    }

    pub fn get_current_original(&self) -> String {
        return self.current_entries[self.current_entry_index]
            .original
            .clone();
    }

    pub fn get_current_description(&self) -> String {
        return self.current_entries[self.current_entry_index]
            .description
            .clone();
    }
}
