use std::path::{Path, PathBuf};

use errors::*;

pub struct Tyon {
    path: PathBuf,
}

impl Tyon {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    /// Gets the current profile
    pub fn get_profile(&self) -> Result<u8> {
        Ok(Profile::read(&self.path)?.index + 1)
    }

    /// Sets the current profile
    pub fn set_profile(&self, index: u8) -> Result<()> {
        ensure!(index <= 5, "Profile {} is out of range", index);
        Profile::write(&self.path, &Profile::new(index - 1))
    }

    pub fn get_common_name<'a>() -> &'a str {
        "Tyon"
    }
}

impl_hidraw! {
    readwrite;
    Profile {
        @constant _report_id: u8 = 0x05,
        @constant _size: u8 = ::std::mem::size_of::<Self>() as u8,
        index: u8,
    }
}
