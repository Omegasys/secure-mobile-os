use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub is_directory: bool,
    pub size_bytes: u64,
}

pub struct FileBrowser {
    current_directory: PathBuf,
}

impl FileBrowser {
    pub fn new() -> Self {
        Self {
            current_directory: PathBuf::from("/"),
        }
    }

    pub fn current_directory(
        &self,
    ) -> &Path {
        &self.current_directory
    }

    pub fn change_directory(
        &mut self,
        path: PathBuf,
    ) {
        self.current_directory = path;
    }

    pub fn list_directory(
        &self,
    ) -> Vec<FileEntry> {
        // Placeholder until VFS integration.
        vec![
            FileEntry {
                path: PathBuf::from(
                    "/documents",
                ),
                is_directory: true,
                size_bytes: 0,
            },
            FileEntry {
                path: PathBuf::from(
                    "/notes.txt",
                ),
                is_directory: false,
                size_bytes: 1024,
            },
        ]
    }

    pub fn open_file(
        &self,
        path: &Path,
    ) {
        println!(
            "Opening file: {}",
            path.display()
        );
    }

    pub fn create_directory(
        &self,
        name: &str,
    ) {
        println!(
            "Creating directory: {}",
            name
        );
    }
}
