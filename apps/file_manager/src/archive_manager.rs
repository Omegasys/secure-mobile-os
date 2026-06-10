use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone)]
pub enum ArchiveFormat {
    Zip,
    Tar,
    TarGz,
    SevenZip,
}

#[derive(Clone)]
pub struct ArchiveEntry {
    pub filename: String,
    pub size_bytes: u64,
}

#[derive(Clone)]
pub struct Archive {
    pub path: PathBuf,
    pub format: ArchiveFormat,
    pub entries: Vec<ArchiveEntry>,
}

pub struct ArchiveManager {
    archives: HashMap<PathBuf, Archive>,
}

impl ArchiveManager {
    pub fn new() -> Self {
        Self {
            archives: HashMap::new(),
        }
    }

    pub fn create_archive(
        &mut self,
        archive_path: PathBuf,
        format: ArchiveFormat,
    ) {
        self.archives.insert(
            archive_path.clone(),
            Archive {
                path: archive_path,
                format,
                entries: Vec::new(),
            },
        );
    }

    pub fn add_file(
        &mut self,
        archive_path: &PathBuf,
        filename: String,
        size_bytes: u64,
    ) {
        if let Some(archive) =
            self.archives.get_mut(archive_path)
        {
            archive.entries.push(
                ArchiveEntry {
                    filename,
                    size_bytes,
                }
            );
        }
    }

    pub fn list_contents(
        &self,
        archive_path: &PathBuf,
    ) -> Option<&[ArchiveEntry]> {
        self.archives
            .get(archive_path)
            .map(|a| a.entries.as_slice())
    }

    pub fn extract_archive(
        &self,
        archive_path: &PathBuf,
        destination: &PathBuf,
    ) {
        println!(
            "Extracting {:?} to {:?}",
            archive_path,
            destination
        );
    }

    pub fn delete_archive(
        &mut self,
        archive_path: &PathBuf,
    ) {
        self.archives.remove(archive_path);
    }

    pub fn archive_count(
        &self,
    ) -> usize {
        self.archives.len()
    }
}

impl Default for ArchiveManager {
    fn default() -> Self {
        Self::new()
    }
}
